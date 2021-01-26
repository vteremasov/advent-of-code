use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Navigation {
    dir: String,
    steps: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}

fn build_route(nav: Vec<Navigation>) -> HashMap<Point, i64> {
    let mut result = HashMap::new();
    let mut distance = 0;
    let mut x = 0;
    let mut y = 0;
    for p in nav {
        for _ in 0..p.steps {
            match p.dir.as_str() {
                "R" => x += 1,
                "L" => x -= 1,
                "U" => y += 1,
                "D" => y -= 1,
                d => eprintln!("Unknown direction: {}", d),
            };
            distance += 1;
            let p = Point { x, y };
            if !result.contains_key(&p) {
                result.insert(p, distance);
            }
        }
    }

    result
}

fn find_crossing(v1: &HashMap<Point, i64>, v2: &HashMap<Point, i64>) -> Vec<Point> {
    let mut result = vec![];
    for (one, _) in v1 {
        if v2.contains_key(one) {
            result.push(one.clone());
        }
    }

    result
}

fn parse_navigation(input: String) -> Vec<Navigation> {
    input
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| String::from(s))
        .map(|s| Navigation {
            dir: s[0..1].to_owned(),
            steps: s[1..].parse::<i64>().expect(&format!(
                "Expected digit but got: {} with len: {}",
                s,
                s.len()
            )),
        })
        .collect::<Vec<Navigation>>()
}

pub fn parse_input_day3(input: String) -> (Vec<Navigation>, Vec<Navigation>) {
    let splitted = input
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    (
        parse_navigation(splitted[0].clone()),
        parse_navigation(splitted[1].clone()),
    )
}

pub fn get_closest_dist(data: (Vec<Navigation>, Vec<Navigation>)) -> i64 {
    let v1 = build_route(data.0);
    let v2 = build_route(data.1);
    let crossings = find_crossing(&v1, &v2);

    let mut closest = std::i64::MAX;

    for cross in crossings {
        if cross.x.abs() + cross.y.abs() < closest {
            closest = cross.x.abs() + cross.y.abs();
        }
    }

    closest
}

pub fn get_fewest_steps(data: (Vec<Navigation>, Vec<Navigation>)) -> i64 {
    let v1 = build_route(data.0);
    let v2 = build_route(data.1);
    let crossings = find_crossing(&v1, &v2);
    let mut fewest = std::i64::MAX;
    for cross in crossings {
        let dist = v1.get(&cross).unwrap() + v2.get(&cross).unwrap();
        if fewest > dist {
            fewest = dist;
        }
    }
    fewest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_fewest_steps() {
        let input = "R8,U5,L5,D3
        U7,R6,D4,L4";
        let res = get_fewest_steps(parse_input_day3(input.into()));
        assert_eq!(res, 30);
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";
        let res = get_fewest_steps(parse_input_day3(input.into()));
        assert_eq!(res, 610);
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let res = get_fewest_steps(parse_input_day3(input.into()));
        assert_eq!(res, 410);
        let parsed = parse_input_day3(input.into());
        let crossings = find_crossing(&build_route(parsed.0), &build_route(parsed.1));
        assert_eq!(crossings.len(), 5);
    }

    #[test]
    fn test_get_nearrest() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";
        let res = get_closest_dist(parse_input_day3(input.into()));
        assert_eq!(res, 159);

        let input1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let res = get_closest_dist(parse_input_day3(input1.into()));
        assert_eq!(res, 135);
    }

    #[test]
    fn test_parse_input() {
        let input = "R1,L25,U33,D45
        L35,U44,U33,L221,D2,R8";
        let res = parse_input_day3(input.to_string());
        assert_eq!(
            res,
            (
                vec![
                    Navigation {
                        dir: "R".to_owned(),
                        steps: 1
                    },
                    Navigation {
                        dir: "L".to_owned(),
                        steps: 25
                    },
                    Navigation {
                        dir: "U".to_owned(),
                        steps: 33
                    },
                    Navigation {
                        dir: "D".to_owned(),
                        steps: 45
                    }
                ],
                vec![
                    Navigation {
                        dir: "L".to_owned(),
                        steps: 35
                    },
                    Navigation {
                        dir: "U".to_owned(),
                        steps: 44
                    },
                    Navigation {
                        dir: "U".to_owned(),
                        steps: 33
                    },
                    Navigation {
                        dir: "L".to_owned(),
                        steps: 221
                    },
                    Navigation {
                        dir: "D".to_owned(),
                        steps: 2
                    },
                    Navigation {
                        dir: "R".to_owned(),
                        steps: 8
                    }
                ]
            )
        )
    }
}
