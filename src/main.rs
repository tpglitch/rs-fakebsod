use std::process::Command;

fn main() {
    let edge_path = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";

    let args = [
        "--kiosk",
        "https://fakebsod.com/generic/",
        "--edge-kiosk-type=fullscreen",
        "--no-first-run",
    ];

    let status = Command::new(edge_path)
        .args(&args)
        .status()
        .expect("Failed to launch Microsoft Edge in kiosk mode");

    println!("Microsoft Edge exited with status: {:?}", status);
}
