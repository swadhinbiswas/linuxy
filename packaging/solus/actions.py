from pisi.actionsapi import shelltools, pisitools, get

def setup():
    shelltools.system("npm install")
    shelltools.system("npm run build")

def build():
    shelltools.system("cd src-tauri && cargo build --release --locked")

def install():
    pisitools.dobin("src-tauri/target/release/linuxy")
    pisitools.insinto("/usr/share/applications", "src-tauri/debian/desktop-template.desktop", "linuxy.desktop")
    for size in ["32", "128", "256", "512"]:
        icon = f"src-tauri/icons/{size}x{size}.png"
        if shelltools.isFile(icon):
            pisitools.insinto(f"/usr/share/icons/hicolor/{size}x{size}/apps", icon, "linuxy.png")
    if (
        not shelltools.isFile("src-tauri/icons/512x512.png")
        and shelltools.isFile("src-tauri/icons/icon.png")
    ):
        pisitools.insinto("/usr/share/icons/hicolor/512x512/apps", "src-tauri/icons/icon.png", "linuxy.png")
    pisitools.insinto("/usr/share/licenses/linuxy", "LICENSE")
