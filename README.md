# Rust Pomodoro Timer

A command-line Pomodoro timer application written in Rust. This application helps you manage your work sessions using the Pomodoro Technique, a time management method that uses a timer to break work into intervals, traditionally 25 minutes in length, separated by short breaks.

## Features

- 20-minute work sessions followed by 5-minute breaks
- Prompts user for the task they're working on before each Pomodoro
- Logs completed Pomodoros to a CSV file
- Flexible timer implementation for easy testing

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/rust-pomodoro-timer.git
   cd rust-pomodoro-timer
   ```

2. Build the project:
   ```
   cargo build --release
   ```

The executable will be created in the `target/release` directory.

## Usage

Run the application using:

```
cargo run --release
```

Or, after building, you can run the executable directly:

```
./target/release/pomodoro_timer
```

Follow the on-screen prompts to start a Pomodoro session or quit the application.

- Enter `start` to begin a new Pomodoro session.
- Enter the task you're working on when prompted.
- The timer will count down for the work session (20 minutes) and then for the break session (5 minutes).
- Completed tasks are logged in `work_done.csv` in the current directory.
- Enter `quit` to exit the application.

## Testing

To run the tests:

```
cargo test
```

This will run both unit tests and integration tests. The tests use mock timers to avoid long waiting times during test execution.

## Project Structure

- `main.rs`: Contains the entire application code, including the `Pomodoro` struct, `Timer` trait, and test modules.
- `Cargo.toml`: Project configuration and dependencies.
- `work_done.csv`: Generated file that logs completed Pomodoro sessions.

## Dependencies

- `chrono`: For timestamp generation
- `csv`: For writing to CSV files

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
