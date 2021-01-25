use std::process;

pub fn parse_input_day2(input: String) -> Vec<i64> {
    input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.trim()
                .parse::<i64>()
                .expect("Input must be numbers splitted by comma")
        })
        .collect()
}

pub fn calc(opcode: i64, v1: i64, v2: i64) -> i64 {
    match opcode {
        1 => v1 + v2,
        2 => v1 * v2,
        oc => {
            eprintln!("Unknown opcode {} in calculation computer", oc);
            process::exit(1);
        }
    }
}

pub fn eloop(mut state: Vec<i64>) -> Vec<i64> {
    let size = state.len();
    let mut ipointer = 0;

    while ipointer < size {
        if ipointer > size || state[ipointer] == 99 {
            return state;
        }

        let to = state[ipointer + 3] as usize;
        let v1_idx = state[ipointer + 1] as usize;
        let v2_idx = state[ipointer + 2] as usize;

        state[to] = calc(state[ipointer], state[v1_idx], state[v2_idx]);

        ipointer += 4;
    }

    state
}

pub fn computer_run(state: Vec<i64>) -> i64 {
    eloop(state)[0]
}

pub fn computer_run_find(state: Vec<i64>, num: i64) -> i64 {
    let mut s;
    for verb in 0..100 {
        for noun in 0..100 {
            s = state.clone();
            s[1] = verb;
            s[2] = noun;
            let result = computer_run(s);
            if result == num {
                return 100 * verb + noun;
            }
        }
    }
    0
}

pub fn restore_state1202(mut state: Vec<i64>) -> Vec<i64> {
    state[1] = 12;
    state[2] = 2;
    state
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eloop() {
        let input = vec![1, 0, 2, 1, 99, 1, 2, 3];
        let res = eloop(input);
        assert_eq!(res, vec![1, 3, 2, 1, 99, 1, 2, 3]);
    }

    #[test]
    fn test_eloop1() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let res = eloop(input);
        assert_eq!(res, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(eloop(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(eloop(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(eloop(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            eloop(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_parse_input_day2() {
        let input = "1,2,3,4, 5,6 , 7, 8";
        let res = parse_input_day2(input.into());
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
