---
title: Installation
---

# Installation

Installing **Mago** is designed to be a quick and simple process.

## Automatic Installation (macOS & Linux)

The easiest way to install Mago is by using our installer script. Open your terminal and run the following command:

```bash
curl -sSL https://install.carthage.software/mago | bash
```

This script will download the latest pre-compiled binary for your system, make it executable, and move it to a standard location (`/usr/local/bin`) so you can run `mago` from anywhere.

## Manual Installation (Windows & Other Systems)

If you're on Windows or prefer to install manually, you can download the latest release from our GitHub page.

1.  Go to the **[Mago Releases Page](https://github.com/carthage-software/mago/releases)**.
2.  Download the appropriate binary for your operating system (e.g., `mago-x86_64-pc-windows-msvc.zip` for Windows).
3.  Unzip the archive.
4.  Place the `mago.exe` (or `mago`) binary in a directory that is included in your system's `PATH`.

Once installed, you can verify it's working by running:

```bash
mago --version
```
