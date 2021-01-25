pub fn calc_fuel(mass: Vec<f64>) -> u64 {
    mass.iter().map(|n| (n / 3.0).floor() as u64 - 2).sum()
}

pub fn parse_input(input: String) -> Vec<f64> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<f64>()
                .expect("input must be string with numbers splitted with a new line")
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input1() {
        let input = "
1
2
3
4";
        let res = parse_input(input.into());

        assert_eq!(res, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test12() {
        let input = vec![12.0];
        let res = calc_fuel(input);
        assert_eq!(res, 2);
    }

    #[test]
    fn test14() {
        let input = vec![14.0];
        let res = calc_fuel(input);
        assert_eq!(res, 2);
    }
    #[test]
    fn test1968() {
        let input = vec![1969.0];
        let res = calc_fuel(input);
        assert_eq!(res, 654);
    }
    #[test]
    fn test100756() {
        let input = vec![100756.0];
        let res = calc_fuel(input);
        assert_eq!(res, 33583);
    }
}
