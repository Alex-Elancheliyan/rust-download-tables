use sqlx::PgPool;
use rust_xlsxwriter::{Workbook, Format, FormatAlign, FormatBorder};
use crate::repository::student_repo;
use csv::Writer;

pub async fn generate_file(
    pool: &PgPool,
    file_type: &str,
) -> Result<(Vec<u8>, &'static str, String), String> {

    let signups = student_repo::get_all_students(pool)
        .await
        .map_err(|e| e.to_string())?;

    match file_type {
        "csv" => {
            let mut wtr = Writer::from_writer(vec![]);
            wtr.write_record(&["ID", "Name", "Email", "Mobile", "Created_At"]).map_err(|e| e.to_string())?;

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

            let title_format = Format::new()
                .set_bold()
                .set_align(FormatAlign::Center)    
                .set_font_size(14)
                .set_background_color("#FFFF00")  
                .set_font_color("#000000");        

           
            worksheet.merge_range(0, 0, 0, 4, "SIGN-UP USER DATA:", &title_format)
                .map_err(|e| e.to_string())?;

          
            let header_format = Format::new()
                .set_bold()
                .set_border(FormatBorder::Thin)
                .set_align(FormatAlign::Center)
                .set_background_color("#52BBDB") 
                .set_font_color("#FFFFFF");  

            let headers = ["ID", "NAME", "EMAIL", "MOBILE", "CREATED AT"];
            for (i, header) in headers.iter().enumerate() {
                worksheet.write_string_with_format(1, i as u16, *header, &header_format)
                    .map_err(|e| e.to_string())?;
            }

            for (i, s) in signups.iter().enumerate() {
                let row = (i + 2) as u32; // +2 because title + header
                worksheet.write_number(row, 0, s.id as f64).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 1, &s.name).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 2, &s.email).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 3, s.mobile.as_deref().unwrap_or("")).map_err(|e| e.to_string())?;
                worksheet.write_string(row, 4,
                    &s.created_at
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default()
                ).map_err(|e| e.to_string())?;
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

        _ => Err("Invalid file type".into()),
    }
}
