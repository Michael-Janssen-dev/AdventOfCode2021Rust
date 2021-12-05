use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("input/test.txt")), 150)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("input/test.txt")), 900)
}

#[derive(Clone)]
enum Move {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn handle_input(file: &str) -> Vec<Move> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|x| x.split_whitespace().collect())
        .map(|x: Vec<&str>| {
            if x[0] == "up" {
                Move::Up(x[1].parse().unwrap())
            } else if x[0] == "down" {
                Move::Down(x[1].parse().unwrap())
            } else {
                Move::Forward(x[1].parse().unwrap())
            }
        })
        .collect()
}

fn part_1(inp: Vec<Move>) -> i32 {
    inp.iter().fold(0, |n, cur| if let Move::Forward(i) = cur {n + i} else {n}) *
     inp.iter().fold(0, |n, cur| if let Move::Up(i) = cur {n - i} else if let Move::Down(i) = cur {n + i} else {n})
}

fn part_2(inp: Vec<Move>) -> i32 {
    let (x, y, _) = inp.iter().fold((0, 0, 0), |(x, y, aim), cur| match cur {
        Move::Forward(i) => (x + i, y + aim * i, aim),
        Move::Up(i) => (x, y, aim - i),
        Move::Down(i) => (x, y, aim + i)
    });
    x * y
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(inp.clone()));
    println!("Part 2: {}", part_2(inp.clone()));
}
