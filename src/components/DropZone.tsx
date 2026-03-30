import { open } from "@tauri-apps/api/dialog";
import { Upload } from "lucide-react";
import React from "react";

interface DropZoneProps {
  onInstall: (path: string) => void;
}

const DropZone: React.FC<DropZoneProps> = ({ onInstall }) => {
  const handleSelectFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "AppImage", extensions: ["AppImage", "appimage"] }],
      });
      if (typeof selected === "string") {
        onInstall(selected);
      }
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div
      style={{
        border: "2px dashed var(--border-color)",
        borderRadius: "8px",
        padding: "30px",
        textAlign: "center",
        cursor: "pointer",
        marginBottom: "20px",
        background: "var(--bg-input)",
        color: "var(--text-secondary)",
        transition: "background-color 0.3s, border-color 0.3s",
      }}
      onClick={handleSelectFile}
    >
      <Upload size={32} style={{ marginBottom: "10px", color: "var(--text-muted)" }} />
      <div>Drag and drop an .AppImage here, or click to browse</div>
    </div>
  );
};

export default DropZone;
