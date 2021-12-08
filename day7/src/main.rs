use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 37);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 168);
}


fn part_1(inp: Vec<isize>) -> isize {
    (inp.iter().min().unwrap().clone()..inp.iter().max().unwrap().clone()).map(|i| inp.iter().fold(0, |accum, item| accum + (item - i).abs())).min().unwrap()
}

fn part_2(inp: Vec<isize>) -> isize {
    (inp.iter().min().unwrap().clone()..inp.iter().max().unwrap().clone()).map(|i| inp.iter().fold(0, |accum, item| accum + (item - i).abs() * (((item - i).abs()) + 1) / 2).abs()).min().unwrap()
}

fn handle_input(path: &str) -> Vec<isize> {
    fs::read_to_string(path).unwrap().split(",").map(|x| x.parse().unwrap()).collect()
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("inp.txt")));
    println!("Part 2: {}", part_2(handle_input("inp.txt")));
}
