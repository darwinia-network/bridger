use std::io::{self, prelude::*};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

fn main() {
    let mut start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut lines: Vec<String>= vec![];
    for line in io::stdin().lock().lines() {
        let line_real = line.unwrap();
        println!("{}", line_real);
        lines.push(line_real);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if (now - start) > Duration::from_millis(5000) { // check every 5 seconds
            if errors_count(lines) > 10 {
                use std::process::Command;
                let _output = Command::new("pkill").arg("-f").arg("bridger").output().unwrap();
                // let result = String::from_utf8(output.stdout);
                println!("Bridger stopped.");
                break
            } else {
                start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                lines = vec![];
            }
        }
    }
}

fn errors_count(lines: Vec<String>) -> i32 {
    let mut count = 0;
    for line in lines {
        if line.contains("Client Error:") {
            count += 1;
        }
    }
    count
}
