use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use chrono::Local;
use csv::Writer;

struct Pomodoro {
    work_duration: u64,
    break_duration: u64,
}

impl Pomodoro {
    fn new() -> Self {
        Pomodoro {
            work_duration: 20,
            break_duration: 5,
        }
    }

    fn run_timer(&self, duration: u64, session_type: &str) {
        let total_seconds = duration * 60;
        for remaining in (0..total_seconds).rev() {
            let minutes = remaining / 60;
            let seconds = remaining % 60;
            print!("\r{} time remaining: {:02}:{:02}", session_type, minutes, seconds);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("\n{} session completed!", session_type);
    }

    fn get_task() -> String {
        print!("Enter the task you're working on: ");
        io::stdout().flush().unwrap();
        let mut task = String::new();
        io::stdin().read_line(&mut task).unwrap();
        task.trim().to_string()
    }

    fn log_work(&self, task: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = Writer::from_path("work_done.csv")?;
        wtr.write_record(&[
            task,
            &self.work_duration.to_string(),
            &Local::now().to_string(),
        ])?;
        wtr.flush()?;
        Ok(())
    }

    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let task = Self::get_task();
        println!("Starting a Pomodoro for: {}", task);
        self.run_timer(self.work_duration, "Work");
        self.log_work(&task)?;
        println!("Time for a break!");
        self.run_timer(self.break_duration, "Break");
        Ok(())
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            print!("Enter command (start/quit): ");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            match command.trim() {
                "start" => self.start()?,
                "quit" => break,
                _ => println!("Invalid command. Please enter 'start' or 'quit'."),
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Rust Pomodoro Timer!");
    let pomodoro = Pomodoro::new();
    pomodoro.run()?;
    println!("Thank you for using the Pomodoro Timer. Goodbye!");
    Ok(())
}
