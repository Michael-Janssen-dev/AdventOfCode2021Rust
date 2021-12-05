use std::collections::HashMap;
use std::fs;
use std::cmp::{min, max};

type Coor = (i32, i32);
type Line = (Coor, Coor);

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 5);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 12);
}

fn part_1(lines: Vec<Line>) -> i32 {
    let mut overlap: HashMap<Coor, u32> = HashMap::new();
    lines.iter().for_each(|((x1, y1), (x2, y2))| {
        if x1 == x2 {
            (*min(y1, y2)..=*max(y1, y2)).for_each(|y| *overlap.entry((*x1, y)).or_insert(0) += 1);
        } else if y1 == y2 {
            (*min(x1, x2)..=*max(x1, x2)).for_each(|x| *overlap.entry((x, *y1)).or_insert(0) += 1);
        }
    });
    //println!("{:?}", overlap);
    overlap.iter().fold(0, |n, (_, value)| if *value >= 2 {n + 1} else {n})
}

fn part_2(lines: Vec<Line>) -> i32 {
    let mut overlap: HashMap<Coor, u32> = HashMap::new();
    lines.iter().for_each(|((x1, y1), (x2, y2))| {
        if x1 == x2 {
            (*min(y1, y2)..=*max(y1, y2)).for_each(|y| *overlap.entry((*x1, y)).or_insert(0) += 1);
        } else if y1 == y2 {
            (*min(x1, x2)..=*max(x1, x2)).for_each(|x| *overlap.entry((x, *y1)).or_insert(0) += 1);
        } else if (y2 - y1).abs() == (x2 - x1).abs() {
            let x_l = min(x1, x2);
            let (y_l, x_r, y_r) = if x_l == x1 {(y1, x2, y2)} else {(y2, x1, y1)};
            if y_r < y_l {
                (0..=(x_r - x_l)).for_each(|n| *overlap.entry((x_l + n, y_l - n)).or_insert(0) += 1);
            } else {
                (0..=(x_r - x_l)).for_each(|n| *overlap.entry((x_l + n, y_l + n)).or_insert(0) += 1);
            }
            
        }
    });
    // println!("{:?}", overlap);
    overlap.iter().fold(0, |n, (_, value)| if *value >= 2 {n + 1} else {n})
}

fn handle_input(file: &str) -> Vec<Line> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|x| x.split("->").collect())
        .map(|x: Vec<&str>| (x[0].split(",").collect(), x[1].split(",").collect()))
        .map(|x: (Vec<&str>, Vec<&str>)| {
            (
                (x.0[0].trim().parse().unwrap(), x.0[1].trim().parse().unwrap()),
                (x.1[0].trim().parse().unwrap(), x.1[1].trim().parse().unwrap()),
            )
        })
        .collect()
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("inp.txt")));
    println!("Part 2: {}", part_2(handle_input("inp.txt")));
}
