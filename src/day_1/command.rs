use std::path::PathBuf;

use anyhow::Context as _;
use clap::Args;

use crate::day_1::{analyze_part1::analyze_part1, analyze_part2::analyze_part2};

#[derive(Args, Debug)]
pub struct Day1Command {
    #[arg()]
    /// Path to input file
    input: PathBuf,
}

impl Day1Command {
    pub fn run(&self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.input).context("Failed to read file")?;
        let result_part_1 = analyze_part1(&input).context("Failed to analyze file")?;
        let result_part_2 = analyze_part2(&input).context("Failed to analyze file")?;

        println!("result (part 1) = {result_part_1}");
        println!("result (part 2) = {result_part_2}");
        Ok(())
    }
}
