use chrono::Local;
use colored::Colorize;
use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::{Duration, Instant};
fn main() {
    let target_addr: SocketAddr = "1.1.1.1:53".parse().unwrap();
    let timeout = Duration::from_secs(1);
    let mut last_status: Option<bool> = None;
    loop {
        let start_time = Instant::now();
        let is_success = TcpStream::connect_timeout(&target_addr, timeout).is_ok();
        if last_status != Some(is_success) {
            let timestamp = Local::now().format("%H:%M:%S").to_string();
            if is_success {
                println!("{}", format!("[{}] True", timestamp).green());
            } else {
                println!("{}", format!("[{}] False", timestamp).red());
            }
            last_status = Some(is_success);
        }
        let elapsed = start_time.elapsed();
        if elapsed < Duration::from_secs(1) {
            sleep(Duration::from_secs(1) - elapsed);
        }
    }
}
