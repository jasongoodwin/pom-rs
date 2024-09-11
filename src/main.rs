use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use chrono::Local;
use csv::Writer;

struct Pomodoro {
    work_duration: u64,
    break_duration: u64,
    timer: Box<dyn Timer>,
}

trait Timer {
    fn sleep(&self, duration: Duration);
    fn now(&self) -> Instant;
}

struct RealTimer;

impl Timer for RealTimer {
    fn sleep(&self, duration: Duration) {
        thread::sleep(duration);
    }

    fn now(&self) -> Instant {
        Instant::now()
    }
}

impl Pomodoro {
    fn new(timer: Box<dyn Timer>) -> Self {
        Pomodoro {
            work_duration: 20,
            break_duration: 5,
            timer,
        }
    }

    fn run_timer(&self, duration: u64, session_type: &str) {
        let total_duration = Duration::from_secs(duration * 60);
        let start_time = self.timer.now();
        
        while self.timer.now().duration_since(start_time) < total_duration {
            let elapsed = self.timer.now().duration_since(start_time);
            let remaining = total_duration - elapsed;
            let minutes = remaining.as_secs() / 60;
            let seconds = remaining.as_secs() % 60;
            
            print!("\r{} time remaining: {:02}:{:02}", session_type, minutes, seconds);
            io::stdout().flush().unwrap();
            
            self.timer.sleep(Duration::from_secs(1));
        }
        println!("\n{} session completed!", session_type);
    }

    fn get_task(input: &mut dyn io::BufRead, output: &mut dyn io::Write) -> io::Result<String> {
        write!(output, "Enter the task you're working on: ")?;
        output.flush()?;
        let mut task = String::new();
        input.read_line(&mut task)?;
        Ok(task.trim().to_string())
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

    fn start(&self, input: &mut dyn io::BufRead, output: &mut dyn io::Write) -> Result<(), Box<dyn std::error::Error>> {
        let task = Self::get_task(input, output)?;
        writeln!(output, "Starting a Pomodoro for: {}", task)?;
        self.run_timer(self.work_duration, "Work");
        self.log_work(&task)?;
        writeln!(output, "Time for a break!")?;
        self.run_timer(self.break_duration, "Break");
        Ok(())
    }

    fn run(&self, input: &mut dyn io::BufRead, output: &mut dyn io::Write) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            write!(output, "Enter command (start/quit): ")?;
            output.flush()?;
            let mut command = String::new();
            input.read_line(&mut command)?;
            match command.trim() {
                "start" => self.start(input, output)?,
                "quit" => break,
                _ => writeln!(output, "Invalid command. Please enter 'start' or 'quit'.")?,
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Rust Pomodoro Timer!");
    let pomodoro = Pomodoro::new(Box::new(RealTimer));
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    pomodoro.run(&mut input, &mut output)?;
    println!("Thank you for using the Pomodoro Timer. Goodbye!");
    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::cell::Cell;

    struct MockTimer {
        current_time: Cell<Instant>,
    }

    impl MockTimer {
        fn new() -> Self {
            MockTimer {
                current_time: Cell::new(Instant::now()),
            }
        }
    }

    impl Timer for MockTimer {
        fn sleep(&self, duration: Duration) {
            let new_time = self.current_time.get() + duration;
            self.current_time.set(new_time);
        }

        fn now(&self) -> Instant {
            self.current_time.get()
        }
    }

    #[test]
    fn test_new_pomodoro() {
        let pomodoro = Pomodoro::new(Box::new(MockTimer::new()));
        assert_eq!(pomodoro.work_duration, 20);
        assert_eq!(pomodoro.break_duration, 5);
    }

    #[test]
    fn test_log_work() {
        let pomodoro = Pomodoro::new(Box::new(MockTimer::new()));
        let task = "Test task";
        pomodoro.log_work(task).unwrap();

        let content = fs::read_to_string("work_done.csv").unwrap();
        assert!(content.contains(task));
        assert!(content.contains(&pomodoro.work_duration.to_string()));

        // Clean up
        fs::remove_file("work_done.csv").unwrap();
    }

    #[test]
    fn test_get_task() {
        let mut input = io::Cursor::new(b"Test task\n");
        let mut output = Vec::new();
        let task = Pomodoro::get_task(&mut input, &mut output).unwrap();
        assert_eq!(task, "Test task");
    }

    #[test]
    fn test_run_timer() {
        let mock_timer = MockTimer::new();
        let pomodoro = Pomodoro::new(Box::new(mock_timer));
        let mut output = Vec::new();
        
        // Redirect stdout to our output vector
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        
        pomodoro.run_timer(1, "Test");  // 1 minute timer
        
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Test session completed!"));
    }
}

// Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::io::Cursor;

    struct InstantTimer;

    impl Timer for InstantTimer {
        fn sleep(&self, _duration: Duration) {}
        fn now(&self) -> Instant { Instant::now() }
    }

    #[test]
    fn test_full_pomodoro_cycle() {
        let pomodoro = Pomodoro::new(Box::new(InstantTimer));
        
        // Mock user input
        let mut input = Cursor::new(b"start\nTest task\nquit\n");
        let mut output = Vec::new();

        // Run a full cycle
        pomodoro.run(&mut input, &mut output).unwrap();

        // Check output
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Starting a Pomodoro for: Test task"));
        assert!(output_str.contains("Work session completed!"));
        assert!(output_str.contains("Time for a break!"));
        assert!(output_str.contains("Break session completed!"));
    }
}
