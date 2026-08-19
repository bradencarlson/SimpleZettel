use std::process::{Command,Stdio};
fn main() {
    let cmd = Command::new("vim")
        .status()
        .expect("Failed to start vim");
}
