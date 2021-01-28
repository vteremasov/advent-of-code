use std::collections::HashMap;

fn is_increasing_and_has_eq_didgits(num: i64) -> bool {
    let mut tmp = num;
    let mut del = 1;
    let mut res = false;

    while tmp > 0 {
        tmp /= 10;
        del *= 10;
        let prev = (num % del) / (del / 10);
        let next = (num / del) % 10;
        if next > prev {
            return false;
        }
        if next == prev {
            res = true;
        }
    }

    res
}

fn has_2_eq_digits(num: &i64) -> bool {
    let mut tmp = *num;
    let mut accumulator = HashMap::new();

    while tmp > 0 {
        let curr = tmp % 10;
        tmp /= 10;
        if accumulator.contains_key(&curr) {
            accumulator.insert(curr, accumulator.get(&curr).unwrap() + 1);
        } else {
            accumulator.insert(curr, 1);
        }
    }

    accumulator.values().any(|v| *v == 2)
}

pub fn parse_input_day4(input: String) -> (i64, i64) {
    let parsed: Vec<i64> = input
        .split("-")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().expect("Input must be range of numbers"))
        .collect();
    (parsed[0], parsed[1])
}

pub fn find_passwords(range: (i64, i64)) -> Vec<i64> {
    let mut result = vec![];
    for num in range.0..range.1 {
        if is_increasing_and_has_eq_didgits(num) {
            result.push(num);
        }
    }

    result
}

pub fn find_passwords_2_eq(range: (i64, i64)) -> Vec<i64> {
    find_passwords(range)
        .iter()
        .filter(|n| has_2_eq_digits(n))
        .map(|n| *n)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_passwords_2_eq() {
        assert!(has_2_eq_digits(&1123456));
        assert!(has_2_eq_digits(&1245699));
        assert!(!has_2_eq_digits(&12344456));
        assert!(has_2_eq_digits(&112233));
        assert!(!has_2_eq_digits(&123444));
        assert!(has_2_eq_digits(&111122));
        assert!(!has_2_eq_digits(&689999));
    }

    #[test]
    fn test_parse_input_day4() {
        assert_eq!(parse_input_day4("12345-123456".into()), (12345, 123456));
    }

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing_and_has_eq_didgits(112345));
        assert!(is_increasing_and_has_eq_didgits(11344));
        assert_eq!(false, is_increasing_and_has_eq_didgits(10345));
        assert_eq!(false, is_increasing_and_has_eq_didgits(12345));
    }
}
