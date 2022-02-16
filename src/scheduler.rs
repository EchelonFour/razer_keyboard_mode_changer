use anyhow::{anyhow, Context, Result};
use std::env::current_exe;
use std::process::Command;

const TASK_NAME: &str = "razer_keyboard_mode_changer";

fn delete_existing_task() -> Result<bool> {
    let delete_status = Command::new("schtasks")
        .args(["/delete", "/tn", TASK_NAME, "/f"])
        .output()
        .context("deleting existing task")?;
    Ok(delete_status.status.success())
}

pub fn uninstall_from_schedule() -> Result<()> {
    let task_deleted = delete_existing_task()?;
    if task_deleted {
        println!("Deleted existing task")
    } else {
        println!("Could not find existing task to uninstall")
    }
    Ok(())
}

pub fn schedule_self_if_not_scheduled() -> Result<()> {
    let exe_path = current_exe().context("getting current exe path")?;
    let task_deleted = delete_existing_task()?;
    if task_deleted {
        println!("Deleted existing task to reinstall new one")
    }
    let output = Command::new("schtasks")
        .args([
            "/create",
            "/sc",
            "onevent",
            "/mo",
            "*[System[(EventID=42)]]",
            "/EC",
            "System",
            "/tn",
            TASK_NAME,
            "/tr",
            exe_path.to_str().context("path was not valid unicode")?,
        ])
        .output()
        .context("launching task scheduler")?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "failed to add task because {}",
            String::from_utf8(output.stdout)?
        ))
    }
}
