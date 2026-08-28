import { LayoutGrid, Settings, RefreshCw, Compass, LogOut, HelpCircle } from "lucide-react";
import React from "react";

interface SidebarProps {
  currentView: "library" | "settings" | "discover";
  onViewChange: (v: "library" | "settings" | "discover") => void;
  onRefresh: () => void;
  onQuit: () => void;
  onOpenShortcuts: () => void;
  appCount?: number;
}

const Sidebar: React.FC<SidebarProps> = ({
  currentView,
  onViewChange,
  onRefresh,
  onQuit,
  onOpenShortcuts,
  appCount = 0,
}) => {
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
        userSelect: "none",
      }}
    >
      <div
        style={{
          padding: "0 20px",
          marginBottom: "28px",
          display: "flex",
          alignItems: "center",
          gap: "10px",
        }}
      >
        <div
          style={{
            width: "32px",
            height: "32px",
            background: "var(--accent-color)",
            borderRadius: "8px",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            fontWeight: "bold",
            color: "#fff",
            fontSize: "18px",
            boxShadow: "var(--shadow-glow)",
          }}
        >
          🐧
        </div>
        <div
          style={{
            fontSize: "22px",
            fontWeight: "bold",
            color: "var(--text-primary)",
            letterSpacing: "-0.5px",
          }}
        >
          linuxy
        </div>
        <span
          style={{
            fontSize: "10px",
            padding: "2px 6px",
            borderRadius: "10px",
            background: "var(--accent-bg)",
            color: "var(--accent-color)",
            fontWeight: "bold",
          }}
        >
          v2.0
        </span>
      </div>

      <button
        style={{
          ...btnStyle,
          background: currentView === "library" ? "var(--accent-bg)" : "transparent",
          color: currentView === "library" ? "var(--accent-color)" : "var(--text-secondary)",
          borderRight:
            currentView === "library" ? "3px solid var(--accent-color)" : "3px solid transparent",
          fontWeight: currentView === "library" ? 600 : "normal",
        }}
        onClick={() => onViewChange("library")}
      >
        <LayoutGrid size={18} style={{ marginRight: 10 }} />
        <span>Library</span>
        {appCount > 0 && (
          <span
            style={{
              marginLeft: "auto",
              fontSize: "11px",
              background: "var(--bg-input)",
              color: "var(--text-muted)",
              padding: "2px 7px",
              borderRadius: "10px",
              fontWeight: "bold",
            }}
          >
            {appCount}
          </span>
        )}
      </button>

      <button
        style={{
          ...btnStyle,
          background: currentView === "discover" ? "var(--accent-bg)" : "transparent",
          color: currentView === "discover" ? "var(--accent-color)" : "var(--text-secondary)",
          borderRight:
            currentView === "discover" ? "3px solid var(--accent-color)" : "3px solid transparent",
          fontWeight: currentView === "discover" ? 600 : "normal",
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
          borderRight:
            currentView === "settings" ? "3px solid var(--accent-color)" : "3px solid transparent",
          fontWeight: currentView === "settings" ? 600 : "normal",
        }}
        onClick={() => onViewChange("settings")}
      >
        <Settings size={18} style={{ marginRight: 10 }} /> Settings
      </button>

      <div style={{ flex: 1 }} />

      <button
        style={{
          ...btnStyle,
          background: "transparent",
          color: "var(--text-secondary)",
          borderRight: "3px solid transparent",
        }}
        onClick={onOpenShortcuts}
        title="Keyboard Shortcuts (?)"
      >
        <HelpCircle size={18} style={{ marginRight: 10 }} /> Shortcuts
      </button>

      <button
        style={{
          ...btnStyle,
          background: "transparent",
          color: "var(--text-secondary)",
          borderRight: "3px solid transparent",
        }}
        onClick={onRefresh}
        title="Refresh library (Ctrl+R)"
      >
        <RefreshCw size={18} style={{ marginRight: 10 }} /> Refresh
      </button>

      <button
        style={{
          ...btnStyle,
          background: "transparent",
          color: "var(--danger-color)",
          borderRight: "3px solid transparent",
        }}
        onClick={onQuit}
        title="Quit Linuxy (Ctrl+Q)"
      >
        <LogOut size={18} style={{ marginRight: 10 }} /> Quit
      </button>
    </div>
  );
};

const btnStyle: React.CSSProperties = {
  display: "flex",
  alignItems: "center",
  padding: "11px 20px",
  border: "none",
  color: "var(--text-primary)",
  cursor: "pointer",
  textAlign: "left",
  fontSize: "15px",
  width: "100%",
  transition: "all 0.2s ease",
};

export default Sidebar;
