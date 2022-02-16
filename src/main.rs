mod scheduler;

use clap::Parser;
use razer_driver_rs::{razer_device::DeviceMode, scan_for_devices, RazerResult};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Installs this command to the task scheduler
    #[clap(short, long)]
    install: bool,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.install {
        let install_result = scheduler::schedule_self_if_not_scheduled();
        match install_result {
            Ok(_) => println!("Installed successfully"),
            Err(_) => println!("Failed to install"),
        }
        return install_result;
    }
    set_keyboard_to_factory()?;
    Ok(())
}
