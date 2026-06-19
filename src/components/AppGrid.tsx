import { convertFileSrc } from "@tauri-apps/api/core";
import { Play, Trash2, ShieldAlert, ShieldCheck, RefreshCw } from "lucide-react";
import React from "react";

import type { AppInfo } from "../App";

interface AppGridProps {
  apps: AppInfo[];
  onLaunch: (path: string) => void;
  onRemove: (app: AppInfo) => void;
  onToggleSandbox: (desktop_path: string, enable: boolean) => void;
  onUpdate: (path: string) => void;
  updatesEnabled: boolean;
}

const AppGrid: React.FC<AppGridProps> = ({
  apps,
  onLaunch,
  onRemove,
  onToggleSandbox,
  onUpdate,
  updatesEnabled,
}) => {
  const formatBytes = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString();
  };

  if (apps.length === 0) {
    return (
      <div style={{ color: "var(--text-muted)", marginTop: "20px" }}>
        No apps installed yet. Drag and drop one to install!
      </div>
    );
  }

  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: "repeat(auto-fill, minmax(220px, 1fr))",
        gap: "20px",
        marginTop: "20px",
      }}
    >
      {apps.map((app) => (
        <div
          key={app.path}
          style={{
            background: "var(--bg-card)",
            borderRadius: "12px",
            padding: "20px 15px",
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            position: "relative",
            border: "1px solid var(--border-color)",
            boxShadow: "var(--shadow-card)",
            transition: "all 0.3s cubic-bezier(0.4, 0, 0.2, 1)",
          }}
        >
          <button
            title={app.sandboxed ? "Sandboxed via Firejail" : "Not sandboxed"}
            onClick={() => onToggleSandbox(app.desktop_path, !app.sandboxed)}
            style={{
              position: "absolute",
              top: 10,
              right: 10,
              background: "transparent",
              border: "none",
              cursor: "pointer",
              color: app.sandboxed ? "var(--accent-color)" : "var(--text-muted)",
            }}
          >
            {app.sandboxed ? <ShieldCheck size={20} /> : <ShieldAlert size={20} />}
          </button>

          {app.icon ? (
            <img
              src={convertFileSrc(app.icon)}
              alt={app.name}
              style={{ width: "64px", height: "64px", marginBottom: "15px", objectFit: "contain" }}
            />
          ) : (
            <div
              style={{
                width: "64px",
                height: "64px",
                background: "var(--border-color)",
                borderRadius: "12px",
                marginBottom: "15px",
              }}
            />
          )}
          <div
            style={{
              fontWeight: "bold",
              textAlign: "center",
              marginBottom: "5px",
              wordBreak: "break-all",
              color: "var(--text-primary)",
            }}
          >
            {app.name}
          </div>

          <div
            style={{
              fontSize: "9px",
              padding: "2px 8px",
              borderRadius: "10px",
              background:
                app.package_type === "AppImage"
                  ? "var(--accent-bg)"
                  : "var(--info-bg, hsla(210, 100%, 65%, 0.15))",
              color:
                app.package_type === "AppImage"
                  ? "var(--accent-color)"
                  : "var(--info-color, hsl(210, 100%, 65%))",
              fontWeight: 600,
              marginBottom: "8px",
            }}
          >
            {app.package_type}
          </div>

          <div style={{ fontSize: "11px", color: "var(--text-muted)", marginBottom: "10px" }}>
            {formatBytes(app.size_bytes)} • {formatDate(app.installed_at)}
          </div>

          {app.categories.length > 0 && (
            <div
              style={{
                display: "flex",
                flexWrap: "wrap",
                gap: "4px",
                marginBottom: "10px",
                justifyContent: "center",
              }}
            >
              {app.categories.slice(0, 3).map((cat) => (
                <span
                  key={cat}
                  style={{
                    fontSize: "9px",
                    padding: "2px 6px",
                    borderRadius: "4px",
                    background: "var(--accent-bg)",
                    color: "var(--accent-color)",
                    fontWeight: 500,
                  }}
                >
                  {cat}
                </span>
              ))}
              {app.categories.length > 3 && (
                <span
                  style={{
                    fontSize: "9px",
                    padding: "2px 6px",
                    borderRadius: "4px",
                    background: "var(--bg-input)",
                    color: "var(--text-muted)",
                  }}
                >
                  +{app.categories.length - 3}
                </span>
              )}
            </div>
          )}

          <div
            style={{
              display: "flex",
              flexWrap: "wrap",
              gap: "10px",
              marginTop: "auto",
              paddingTop: "15px",
              justifyContent: "center",
            }}
          >
            <button
              onClick={() => onLaunch(app.path)}
              style={{
                background: "var(--accent-color)",
                color: "#fff",
                border: "none",
                padding: "8px",
                borderRadius: "4px",
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
              }}
            >
              <Play size={16} /> <span style={{ marginLeft: "5px", fontSize: "12px" }}>Launch</span>
            </button>
            <button
              onClick={() => onUpdate(app.path)}
              disabled={!updatesEnabled}
              style={{
                background: "var(--info-color)",
                color: "#fff",
                border: "none",
                padding: "8px",
                borderRadius: "4px",
                cursor: updatesEnabled ? "pointer" : "not-allowed",
                display: "flex",
                alignItems: "center",
                opacity: updatesEnabled ? 1 : 0.55,
              }}
              title={
                updatesEnabled
                  ? "Check for updates"
                  : "Install appimageupdatetool to enable updates"
              }
            >
              <RefreshCw size={16} />
            </button>
            <button
              onClick={() => onRemove(app)}
              style={{
                background: "var(--danger-color)",
                color: "#fff",
                border: "none",
                padding: "8px",
                borderRadius: "4px",
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
              }}
            >
              <Trash2 size={16} />
            </button>
          </div>
        </div>
      ))}
    </div>
  );
};

export default AppGrid;
