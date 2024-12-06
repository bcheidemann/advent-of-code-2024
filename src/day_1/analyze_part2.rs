use std::collections::HashMap;

use super::{
    error::{AnalyzeError, AnalyzeResult},
    list_parser::{parse_lists, LocationId},
};

pub fn analyze_part2(input: &str) -> AnalyzeResult<u32> {
    let (left_list, right_list) = parse_lists(input).map_err(AnalyzeError::from)?;

    let mut right_list_counts = HashMap::<LocationId, u32>::new();

    for location_id in right_list {
        if let Some(count) = right_list_counts.get_mut(&location_id) {
            match count.overflowing_add(1) {
                (_, true) => return Err(AnalyzeError::IntegerOverflow),
                (new_count, false) => *count = new_count,
            }
        } else {
            right_list_counts.insert(location_id, 1);
        }
    }

    // Doesn't need to be mutable anymore
    let right_list_counts = right_list_counts;

    let mut similarity_score = 0_u32;
    for LocationId(id) in left_list {
        let right_list_count = right_list_counts.get(&LocationId(id)).unwrap_or(&0);

        let increment = match id.overflowing_mul(*right_list_count) {
            (_, true) => return Err(AnalyzeError::IntegerOverflow),
            (increment, false) => increment,
        };

        match similarity_score.overflowing_add(increment) {
            (_, true) => return Err(AnalyzeError::IntegerOverflow),
            (increment, false) => {
                similarity_score = increment;
            }
        }
    }

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzes_example() {
        let result = analyze_part2(include_str!("data/example.data"));

        assert_eq!(Ok(31), result);
    }
}
