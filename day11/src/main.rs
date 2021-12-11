use std::convert::TryInto;
use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 1656);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 195);
}

fn handle_input(path: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn flashed(oct: &mut Vec<Vec<u8>>, x: usize, y: usize, biem: &mut Vec<(usize, usize)>) {
    let item = oct.get_mut(y).unwrap().get_mut(x).unwrap();
    if *item != 0 {
        *item += 1;
        if *item > 9 {
            *item = 0;
            biem.push((x, y));
        }
    }
} 

fn flash(oct: &mut Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    let mut biem: Vec<(usize, usize)> = Vec::new();
    oct.iter_mut().enumerate()
        .for_each(|(y, line)| line.iter_mut().enumerate().for_each(|(x, oc)| {*oc += 1; if *oc > 9 {biem.push((x, y)); *oc = 0}}));
    while !biem.is_empty() {
        let (x, y) = biem.pop().unwrap();
        count += 1;
        if y > 0 {
            flashed(oct, x, y - 1, &mut biem);
            if x > 0 {
                flashed(oct, x - 1, y - 1, &mut biem);
            }
        }
        if x > 0 {
            flashed(oct, x - 1, y, &mut biem);
            if y < oct.len() - 1 {
                flashed(oct, x - 1, y + 1, &mut biem);
            } 
        }
        if y < oct.len() - 1 {
            flashed(oct, x, y + 1, &mut biem);
            if x < oct.get(x).unwrap().len() - 1 {
                flashed(oct, x + 1, y + 1, &mut biem);
            }
        }
        if x < oct.get(x).unwrap().len() - 1 {
            flashed(oct, x + 1, y, &mut biem);
            if y > 0 {
                flashed(oct, x + 1, y - 1, &mut biem);
            }
        }
    }
    count
}

fn part_1(mut oct: Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    for _ in 0..100 {
        count += flash(&mut oct);
    }
    count
}

fn part_2(mut oct: Vec<Vec<u8>>) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        if flash(&mut oct) == (oct.len() * oct.get(0).unwrap().len()).try_into().unwrap() {
            return i
        }
    }
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("input.txt")));
    println!("Part 2: {}", part_2(handle_input("input.txt")));
}
