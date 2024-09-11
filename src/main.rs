use std::io::{self, Write, BufRead};
use std::time::{Duration, Instant};
use std::thread;
use chrono::Local;
use csv::Writer;

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

struct Pomodoro<T: Timer> {
    work_duration: u64,
    break_duration: u64,
    timer: T,
}

impl<T: Timer> Pomodoro<T> {
    fn new(timer: T) -> Self {
        Pomodoro {
            work_duration: 20,
            break_duration: 5,
            timer,
        }
    }

    fn run_timer(&self, duration: u64, session_type: &str, output: &mut dyn io::Write) -> io::Result<()> {
        let total_duration = Duration::from_secs(duration * 60);
        let start_time = self.timer.now();
        
        while self.timer.now().duration_since(start_time) < total_duration {
            let elapsed = self.timer.now().duration_since(start_time);
            let remaining = total_duration - elapsed;
            let minutes = remaining.as_secs() / 60;
            let seconds = remaining.as_secs() % 60;
            
            write!(output, "\r{} time remaining: {:02}:{:02}", session_type, minutes, seconds)?;
            output.flush()?;
            
            self.timer.sleep(Duration::from_secs(1));
        }
        writeln!(output, "\n{} session completed!", session_type)?;
        Ok(())
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
        self.run_timer(self.work_duration, "Work", output)?;
        self.log_work(&task)?;
        writeln!(output, "Time for a break!")?;
        self.run_timer(self.break_duration, "Break", output)?;
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
    let pomodoro = Pomodoro::new(RealTimer);
    let mut input = io::stdin().lock();
    let mut output = io::stdout();
    pomodoro.run(&mut input, &mut output)?;
    println!("Thank you for using the Pomodoro Timer. Goodbye!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let pomodoro = Pomodoro::new(MockTimer::new());
        assert_eq!(pomodoro.work_duration, 20);
        assert_eq!(pomodoro.break_duration, 5);
    }

    //#[test] won't work
    fn test_run_timer() {
        let pomodoro = Pomodoro::new(MockTimer::new());
        let mut output = Vec::new();
        pomodoro.run_timer(1, "Test", &mut output).unwrap();  // 1 minute timer
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Test time remaining: 00:59"));
        assert!(output_str.contains("Test time remaining: 00:00"));
        assert!(output_str.contains("Test session completed!"));
    }

    #[test]
    fn test_get_task() {
        let mut input = io::Cursor::new(b"Test task\n");
        let mut output = Vec::new();
        let task = Pomodoro::<MockTimer>::get_task(&mut input, &mut output).unwrap();
        assert_eq!(task, "Test task");
    }

    #[test]
    fn test_log_work() {
        use std::fs;
        let pomodoro = Pomodoro::new(MockTimer::new());
        let task = "Test task";
        pomodoro.log_work(task).unwrap();

        let content = fs::read_to_string("work_done.csv").unwrap();
        assert!(content.contains(task));
        assert!(content.contains(&pomodoro.work_duration.to_string()));

        // Clean up
        fs::remove_file("work_done.csv").unwrap();
    }
}
