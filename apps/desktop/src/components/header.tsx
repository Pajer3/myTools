import { getCurrentWindow } from "@tauri-apps/api/window";
import "../styles/header/header.css";
import "../styles/header/windowControles.css";
import notifyImg from "../assets/header/notify.svg?url";
// import minimizeImg from "../assets/header/minimize.svg?url";
import maximizeImg from "../assets/header/maximize.svg?url";
import closeImg from "../assets/header/close.svg?url";

export default function Header() {
    const appWindow = getCurrentWindow();

    return (
        <div className="Header">
            {/* Drag Region */}
            <div
                data-tauri-drag-region
                style={{
                    position: "absolute",
                    top: 0,
                    left: 0,
                    width: "100%",
                    height: "100%",
                    zIndex: 0
                }}
            />

            <div className="notifications" style={{ zIndex: 1, pointerEvents: "none" }}>
                <img src={notifyImg} alt="notifications" style={{ pointerEvents: "auto" }} />
            </div>
            <div className="window-controls" style={{ zIndex: 1 }}>
                {/* <button onClick={() => appWindow.minimize()}><img src={minimizeImg} alt="minimize" /></button> */}
                <button onClick={() => appWindow.toggleMaximize()}><img src={maximizeImg} alt="maximize" /></button>
                <button onClick={() => appWindow.close()}><img src={closeImg} alt="close" /></button>
            </div>
        </div>
    )
}