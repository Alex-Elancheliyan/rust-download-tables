use sqlx::PgPool;
use rust_xlsxwriter::{Workbook, Format, FormatAlign, FormatBorder};
use csv::Writer;
use printpdf::*;
use std::io::{BufWriter, Cursor};
use crate::repository::student_repo;

pub async fn generate_file(pool: &PgPool,file_type: &str,
) -> Result<(Vec<u8>, &'static str, String), String> {
    let signups = student_repo::get_all_students(pool)
        .await
        .map_err(|e| e.to_string())?;

    match file_type {
        "csv" => {
            let mut wtr = Writer::from_writer(vec![]);
            wtr.write_record(&["ID", "Name", "Email", "Mobile", "Created_At"])
                .map_err(|e| e.to_string())?;
            for s in signups {
                wtr.write_record(&[
                    s.id.to_string(),
                    s.name,
                    s.email,
                    s.mobile.clone().unwrap_or_default(),
                    s.created_at
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default(),
                ])
                .map_err(|e| e.to_string())?;
            }
            let data = wtr.into_inner().map_err(|e| e.to_string())?;
            Ok((data, "text/csv", "signup_data.csv".to_string()))
        }

        "xlsx" => {
            let mut workbook = Workbook::new();
            let worksheet = workbook.add_worksheet();

            let title_format = Format::new().set_bold().set_align(FormatAlign::Center)
                .set_font_size(14).set_background_color("#FFFF00").set_font_color("#000000");

            worksheet.merge_range(0, 0, 0, 4, "SIGN-UP USER DATA:", &title_format).map_err(|e| e.to_string())?;

            let header_format = Format::new().set_bold().set_border(FormatBorder::Thin).set_align(FormatAlign::Center)
                .set_background_color("#52BBDB").set_font_color("#000000");

            let headers = ["ID", "NAME", "EMAIL", "MOBILE", "CREATED AT"];
            for (i, header) in headers.iter().enumerate() {
                worksheet.write_string_with_format(1, i as u16, *header, &header_format).map_err(|e| e.to_string())?;
            }

            for (i, s) in signups.iter().enumerate() {
                let row = (i + 2) as u32;
                worksheet.write_number(row, 0, s.id as f64).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 1, &s.name).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 2, &s.email).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 3, s.mobile.as_deref().unwrap_or("")).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 4, &s.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default(),)
                                       .map_err(|e| e.to_string())?;
            }

            worksheet.set_column_width(0, 20).map_err(|e| e.to_string())?;
            worksheet.set_column_width(1, 25).map_err(|e| e.to_string())?;
            worksheet.set_column_width(2, 30).map_err(|e| e.to_string())?;
            worksheet.set_column_width(3, 20).map_err(|e| e.to_string())?;
            worksheet.set_column_width(4, 25).map_err(|e| e.to_string())?;

            let buffer = workbook.save_to_buffer().map_err(|e| e.to_string())?;
            Ok((
                buffer,
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                "signup_data.xlsx".to_string(),
            ))
        }

        "pdf" => {
            let ( doc, page1, layer1) =PdfDocument::new("Sign-up User Data", Mm(210.0), Mm(297.0), "Layer 1");
            let mut layer = doc.get_page(page1).get_layer(layer1);

            let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).map_err(|e| e.to_string())?;
            let font = doc.add_builtin_font(BuiltinFont::Helvetica).map_err(|e| e.to_string())?;

            let mut y = 270.0;
            let left_x = 20.0;
            let line_spacing = 8.0;
            let section_spacing = 12.0;

            layer.use_text("SIGN-UP USER DATA", 16.0, Mm(left_x), Mm(y), &font_bold);
            y -= 15.0;

            for (i, s) in signups.iter().enumerate() {
                if y < 40.0 {let (page, layer_id) = doc.add_page(Mm(210.0), Mm(297.0), &format!("Page {}", i + 2));
                    layer = doc.get_page(page).get_layer(layer_id);
                    y = 270.0;
                }

                let data = vec![
                    format!("ID: {}", s.id),
                    format!("Name: {}", s.name),
                    format!("Email: {}", s.email),
                    format!("Mobile: {}", s.mobile.clone().unwrap_or_default()),
                    format!("Created At: {}", s.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()
                    ),
                ];

                for line in data {
                    layer.use_text(line, 11.0, Mm(left_x), Mm(y), &font);
                    y -= line_spacing;
                }

                let points = vec![(Point::new(Mm(left_x), Mm(y)), false),(Point::new(Mm(190.0), Mm(y)), false),];

                let line = Line { points, is_closed: false };
                layer.set_outline_thickness(0.5);
                layer.set_outline_color(Color::Rgb(Rgb::new(0.2, 0.2, 0.2, None)));
                layer.add_line(line);

                y -= section_spacing;
            }

            let mut buffer = Cursor::new(Vec::<u8>::new());
            doc.save(&mut BufWriter::new(&mut buffer)).map_err(|e| e.to_string())?;

            Ok((
                buffer.into_inner(),
                "application/pdf",
                "signup_data.pdf".to_string(),
            ))
        }

        _ => Err("Invalid file type".into()),
    }
}
