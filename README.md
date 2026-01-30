# Chrome-ultrabook Tools (GUI)

A cross-platform desktop application designed for Chromebooks running Linux/Windows. This tool provides a user-friendly interface for low-level hardware management, bridging the gap between firmware and the OS.

## 🚀 Features

* **Fan Control:** Custom fan curves (Silent, Balanced, Turbo) for thermal management.
* **Hardware Diagnostics:** Real-time monitoring of CPU temp, battery health, and fan RPM.
* **Key Remapping:** GUI-based tool to remap Chromebook-specific keys (e.g., Search -> CapsLock).
* **Hybrid Architecture:** Built with **Rust (Tauri)** for system-level safety and **Angular** for a responsive UI.

## 🛠️ Technology Stack

* **Frontend:** Angular (TypeScript), HTML5, CSS3
* **Backend:** Rust (Tauri API)
* **System Integration:** Custom EC (Embedded Controller) calls via Rust

## 📦 Setup & Run

1.  **Install Dependencies:**
    ```bash
    npm install
    ```

2.  **Run in Development Mode:**
    ```bash
    npm run tauri dev
    ```

3.  **Build for Production:**
    ```bash
    npm run tauri build
    ```
