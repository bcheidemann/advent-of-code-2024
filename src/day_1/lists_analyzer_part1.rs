use crate::day_1::error::AnalyzeError;

use super::{error::AnalyzeResult, list_parser::List};

pub fn analyze_lists_part1(mut left_list: List, mut right_list: List) -> AnalyzeResult<u32> {
    left_list.sort();
    right_list.sort();

    let diffs = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.0.abs_diff(r.0));
    let mut sum: u32 = 0;

    for diff in diffs {
        match sum.overflowing_add(diff) {
            (_, true) => return Err(AnalyzeError::IntegerOverflow),
            (new_sum, _) => sum = new_sum,
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::day_1::list_parser::LocationId;

    use super::*;

    #[test]
    fn analyzes_lists() {
        let left_list = vec![LocationId(2), LocationId(3)];
        let right_list = vec![LocationId(4), LocationId(1)];

        let result = analyze_lists_part1(left_list, right_list);

        assert_eq!(Ok(2), result);
    }

    #[test]
    fn handles_overflows() {
        let left_list = vec![LocationId(u32::MAX), LocationId(u32::MAX)];
        let right_list = vec![LocationId(0), LocationId(0)];

        let result = analyze_lists_part1(left_list, right_list);

        assert_eq!(Err(AnalyzeError::IntegerOverflow), result);
    }
}
