use std::collections::HashMap;
use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 5934);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 26984457539);
}

fn part_1(fish: HashMap<u32, usize>) -> usize {
    mult_fish(fish, 80)
}

fn part_2(fish: HashMap<u32, usize>) -> usize {
    mult_fish(fish, 256)
}

fn mult_fish(mut fish: HashMap<u32, usize>, days: u32) -> usize {
    for i in 0..days {
        let x = fish.remove(&i);
        x.unwrap_or(0);
        match x {
            Some(y) => {
                *fish.entry(i + 7).or_insert(0) += y;
                *fish.entry(i + 9).or_insert(0) += y;
            }
            _ => continue
        }
        
    }
    fish.into_iter().fold(0, |accum, (_, value)| accum + value)
}

fn handle_input(path: &str) -> HashMap<u32, usize> {
    let fish: Vec<u32> = fs::read_to_string(path)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut fish_map: HashMap<u32, usize> = HashMap::new();
    for f in fish {
        *fish_map.entry(f).or_insert(0) += 1;
    }
    fish_map
}

fn main() {
    let fish = handle_input("inp.txt");
    println!("Part 1: {}", part_1(fish.clone()));
    println!("Part 2: {}", part_2(fish.clone()));
}
