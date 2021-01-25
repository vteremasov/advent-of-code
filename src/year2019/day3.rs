#[derive(Debug, Clone, PartialEq)]
pub struct Navigation {
    dir: String,
    steps: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}

fn build_route(nav: Vec<Navigation>) -> Vec<Point> {
    let mut vector = vec![Point { x: 0, y: 0 }];
    let mut counter = 0;
    for p in nav {
        for _ in 0..p.steps {
            match p.dir.as_str() {
                "R" => vector.push(Point {
                    y: vector[counter].y,
                    x: vector[counter].x + 1,
                }),
                "L" => vector.push(Point {
                    y: vector[counter].y,
                    x: vector[counter].x - 1,
                }),
                "U" => vector.push(Point {
                    y: vector[counter].y + 1,
                    x: vector[counter].x,
                }),
                "D" => vector.push(Point {
                    y: vector[counter].y - 1,
                    x: vector[counter].x,
                }),
                d => eprintln!("Unknown direction: {}", d),
            };
            counter += 1;
        }
    }

    vector
}

fn find_crossing(v1: Vec<Point>, v2: Vec<Point>) -> Vec<Point> {
    let mut result = vec![];
    let zero_point = Point { x: 0, y: 0 };
    for one in &v1 {
        for two in &v2 {
            if one == two && one != &zero_point {
                result.push(one.clone());
            }
        }
    }

    result
}

fn parse_vec(input: String) -> Vec<Navigation> {
    input
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| String::from(s))
        .map(|s| Navigation {
            dir: s[0..1].to_owned(),
            steps: s[1..].parse::<i64>().unwrap(),
        })
        .collect::<Vec<Navigation>>()
}

pub fn parse_input_day3(input: String) -> (Vec<Navigation>, Vec<Navigation>) {
    let splitted = input
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    (
        parse_vec(splitted[0].clone()),
        parse_vec(splitted[1].clone()),
    )
}

pub fn get_closest_dist(data: (Vec<Navigation>, Vec<Navigation>)) -> i64 {
    let v1 = build_route(data.0);
    let v2 = build_route(data.1);
    let crossings = find_crossing(v1, v2);

    let mut closest = std::i64::MAX;

    for cross in crossings {
        if cross.x.abs() + cross.y.abs() < closest {
            closest = cross.x.abs() + cross.y.abs();
        }
    }

    closest
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn test_find_crossings() {
        let input1 = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 2 },
            Point { x: -1, y: 3 },
            Point { x: -1, y: -1 },
            Point { x: -1, y: -2 },
        ];

        let input2 = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: -1, y: 3 },
            Point { x: -1, y: 2 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: -1 },
            Point { x: -1, y: -2 },
        ];

        let res = find_crossing(input1, input2);

        assert_eq!(
            res,
            vec![
                Point { x: 1, y: 0 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: -1, y: 2 },
                Point { x: -1, y: 3 },
                Point { x: -1, y: -1 },
                Point { x: -1, y: -2 },
            ]
        );
    }

    #[test]
    fn test_build_route() {
        let input = vec![
            Navigation {
                dir: "R".to_owned(),
                steps: 1,
            },
            Navigation {
                dir: "L".to_owned(),
                steps: 2,
            },
            Navigation {
                dir: "U".to_owned(),
                steps: 3,
            },
            Navigation {
                dir: "D".to_owned(),
                steps: 5,
            },
        ];
        let res = build_route(input);
        assert_eq!(
            res,
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 0 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: -1, y: 2 },
                Point { x: -1, y: 3 },
                Point { x: -1, y: 2 },
                Point { x: -1, y: 1 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: -1 },
                Point { x: -1, y: -2 },
            ]
        );
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
