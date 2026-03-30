import { invoke } from "@tauri-apps/api/tauri";
import { Download, Search, Github, Globe } from "lucide-react";
import React, { useState, useEffect } from "react";

interface StoreApp {
  name: string;
  description: string;
  authors?: { name: string }[];
  links?: { type: string; url: string }[];
  icons?: string[];
  github_url?: string;
  stargazers_count?: number;
}

const Storefront: React.FC = () => {
  const [apps, setApps] = useState<StoreApp[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [source, setSource] = useState<"appimagehub" | "github">("appimagehub");

  useEffect(() => {
    fetchData();
  }, [source]);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      if (source === "appimagehub") {
        const response = await fetch("https://appimage.github.io/feed.json");
        if (!response.ok) throw new Error("Failed to fetch AppImageHub data");
        const data = await response.json();
        setApps(data.items || []);
      } else {
        const response = await fetch(
          "https://api.github.com/search/repositories?q=topic:appimage&sort=stars&order=desc&per_page=100"
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
    } catch (err) {
      console.error(err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const filteredApps = apps.filter(
    (app) =>
      app.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      (app.description && app.description.toLowerCase().includes(searchTerm.toLowerCase()))
  );

  return (
    <div style={{ paddingBottom: "40px" }}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          marginBottom: "20px",
        }}
      >
        <div>
          <h2 style={{ color: "var(--text-primary)" }}>Discover AppImages</h2>
          <p style={{ color: "var(--text-secondary)" }}>
            {source === "appimagehub"
              ? "Browsing AppImageHub Catalog"
              : "Browsing Popular GitHub AppImage Projects"}
          </p>
        </div>
        <div
          style={{
            display: "flex",
            gap: "10px",
            background: "var(--bg-input)",
            padding: "5px",
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
              padding: "8px 12px",
              border: "none",
              borderRadius: "6px",
              cursor: "pointer",
              background: source === "appimagehub" ? "var(--bg-card)" : "transparent",
              color: "var(--text-primary)",
              boxShadow: source === "appimagehub" ? "0 2px 4px rgba(0,0,0,0.2)" : "none",
            }}
          >
            <Globe size={16} /> Hub
          </button>
          <button
            onClick={() => setSource("github")}
            style={{
              display: "flex",
              alignItems: "center",
              gap: "8px",
              padding: "8px 12px",
              border: "none",
              borderRadius: "6px",
              cursor: "pointer",
              background: source === "github" ? "var(--bg-card)" : "transparent",
              color: "var(--text-primary)",
              boxShadow: source === "github" ? "0 2px 4px rgba(0,0,0,0.2)" : "none",
            }}
          >
            <Github size={16} /> GitHub
          </button>
        </div>
      </div>

      <div style={{ position: "relative", marginBottom: "20px", maxWidth: "400px" }}>
        <Search
          size={18}
          style={{ position: "absolute", left: "10px", top: "10px", color: "var(--text-muted)" }}
        />
        <input
          type="text"
          placeholder="Search catalog..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          style={{
            width: "100%",
            padding: "10px 10px 10px 35px",
            background: "var(--bg-input)",
            border: "1px solid var(--border-color)",
            borderRadius: "6px",
            color: "var(--text-primary)",
            fontSize: "16px",
          }}
        />
      </div>

      {loading && (
        <div style={{ textAlign: "center", padding: "40px", color: "var(--text-secondary)" }}>
          Loading {source === "github" ? "GitHub projects" : "Hub catalog"}...
        </div>
      )}
      {error && (
        <div
          style={{
            color: "var(--danger-color)",
            padding: "20px",
            background: "rgba(255,0,0,0.1)",
            borderRadius: "8px",
          }}
        >
          {error}
        </div>
      )}

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fill, minmax(300px, 1fr))",
          gap: "20px",
        }}
      >
        {filteredApps.map((app, idx) => {
          const githubLink = app.links?.find(
            (l) => l.type.toLowerCase() === "github" || l.type.toLowerCase() === "homepage"
          );
          const downloadLink =
            app.links?.find((l) => l.type.toLowerCase() === "download") || githubLink;

          return (
            <div
              key={idx}
              style={{
                background: "var(--bg-card)",
                borderRadius: "8px",
                padding: "20px",
                display: "flex",
                flexDirection: "column",
                border: "1px solid var(--border-color)",
                transition: "background-color 0.3s, border-color 0.3s",
              }}
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
                      borderRadius: "8px",
                    }}
                  />
                ) : (
                  <div
                    style={{
                      width: "56px",
                      height: "56px",
                      background: "var(--border-color)",
                      borderRadius: "8px",
                      marginRight: "15px",
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                    }}
                  >
                    <Globe size={24} color="var(--text-muted)" />
                  </div>
                )}
                <div style={{ overflow: "hidden" }}>
                  <div
                    style={{
                      fontWeight: "bold",
                      fontSize: "18px",
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
                      fontSize: "13px",
                      color: "var(--text-secondary)",
                      display: "flex",
                      alignItems: "center",
                      gap: "5px",
                    }}
                  >
                    {source === "github" && <Github size={12} />}{" "}
                    {app.authors?.[0]?.name || "Unknown"}
                    {app.stargazers_count !== undefined && (
                      <span style={{ color: "#e3b341" }}>★ {app.stargazers_count}</span>
                    )}
                  </div>
                </div>
              </div>

              <div
                style={{
                  fontSize: "14px",
                  color: "var(--text-secondary)",
                  marginBottom: "20px",
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

              <div style={{ display: "flex", gap: "10px" }}>
                <button
                  disabled={!downloadLink && source !== "github"}
                  onClick={async () => {
                    try {
                      if (source === "github" && app.github_url) {
                        // Extract owner/repo
                        const parts = app.github_url.split("/");
                        const owner = parts[parts.length - 2];
                        const repo = parts[parts.length - 1];

                        // Fetch releases
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
                          // Fallback to browser if no asset found
                          await invoke("open_url", { url: `${app.github_url}/releases` });
                        }
                      } else if (downloadLink) {
                        await invoke("download_and_install", {
                          url: downloadLink.url,
                          name: app.name,
                        });
                      }
                    } catch (e) {
                      setError(String(e));
                    }
                  }}
                  style={{
                    flex: 1,
                    background: "#4caf50",
                    color: "#fff",
                    border: "none",
                    padding: "10px",
                    borderRadius: "6px",
                    cursor: "pointer",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontWeight: "bold",
                    gap: "8px",
                  }}
                >
                  <Download size={16} /> Install
                </button>
                {app.github_url && (
                  <button
                    onClick={() => invoke("open_url", { url: app.github_url })}
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
        <div style={{ textAlign: "center", padding: "40px", color: "var(--text-muted)" }}>
          No applications found matching your search.
        </div>
      )}
    </div>
  );
};

export default Storefront;
