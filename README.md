
# Jade

Jade is an installer backend for crystal linux.

## Backend usage

### Autopartition the drive
```sh
# autopartition /dev/sda with efi enabled
# crystal-jade partition auto /dev/sda --efi

# autopartition /dev/nvmen0 with efi disabled
# crystal-jade partition auto /dev/nvmen0
```

### Install base packages
```sh
# crystal-jade install-base
```

### Install bootloader
```sh
# install as efi with esp being /boot/efi
# crystal-jade bootloader grub-efi /boot/efi

# install as legacy on /dev/sda
# crystal-jade bootloader grub-legacy /dev/sda
```

### Generate fstab
```sh
# crystal-jade genfstab
```

### Configuring locale settings
```sh
# set the keyboard layout to colemak, the timezone to Europe/Berlin and set en_US.UTF-8 as the locale
# crystal-jade locale colemak Europe/Berlin en_US.UTF-8 UTF-8
```

### Configure network settings
```sh
# set the hostname to getcryst.al with ipv6 disabled
# crystal-jade networking getcryst.al 

# set the hostname to getcryst.al with ipv6 enabled
# crystal-jade networking getcryst.al --ipv6
```

### Setup zramd
```sh
# install and enable zramd
# crystal-jade zramd
```

### Configure users
```sh
# make a new user called nonRootHaver, without sudo, easytohack as the password and bash as the default shell
# crystal-jade users new-user nonRootHaver easytohack bash

# make a user called rootHaver, with sudo, omgsosuperhardtohack as the password and fish as the default shell
# crystal-jade users new-user rootHaver omgsuperhardtohack fish --hasroot
```

### Set root password
```sh
# set the root password to 'muchSecurity,veryHardToHack'
# crystal-jade users root-password muchSecurity,veryHardToHack
```

### Install a desktop environment
```sh
# install onyx
# crystal-jade desktops onyx

# install gnome
# crystal-jade desktops gnome
```

### Setup timeshift
```sh
# crystal-jade setup-timeshift
```

### Setup flatpak
```sh
# crystal-jade flatpak
```

### Debug logging

debug messages:
```sh
# crystal-jade -v
```

traces:
```sh
# crystal-jade -vv
```


## Non-secret Secret
$ echo "crystal-jade_UWU=true" >> ~/.zshrc 

$ echo "crystal-jade_UWU=true" >> ~/.bashrc 

$ set -Ux crystal-jade_UWU true 


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
