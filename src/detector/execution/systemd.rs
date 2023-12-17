use std::process::Command;

pub fn start_device_setup_impl() {
    match Command::new("/usr/bin/systemctl").args(["start", "device-setup.service"]).output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("'systemctl' returned a failure code {}: {}", output.status, std::str::from_utf8(&output.stdout).unwrap_or("ERROR: NOT UTF-8"));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute 'systemctl': {}", e);
        }

    }
}

pub fn stop_device_setup_impl() {
    match Command::new("/usr/bin/systemctl").args(["stop","device-setup.service"]).output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("'systemctl' returned a failure code {}: {}", output.status, std::str::from_utf8(&output.stdout).unwrap_or("ERROR: NOT UTF-8"));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute 'systemctl': {}", e);
        }

    }
}