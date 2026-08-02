import { invoke } from "@tauri-apps/api/core";
import { Download, Search, Github, Globe, CheckCircle } from "lucide-react";
import React, { useState, useEffect } from "react";

interface StoreApp {
  name: string;
  description: string;
  authors?: { name: string }[];
  links?: { type: string; url: string }[];
  icons?: string[];
  github_url?: string;
  stargazers_count?: number;
  categories?: string[];
}

interface StorefrontProps {
  installedAppNames?: string[];
  onRefreshLibrary?: () => void;
}

const Storefront: React.FC<StorefrontProps> = ({ installedAppNames = [], onRefreshLibrary }) => {
  const [apps, setApps] = useState<StoreApp[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [source, setSource] = useState<"appimagehub" | "github">("appimagehub");
  const [downloadingName, setDownloadingName] = useState<string | null>(null);

  useEffect(() => {
    fetchData();
  }, [source]);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      const controller = new AbortController();
      const timeout = setTimeout(() => controller.abort(), 15000);
      if (source === "appimagehub") {
        const response = await fetch("https://appimage.github.io/feed.json", {
          signal: controller.signal,
        });
        if (!response.ok) throw new Error("Failed to fetch AppImageHub data");
        const data = await response.json();
        setApps(data.items || []);
      } else {
        const response = await fetch(
          "https://api.github.com/search/repositories?q=topic:appimage&sort=stars&order=desc&per_page=100",
          { signal: controller.signal }
        );
        if (!response.ok) throw new Error("Failed to fetch GitHub data");
        const data = await response.json();
        const githubApps = data.items.map((item: any) => ({
          name: item.name,
          description: item.description,
          authors: [{ name: item.owner.login }],
          links: [{ type: "GitHub", url: item.html_url }],
          github_url: item.html_url,
          stargazers_count: item.stargazers_count,
          icons: [item.owner.avatar_url],
        }));
        setApps(githubApps);
      }
      clearTimeout(timeout);
    } catch (err: any) {
      console.error(err);
      if (err?.name === "AbortError") {
        setError("Request timed out. Check your internet connection.");
      } else {
        setError(String(err));
      }
    } finally {
      setLoading(false);
    }
  };

  const isInstalled = (appName: string) => {
    const lowerName = appName.toLowerCase().trim();
    return installedAppNames.some(
      (n) => n.toLowerCase().trim() === lowerName || lowerName.includes(n.toLowerCase().trim())
    );
  };

  const filteredApps = apps.filter(
    (app) =>
      app.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      (app.description && app.description.toLowerCase().includes(searchTerm.toLowerCase()))
  );

  return (
    <div style={{ paddingBottom: "40px" }} className="animate-fade-in">
      <div
        style={{
          position: "sticky",
          top: "-24px",
          zIndex: 10,
          background: "var(--bg-glass)",
          backdropFilter: "blur(12px)",
          WebkitBackdropFilter: "blur(12px)",
          borderBottom: "1px solid var(--border-color)",
          margin: "-24px -24px 20px -24px",
          padding: "24px 24px 20px 24px",
        }}
      >
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            flexWrap: "wrap",
            gap: "15px",
            marginBottom: "20px",
          }}
        >
          <div>
            <h2 style={{ color: "var(--text-primary)", margin: "0 0 6px 0" }}>Discover AppImages</h2>
            <p style={{ color: "var(--text-secondary)", margin: 0, fontSize: "14px" }}>
              {source === "appimagehub"
                ? "Browsing AppImageHub Official Catalog"
                : "Browsing Popular GitHub AppImage Projects"}
            </p>
          </div>
          <div
            style={{
              display: "flex",
              gap: "6px",
              background: "var(--bg-input)",
              padding: "4px",
              borderRadius: "8px",
              border: "1px solid var(--border-color)",
            }}
          >
            <button
              onClick={() => setSource("appimagehub")}
              style={{
                display: "flex",
                alignItems: "center",
                gap: "8px",
                padding: "7px 14px",
                border: "none",
                borderRadius: "6px",
                cursor: "pointer",
                fontSize: "13px",
                fontWeight: 600,
                background: source === "appimagehub" ? "var(--accent-color)" : "transparent",
                color: source === "appimagehub" ? "#fff" : "var(--text-primary)",
              }}
            >
              <Globe size={15} /> Hub Catalog
            </button>
            <button
              onClick={() => setSource("github")}
              style={{
                display: "flex",
                alignItems: "center",
                gap: "8px",
                padding: "7px 14px",
                border: "none",
                borderRadius: "6px",
                cursor: "pointer",
                fontSize: "13px",
                fontWeight: 600,
                background: source === "github" ? "var(--accent-color)" : "transparent",
                color: source === "github" ? "#fff" : "var(--text-primary)",
              }}
            >
              <Github size={15} /> GitHub Trending
            </button>
          </div>
        </div>

        <div style={{ position: "relative", maxWidth: "420px" }}>
          <Search
            size={18}
            style={{
              position: "absolute",
              left: "12px",
              top: "50%",
              transform: "translateY(-50%)",
              color: "var(--text-muted)",
            }}
          />
          <input
            type="text"
            placeholder="Search AppImages by name or keyword..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            style={{
              width: "100%",
              padding: "10px 14px 10px 40px",
              background: "var(--bg-input)",
              border: "1px solid var(--border-color)",
              borderRadius: "8px",
              color: "var(--text-primary)",
              fontSize: "14px",
              outline: "none",
            }}
          />
        </div>
      </div>

      {loading && (
        <div style={{ textAlign: "center", padding: "50px", color: "var(--text-secondary)" }}>
          <div
            style={{
              width: "32px",
              height: "32px",
              border: "3px solid var(--border-color)",
              borderTopColor: "var(--accent-color)",
              borderRadius: "50%",
              animation: "spin 1s linear infinite",
              margin: "0 auto 15px auto",
            }}
          />
          Fetching {source === "github" ? "GitHub projects" : "Hub catalog"}...
        </div>
      )}

      {error && (
        <div
          style={{
            color: "var(--danger-color)",
            padding: "16px 20px",
            background: "var(--danger-bg)",
            borderRadius: "10px",
            border: "1px solid var(--danger-color)",
            fontSize: "14px",
          }}
        >
          {error}
        </div>
      )}

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill, minmax(310px, 1fr))",
          gap: "20px",
        }}
      >
        {filteredApps.map((app, idx) => {
          const githubLink = app.links?.find(
            (l) => l.type.toLowerCase() === "github" || l.type.toLowerCase() === "homepage"
          );
          const downloadLink =
            app.links?.find((l) => l.type.toLowerCase() === "download") || githubLink;
          const installed = isInstalled(app.name);
          const isBusy = downloadingName === app.name;

          return (
            <div
              key={idx}
              className="app-card"
              style={{ padding: "20px", display: "flex", flexDirection: "column" }}
            >
              <div style={{ display: "flex", alignItems: "center", marginBottom: "15px" }}>
                {app.icons && app.icons.length > 0 ? (
                  <img
                    src={
                      app.icons[0].startsWith("http")
                        ? app.icons[0]
                        : `https://appimage.github.io/database/${app.icons[0]}`
                    }
                    alt={app.name}
                    style={{
                      width: "56px",
                      height: "56px",
                      marginRight: "15px",
                      objectFit: "contain",
                      borderRadius: "10px",
                      background: "var(--bg-input)",
                    }}
                  />
                ) : (
                  <div
                    style={{
                      width: "56px",
                      height: "56px",
                      background: "var(--bg-input)",
                      borderRadius: "10px",
                      marginRight: "15px",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                    }}
                  >
                    <Globe size={24} color="var(--text-muted)" />
                  </div>
                )}
                <div style={{ overflow: "hidden", flex: 1 }}>
                  <div
                    style={{
                      fontWeight: "bold",
                      fontSize: "17px",
                      whiteSpace: "nowrap",
                      overflow: "hidden",
                      textOverflow: "ellipsis",
                      color: "var(--text-primary)",
                    }}
                  >
                    {app.name}
                  </div>
                  <div
                    style={{
                      fontSize: "12px",
                      color: "var(--text-secondary)",
                      display: "flex",
                      alignItems: "center",
                      gap: "6px",
                      marginTop: "4px",
                    }}
                  >
                    {source === "github" && <Github size={12} />}
                    <span>{app.authors?.[0]?.name || "Community"}</span>
                    {app.stargazers_count !== undefined && (
                      <span style={{ color: "#e3b341", fontWeight: 600 }}>
                        ★ {app.stargazers_count}
                      </span>
                    )}
                  </div>
                </div>
              </div>

              <div
                style={{
                  fontSize: "13px",
                  color: "var(--text-secondary)",
                  marginBottom: "18px",
                  flex: 1,
                  overflow: "hidden",
                  display: "-webkit-box",
                  WebkitLineClamp: 3,
                  WebkitBoxOrient: "vertical",
                  lineHeight: "1.5",
                }}
              >
                {app.description || "No description available for this project."}
              </div>

              <div style={{ display: "flex", gap: "10px", marginTop: "auto" }}>
                {installed ? (
                  <div
                    style={{
                      flex: 1,
                      background: "var(--accent-bg)",
                      color: "var(--accent-color)",
                      border: "1px solid var(--accent-color)",
                      padding: "10px",
                      borderRadius: "6px",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      fontWeight: "bold",
                      fontSize: "13px",
                      gap: "6px",
                    }}
                  >
                    <CheckCircle size={16} /> Installed
                  </div>
                ) : (
                  <button
                    disabled={(!downloadLink && source !== "github") || isBusy}
                    onClick={async () => {
                      try {
                        setDownloadingName(app.name);
                        if (source === "github" && app.github_url) {
                          const parts = app.github_url.split("/");
                          const owner = parts[parts.length - 2];
                          const repo = parts[parts.length - 1];
                          const res = await fetch(
                            `https://api.github.com/repos/${owner}/${repo}/releases/latest`
                          );
                          const release = await res.json();
                          const asset = release.assets?.find((a: any) =>
                            a.name.toLowerCase().endsWith(".appimage")
                          );
                          if (asset) {
                            await invoke("download_and_install", {
                              url: asset.browser_download_url,
                              name: app.name,
                            });
                          } else {
                            await invoke("open_url", { url: `${app.github_url}/releases` });
                          }
                        } else if (downloadLink) {
                          await invoke("download_and_install", {
                            url: downloadLink.url,
                            name: app.name,
                          });
                        }
                        if (onRefreshLibrary) onRefreshLibrary();
                      } catch (e) {
                        setError(String(e));
                      } finally {
                        setDownloadingName(null);
                      }
                    }}
                    style={{
                      flex: 1,
                      background: "var(--accent-color)",
                      color: "#fff",
                      border: "none",
                      padding: "10px",
                      borderRadius: "6px",
                      cursor: isBusy ? "not-allowed" : "pointer",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                      fontWeight: "bold",
                      fontSize: "13px",
                      gap: "8px",
                      opacity: isBusy ? 0.7 : 1,
                    }}
                  >
                    <Download size={16} /> {isBusy ? "Installing..." : "Install"}
                  </button>
                )}

                {app.github_url && (
                  <button
                    onClick={() => invoke("open_url", { url: app.github_url })}
                    title="View GitHub Repository"
                    style={{
                      background: "var(--bg-input)",
                      color: "var(--text-primary)",
                      border: "1px solid var(--border-color)",
                      width: "40px",
                      borderRadius: "6px",
                      cursor: "pointer",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                    }}
                  >
                    <Github size={18} />
                  </button>
                )}
              </div>
            </div>
          );
        })}
      </div>

      {!loading && filteredApps.length === 0 && (
        <div
          style={{
            textAlign: "center",
            padding: "40px",
            color: "var(--text-muted)",
            background: "var(--bg-card)",
            borderRadius: "12px",
            border: "1px dashed var(--border-color)",
          }}
        >
          No applications found matching your search.
        </div>
      )}
    </div>
  );
};

export default Storefront;
