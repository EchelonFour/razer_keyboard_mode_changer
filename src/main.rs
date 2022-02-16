mod scheduler;

use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use razer_driver_rs::{razer_device::DeviceMode, scan_for_devices, RazerResult};
use scheduler::{schedule_self_if_not_scheduled, uninstall_from_schedule};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("installation")
        .required(false)
        .args(&["install", "uninstall"]),
))]
struct Cli {
    /// Installs this command to the task scheduler to run every wake from sleep
    #[clap(long)]
    install: bool,
    /// Uninstall this command from the system
    #[clap(long)]
    uninstall: bool,
}

fn set_keyboard_to_factory() -> RazerResult<()> {
    let devices = scan_for_devices()?;
    if devices.keyboards.is_empty() {
        println!("Could not find any razer keyboards");
    }
    if devices.keyboards.len() > 1 {
        println!("WARNING: Found more than one keyboard. Setting all to factory testing.");
    }
    for keyboard in devices.keyboards {
        keyboard.set_device_mode(DeviceMode::FactoryTesting)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.install {
        let install_result = schedule_self_if_not_scheduled();
        match install_result {
            Ok(_) => println!("Installed successfully"),
            Err(_) => println!("Failed to install"),
        }
        install_result.context("installing scheduled task")
    } else if cli.uninstall {
        let uninstall_result = uninstall_from_schedule();
        match uninstall_result {
            Ok(_) => println!("Uninstalled successfully"),
            Err(_) => println!("Failed to uninstall"),
        }
        uninstall_result.context("uninstalling scheduled task")
    } else {
        set_keyboard_to_factory().context("sending device mode change")?;
        println!("Device mode changed");
        Ok(())
    }
}
