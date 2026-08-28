import { CheckCircle2, AlertCircle, Info, X } from "lucide-react";
import React, { useEffect } from "react";

export interface ToastMessage {
  id: string;
  type: "success" | "error" | "info";
  title: string;
  message?: string;
}

interface ToastProps {
  toasts: ToastMessage[];
  onDismiss: (id: string) => void;
}

const ToastContainer: React.FC<ToastProps> = ({ toasts, onDismiss }) => {
  return (
    <div
      style={{
        position: "fixed",
        bottom: 24,
        right: 24,
        zIndex: 50,
        display: "flex",
        flexDirection: "column",
        gap: "10px",
        pointerEvents: "none",
        maxWidth: "380px",
        width: "100%",
      }}
    >
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onDismiss={onDismiss} />
      ))}
    </div>
  );
};

const ToastItem: React.FC<{ toast: ToastMessage; onDismiss: (id: string) => void }> = ({
  toast,
  onDismiss,
}) => {
  useEffect(() => {
    const timer = setTimeout(() => {
      onDismiss(toast.id);
    }, 4500);
    return () => clearTimeout(timer);
  }, [toast.id, onDismiss]);

  const getIcon = () => {
    switch (toast.type) {
      case "success":
        return <CheckCircle2 size={18} color="var(--accent-color)" />;
      case "error":
        return <AlertCircle size={18} color="var(--danger-color)" />;
      default:
        return <Info size={18} color="var(--info-color)" />;
    }
  };

  const getBorderColor = () => {
    switch (toast.type) {
      case "success":
        return "var(--accent-color)";
      case "error":
        return "var(--danger-color)";
      default:
        return "var(--info-color)";
    }
  };

  return (
    <div
      className="animate-fade-in"
      style={{
        pointerEvents: "auto",
        background: "var(--bg-card)",
        border: `1px solid var(--border-color)`,
        borderLeft: `4px solid ${getBorderColor()}`,
        borderRadius: "10px",
        padding: "12px 16px",
        boxShadow: "var(--shadow-card)",
        display: "flex",
        alignItems: "flex-start",
        gap: "12px",
      }}
    >
      <div style={{ marginTop: "2px" }}>{getIcon()}</div>
      <div style={{ flex: 1 }}>
        <div style={{ fontWeight: 600, fontSize: "14px", color: "var(--text-primary)" }}>
          {toast.title}
        </div>
        {toast.message && (
          <div style={{ fontSize: "12px", color: "var(--text-secondary)", marginTop: "2px" }}>
            {toast.message}
          </div>
        )}
      </div>
      <button
        onClick={() => onDismiss(toast.id)}
        style={{
          background: "none",
          border: "none",
          color: "var(--text-muted)",
          cursor: "pointer",
          padding: "2px",
          display: "flex",
          alignItems: "center",
        }}
        aria-label="Dismiss notification"
      >
        <X size={14} />
      </button>
    </div>
  );
};

export default ToastContainer;
