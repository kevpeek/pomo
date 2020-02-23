# Pomo
A simple Pomodoro timer.

![Rust](https://github.com/kevpeek/pomo/workflows/Rust/badge.svg)

## Installation

Install using Cargo:

`cargo install --force --path .`

## Usage

For a 25 minute Pomodoro with 5 minute break, simply run:

`pomo`

```
USAGE:
    pomo [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --duration <duration>    The duration of the pomodoro, in minutes. [default: 25]
```