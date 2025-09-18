# aura

**AUR helper for ArchLinux's [Github AUR mirror repo](https://github.com/archlinux/aur)**

See also: https://archlinux.org/news/recent-services-outages/

## Usage

### Install Packages

```bash
aura -S visual-studio-code-bin
```

### Uninstall Packages

Use `pacman`, `paru` or `yay` to uninstall packages.

```bash
paru -R visual-studio-code-bin
sudo pacman -R visual-studio-code-bin
```

### Clear Cache Folder

```bash
aura -Sc
aura -Scc
```
