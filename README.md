# Waybar Config Changer
Change the wm specified on ~/.config/waybar/config.jsonc
(it can be used for other files, waybar is just the only use case I have)
## Usage
```bash
wcc /home/{your_user}/.config/waybar/config.jsonc hyprland
```
## How to build
```bash
git clone https://github.com/posavacc/waybar-config-changer
cd waybar-config-changer
cargo install --path .
```
(make sure ~/.cargo/bin is in your PATH to be able to just use wcc)
