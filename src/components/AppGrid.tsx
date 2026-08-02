import { convertFileSrc } from "@tauri-apps/api/core";
import { Play, Trash2, ShieldAlert, ShieldCheck, RefreshCw, Info, ArrowUpDown } from "lucide-react";
import React, { useState, useMemo, useEffect } from "react";

import type { AppInfo } from "../App";
import { formatBytes } from "../utils/formatBytes";

interface AppGridProps {
  apps: AppInfo[];
  onLaunch: (path: string) => void;
  onRemove: (app: AppInfo) => void;
  onToggleSandbox: (desktop_path: string, enable: boolean) => void;
  onUpdate: (path: string) => void;
  onSelectApp: (app: AppInfo) => void;
  updatesEnabled: boolean;
}

type SortField = "name" | "size" | "date";
type PackageFilter = "ALL" | "AppImage" | "DEB" | "RPM" | "Executable";

const AppGrid: React.FC<AppGridProps> = ({
  apps,
  onLaunch,
  onRemove,
  onToggleSandbox,
  onUpdate,
  onSelectApp,
  updatesEnabled,
}) => {
  const [filterType, setFilterType] = useState<PackageFilter>("ALL");
  const [sortField, setSortField] = useState<SortField>("name");
  const [sortAsc, setSortAsc] = useState<boolean>(true);
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    app: AppInfo;
  } | null>(null);

  useEffect(() => {
    const handleCloseMenu = () => setContextMenu(null);
    window.addEventListener("click", handleCloseMenu);
    window.addEventListener("scroll", handleCloseMenu, true);
    return () => {
      window.removeEventListener("click", handleCloseMenu);
      window.removeEventListener("scroll", handleCloseMenu, true);
    };
  }, []);

  const formatDate = (timestamp: number) => {
    if (!timestamp) return "Unknown";
    return new Date(timestamp * 1000).toLocaleDateString();
  };

  const filteredApps = useMemo(() => {
    return apps.filter((app) => {
      if (filterType === "ALL") return true;
      return app.package_type.toLowerCase() === filterType.toLowerCase();
    });
  }, [apps, filterType]);

  const sortedApps = useMemo(() => {
    return [...filteredApps].sort((a, b) => {
      let result = 0;
      if (sortField === "name") {
        result = a.name.localeCompare(b.name);
      } else if (sortField === "size") {
        result = a.size_bytes - b.size_bytes;
      } else if (sortField === "date") {
        result = a.installed_at - b.installed_at;
      }
      return sortAsc ? result : -result;
    });
  }, [filteredApps, sortField, sortAsc]);

  if (apps.length === 0) {
    return (
      <div
        style={{
          color: "var(--text-muted)",
          marginTop: "40px",
          textAlign: "center",
          padding: "40px",
          background: "var(--bg-card)",
          borderRadius: "12px",
          border: "1px dashed var(--border-color)",
        }}
      >
        <div style={{ fontSize: "16px", fontWeight: 600, color: "var(--text-primary)" }}>
          No applications installed yet
        </div>
        <div style={{ fontSize: "13px", marginTop: "8px" }}>
          Drag and drop an .AppImage, .deb, .rpm, or binary above, or browse the Storefront!
        </div>
      </div>
    );
  }

  return (
    <div style={{ marginTop: "20px" }}>
      {/* Controls Bar: Type Filter & Sorting */}
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          flexWrap: "wrap",
          gap: "10px",
          marginBottom: "18px",
          background: "var(--bg-card)",
          padding: "10px 16px",
          borderRadius: "10px",
          border: "1px solid var(--border-color)",
        }}
      >
        <div style={{ display: "flex", gap: "6px", flexWrap: "wrap" }}>
          {(["ALL", "AppImage", "DEB", "RPM", "Executable"] as PackageFilter[]).map((type) => (
            <button
              key={type}
              onClick={() => setFilterType(type)}
              style={{
                background: filterType === type ? "var(--accent-color)" : "var(--bg-input)",
                color: filterType === type ? "#fff" : "var(--text-secondary)",
                border: "none",
                padding: "6px 12px",
                borderRadius: "6px",
                fontSize: "12px",
                fontWeight: 600,
                cursor: "pointer",
                transition: "all 0.2s",
              }}
            >
              {type}
            </button>
          ))}
        </div>

        <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
          <ArrowUpDown size={14} color="var(--text-muted)" />
          <span style={{ fontSize: "12px", color: "var(--text-muted)", fontWeight: 500 }}>
            Sort by:
          </span>
          <select
            value={sortField}
            onChange={(e) => setSortField(e.target.value as SortField)}
            style={{
              background: "var(--bg-input)",
              color: "var(--text-primary)",
              border: "1px solid var(--border-color)",
              padding: "5px 10px",
              borderRadius: "6px",
              fontSize: "12px",
              outline: "none",
              cursor: "pointer",
            }}
          >
            <option value="name">Name</option>
            <option value="size">Size</option>
            <option value="date">Date Installed</option>
          </select>
          <button
            onClick={() => setSortAsc(!sortAsc)}
            title="Toggle Sort Direction"
            style={{
              background: "var(--bg-input)",
              color: "var(--text-primary)",
              border: "1px solid var(--border-color)",
              padding: "5px 10px",
              borderRadius: "6px",
              fontSize: "12px",
              cursor: "pointer",
            }}
          >
            {sortAsc ? "▲ Asc" : "▼ Desc"}
          </button>
        </div>
      </div>

      {/* Grid */}
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill, minmax(220px, 1fr))",
          gap: "20px",
        }}
      >
        {sortedApps.map((app) => (
          <div
            key={app.path}
            className="app-card"
            style={{
              padding: "20px 15px",
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              position: "relative",
            }}
            onContextMenu={(e) => {
              e.preventDefault();
              setContextMenu({ x: e.clientX, y: e.clientY, app });
            }}
          >
            {/* Top Badge Action Buttons */}
            <div style={{ position: "absolute", top: 10, right: 10, display: "flex", gap: "4px" }}>
              <button
                title="View details & integrity"
                aria-label={`View details for ${app.name}`}
                onClick={() => onSelectApp(app)}
                style={{
                  background: "transparent",
                  border: "none",
                  cursor: "pointer",
                  color: "var(--text-muted)",
                  padding: "2px",
                }}
              >
                <Info size={18} />
              </button>
              <button
                title={app.sandboxed ? "Sandboxed via Firejail" : "Not sandboxed"}
                aria-label={app.sandboxed ? "Disable sandbox" : "Enable sandbox"}
                onClick={() => onToggleSandbox(app.desktop_path, !app.sandboxed)}
                style={{
                  background: "transparent",
                  border: "none",
                  cursor: "pointer",
                  color: app.sandboxed ? "var(--accent-color)" : "var(--text-muted)",
                  padding: "2px",
                }}
              >
                {app.sandboxed ? <ShieldCheck size={18} /> : <ShieldAlert size={18} />}
              </button>
            </div>

            {app.icon ? (
              <img
                src={convertFileSrc(app.icon)}
                alt={app.name}
                style={{
                  width: "64px",
                  height: "64px",
                  marginBottom: "14px",
                  objectFit: "contain",
                  borderRadius: "12px",
                  cursor: "pointer",
                }}
                onClick={() => onSelectApp(app)}
              />
            ) : (
              <div
                style={{
                  width: "64px",
                  height: "64px",
                  background: "var(--bg-input)",
                  borderRadius: "14px",
                  marginBottom: "14px",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  cursor: "pointer",
                }}
                onClick={() => onSelectApp(app)}
              >
                <Info size={24} color="var(--text-muted)" />
              </div>
            )}

            <div
              onClick={() => onSelectApp(app)}
              style={{
                fontWeight: "bold",
                textAlign: "center",
                marginBottom: "6px",
                wordBreak: "break-all",
                color: "var(--text-primary)",
                fontSize: "15px",
                cursor: "pointer",
              }}
            >
              {app.name}
            </div>

            <div
              style={{
                fontSize: "9px",
                padding: "2px 8px",
                borderRadius: "10px",
                background: app.package_type === "AppImage" ? "var(--accent-bg)" : "var(--info-bg)",
                color:
                  app.package_type === "AppImage" ? "var(--accent-color)" : "var(--info-color)",
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
                  marginBottom: "12px",
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
                gap: "8px",
                marginTop: "auto",
                paddingTop: "12px",
                justifyContent: "center",
                width: "100%",
              }}
            >
              <button
                onClick={() => onLaunch(app.path)}
                style={{
                  flex: 1,
                  background: "var(--accent-color)",
                  color: "#fff",
                  border: "none",
                  padding: "8px",
                  borderRadius: "6px",
                  cursor: "pointer",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  fontWeight: 600,
                  fontSize: "12px",
                }}
              >
                <Play size={14} style={{ marginRight: 4 }} /> Launch
              </button>
              <button
                onClick={() => onUpdate(app.path)}
                disabled={!updatesEnabled}
                aria-label={`Check updates for ${app.name}`}
                style={{
                  background: "var(--info-color)",
                  color: "#fff",
                  border: "none",
                  padding: "8px 10px",
                  borderRadius: "6px",
                  cursor: updatesEnabled ? "pointer" : "not-allowed",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  opacity: updatesEnabled ? 1 : 0.55,
                }}
                title={
                  updatesEnabled
                    ? "Check for updates"
                    : "Install appimageupdatetool to enable updates"
                }
              >
                <RefreshCw size={14} />
              </button>
              <button
                onClick={() => onRemove(app)}
                aria-label={`Remove ${app.name}`}
                style={{
                  background: "var(--danger-color)",
                  color: "#fff",
                  border: "none",
                  padding: "8px 10px",
                  borderRadius: "6px",
                  cursor: "pointer",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
                title="Remove application"
              >
                <Trash2 size={14} />
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* Right-Click Context Menu */}
      {contextMenu && (
        <div
          style={{
            position: "fixed",
            top: contextMenu.y,
            left: contextMenu.x,
            zIndex: 40,
            background: "var(--bg-card)",
            border: "1px solid var(--border-color)",
            borderRadius: "8px",
            boxShadow: "var(--shadow-card)",
            padding: "6px 0",
            minWidth: "160px",
          }}
          onClick={(e) => e.stopPropagation()}
        >
          <button
            style={contextMenuItemStyle}
            onClick={() => {
              onLaunch(contextMenu.app.path);
              setContextMenu(null);
            }}
          >
            <Play size={14} style={{ marginRight: 8 }} /> Launch
          </button>
          <button
            style={contextMenuItemStyle}
            onClick={() => {
              onSelectApp(contextMenu.app);
              setContextMenu(null);
            }}
          >
            <Info size={14} style={{ marginRight: 8 }} /> View Details & Hash
          </button>
          <button
            style={contextMenuItemStyle}
            onClick={() => {
              onToggleSandbox(contextMenu.app.desktop_path, !contextMenu.app.sandboxed);
              setContextMenu(null);
            }}
          >
            {contextMenu.app.sandboxed ? (
              <ShieldAlert size={14} style={{ marginRight: 8 }} />
            ) : (
              <ShieldCheck size={14} style={{ marginRight: 8 }} />
            )}
            {contextMenu.app.sandboxed ? "Disable Sandbox" : "Enable Sandbox"}
          </button>
          <button
            style={{
              ...contextMenuItemStyle,
              opacity: updatesEnabled ? 1 : 0.5,
              cursor: updatesEnabled ? "pointer" : "not-allowed",
            }}
            disabled={!updatesEnabled}
            onClick={() => {
              if (updatesEnabled) {
                onUpdate(contextMenu.app.path);
                setContextMenu(null);
              }
            }}
          >
            <RefreshCw size={14} style={{ marginRight: 8 }} /> Check Update
          </button>
          <div style={{ height: "1px", background: "var(--border-color)", margin: "4px 0" }} />
          <button
            style={{ ...contextMenuItemStyle, color: "var(--danger-color)" }}
            onClick={() => {
              onRemove(contextMenu.app);
              setContextMenu(null);
            }}
          >
            <Trash2 size={14} style={{ marginRight: 8 }} /> Remove Application
          </button>
        </div>
      )}
    </div>
  );
};

const contextMenuItemStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  width: "100%",
  padding: "8px 14px",
  background: "none",
  border: "none",
  color: "var(--text-primary)",
  fontSize: "13px",
  cursor: "pointer",
  textAlign: "left",
};

export default AppGrid;
