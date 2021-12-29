# Dust

## protocol

A package has a 40 bit header and a variable length of data.

```
u32            u8        vec<u8>
^              ^         ^
pkg data size  pkg type  pkg data
```

```bash
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```
