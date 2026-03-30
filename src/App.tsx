import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { Moon, Sun, Monitor, FolderOpen, Shield, ShieldCheck, ShieldAlert } from "lucide-react";
import { useState, useEffect } from "react";

import AppGrid from "./components/AppGrid";
import DropZone from "./components/DropZone";
import Sidebar from "./components/Sidebar";
import Storefront from "./components/Storefront";

import "./styles/main.css";

export interface AppInfo {
  name: string;
  exec: string;
  icon: string | null;
  path: string;
  desktop_path: string;
  sandboxed: boolean;
  size_bytes: number;
  installed_at: number;
}

type Theme = "dark" | "light" | "system";

interface ModalAction {
  label: string;
  variant: "primary" | "danger" | "secondary";
  closeOnClick?: boolean;
  onClick?: () => Promise<void> | void;
}

interface AppModal {
  title: string;
  message: string;
  actions: ModalAction[];
}

function App() {
  const [apps, setApps] = useState<AppInfo[]>([]);
  const [view, setView] = useState<"library" | "settings" | "discover">("library");
  const [loading, setLoading] = useState(false);
  const [installProgress, setInstallProgress] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [modal, setModal] = useState<AppModal | null>(null);
  const [modalBusy, setModalBusy] = useState(false);
  const [theme, setTheme] = useState<Theme>((localStorage.getItem("theme") as Theme) || "dark");
  const [stats, setStorageStats] = useState<{ total_size_bytes: number; app_count: number } | null>(
    null
  );
  const [firejailInstalled, setFirejailInstalled] = useState<boolean>(false);
  const [updateToolInstalled, setUpdateToolInstalled] = useState<boolean>(false);

  const loadApps = async () => {
    try {
      setLoading(true);
      const appList = await invoke<AppInfo[]>("get_installed_apps");
      setApps(appList);

      const storageStats = await invoke<{ total_size_bytes: number; app_count: number }>(
        "get_storage_stats"
      );
      setStorageStats(storageStats);

      // Check if firejail is installed
      const isInstalled = await invoke<boolean>("is_firejail_installed");
      setFirejailInstalled(isInstalled);

      const isUpdateToolAvailable = await invoke<boolean>("is_update_tool_installed");
      setUpdateToolInstalled(isUpdateToolAvailable);

      setError(null);
    } catch (err) {
      console.error(err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadApps();
  }, []);

  useEffect(() => {
    // Apply theme to document
    const root = document.documentElement;
    if (theme === "system") {
      const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      root.setAttribute("data-theme", isDark ? "dark" : "light");
    } else {
      root.setAttribute("data-theme", theme);
    }
    localStorage.setItem("theme", theme);
  }, [theme]);

  const showInfoModal = (title: string, message: string) => {
    setModal({
      title,
      message,
      actions: [{ label: "OK", variant: "primary" }],
    });
  };

  const installAppImage = async (path: string) => {
    try {
      setLoading(true);
      setInstallProgress("Initializing...");
      await invoke("install_appimage", { path });
      await loadApps();
      return true;
    } catch (err) {
      console.error(err);
      setError(String(err));
      setInstallProgress(null);
      return false;
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const unlistenProgress = listen<string>("install-progress", (event) => {
      setInstallProgress(event.payload);
      if (event.payload === "Done") {
        setTimeout(() => setInstallProgress(null), 2000);
      }
    });

    const unlistenDetected = listen<string>("appimage-detected", (event) => {
      const fullPath = event.payload;
      const fileName = fullPath.split("/").pop() || fullPath;
      setModal({
        title: "Install Detected AppImage",
        message: `New AppImage detected:\n${fileName}\n\nInstall it to your library?`,
        actions: [
          { label: "Later", variant: "secondary" },
          {
            label: "Install",
            variant: "primary",
            closeOnClick: false,
            onClick: async () => {
              const installed = await installAppImage(fullPath);
              if (installed) {
                setModal(null);
              }
            },
          },
        ],
      });
    });

    const unlisten = listen<string[]>("tauri://file-drop", async (event) => {
      const files = event.payload;
      if (files && files.length > 0) {
        const file = files[0];
        if (file.toLowerCase().endsWith(".appimage")) {
          await installAppImage(file);
        } else {
          setError("Only .AppImage files are supported for drag and drop.");
        }
      }
    });
    return () => {
      unlisten.then((f) => f());
      unlistenProgress.then((f) => f());
      unlistenDetected.then((f) => f());
    };
  }, []);

  const launchApp = async (path: string) => {
    try {
      await invoke("launch_app", { path });
    } catch (err) {
      setError(String(err));
    }
  };

  const removeApp = async (path: string) => {
    try {
      await invoke("remove_app", { path });
      await loadApps();
      return true;
    } catch (err) {
      setError(String(err));
      return false;
    }
  };

  const toggleSandbox = async (desktop_path: string, enable: boolean) => {
    try {
      if (enable && !firejailInstalled) {
        setError(
          "Firejail is not installed. Please install firejail first (e.g., sudo apt install firejail)."
        );
        return;
      }
      await invoke("toggle_sandbox", { desktopPath: desktop_path, enable });
      await loadApps();
    } catch (err) {
      setError(String(err));
    }
  };

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  const updateAppImage = async (path: string) => {
    try {
      const appName = apps.find((app) => app.path === path)?.name || "This AppImage";

      if (!updateToolInstalled) {
        showInfoModal(
          "Updates Require appimageupdatetool",
          `${appName} can only use delta updates when appimageupdatetool is installed on your system.`
        );
        return;
      }

      const isUpdateAvailable = await invoke<boolean>("check_for_update", { path });
      if (isUpdateAvailable) {
        setModal({
          title: "Update Available",
          message: `${appName} has an update available.\n\nDownload and apply it now?`,
          actions: [
            { label: "Later", variant: "secondary" },
            {
              label: "Update",
              variant: "primary",
              closeOnClick: false,
              onClick: async () => {
                await invoke("apply_update", { path });
                await loadApps();
                setModal(null);
              },
            },
          ],
        });
      } else {
        showInfoModal("No Update Available", `${appName} is already up to date.`);
      }
    } catch (err) {
      setError(String(err));
      setInstallProgress(null);
    }
  };

  const requestRemoveApp = (app: AppInfo) => {
    setModal({
      title: "Remove AppImage",
      message: `Remove ${app.name} from your library?\n\nThis deletes the AppImage file and its desktop integration.`,
      actions: [
        { label: "Cancel", variant: "secondary" },
        {
          label: "Delete",
          variant: "danger",
          closeOnClick: false,
          onClick: async () => {
            const removed = await removeApp(app.path);
            if (removed) {
              setModal(null);
            }
          },
        },
      ],
    });
  };

  const handleModalAction = async (action: ModalAction) => {
    if (modalBusy) {
      return;
    }

    if (!action.onClick) {
      setModal(null);
      return;
    }

    setModalBusy(true);
    try {
      await action.onClick();
      if (action.closeOnClick !== false) {
        setModal(null);
      }
    } finally {
      setModalBusy(false);
    }
  };

  return (
    <div style={{ display: "flex", width: "100%", height: "100%", background: "var(--bg-main)" }}>
      <Sidebar currentView={view} onViewChange={setView} onRefresh={loadApps} />
      <div style={{ flex: 1, padding: "20px", overflowY: "auto", position: "relative" }}>
        {error && (
          <div
            style={{
              background: "var(--danger-color)",
              color: "#fff",
              padding: "10px",
              marginBottom: "10px",
              borderRadius: "4px",
            }}
          >
            {error}{" "}
            <button
              onClick={() => setError(null)}
              style={{
                background: "none",
                border: "none",
                color: "#fff",
                cursor: "pointer",
                fontWeight: "bold",
              }}
            >
              X
            </button>
          </div>
        )}
        {installProgress && (
          <div
            style={{
              position: "absolute",
              top: 20,
              right: 20,
              background: "var(--accent-color)",
              color: "#fff",
              padding: "10px 20px",
              borderRadius: "8px",
              fontWeight: "bold",
              zIndex: 10,
              boxShadow: "0 4px 6px rgba(0,0,0,0.3)",
            }}
          >
            {installProgress}
          </div>
        )}
        {loading && !installProgress && (
          <div style={{ position: "absolute", top: 20, right: 20, color: "var(--text-muted)" }}>
            Loading...
          </div>
        )}

        {view === "library" && (
          <>
            <h2 style={{ color: "var(--text-primary)" }}>AppImage Library</h2>
            <DropZone onInstall={installAppImage} />
            <h3 style={{ color: "var(--text-secondary)", marginTop: "30px" }}>Installed Apps</h3>
            <AppGrid
              apps={apps}
              onLaunch={launchApp}
              onRemove={requestRemoveApp}
              onToggleSandbox={toggleSandbox}
              onUpdate={updateAppImage}
              updatesEnabled={updateToolInstalled}
            />
          </>
        )}

        {view === "discover" && <Storefront />}

        {view === "settings" && (
          <div style={{ maxWidth: "800px" }}>
            <h2 style={{ color: "var(--text-primary)", marginBottom: "30px" }}>Settings</h2>

            {/* Appearance Section */}
            <div style={settingsSectionStyle}>
              <div
                style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}
              >
                <Sun size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Appearance</h3>
              </div>
              <div style={{ display: "flex", gap: "15px" }}>
                <ThemeButton
                  active={theme === "dark"}
                  onClick={() => setTheme("dark")}
                  icon={<Moon size={18} />}
                  label="Dark"
                />
                <ThemeButton
                  active={theme === "light"}
                  onClick={() => setTheme("light")}
                  icon={<Sun size={18} />}
                  label="Light"
                />
                <ThemeButton
                  active={theme === "system"}
                  onClick={() => setTheme("system")}
                  icon={<Monitor size={18} />}
                  label="System"
                />
              </div>
            </div>

            {/* Storage Section */}
            <div style={settingsSectionStyle}>
              <div
                style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}
              >
                <FolderOpen size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Library Storage</h3>
              </div>

              {stats && (
                <div style={{ display: "flex", gap: "20px", marginBottom: "20px" }}>
                  <div
                    style={{
                      flex: 1,
                      background: "var(--bg-input)",
                      padding: "15px",
                      borderRadius: "8px",
                      border: "1px solid var(--border-color)",
                    }}
                  >
                    <div
                      style={{
                        color: "var(--text-muted)",
                        fontSize: "12px",
                        textTransform: "uppercase",
                        fontWeight: "bold",
                      }}
                    >
                      Total Disk Usage
                    </div>
                    <div
                      style={{
                        color: "var(--text-primary)",
                        fontSize: "24px",
                        fontWeight: "bold",
                        marginTop: "5px",
                      }}
                    >
                      {formatBytes(stats.total_size_bytes)}
                    </div>
                  </div>
                  <div
                    style={{
                      flex: 1,
                      background: "var(--bg-input)",
                      padding: "15px",
                      borderRadius: "8px",
                      border: "1px solid var(--border-color)",
                    }}
                  >
                    <div
                      style={{
                        color: "var(--text-muted)",
                        fontSize: "12px",
                        textTransform: "uppercase",
                        fontWeight: "bold",
                      }}
                    >
                      Installed Apps
                    </div>
                    <div
                      style={{
                        color: "var(--text-primary)",
                        fontSize: "24px",
                        fontWeight: "bold",
                        marginTop: "5px",
                      }}
                    >
                      {stats.app_count}
                    </div>
                  </div>
                </div>
              )}

              <div
                style={{
                  background: "var(--bg-input)",
                  padding: "15px",
                  borderRadius: "6px",
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                }}
              >
                <div>
                  <div style={{ color: "var(--text-primary)", fontWeight: "500" }}>
                    Primary Directory
                  </div>
                  <div style={{ color: "var(--text-muted)", fontSize: "13px", marginTop: "4px" }}>
                    ~/.local/appimages/
                  </div>
                </div>
                <button
                  onClick={() => invoke("open_directory", { dir_name: "appimages" })}
                  style={{
                    background: "var(--accent-color)",
                    color: "#fff",
                    border: "none",
                    padding: "8px 15px",
                    borderRadius: "4px",
                    cursor: "pointer",
                    fontWeight: "bold",
                  }}
                >
                  Open Folder
                </button>
              </div>
            </div>

            {/* Security Section */}
            <div style={settingsSectionStyle}>
              <div
                style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}
              >
                <Shield size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Security & Sandbox</h3>
              </div>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: "1.5" }}>
                Linuxy uses <strong>Firejail</strong> to provide one-click sandboxing for AppImages.
                When enabled, apps are restricted from accessing your personal files unless
                explicitly permitted.
              </p>
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "10px",
                  color: firejailInstalled ? "var(--accent-color)" : "var(--danger-color)",
                  fontSize: "13px",
                  marginTop: "10px",
                  padding: "10px",
                  background: firejailInstalled
                    ? "rgba(76, 175, 80, 0.1)"
                    : "rgba(244, 67, 54, 0.1)",
                  borderRadius: "6px",
                }}
              >
                {firejailInstalled ? <ShieldCheck size={16} /> : <ShieldAlert size={16} />}
                <span>
                  {firejailInstalled
                    ? "Firejail is installed and ready to use."
                    : "Firejail is NOT installed. Install it to use sandboxing features."}
                </span>
              </div>
              {!firejailInstalled && (
                <div style={{ marginTop: "10px", fontSize: "13px", color: "var(--text-muted)" }}>
                  Install with:{" "}
                  <code
                    style={{
                      background: "var(--bg-input)",
                      padding: "2px 6px",
                      borderRadius: "4px",
                    }}
                  >
                    sudo apt install firejail
                  </code>{" "}
                  (Debian/Ubuntu) or your distro&apos;s package manager.
                </div>
              )}
            </div>

            <div style={settingsSectionStyle}>
              <div
                style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}
              >
                <Monitor size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Updates</h3>
              </div>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: "1.5" }}>
                Linuxy uses <strong>appimageupdatetool</strong> for delta updates on AppImages that
                publish update metadata.
              </p>
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "10px",
                  color: updateToolInstalled ? "var(--accent-color)" : "var(--danger-color)",
                  fontSize: "13px",
                  marginTop: "10px",
                  padding: "10px",
                  background: updateToolInstalled
                    ? "rgba(76, 175, 80, 0.1)"
                    : "rgba(244, 67, 54, 0.1)",
                  borderRadius: "6px",
                }}
              >
                {updateToolInstalled ? <ShieldCheck size={16} /> : <ShieldAlert size={16} />}
                <span>
                  {updateToolInstalled
                    ? "appimageupdatetool is installed."
                    : "appimageupdatetool is not installed. Update actions are disabled."}
                </span>
              </div>
            </div>

            {/* About Section */}
            <div
              style={{
                marginTop: "40px",
                borderTop: "1px solid var(--border-color)",
                paddingTop: "20px",
                textAlign: "center",
                color: "var(--text-muted)",
                fontSize: "12px",
              }}
            >
              linuxy v1.0.0 • Built with Rust & Tauri
            </div>
          </div>
        )}

        {modal && (
          <div
            style={{
              position: "fixed",
              inset: 0,
              background: "rgba(0, 0, 0, 0.45)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              zIndex: 20,
              padding: "24px",
            }}
            onClick={() => !modalBusy && setModal(null)}
          >
            <div
              style={{
                width: "min(420px, 100%)",
                background: "var(--bg-card)",
                border: "1px solid var(--border-color)",
                borderRadius: "14px",
                padding: "24px",
                boxShadow: "0 18px 45px rgba(0, 0, 0, 0.28)",
              }}
              onClick={(event) => event.stopPropagation()}
            >
              <h3 style={{ marginTop: 0, marginBottom: "10px", color: "var(--text-primary)" }}>
                {modal.title}
              </h3>
              <p
                style={{
                  margin: 0,
                  color: "var(--text-secondary)",
                  lineHeight: "1.6",
                  whiteSpace: "pre-line",
                }}
              >
                {modal.message}
              </p>
              <div
                style={{
                  display: "flex",
                  justifyContent: "flex-end",
                  gap: "12px",
                  marginTop: "24px",
                }}
              >
                {modal.actions.map((action) => (
                  <button
                    key={action.label}
                    onClick={() => handleModalAction(action)}
                    disabled={modalBusy}
                    style={{
                      background: getModalButtonBackground(action.variant),
                      color: action.variant === "secondary" ? "var(--text-primary)" : "#fff",
                      border:
                        action.variant === "secondary" ? "1px solid var(--border-color)" : "none",
                      padding: "10px 16px",
                      borderRadius: "8px",
                      cursor: modalBusy ? "not-allowed" : "pointer",
                      fontWeight: action.variant === "secondary" ? "normal" : "bold",
                      opacity: modalBusy ? 0.7 : 1,
                    }}
                  >
                    {action.label}
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

const settingsSectionStyle: React.CSSProperties = {
  background: "var(--bg-card)",
  padding: "25px",
  borderRadius: "12px",
  border: "1px solid var(--border-color)",
  marginBottom: "20px",
  transition: "background-color 0.3s, border-color 0.3s",
};

const getModalButtonBackground = (variant: ModalAction["variant"]) => {
  if (variant === "danger") {
    return "var(--danger-color)";
  }
  if (variant === "primary") {
    return "var(--accent-color)";
  }
  return "var(--bg-input)";
};

const ThemeButton = ({
  active,
  onClick,
  icon,
  label,
}: {
  active: boolean;
  onClick: () => void;
  icon: React.ReactNode;
  label: string;
}) => (
  <button
    onClick={onClick}
    style={{
      flex: 1,
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      gap: "10px",
      padding: "15px",
      background: active ? "var(--accent-color)" : "var(--bg-input)",
      color: active ? "#fff" : "var(--text-primary)",
      border: "1px solid " + (active ? "var(--accent-color)" : "var(--border-color)"),
      borderRadius: "8px",
      cursor: "pointer",
      transition: "all 0.2s",
    }}
  >
    {icon}
    <span style={{ fontSize: "14px", fontWeight: "500" }}>{label}</span>
  </button>
);

export default App;
