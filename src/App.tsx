import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import {
  Moon,
  Sun,
  Monitor,
  FolderOpen,
  Shield,
  ShieldCheck,
  ShieldAlert,
  Search,
  X,
  RefreshCw,
  Save,
  Upload,
} from "lucide-react";
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
  categories: string[];
  package_type: string;
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
  const [searchQuery, setSearchQuery] = useState("");
  const [checkingUpdates, setCheckingUpdates] = useState(false);
  const [cleanupStats, setCleanupStats] = useState<{
    orphaned_icons: number;
    orphaned_desktops: number;
    temp_files: number;
    reclaimable_bytes: number;
  } | null>(null);
  const [analyzingCleanup, setAnalyzingCleanup] = useState(false);

  const loadApps = async () => {
    try {
      setLoading(true);
      const appList = await invoke<AppInfo[]>("get_installed_apps");
      setApps(appList);

      const storageStats = await invoke<{ total_size_bytes: number; app_count: number }>(
        "get_storage_stats"
      );
      setStorageStats(storageStats);

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
      setInstallProgress("Initializing...");
      await invoke("install_appimage", { path });
      return true;
    } catch (err) {
      console.error(err);
      setError(String(err));
      return false;
    }
  };

  const installDeb = async (path: string) => {
    try {
      setInstallProgress("Initializing DEB installation...");
      await invoke("install_deb", { path });
      return true;
    } catch (err) {
      console.error(err);
      setError(String(err));
      return false;
    }
  };

  const installRpm = async (path: string) => {
    try {
      setInstallProgress("Initializing RPM installation...");
      await invoke("install_rpm", { path });
      return true;
    } catch (err) {
      console.error(err);
      setError(String(err));
      return false;
    }
  };

  const installExecutable = async (path: string) => {
    try {
      setInstallProgress("Initializing Executable installation...");
      await invoke("install_executable", { path });
      return true;
    } catch (err) {
      console.error(err);
      setError(String(err));
      return false;
    }
  };

  const handleInstall = async (paths: string[]) => {
    try {
      setLoading(true);
      for (let i = 0; i < paths.length; i++) {
        const path = paths[i];
        const prefix = paths.length > 1 ? `[${i + 1}/${paths.length}] ` : "";
        setInstallProgress(`${prefix}Installing ${path.split("/").pop() || path}...`);
        if (path.toLowerCase().endsWith(".appimage")) {
          await installAppImage(path);
        } else if (path.toLowerCase().endsWith(".deb")) {
          await installDeb(path);
        } else if (path.toLowerCase().endsWith(".rpm")) {
          await installRpm(path);
        } else {
          await installExecutable(path);
        }
      }
      await loadApps();
    } catch (err) {
      console.error(err);
      setError(String(err));
    } finally {
      setLoading(false);
      setInstallProgress(null);
    }
  };

  const checkAllUpdates = async () => {
    try {
      setCheckingUpdates(true);
      const results =
        await invoke<{ path: string; name: string; has_update: boolean; error: string | null }[]>(
          "check_all_updates"
        );

      const available = results.filter((r) => r.has_update);
      const errors = results.filter((r) => r.error);

      if (available.length > 0) {
        setModal({
          title: "Updates Available",
          message: `${available.length} app(s) have updates:\n${available.map((r) => `• ${r.name}`).join("\n")}${errors.length > 0 ? `\n\n${errors.length} app(s) had errors.` : ""}`,
          actions: [
            { label: "Close", variant: "secondary" },
            {
              label: "Update All",
              variant: "primary",
              closeOnClick: false,
              onClick: async () => {
                for (const r of available) {
                  setInstallProgress(`Updating ${r.name}...`);
                  await invoke("apply_update", { path: r.path });
                }
                setInstallProgress(null);
                await loadApps();
                setModal(null);
              },
            },
          ],
        });
      } else if (errors.length > 0) {
        setModal({
          title: "Update Check Complete",
          message: `No updates found, but ${errors.length} app(s) had errors:\n${errors.map((r) => `• ${r.name}: ${r.error}`).join("\n")}`,
          actions: [{ label: "OK", variant: "primary" }],
        });
      } else {
        setModal({
          title: "All Up to Date",
          message: "All installed apps are up to date.",
          actions: [{ label: "OK", variant: "primary" }],
        });
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setCheckingUpdates(false);
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
      const ignored = JSON.parse(localStorage.getItem("ignoredAppImages") || "[]");
      if (ignored.includes(fullPath)) return;

      const fileName = fullPath.split("/").pop() || fullPath;
      setModal({
        title: "Install Detected AppImage",
        message: `New AppImage detected:\n${fileName}\n\nInstall it to your library?`,
        actions: [
          {
            label: "Later",
            variant: "secondary",
            onClick: () => {
              ignored.push(fullPath);
              localStorage.setItem("ignoredAppImages", JSON.stringify(ignored));
            },
          },
          {
            label: "Install",
            variant: "primary",
            closeOnClick: false,
            onClick: async () => {
              setLoading(true);
              if (fullPath.toLowerCase().endsWith(".appimage")) {
                await installAppImage(fullPath);
              } else if (fullPath.toLowerCase().endsWith(".deb")) {
                await installDeb(fullPath);
              } else if (fullPath.toLowerCase().endsWith(".rpm")) {
                await installRpm(fullPath);
              } else {
                await installExecutable(fullPath);
              }
              await loadApps();
              setLoading(false);
              setModal(null);
            },
          },
        ],
      });
    });

    const unlisten = listen<string[]>("tauri://file-drop", async (event) => {
      const files = event.payload;
      if (files && files.length > 0) {
        await handleInstall(files);
      }
    });
    return () => {
      unlisten.then((f) => f());
      unlistenProgress.then((f) => f());
      unlistenDetected.then((f) => f());
    };
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const isCtrl = e.ctrlKey || e.metaKey;

      if (e.key === "Escape" && modal) {
        e.preventDefault();
        setModal(null);
        return;
      }

      if (isCtrl && e.key === "k") {
        e.preventDefault();
        if (view === "library" && apps.length > 0) {
          const input = document.querySelector<HTMLInputElement>(
            'input[placeholder="Search installed apps..."]'
          );
          input?.focus();
        }
        return;
      }

      if (isCtrl && e.key === "u" && updateToolInstalled) {
        e.preventDefault();
        checkAllUpdates();
        return;
      }

      if (isCtrl && e.key === "r") {
        e.preventDefault();
        loadApps();
        return;
      }

      if (isCtrl && e.key === "d" && view === "library") {
        e.preventDefault();
        setView("discover");
        return;
      }

      if (isCtrl && e.key === "s") {
        e.preventDefault();
        setView("settings");
        return;
      }

      if (isCtrl && e.key === "l") {
        e.preventDefault();
        setView("library");
        return;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [view, modal, updateToolInstalled, apps]);

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
      const app = apps.find((app) => app.path === path);
      const appName = app?.name || "This App";

      if (app?.package_type !== "AppImage") {
        showInfoModal(
          "Updates Not Supported",
          `${appName} is not an AppImage and does not support delta updates via appimageupdatetool.`
        );
        return;
      }

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
      title: "Remove App",
      message: `Remove ${app.name} from your library?\n\nThis deletes the app file and its desktop integration.`,
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
            <h2 style={{ color: "var(--text-primary)" }}>App Library</h2>
            <DropZone onInstall={handleInstall} />
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
                marginTop: "30px",
              }}
            >
              <h3 style={{ color: "var(--text-secondary)", margin: 0 }}>Installed Apps</h3>
              {apps.length > 0 && updateToolInstalled && (
                <button
                  onClick={checkAllUpdates}
                  disabled={checkingUpdates}
                  style={{
                    background: "var(--info-color)",
                    color: "#fff",
                    border: "none",
                    padding: "8px 16px",
                    borderRadius: "6px",
                    cursor: checkingUpdates ? "not-allowed" : "pointer",
                    display: "flex",
                    alignItems: "center",
                    gap: "6px",
                    fontSize: "13px",
                    fontWeight: 600,
                    opacity: checkingUpdates ? 0.6 : 1,
                  }}
                >
                  <RefreshCw
                    size={16}
                    style={{ animation: checkingUpdates ? "spin 1s linear infinite" : "none" }}
                  />
                  {checkingUpdates ? "Checking..." : "Check All Updates"}
                </button>
              )}
            </div>
            {apps.length > 0 && (
              <div style={{ position: "relative", marginTop: "15px", maxWidth: "400px" }}>
                <Search
                  size={18}
                  style={{
                    position: "absolute",
                    left: "12px",
                    top: "50%",
                    transform: "translateY(-50%)",
                    color: "var(--text-muted)",
                    pointerEvents: "none",
                  }}
                />
                <input
                  type="text"
                  placeholder="Search installed apps..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  style={{
                    width: "100%",
                    padding: "10px 36px 10px 40px",
                    borderRadius: "8px",
                    border: "1px solid var(--border-color)",
                    background: "var(--bg-input)",
                    color: "var(--text-primary)",
                    fontSize: "14px",
                    outline: "none",
                  }}
                />
                {searchQuery && (
                  <button
                    onClick={() => setSearchQuery("")}
                    style={{
                      position: "absolute",
                      right: "8px",
                      top: "50%",
                      transform: "translateY(-50%)",
                      background: "none",
                      border: "none",
                      cursor: "pointer",
                      color: "var(--text-muted)",
                      padding: "4px",
                      display: "flex",
                      alignItems: "center",
                    }}
                  >
                    <X size={16} />
                  </button>
                )}
              </div>
            )}
            <AppGrid
              apps={apps.filter((app) =>
                app.name.toLowerCase().includes(searchQuery.toLowerCase())
              )}
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

            {/* Appearance */}
            <div style={settingsSectionStyle}>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
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

            {/* Storage */}
            <div style={settingsSectionStyle}>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                <FolderOpen size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Library Storage</h3>
              </div>
              {stats && (
                <div style={{ display: "flex", gap: "20px", marginBottom: "20px" }}>
                  <div style={{ flex: 1, background: "var(--bg-input)", padding: "15px", borderRadius: "8px", border: "1px solid var(--border-color)" }}>
                    <div style={{ color: "var(--text-muted)", fontSize: "12px", textTransform: "uppercase", fontWeight: "bold" }}>
                      Total Disk Usage
                    </div>
                    <div style={{ color: "var(--text-primary)", fontSize: "24px", fontWeight: "bold", marginTop: "5px" }}>
                      {formatBytes(stats.total_size_bytes)}
                    </div>
                  </div>
                  <div style={{ flex: 1, background: "var(--bg-input)", padding: "15px", borderRadius: "8px", border: "1px solid var(--border-color)" }}>
                    <div style={{ color: "var(--text-muted)", fontSize: "12px", textTransform: "uppercase", fontWeight: "bold" }}>
                      Installed Apps
                    </div>
                    <div style={{ color: "var(--text-primary)", fontSize: "24px", fontWeight: "bold", marginTop: "5px" }}>
                      {stats.app_count}
                    </div>
                  </div>
                </div>
              )}
              <div style={{ background: "var(--bg-input)", padding: "15px", borderRadius: "6px", display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                <div>
                  <div style={{ color: "var(--text-primary)", fontWeight: "500" }}>Primary Directory</div>
                  <div style={{ color: "var(--text-muted)", fontSize: "13px", marginTop: "4px" }}>~/.local/appimages/</div>
                </div>
                <button onClick={() => invoke("open_directory", { dirName: "appimages" })} style={{ background: "var(--accent-color)", color: "#fff", border: "none", padding: "8px 15px", borderRadius: "4px", cursor: "pointer", fontWeight: "bold" }}>
                  Open Folder
                </button>
              </div>
            </div>

            {/* Cleanup */}
            <div style={settingsSectionStyle}>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                <FolderOpen size={20} color="var(--danger-color)" />
                <h3 style={{ margin: 0 }}>Storage Cleanup</h3>
              </div>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: "1.5" }}>
                Find and remove orphaned icons, desktop entries, and leftover temp files from failed installations.
              </p>
              {cleanupStats && cleanupStats.reclaimable_bytes > 0 && (
                <div style={{ background: "var(--bg-input)", padding: "15px", borderRadius: "6px", marginBottom: "15px" }}>
                  <div style={{ color: "var(--text-primary)", fontWeight: 500, marginBottom: "8px" }}>
                    Found {cleanupStats.orphaned_icons} orphaned icons, {cleanupStats.orphaned_desktops} orphaned desktops, and {cleanupStats.temp_files} temp files
                  </div>
                  <div style={{ color: "var(--danger-color)", fontWeight: 600 }}>
                    {formatBytes(cleanupStats.reclaimable_bytes)} can be reclaimed
                  </div>
                </div>
              )}
              {cleanupStats && cleanupStats.reclaimable_bytes === 0 && (
                <div style={{ background: "var(--accent-bg)", padding: "15px", borderRadius: "6px", marginBottom: "15px", color: "var(--accent-color)" }}>
                  No cleanup needed. Everything looks clean!
                </div>
              )}
              <div style={{ display: "flex", gap: "10px" }}>
                <button onClick={async () => { setAnalyzingCleanup(true); try { const s = await invoke<typeof cleanupStats>("analyze_storage"); setCleanupStats(s); } catch (err) { setError(String(err)); } finally { setAnalyzingCleanup(false); } }} disabled={analyzingCleanup} style={{ background: "var(--bg-input)", color: "var(--text-primary)", border: "1px solid var(--border-color)", padding: "8px 15px", borderRadius: "6px", cursor: analyzingCleanup ? "not-allowed" : "pointer", fontWeight: 500, fontSize: "13px" }}>
                  {analyzingCleanup ? "Analyzing..." : "Analyze"}
                </button>
                {cleanupStats && cleanupStats.reclaimable_bytes > 0 && (
                  <button onClick={async () => { try { await invoke("cleanup_storage"); showInfoModal("Cleanup Complete", "Orphaned files and temp files have been removed."); setCleanupStats(null); } catch (err) { setError(String(err)); } }} style={{ background: "var(--danger-color)", color: "#fff", border: "none", padding: "8px 15px", borderRadius: "6px", cursor: "pointer", fontWeight: 600, fontSize: "13px" }}>
                    Clean Up Now
                  </button>
                )}
              </div>
            </div>

            {/* Security */}
            <div style={settingsSectionStyle}>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                <Shield size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Security & Sandbox</h3>
              </div>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: "1.5" }}>
                Linuxy uses <strong>Firejail</strong> to provide one-click sandboxing for AppImages. When enabled, apps are restricted from accessing your personal files unless explicitly permitted.
              </p>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", color: firejailInstalled ? "var(--accent-color)" : "var(--danger-color)", fontSize: "13px", marginTop: "10px", padding: "10px", background: firejailInstalled ? "var(--accent-bg)" : "var(--danger-bg)", borderRadius: "6px" }}>
                {firejailInstalled ? <ShieldCheck size={16} /> : <ShieldAlert size={16} />}
                <span>{firejailInstalled ? "Firejail is installed and ready to use." : "Firejail is NOT installed. Install it to use sandboxing features."}</span>
              </div>
            </div>

            {/* Updates */}
            <div style={settingsSectionStyle}>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                <Monitor size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Updates</h3>
              </div>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: "1.5" }}>
                Linuxy uses <strong>appimageupdatetool</strong> for delta updates on AppImages that publish update metadata.
              </p>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", color: updateToolInstalled ? "var(--accent-color)" : "var(--danger-color)", fontSize: "13px", marginTop: "10px", padding: "10px", background: updateToolInstalled ? "var(--accent-bg)" : "var(--danger-bg)", borderRadius: "6px" }}>
                {updateToolInstalled ? <ShieldCheck size={16} /> : <ShieldAlert size={16} />}
                <span>{updateToolInstalled ? "appimageupdatetool is installed." : "appimageupdatetool is not installed. Update actions are disabled."}</span>
              </div>
            </div>

            {/* Backup */}
            <div style={settingsSectionStyle}>
              <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                <Save size={20} color="var(--accent-color)" />
                <h3 style={{ margin: 0 }}>Backup & Restore</h3>
              </div>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: "1.5" }}>
                Export your app library as a JSON backup file, or restore from a previous backup.
              </p>
              <div style={{ display: "flex", gap: "10px", marginTop: "15px" }}>
                <button
                  onClick={async () => {
                    try {
                      const { save } = await import("@tauri-apps/plugin-dialog");
                      const selected = await save({
                        filters: [{ name: "JSON", extensions: ["json"] }],
                        defaultPath: `linuxy-backup-${new Date().toISOString().split("T")[0]}.json`,
                      });
                      if (typeof selected === "string") {
                        await invoke("export_library", { backupPath: selected });
                        showInfoModal("Backup Complete", `Library exported to:\n${selected}`);
                      }
                    } catch (err) {
                      setError(String(err));
                    }
                  }}
                  style={{ flex: 1, background: "var(--accent-color)", color: "#fff", border: "none", padding: "10px 15px", borderRadius: "6px", cursor: "pointer", fontWeight: 600, fontSize: "13px", display: "flex", alignItems: "center", justifyContent: "center", gap: "8px" }}
                >
                  <Save size={16} /> Export Library
                </button>
                <button
                  onClick={async () => {
                    try {
                      const { open } = await import("@tauri-apps/plugin-dialog");
                      const selected = await open({
                        multiple: false,
                        filters: [{ name: "JSON", extensions: ["json"] }],
                      });
                      if (typeof selected === "string") {
                        const result = await invoke<string>("import_library", { backupPath: selected });
                        showInfoModal("Restore Complete", result);
                        await loadApps();
                      }
                    } catch (err) {
                      setError(String(err));
                    }
                  }}
                  style={{ flex: 1, background: "var(--bg-input)", color: "var(--text-primary)", border: "1px solid var(--border-color)", padding: "10px 15px", borderRadius: "6px", cursor: "pointer", fontWeight: 600, fontSize: "13px", display: "flex", alignItems: "center", justifyContent: "center", gap: "8px" }}
                >
                  <Upload size={16} /> Import Library
                </button>
              </div>
            </div>

            {/* About */}
            <div style={{ marginTop: "40px", borderTop: "1px solid var(--border-color)", paddingTop: "20px", textAlign: "center", color: "var(--text-muted)", fontSize: "12px" }}>
              linuxy v2.0.0 • Built with Rust & Tauri 2
            </div>
          </div>
        )}

        {modal && (
          <div style={{ position: "fixed", inset: 0, background: "rgba(0, 0, 0, 0.45)", display: "flex", alignItems: "center", justifyContent: "center", zIndex: 20, padding: "24px" }} onClick={() => !modalBusy && setModal(null)}>
            <div style={{ width: "min(420px, 100%)", background: "var(--bg-card)", border: "1px solid var(--border-color)", borderRadius: "14px", padding: "24px", boxShadow: "0 18px 45px rgba(0, 0, 0, 0.28)" }} onClick={(event) => event.stopPropagation()}>
              <h3 style={{ marginTop: 0, marginBottom: "10px", color: "var(--text-primary)" }}>{modal.title}</h3>
              <p style={{ margin: 0, color: "var(--text-secondary)", lineHeight: "1.6", whiteSpace: "pre-line" }}>{modal.message}</p>
              <div style={{ display: "flex", justifyContent: "flex-end", gap: "12px", marginTop: "24px" }}>
                {modal.actions.map((action) => (
                  <button
                    key={action.label}
                    onClick={() => handleModalAction(action)}
                    disabled={modalBusy}
                    style={{
                      background: getModalButtonBackground(action.variant),
                      color: action.variant === "secondary" ? "var(--text-primary)" : "#fff",
                      border: action.variant === "secondary" ? "1px solid var(--border-color)" : "none",
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
  if (variant === "danger") return "var(--danger-color)";
  if (variant === "primary") return "var(--accent-color)";
  return "var(--bg-input)";
};

const ThemeButton = ({
  active, onClick, icon, label,
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
