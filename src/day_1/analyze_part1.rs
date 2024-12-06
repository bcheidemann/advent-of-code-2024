use crate::day_1::lists_analyzer_part1::analyze_lists_part1;

use super::{
    error::{AnalyzeError, AnalyzeResult},
    list_parser::parse_lists,
};

pub fn analyze_part1(input: &str) -> AnalyzeResult<u32> {
    let (left_list, right_list) = parse_lists(input).map_err(AnalyzeError::from)?;
    analyze_lists_part1(left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzes_example() {
        let result = analyze_part1(include_str!("data/example.data"));

        assert_eq!(Ok(11), result);
    }
}
