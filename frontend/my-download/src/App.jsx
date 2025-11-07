import { useState } from "react";

function App() {
  const [fileType, setFileType] = useState("csv");

  const handleDownload = async () => {
    const url = `http://127.0.0.1:3000/download/${fileType}`;

    try {
      const res = await fetch(url);
      if (!res.ok) throw new Error(`Server error ${res.status}`);

      const blob = await res.blob();
      const fileName = fileType === "csv" ? "data.csv" : "data.xlsx";

      // trigger browser download
      const link = document.createElement("a");
      link.href = URL.createObjectURL(blob);
      link.download = fileName;
      link.click();
      URL.revokeObjectURL(link.href);
    } catch (err) {
      alert("Download failed: " + err.message);
      console.error(err);
    }
  };

  return (
    <div style={{ textAlign: "center", marginTop: 50 }}>
      <h2>Download Signup Data</h2>

      <select
        value={fileType}
        onChange={(e) => setFileType(e.target.value)}
        style={{ marginRight: 10, padding: "6px 10px" }}
      >
        <option value="csv">CSV</option>
        <option value="xlsx">XLSX</option>
      </select>

      <button onClick={handleDownload} style={{ padding: "6px 12px" }}>
        Download File
      </button>
    </div>
  );
}

export default App;
