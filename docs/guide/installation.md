---
title: Installation
---

# Installation

Installing **Mago** is a quick process with several options to suit your environment and preferences.

## Shell Installer (macOS & Linux)

This is the recommended method for most macOS and Linux users. Our script automatically downloads the correct binary for your system and adds it to your path.

#### Using `curl`

```sh
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | bash
```

#### Using `wget`

```sh
wget -qO- https://carthage.software/mago.sh | bash
```

---

## Package Managers

### Homebrew (macOS)

If you're using Homebrew on macOS, you can install Mago with a single command:

```sh
brew install mago
```

You can later update Mago by running `mago self-update`.

### Composer (PHP Project)

To add Mago as a development dependency to your PHP project via Composer:

```sh
composer require --dev carthage-software/mago:1.0.0-beta.1
```

### Cargo (Rust)

If you have the Rust toolchain installed, you can install Mago directly from Crates.io:

```sh
cargo install mago
```

---

## Manual Download

You can always download a pre-compiled binary directly from our GitHub Releases page. This is the recommended method for **Windows** and other systems not covered above.

1.  Navigate to the **[Mago Releases Page](https://github.com/carthage-software/mago/releases)**.
2.  Download the appropriate archive for your operating system (e.g., `mago-x86_64-pc-windows-msvc.zip` for Windows).
3.  Unzip the archive.
4.  Place the `mago.exe` (or `mago`) executable in a directory that is part of your system's `PATH` environment variable.

---

## Verify Installation

Once installed, you can verify that Mago is working correctly by checking its version:

```sh
mago --version
```
