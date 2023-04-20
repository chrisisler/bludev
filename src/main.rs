use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct PairedDevice {
    /// The Device ID
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    recent_access_date: Option<String>,
    favourite: bool,
    name: String,
    /// `true` if the device is connected
    connected: bool,
    /// Always true
    /// Unpair = "Forget This Device"
    paired: bool,
}

// List devices 0..N
// Get input character (0..N) is the selected device
// Display actions for that device (pair, unpair, disconnect, etc)
fn main() -> Result<(), serde_json::Error> {
    // Run output
    let output = Command::new("blueutil")
        .arg("--version")
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("failed to execute process");

    // Ensure dependency is installed or fail with message
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(output.status.code().unwrap_or(1));
    }

    // Run command and get output
    let list = Command::new("blueutil")
        .arg("--paired")
        .arg("--format")
        .arg("json")
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("failed to execute process list");

    // Convert output to string(ish)
    let str = String::from_utf8_lossy(&list.stdout);

    // Convert string to mutable list of objs
    let mut devices: Vec<PairedDevice> = serde_json::from_str(&str).unwrap();

    // Sort by connected first
    devices.sort_by(|a, b| a.connected.cmp(&b.connected).reverse());

    // List devices 0..N
    // Get input character (0..N) is the selected device
    // Display actions for that device (pair, unpair, disconnect, etc)
    println!("Bluetooth Devices");

    for i in 0..devices.len() {
        print!("{})", i + 1);
        print!("{}", if devices[i].connected { " x " } else { "   " });
        println!("{:?}", devices[i].name);
    }

    let selected_index = get_stdin_number();

    // Get selected device
    let device: PairedDevice = devices.into_iter().nth(selected_index - 1).unwrap();

    println!("Selected Device: {:?}", device.name);

    // Display actions for selected device
    println!(
        "1) {}",
        if device.connected {
            "Disconnect"
        } else {
            "Connect"
        }
    );
    println!("0) Forget This Device");

    let action = get_stdin_number();

    match action {
        1 => {
            // Connect / Disconnect device
            Command::new("blueutil")
                .arg(if device.connected {
                    "--disconnect"
                } else {
                    "--connect"
                })
                .arg(&device.address)
                .output()
                .expect("failed to execute process");
            println!(
                "Successfully {}.",
                if device.connected {
                    "disconnected"
                } else {
                    "connected"
                }
            );
        }
        0 => {
            // Forget device
            Command::new("blueutil")
                .arg("--unpair")
                .arg(&device.address)
                .output()
                .expect("failed to execute process");
            println!("Successfully unpaired");
        }
        _ => unreachable!(),
    };

    Ok(())
}

fn get_stdin_number() -> usize {
    let mut input_char = String::new();
    std::io::stdin().read_line(&mut input_char).unwrap();
    input_char.trim().parse::<usize>().unwrap()
}
