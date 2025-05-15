
<img src="docs/assets/rotor.png" width="300" />

# Rotor

![](https://img.shields.io/badge/license-Zlib%2Flibpng-blue)
![](https://img.shields.io/github/v/release/rotor-lang/rotor?display_name=release&style=flat&link=https%3A%2F%2Fgithub.com%2Frotor-lang%2Frotor%2Freleases)
![](https://img.shields.io/github/last-commit/rotor-lang/rotor)
![](https://img.shields.io/github/repo-size/rotor-lang/rotor)
![](https://img.shields.io/github/issues/rotor-lang/rotor)
![](https://img.shields.io/github/issues-pr/rotor-lang/rotor)
![](https://img.shields.io/github/contributors/rotor-lang/rotor)
![](https://img.shields.io/github/commit-activity/m/rotor-lang/rotor)

---

> Rotor is currently in early alpha and ***not recommended*** for production use. The installation guide may be incomplete or subject to change. Thanks for your patience!

---

### Rotor is a all-purpose fast language inspired by [Rust](https://rust-lang.org) and [Carbon](https://docs.carbon-lang.dev) and built for anything and anyone.

- [How to Install](#how-to-install)
    - [Windows](#windows)
    - [macOS](#macos)
    - [Linux](#linux)

---

## How to Install

> Rotor currently requires you to build from source.

### Windows



1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/downloads) installed.
2. Clone the repository:
    ```powershell
    git clone https://github.com/rotor-lang/rotor.git
    cd rotor
    ```
3. Build the project:
    ```powershell
    cargo build --release
    ```
4. Run it:
    ```powershell
    .\target\release\rotor.exe --version
    ```

If you see a version output, congrats — Rotor is installed!

---

### MacOS

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/downloads) installed.
2. Clone the repository:
    ```bash
    git clone https://github.com/rotor-lang/rotor.git
    cd rotor
    ```
3. Build the project:
    ```bash
    cargo build --release
    ```
4. (Optional) Add the binary to your PATH:
    ```bash
    sudo cp target/release/rotor /usr/local/bin/rotor
    ```
5. Run it:
    ```bash
    rotor --version
    ```

If you see a version output, congrats — Rotor is installed!

### Want to Contribute?

Rotor is in active development! If you have ideas, bug fixes, or want to help, [open an issue](https://github.com/rotor-lang/rotor/issues) or make a pull request.

---

### License

This project is licensed under the Zlib/libpng license.

```