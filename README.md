
# Jade

Jade is an installer backend for eweOS, which is a forked version of [Jade](https://gitlab.com/crystal-linux/software/jade/) from [Crystal Linux](https://getcryst.al).

## Backend usage

### Autopartition the drive
```sh
# autopartition /dev/sda with efi enabled
# jade partition auto /dev/sda --efi

# autopartition /dev/nvmen0 with efi disabled
# jade partition auto /dev/nvmen0
```

### Install base packages
```sh
# jade install-base
```

### Install bootloader
```sh
# install as efi with esp being /boot/efi
# jade bootloader limine-efi /boot/efi

# install as legacy on /dev/sda
# jade bootloader limine-legacy /dev/sda
```

### Generate fstab
```sh
# jade genfstab
```

### Configuring locale settings
```sh
# set the keyboard layout to colemak, the timezone to Europe/Berlin and set en_US.UTF-8 as the locale
# jade locale colemak Europe/Berlin en_US.UTF-8 UTF-8
```

### Configure network settings
```sh
# set the hostname to os.ewe.moe with ipv6 disabled
# jade networking os.ewe.moe 

# set the hostname to os.ewe.moe with ipv6 enabled
# jade networking os.ewe.moe --ipv6
```

### Configure users
```sh
# make a new user called nonRootHaver, without sudo, easytohack as the password and bash as the default shell
# jade users new-user nonRootHaver easytohack bash

# make a user called rootHaver, with sudo, omgsosuperhardtohack as the password and fish as the default shell
# jade users new-user rootHaver omgsuperhardtohack fish --hasroot
```

### Set root password
```sh
# set the root password to 'muchSecurity,veryHardToHack'
# jade users root-password muchSecurity,veryHardToHack
```

### Install a desktop environment
```sh
# install hyprland
# jade desktops hyprland
```

### Setup flatpak
```sh
# jade flatpak
```

### Debug logging

debug messages:
```sh
# jade -v
```

traces:
```sh
# jade -vv
```


## Non-secret Secret
$ echo "JADE_UWU=true" >> ~/.zshrc 

$ echo "JADE_UWU=true" >> ~/.bashrc 

$ set -Ux JADE_UWU true 


if you want to have your log and crash output be “cute”

## 🙌 Contributing

If you'd like to contribute to **Jade**, please follow the [Crystal Linux contributing guidelines](https://git.getcryst.al/crystal/info/-/blob/main/CONTRIBUTING.md)!

This project uses `rustup`, to set up `cargo` for **Jade** development, please follow the guidelines below:


#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`



## 📜 License

[GPLv3-only](https://choosealicense.com/licenses/gpl-3.0/)

![](https://git.getcryst.al/crystal/misc/branding/-/raw/main/banners/README-banner.png)
