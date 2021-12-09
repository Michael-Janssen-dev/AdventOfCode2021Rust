use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 15);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 1134);
}

fn handle_input(path: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}

fn get_item<'a, T>(inp: &'a Vec<Vec<T>>, x: usize, y: usize) -> &'a T {
    inp.get(y).unwrap().get(x).unwrap()
}

fn get_low_points(inp: &Vec<Vec<u32>>) -> Vec<Coor> {
    let mut coor = Vec::new();
    for i in 0..inp.len() {
        for j in 0..inp.get(i).unwrap().len() {
            let y = get_item(&inp, j, i);
            if (j == 0 || y < get_item(&inp, j - 1, i))
                && (j == inp.get(i).unwrap().len() - 1 || y < get_item(&inp, j + 1, i))
                && (i == 0 || y < get_item(&inp, j, i - 1))
                && (i == inp.len() - 1 || y < get_item(&inp, j, i + 1))
            {
                coor.push((j, i))
            }
        }
    }
    coor
}

fn part_1(inp: Vec<Vec<u32>>) -> u32 {
    get_low_points(&inp).iter().map(|(x, y)| get_item(&inp, *x, *y) + 1).sum()
}

type Coor = (usize, usize);

fn flood(points: &Vec<Vec<u32>>, sx: usize, sy: usize) -> usize {
    let mut queue: VecDeque<Coor> = VecDeque::new();
    queue.push_back((sx, sy));
    let mut visited = Vec::new();
    visited.push((sx, sy));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if y != 0 && !visited.contains(&(x, y - 1)) && *get_item(points, x, y - 1) != 9 {
            queue.push_front((x, y - 1));
            visited.push((x, y - 1));
        }
        if x != 0 && !visited.contains(&(x - 1, y)) && *get_item(points, x - 1, y) != 9 {
            queue.push_front((x - 1, y));
            visited.push((x - 1, y));
        }
        if y != points.len() - 1 && !visited.contains(&(x, y + 1)) && *get_item(points, x, y + 1) != 9 {
            queue.push_front((x, y + 1));
            visited.push((x, y + 1));
        }
        if x != points.get(y).unwrap().len() - 1 && !visited.contains(&(x + 1, y)) && *get_item(points, x + 1, y) != 9 {
            queue.push_front((x + 1, y));
            visited.push((x + 1, y));
        }
    }
    visited.len()
}

fn part_2(inp: Vec<Vec<u32>>) -> usize {
    let mut basins: Vec<usize> = Vec::new();
    let low_points = get_low_points(&inp);
    for (x, y) in low_points {
        basins.push(flood(&inp, x, y))
    }
    basins.sort();
    basins.iter().rev().take(3).fold(1, |accum, x| accum * x)
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("input.txt")));
    println!("Part 2: {}", part_2(handle_input("input.txt")));
}
