import { open } from "@tauri-apps/plugin-dialog";
import { Upload } from "lucide-react";
import React from "react";

interface DropZoneProps {
  onInstall: (paths: string[]) => void;
}

const DropZone: React.FC<DropZoneProps> = ({ onInstall }) => {
  const handleSelectFile = async () => {
    try {
      const selected = await open({
        multiple: true,
        filters: [
          { name: "AppImage", extensions: ["AppImage", "appimage"] },
          { name: "DEB Package", extensions: ["deb"] },
          { name: "RPM Package", extensions: ["rpm"] },
          { name: "Executable", extensions: ["sh", "bin", "run"] },
          { name: "All Files", extensions: ["*"] },
        ],
      });
      if (typeof selected === "string") {
        onInstall([selected]);
      } else if (Array.isArray(selected)) {
        onInstall(selected.filter((p): p is string => typeof p === "string"));
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
      <div>Drag and drop an .AppImage, .deb, .rpm, or executable file here, or click to browse</div>
    </div>
  );
};

export default DropZone;
