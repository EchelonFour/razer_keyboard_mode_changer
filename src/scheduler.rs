use anyhow::{anyhow, Context, Result};
use std::env::current_exe;
use std::process::Command;
use tempfile::NamedTempFile;

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
    let task_xml_contents = format!(
        include_str!("task.xml"),
        exe_path = exe_path.to_str().context("os path was not valid unicode")?,
    );
    let task_xml_file = NamedTempFile::new().context("could not make temp file")?;
    std::fs::write(&task_xml_file, task_xml_contents).context("could not write to temp file")?;
    let task_xml_file_path = task_xml_file.into_temp_path();
    let task_xml_file_path_str = task_xml_file_path
        .to_str()
        .context("os path was not valid unicode")?;
    println!("path: {}", task_xml_file_path_str);
    let task_deleted = delete_existing_task()?;
    if task_deleted {
        println!("Deleted existing task to reinstall new one")
    }
    let output = Command::new("schtasks")
        .args(["/create", "/tn", TASK_NAME, "/xml", task_xml_file_path_str])
        .output()
        .context("launching task scheduler")?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "failed to add task because: {}\n{}",
            String::from_utf8(output.stdout)?,
            String::from_utf8(output.stderr)?
        ))
    }
}
