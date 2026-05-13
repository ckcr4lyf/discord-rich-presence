use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new(&std::env::args().collect::<Vec<String>>()[1]);

    client.connect()?;
    client.set_activity(activity::Activity::new().state("foo").details("bar"))?;

    println!("Activity set! Press enter to exit...");

    #[cfg(target_os = "windows")]
    let output = std::process::Command::new("tasklist")
        .args(["/fo", "csv", "/nh"])
        .output()
        .unwrap();
    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new("ps")
        .args(["-eo", "pid,comm"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    let mut dummy = String::new();
    match std::io::stdin().read_line(&mut dummy) {
        _ => (),
    }

    println!("Clearing activity...");
    client.clear_activity()?;

    println!("Activity cleared! Going to exit...");
    match client.close() {
        Ok(()) => (),
        Err(err) => println!("Error closing IPC connection: {}", err),
    }

    Ok(())
}
