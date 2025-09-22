# unfocol

**unfocol** (UNfocused FOcus using COLors) is a small TUI focus timer.  
It displays a block of color that changes gradually over time, using the active terminal theme’s colors. The idea is to give a peripheral sense of time passing without having to look at a countdown.

At present, **unfocol** is primarily designed for Omarchy (Arch + Hyprland) users, and is meant to integrate with the beautiful themes that distro provides. It should work on any Arch-based system with a terminal emulator such as Alacritty or Ghostty. On other setups, you may need to adapt configuration paths or theme handling, or change the colors to something you prefer in the source code before building.

## Installation

### From the AUR (recommended for Arch/Omarchy users)

With `yay`:
```bash
yay -S unfocol
```

With `paru`:
```bash
paru -S unfocol
```

### Build from source

Clone the repository and build with Cargo:
```bash
git clone https://github.com/MrOnijohn/unfocol.git
cd unfocol
cargo build --release
```

Copy the binary into your PATH:
```bash
install -Dm755 target/release/unfocol ~/.local/bin/unfocol
```

## Notes for non-Omarchy users

- unfocol reads terminal color definitions from:
  ```
  ~/.config/omarchy/current/theme/alacritty.toml
  ```
  If that file is not found, default, not very pretty, colors are used instead.
  If you are not using Omarchy, adjust the path in the source or provide a compatible file at that location.
- Works best in terminal emulators that support proper color schemes (Alacritty, Ghostty, Kitty, etc.).
- Window placement is handled by your compositor (e.g., Hyprland). To snap unfocol to a specific position/size, configure compositor window rules.

## License

MIT, see [LICENSE](LICENSE).
