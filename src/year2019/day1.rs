pub fn calc(needed: &f64) -> i64 {
    (needed / 3.0).floor() as i64 - 2
}

pub fn calc_fuel(mass: Vec<f64>) -> i64 {
    mass.iter().map(|n| calc(n)).sum()
}

pub fn calc_additional_fuel(mass: Vec<f64>) -> i64 {
    mass.iter()
        .map(|m| calc_fuel_for_fuel(calc(m)))
        .flatten()
        .sum()
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

pub fn calc_fuel_for_fuel(needed: i64) -> Vec<i64> {
    if needed <= 0 {
        return vec![0];
    }

    let mut result = vec![needed];

    result.extend(&calc_fuel_for_fuel(calc(&(needed as f64))));

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_additional_fuel() {
        let input = vec![100756.0];
        let res = calc_additional_fuel(input);
        assert_eq!(res, 50346);
    }

    #[test]
    fn test_additional_fuel1() {
        let input = vec![1969.0];
        let res = calc_additional_fuel(input);
        assert_eq!(res, 966);
    }

    #[test]
    fn test_additional_fuel2() {
        let input = vec![100756.0, 1969.0];
        let res = calc_additional_fuel(input);
        assert_eq!(res, 51312);
    }

    #[test]
    fn test_additional_fuel3() {
        let input = vec![100756.0, 1969.0, 14.0, 12.0];
        let res = calc_additional_fuel(input);
        assert_eq!(res, 51316);
    }

    #[test]
    fn test_f_for_f() {
        let input = 2;
        let res = calc_fuel_for_fuel(input);
        assert_eq!(res, vec![2, 0]);
    }

    #[test]
    fn test_f_for_f1() {
        let input = 33583;
        let res = calc_fuel_for_fuel(input);
        assert_eq!(res, vec![33583, 11192, 3728, 1240, 411, 135, 43, 12, 2, 0]);
    }

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
