use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio, exit},
    vec,
};

#[derive(Debug, Default)]
struct Device {
    name: String,
    uuid: String,
}

fn main() {
    let mut devices: Vec<Device> = vec![];

    let mut argfiles = std::env::var("NAUTILUS_SCRIPT_SELECTED_FILE_PATHS").unwrap_or_default();
    if let Some(value) = argfiles.trim().strip_suffix('\n') {
        argfiles = value.to_owned();
    }

    let mut files: Vec<&str> = vec![];

    for file in argfiles.split('\n').collect::<Vec<&str>>() {
        if file.trim().is_empty() {
            continue;
        }
        files.push(file)
    }

    if files.is_empty() {
        send_notification("No files passed");
        exit(1);
    }

    let available_devices = String::from_utf8(
        run_command(vec!["kdeconnect-cli", "-a", "-l"]).expect("Failed to get available devices"),
    )
    .unwrap();

    if available_devices.lines().count() < 1 {
        send_notification("No connected devices");
        exit(1);
    }

    for mut device in available_devices.lines() {
        if let Some(value) = device.strip_prefix("- ") {
            device = value;
        }
        let device: Vec<&str> = device.split(":").collect();
        let name = device[0].trim();
        let uuid = device[1].trim().split(" ").collect::<Vec<&str>>()[0];

        devices.push(Device {
            name: String::from(name),
            uuid: String::from(uuid),
        });
    }

    let names = devices
        .iter()
        .map(|device| device.name.clone())
        .collect::<Vec<String>>()
        .join("\n");

    let mut choice: String = String::new();

    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg("Pick a device")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run rofi");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(names.as_bytes())
            .expect("Failed to pass options to rofi");
    }

    let result = child.stdout.take().expect("Failed to grab stdout");
    let reader = BufReader::new(result);

    if let Ok(input) = reader.lines().next().expect("Failed to grab output") {
        choice = input;
    }

    let mut selected_device: Device = Device::default();
    for device in devices {
        if device.name == choice {
            selected_device = device;
        }
    }

    if selected_device.uuid.is_empty() {
        send_notification("Error in selecing device");
        exit(1);
    }

    let mut failed_log: Vec<String> = vec![];

    for file in files.clone() {
        if let Err(err) = run_command(vec![
            "kdeconnect-cli",
            "--share",
            &file,
            "--device",
            &selected_device.uuid,
        ]) {
            failed_log.push(err);
        }
    }

    if !failed_log.is_empty() {
        send_notification(
            format!("The following errors occured: {}", failed_log.join("\n")).as_str(),
        );
        exit(1);
    } else if files.len() == 1 {
        let basename = files[0].split('/').last().unwrap();
        send_notification(format!("Sent {} to {}", basename, selected_device.name).as_str());
    } else {
        send_notification(
            format!(
                "Sent {} file{} to {}",
                files.len(),
                if files.len() > 1 { "s" } else { "" },
                selected_device.name
            )
            .as_str(),
        );
    }
}

fn run_command(cmds: Vec<&str>) -> Result<Vec<u8>, String> {
    let exe: &str = cmds[0];
    let args = &cmds[1..];

    let command = Command::new(exe).args(args).output();

    if command.is_err() {
        return Err(format!(
            "Error in running command: {}",
            command.unwrap_err()
        ));
    }

    Ok(command.unwrap().stdout)
}

fn send_notification(msg: &str) {
    run_command(vec!["notify-send", "KDE Connect", "-i", "kdeconnect", msg])
        .expect(format!("Failed to run notification command in: {}", msg).as_str());
}
