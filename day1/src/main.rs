use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(&handle_input("input/test.txt")), 7)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&handle_input("input/test.txt")), 5)
}

fn handle_input(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

fn part_1(inp: &str) -> u32 {
    let numbers: Vec<u32> = inp.lines().map(|x| x.parse().unwrap()).collect();
    let start = numbers[0];
    numbers.into_iter().fold((0, start), |(n, prev), cur| if cur > prev {(n + 1, cur)} else {(n, cur)}).0
}

fn part_2(inp: &str) -> u32 {
    let numbers: Vec<u32> = inp.lines().map(|x| x.parse().unwrap()).collect();
    numbers.iter().enumerate().fold(0, |n, (i, cur)| if i >= 3 && cur > &numbers[i - 3] {n + 1} else {n})
}

fn main() {
    let inp = handle_input("input/inp.txt");
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}
