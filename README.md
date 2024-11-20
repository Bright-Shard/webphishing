# webphishing

WEBFISHING haxx

# why

yes

# status

this project builds a DLL that injects into WEBFISHING and hooks a Godot engine function. i need to call another engine function to load GDScript to load the actual hacks.

# instructions

## usage in-game

there will be a button in the top-left corner of the game. pressing it will open webphising's menu.

## normal installation

download `webphishing.zip` from [GitHub releases](/releases), or by pressing this button:

[![download](https://img.shields.io/badge/download-latest%20release-green)](releases/download/latest/webphishing.zip)

unzip its contents into WEBFISHING's folder (the same folder that has WEBFISHING.exe).

if you're on Linux running the game under Proton, add `WINEDLLOVERRIDES="winmm=n,b" %command%` to your launch arguments in Steam.

## building from source

you'll need Rust and mingw.

make sure you have the Windows target in rustup:

```sh
rustup target add x86_64-pc-windows-gnu
```

installing mingw will depend on your platform. on Arch Linux, it's as easy as:

```sh
sudo pacman -S mingw-w64
```

you can then build this project with `cargo b -r`. after it finishes, the webphishing DLL will be at `target/x86-64-pc-windows-gnu/release/webphishing.dll`. copy this file to wherever WEBFISHING is installed, then rename it `winmm.dll`. finally, copy the original (legitimate) `winmm.dll` file to WEBFISHING's folder, and rename it to `winmm_orig.dll`. on Linux `winmm` is available through wine at `/usr/lib/wine/x86_64-windows/winmm.dll`; on Windows it's at `C:\Windows\System32\winmm.dll`.

# credits

[NotNite](https://notnite.com/) and [Zeropio](https://doing.re/) for helping me learn several techniques used in this project.

[GDWeave](https://github.com/NotNite/GDWeave) as a reference for modding Godot games.

the DLL proxy uses [this DLL Proxy method](https://github.com/tothi/dll-hijack-by-proxying?tab=readme-ov-file).

# todo

- [ ] hook functions by their signature instead of exact address
- [ ] replace retour-rs with my own detouring library
