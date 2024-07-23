mod args;
mod functions;
mod internal;
mod logging;

use crate::args::{BootloaderSubcommand, Command, Opt, UsersSubcommand};
use crate::functions::*;
use clap::Parser;

fn main() {
    human_panic::setup_panic!();
    let opt: Opt = Opt::parse();
    logging::init(opt.verbose);
    match opt.command {
        Command::Partition(args) => {
            let mut partitions = args.partitions;
            partition::partition(
                args.device,
                args.mode,
                args.efi,
                &mut partitions,
            );
        }
        Command::InstallBase(args) => {
            base::install_base_packages(args.kernel);
        }
        Command::GenFstab => {
            base::genfstab();
        }
        Command::Bootloader { subcommand } => match subcommand {
            BootloaderSubcommand::LimineEfi => {
                base::install_bootloader_efi();
            }
            BootloaderSubcommand::LimineLegacy { device } => {
                base::install_bootloader_legacy(device);
            }
        },
        Command::Locale(args) => {
            locale::set_locale(args.locales.join(" "));
            locale::set_timezone(&args.timezone);
        }
        Command::Networking(args) => {
            if args.ipv6 {
                network::create_hosts();
                network::enable_ipv6()
            } else {
                network::create_hosts();
            }
            network::set_hostname(&args.hostname);
        }
        Command::Users { subcommand } => match subcommand {
            UsersSubcommand::NewUser(args) => {
                users::new_user(
                    &args.username,
                    args.hasroot,
                    &args.password,
                    true,
                    &args.shell,
                );
            }
            UsersSubcommand::RootPass { password } => {
                users::root_pass(&password);
            }
        },
        Command::Flatpak => {
            base::install_flatpak();
        }
        Command::Config { config } => {
            crate::internal::config::read_config(config);
        }
        Command::Desktops { desktop } => {
            desktops::install_desktop_setup(desktop);
        }
    }
}
