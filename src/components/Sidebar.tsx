import { LayoutGrid, Settings, RefreshCw, Compass } from "lucide-react";
import React from "react";

interface SidebarProps {
  currentView: "library" | "settings" | "discover";
  onViewChange: (v: "library" | "settings" | "discover") => void;
  onRefresh: () => void;
}

const Sidebar: React.FC<SidebarProps> = ({ currentView, onViewChange, onRefresh }) => {
  return (
    <div
      style={{
        width: "250px",
        background: "var(--bg-sidebar)",
        display: "flex",
        flexDirection: "column",
        padding: "20px 0",
        borderRight: "1px solid var(--border-color)",
        transition: "background-color 0.3s, border-color 0.3s",
      }}
    >
      <div
        style={{
          padding: "0 20px",
          marginBottom: "30px",
          fontSize: "24px",
          fontWeight: "bold",
          color: "var(--text-primary)",
        }}
      >
        linuxy
      </div>

      <button
        style={{
          ...btnStyle,
          background: currentView === "library" ? "var(--accent-bg)" : "transparent",
          color: currentView === "library" ? "var(--accent-color)" : "var(--text-secondary)",
          borderRight: currentView === "library" ? "3px solid var(--accent-color)" : "3px solid transparent",
        }}
        onClick={() => onViewChange("library")}
      >
        <LayoutGrid size={18} style={{ marginRight: 10 }} /> Library
      </button>

      <button
        style={{
          ...btnStyle,
          background: currentView === "discover" ? "var(--accent-bg)" : "transparent",
          color: currentView === "discover" ? "var(--accent-color)" : "var(--text-secondary)",
          borderRight: currentView === "discover" ? "3px solid var(--accent-color)" : "3px solid transparent",
        }}
        onClick={() => onViewChange("discover")}
      >
        <Compass size={18} style={{ marginRight: 10 }} /> Discover
      </button>

      <button
        style={{
          ...btnStyle,
          background: currentView === "settings" ? "var(--accent-bg)" : "transparent",
          color: currentView === "settings" ? "var(--accent-color)" : "var(--text-secondary)",
          borderRight: currentView === "settings" ? "3px solid var(--accent-color)" : "3px solid transparent",
        }}
        onClick={() => onViewChange("settings")}
      >
        <Settings size={18} style={{ marginRight: 10 }} /> Settings
      </button>

      <div style={{ flex: 1 }} />

      <button style={{ ...btnStyle, background: "transparent", color: "var(--text-secondary)", borderRight: "3px solid transparent" }} onClick={onRefresh}>
        <RefreshCw size={18} style={{ marginRight: 10 }} /> Refresh
      </button>
    </div>
  );
};

const btnStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  padding: "10px 20px",
  border: "none",
  color: "var(--text-primary)",
  cursor: "pointer",
  textAlign: "left",
  fontSize: "16px",
  width: "100%",
  transition: "background 0.2s",
};

export default Sidebar;
