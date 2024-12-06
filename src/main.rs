use advent_of_code_2024::day_1::command::Day1Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Command {
    /// Day 1
    Day1(Day1Command),
}

impl Command {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Day1(command) => command.run(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    Command::parse().run()
}
