# Installation

Mago provides multiple installation methods to suit different environments and preferences. Follow the instructions below to install Mago on your system.

---

## One-Line Installation (Recommended)

The easiest way to install Mago on macOS or Linux is to use our one-line installer script.

### Using `curl`

Run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | bash
```

### Using `wget`

Alternatively, if you prefer `wget`:

```bash
wget -qO- https://carthage.software/mago.sh | bash
```

### Custom Installation Directory

To specify a custom directory for the binary, use the `--install-dir` option:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | bash -s -- --install-dir="/.bin"
```

If the directory is not in your `PATH`, the script will provide instructions to add it.

### Installing with `sudo`

If you need to install Mago system-wide, you can use `sudo` with the installation command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | sudo bash
```

---

## Installation via Homebrew

If you are using macOS, you can install Mago using Homebrew:

```bash
brew install mago
```

---

## Installation via Composer

If you're already using Composer in your PHP project, you can install Mago as a Composer dependency. Follow these steps:

1. **Add Mago as a dependency**:
   Run the following command in your terminal:

   ```bash
   composer require carthage-software/mago
   ```

2. **Enable the Mago plugin**:
   Update your composer.json file to enable the Mago plugin by adding the following under the "config" section:

   ```json
   {
     "config": {
       "allow-plugins": {
         "carthage-software/mago": true
       }
     }
   }
   ```

3. **Install the Mago binary**:
   Run the following command to install the Mago binary:

   ```bash
   composer mago:install-binary
   ```

4. **Run Mago**:
   You can now use Mago through the vendor/bin directory:

   ```bash
   vendor/bin/mago --help
   ```

> ### Note on Performance
>
> When using Mago via Composer (`vendor/bin/mago`), performance on small-to-medium applications might seem slower compared to running the binary directly.
> This is because the `vendor/bin/mago` script is a PHP file that proxies commands to the Mago binary. As a result, you're effectively running:
>
> - PHP (to process the script),
> - The script itself,
> - The Mago binary.
>
> This extra overhead might be noticeable in smaller applications, but it's less significant for larger projects.

---

## Pre-compiled Binaries

You can find precompiled binaries for various platforms on our [Releases page](https://github.com/carthage-software/mago/releases).

1. Download the archive for your platform.
2. Extract the archive.
3. Place the `mago` binary somewhere in your `PATH`.

---

## Installation via Cargo

If you have Rust installed, you can install Mago using Cargo:

```bash
cargo install mago
```

---

## Installation from source

To build and install Mago from source:

1. Clone the repository:

```bash
git clone https://github.com/carthage-software/mago
```

2. Navigate to the project directory:

```bash
cd mago
```

3. Build and install the project using Cargo:

```bash
cargo install --path .
```
