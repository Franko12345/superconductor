# Conductor — FRC Driver Station for Linux

A Driver Station for FRC robots on Linux, supporting nearly every feature of the NI Driver Station.

> **Note:** Starting in 2027, the official WPILib Driver Station gains cross-platform support. See [FirstDriverStation-Public](https://github.com/wpilibsuite/FirstDriverStation-Public). Conductor is maintained here for teams that need it in the meantime.

## Architecture

| Component | Stack | Description |
|---|---|---|
| **Backend** | Rust + Actix-web 4 | roboRIO comms (`ds`), joystick input (`gilrs`), HTTP + WebSocket server |
| **Main Window** | React 18 + Vite 4 + Bootstrap 4 | Controls, status, joystick mapping |
| **Console Window** | React 18 + Vite 4 + Bootstrap 4 | riolog/stdout output |

Both frontends are compiled and **embedded into the Rust binary** via `rust-embed` — the result is a single self-contained executable with no runtime file dependencies.

---

## Building

### Arch Linux

```bash
# System dependencies
sudo pacman -S --needed base-devel nodejs npm webkit2gtk libsoup3 gtk3 libxi

# Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Clone and build
git clone https://github.com/Redrield/Conductor
cd Conductor
make setup && make release
```

> **Node.js version:** Vite 4 requires Node.js v14–v18. If your system Node is newer and causes issues, use [nvm](https://github.com/nvm-sh/nvm):
> ```bash
> nvm install 18 && nvm use 18
> ```

### Ubuntu / Debian

```bash
# System dependencies
sudo apt-get install -y \
    nodejs npm libudev-dev \
    libx11-dev libxi-dev libpango1.0-dev libatk1.0-dev \
    libsoup2.4-dev libgtk-3-dev libwebkit2gtk-4.0-dev

# Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Clone and build
git clone https://github.com/Redrield/Conductor
cd Conductor
make setup && make release
```

---

## Makefile targets

| Target | Description |
|---|---|
| `make setup` | Install npm dependencies (run once after cloning) |
| `make` | Build main window + run debug binary |
| `make all` | Build both windows + run debug binary |
| `make frontend` | Build both React apps without running |
| `make release` | Build both React apps + optimized release binary |

The release binary is output to `target/release/conductor`.

---

## Running

```bash
./target/release/conductor
```

The binary is fully self-contained. On Linux it automatically sets the required GTK/WebKit environment variables at startup so it works correctly on both **X11 and Wayland** (via XWayland) without any wrappers:

- `GDK_BACKEND=x11`
- `WEBKIT_DISABLE_COMPOSITING_MODE=1`
- `WEBKIT_DISABLE_DMABUF_RENDERER=1`

---

## Usage

### Control Tab
- **Mode**: Autonomous / Teleoperated / Test
- **Enable / Disable** the robot
- **Status indicators**: Communications, Robot Code, Joysticks
- **Team number**, **voltage**, **alliance station**

### Config Tab
- **Team Number** — sets the target IP (`10.TE.AM.2`)
- **Connect via USB** — connects to `172.22.11.2` instead
- **Game Specific Message (GSM)**
- **Restart Code / Reboot RIO**

### Joysticks Tab
Drag-and-drop list of detected controllers. Order = port number sent to the robot. Each connected controller shows live signal data:
- **Axes** — values from −1.0 to 1.0
- **Buttons** — highlighted green when pressed
- **POV / D-Pad** — angle in degrees

The first slot is the **Virtual Joystick**, used to suppress WPILib warnings about missing controllers.

### Console Window
Separate window for riolog output streamed from the robot in real time.
