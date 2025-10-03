
# RustySip

**Take a sip every 30 minutes — powered by Rust.**

RustySip is a lightweight desktop notification tool for Linux that reminds you to drink water or take a short break every 30 minutes. It runs quietly in the background and uses native system notifications, so you won’t miss a reminder.

---

## Features

- Sends notifications every 30 minutes.
- Uses D-Bus for communication.
- Runs automatically at login using **systemd user service**.
- Lightweight, minimal dependencies, fully written in Rust.

---

## Installation

### 1. Clone or Download the Project

```bash
git clone https://github.com/Kartik213/Rusty
cd Rusty/rusty-sip
```

### 2. Build the Project

Compile the Rust binary in release mode:

```bash
cargo build --release
```

This produces an optimized binary at `target/release/rusty-sip`.

### 3. Install the Binary

Copy the binary to a directory in your PATH:

```bash
cp target/release/rusty-sip ~/.local/bin/
```

This allows you to run `rusty-sip` from anywhere.

---

## Setup systemd User Service

RustySip runs best as a **background service** so that it starts automatically on login.

### 1. Create systemd User Directory

```bash
mkdir -p ~/.config/systemd/user
```

### 2. Create the Service File

```bash
nano ~/.config/systemd/user/water-reminder.service
```

Paste the following content:

```ini
[Unit]
Description=Water Reminder Notifications

[Service]
ExecStart=/home/user/.local/bin/rusty-sip
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
```

> ⚠️ Make sure to update `ExecStart` to match the path where you installed the binary.

### 3. Reload systemd and Enable the Service

```bash
systemctl --user daemon-reload
systemctl --user enable --now water-reminder.service
```

- `daemon-reload` reloads systemd to recognize the new service.  
- `enable --now` starts the service immediately and ensures it runs at login.

---

## Verify the Service

Check that RustySip is running:

```bash
systemctl --user status water-reminder.service
```

You should see logs confirming notifications are being sent every 30 minutes.

---

## Customization

- Change the notification interval by editing the Rust source code:

```rust
Duration::from_secs(1800) // 30 minutes
```

- Customize the notification message inside the `notify()` call in `main.rs`.

```rust
"Hey Buddy! 30 minutes have passed, Take a water break";
```

---

## Uninstall / Stop

To stop RustySip:

```bash
systemctl --user stop water-reminder.service
```

To disable auto-start:

```bash
systemctl --user disable water-reminder.service
```

To remove the binary:

```bash
rm ~/.local/bin/rusty-sip
```

## Enjoy!
