<p align="center">
  <a href="https://github.com/crystal-linux/jade/">
    <img src="https://getcryst.al/site/assets/other/logo.png" alt="Logo" width="150" height="150">
  </a>
</p>
<h2 align="center">Jade</h2>
<p align="center">
    <a href="https://github.com/crystal-linux/.github/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-blue.svg" alt="License">
    <a href="https://github/crystal-linux/jade"><img alt="GitHub isses" src="https://img.shields.io/github/issues-raw/crystal-linux/jade"></a>
    <a href="https://github/crystal-linux/jade"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr-raw/crystal-linux/jade"></a><br>
    <a href="https://twitter.com/intent/user?screen_name=crystal_linux"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/crystal_linux?style=flat?color=blue">
    <a href="https://discord.gg/hYJgu8K5aA"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"> </a>
    <a href="https://github.com/axtloss"><img src="https://img.shields.io/badge/Maintainer-@axtloss-brightgreen" alt="The maintainer of this repository" href="https://github.com/axtloss"></a>
</p>

<p align="center">Jade is the backend and TUI installer for crystal linux.</p>

## Use the TUI (not implemented yet)
just run <br>
`jade`

## Backend usage

### autopartition the drive
```sh
# autopartition /dev/sda with efi enabled
jade partition auto /dev/sda --efi

# autopartition /dev/nvmen0 with efi disabled
jade partition auto /dev/nvmen0
```

### install base packages
```sh
jade install-base
```

### install bootloader
```sh
# install as efi with esp being /boot/efi
jade bootloader grub-efi /boot/efi

# install as legacy on /dev/sda
jade bootloader grub-legacy /dev/sda
```

### generate fstab
```sh
jade genfstab
```

### configuring locale settings
```sh
# set the keyboard layout to colemak, the timezone to Europe/Berlin and set en_US.UTF-8 as the locale
jade locale colemak Europe/Berlin en_US.UTF-8 UTF-8
```

### configure network settings
```sh
# set the hostname to getcryst.al with ipv6 disabled
jade networking getcryst.al 

# set the hostname to getcryst.al with ipv6 enabled
jade networking getcryst.al --ipv6
```

### configure users
```sh
# make a new user called nonRootHaver, without sudo and easytohack as the password
# jade uses prehashed passwords for user creation, so you'll have to calculate the hash of the password
jade users newUser nonRootHaver $(openssl passwd -1 easytohack)

# make a user called rootHaver, with sudo and omgsosuperhardtohack as the password
jade users newUser rootHaver $(openssl passwd -1 omgsuperhardtohack) --sudoer
```

### set root password
```sh
# set the root password to 'muchSecurity,veryHardToHack'
# the same hashing thing goes for root passwords
jade users rootPass $(openssl passwd -1 muchSecurity,veryHardToHack)
```

### install a desktop environment
```sh
# install onyx
jade desktops onyx

# install gnome
jade desktops gnome
```

### setup timeshift
```sh
jade setup-timeshift
```

### setup flatpak
```sh
jade flatpak
```

### debug logging

debug messages:
```sh
jade -v
```

traces:
```sh
jade -vv
```

## How to build:

Tested on latest Cargo (1.60.0-nightly)

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`

## Non-secret Secret
echo "JADE_UWU=true" >> ~/.zshrc <br>
echo "JADE_UWU=true" >> ~/.bashrc <br>
set -Ux JADE_UWU true <br>
<br>
if you want to have your log and crash output be "cute"
