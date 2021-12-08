use std::convert::TryInto;
use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 26);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 61229);
}

fn handle_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn part_1(inp: String) -> isize {
    inp.lines()
        .map(|x| x.split_once(" | ").unwrap().1)
        .fold(0, |count, line| {
            count
                + line.split_whitespace().fold(0, |accum, x| {
                    if [2, 3, 4, 7].contains(&x.len()) {
                        accum + 1
                    } else {
                        accum
                    }
                })
        })
}

fn union_len(first: &str, second: &str) -> isize {
    return second.chars().fold(
        0,
        |accum, x| if first.contains(x) { accum + 1 } else { accum },
    );
}

fn part_2(inp: String) -> isize {
    let mut count = 0;
    for line in inp.lines() {
        let (left, right) = line.split_once(" | ").unwrap();
        let inputs: Vec<&str> = left.split_whitespace().collect();
        let mut mapping: Vec<&str> = Vec::with_capacity(10);
        for _ in 0..10 {
            mapping.push("");
        }
        mapping[1] = inputs.iter().find(|&x| x.len() == 2).unwrap();
        mapping[4] = inputs.iter().find(|&x| x.len() == 4).unwrap();
        mapping[7] = inputs.iter().find(|&x| x.len() == 3).unwrap();
        mapping[8] = inputs.iter().find(|&x| x.len() == 7).unwrap();
        // println!("{}", union_len(mapping[4], "dabc"));
        mapping[2] = inputs
            .iter()
            .filter(|&x| x.len() == 5)
            .find(|x| union_len(mapping[4], x) == 2)
            .unwrap();
        mapping[3] = inputs
            .iter()
            .filter(|&x| x.len() == 5)
            .find(|x| union_len(mapping[1], x) == 2)
            .unwrap();
        mapping[5] = inputs
            .iter()
            .filter(|&x| x.len() == 5)
            .find(|x| x != &&mapping[2] && x != &&mapping[3])
            .unwrap();
        mapping[6] = inputs
            .iter()
            .filter(|&x| x.len() == 6)
            .find(|x| union_len(mapping[1], x) == 1)
            .unwrap();
        mapping[9] = inputs
            .iter()
            .filter(|&x| x.len() == 6)
            .find(|x| union_len(mapping[4], x) == 4)
            .unwrap();
        mapping[0] = inputs
            .iter()
            .filter(|&x| x.len() == 6)
            .find(|x| x != &&mapping[6] && x != &&mapping[9])
            .unwrap();
        for (i, wires) in right.split_whitespace().rev().enumerate() {

            let pos: isize = mapping.iter().position(|x| union_len(x, wires) == wires.len().max(x.len()).try_into().unwrap()).unwrap().try_into().unwrap();
            count += 10_isize.pow(i.try_into().unwrap()) * pos
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("input.txt")));
    println!("Part 2: {}", part_2(handle_input("input.txt")));
}
