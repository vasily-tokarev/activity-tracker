use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;
use std::env;
use std::process::Command;
use std::io;
use std::process;

fn formatted_number(seconds_left: i32) -> String {
    if seconds_left < 10 {
        return format!("{}{}", "0", seconds_left.to_string());
    } else {
        if seconds_left > 60 {
            return (seconds_left / 60).to_string();
        } else {
            return seconds_left.to_string();
        }
    }
}
fn formatted_time(seconds_left: i32) -> String {
    let remainder: i32 = seconds_left % 60;
    let formatted_minutes: String = if seconds_left < 60 {
        0.to_string()
    } else {
        formatted_number(seconds_left)
    };
    return format!("{}:{}", formatted_minutes, formatted_number(remainder));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: Argument reference and validation.
    // Write activities to file or DB.
    // Tracking window activity?
    let input = &args[1];
    let activity: String = args[2].parse().unwrap();
    let input_time: i32 = input.parse().unwrap();
    let mut seconds_left: i32 = input_time * 60;

    let (timer_tx, timer_rx) = channel();
    let mut paused: bool = false;

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));

            match timer_rx.try_recv() {
                Err(TryRecvError::Disconnected) => {
                    println!("Timer channel has disconnected");
                    break;
                }
                Ok("pause") => {
                    if paused == false {
                        paused = true;
                        println!("Paused {}", activity)
                    } else {
                        paused = false;
                        println!("Resumed {}", activity)
                    }
                }
                Ok(_) => {}
                Err(TryRecvError::Empty) => {}
            }

            seconds_left = if paused {
                seconds_left
            } else {
                seconds_left - 1
            };
            if seconds_left == 0 || seconds_left < 0 {
                let _ = Command::new("terminal-notifier")
                    .arg("-message")
                    .arg(format!("Time is up for {}!", activity))
                    .output()
                    .expect("failed to execute process");
                process::exit(1);
            }

            if !paused {
                // Clean up the screen.
                print!("{}[2J", 27 as char);
                println!("Time left: {}", formatted_time(seconds_left));
            }
        }
    });

    loop {
        let mut input = String::new();
        print!("{}[2J", 27 as char);
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "q" {
                    break;
                } else {
                    timer_tx.send("pause").unwrap();
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
