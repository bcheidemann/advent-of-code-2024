use super::error::{ParseError, ParseResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationId(pub(crate) u32);

pub type List = Vec<LocationId>;

pub fn parse_lists(input: &str) -> ParseResult<(List, List)> {
    let mut left_list = Vec::<LocationId>::new();
    let mut right_list = Vec::<LocationId>::new();

    let lines = input
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty());

    for line in lines {
        let mut location_ids = line.split_whitespace().take(2);
        let left_location_id = location_ids.next();
        let right_location_id = location_ids.next();

        let (left_location_id, right_location_id) = match (left_location_id, right_location_id) {
            (Some(l), Some(r)) => (l, r),
            _ => {
                return Err(ParseError::InvalidRow {
                    row: line.to_string(),
                })
            }
        };

        let (left_location_id, right_location_id) = match (
            left_location_id.parse::<u32>(),
            right_location_id.parse::<u32>(),
        ) {
            (Ok(l), Ok(r)) => (l, r),
            (Err(err), _) => {
                return Err(ParseError::InvalidLocationId {
                    location_id: left_location_id.to_string(),
                    reason: err.to_string(),
                })
            }
            (_, Err(err)) => {
                return Err(ParseError::InvalidLocationId {
                    location_id: right_location_id.to_string(),
                    reason: err.to_string(),
                })
            }
        };

        left_list.push(LocationId(left_location_id));
        right_list.push(LocationId(right_location_id));
    }

    Ok((left_list, right_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_no_data() {
        let input = r#""#;

        let result = parse_lists(input);

        assert_eq!(Ok((vec![], vec![])), result);
    }

    #[test]
    fn parses_valid_data() {
        let input = r#"
            1 2
            3 4
            5 6
        "#;

        let result = parse_lists(input);

        assert_eq!(
            Ok((
                vec![LocationId(1), LocationId(3), LocationId(5),],
                vec![LocationId(2), LocationId(4), LocationId(6),]
            )),
            result
        );
    }

    #[test]
    fn invalid_row() {
        let input = r#"
            1 2
            3
            4 5
        "#;

        let result = parse_lists(input);

        assert_eq!(
            Err(ParseError::InvalidRow {
                row: "3".to_string()
            }),
            result
        );
    }

    #[test]
    fn invalid_location_id() {
        let input = r#"
            1 2
            invalid invalid
            4 5
        "#;

        let result = parse_lists(input);

        assert_eq!(
            Err(ParseError::InvalidLocationId {
                location_id: "invalid".to_string(),
                reason: "invalid digit found in string".to_string(),
            }),
            result
        );
    }
}
