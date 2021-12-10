use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1("test.txt"), 26397);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("test.txt"), 288957);
}

fn part_1(path: &str) -> usize {
    let file = fs::read_to_string(path).unwrap();
    let mut count = 0;
    for line in file.lines() {
        let mut queue: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '{' || c == '[' || c == '(' || c == '<' {
                queue.push(c);
            } else {
                let item = queue.pop().unwrap();
                if c == ')' && item != '(' {
                    count += 3;
                    break;
                } else if c == ']' && item != '[' {
                    count += 57;
                    break;
                } else if c == '}' && item != '{' {
                    count += 1197;
                    break;
                } else if c == '>' && item != '<' {
                    count += 25137;
                    break;
                }
            }
        }
    }
    count
}

fn part_2(path: &str) -> usize {
    let file = fs::read_to_string(path).unwrap();
    let mut counts = Vec::new();
    'lineloop: for line in file.lines() {
        let mut queue: Vec<char> = Vec::new();
        for c in line.chars() {
            if c == '{' || c == '[' || c == '(' || c == '<' {
                queue.push(c);
            } else {
                //println!("{:?}", queue);
                let item = queue.pop().unwrap();
                if c == ')' && item != '(' {
                    continue 'lineloop;
                } else if c == ']' && item != '[' {
                    continue 'lineloop;
                } else if c == '}' && item != '{' {
                    continue 'lineloop;
                } else if c == '>' && item != '<' {
                    continue 'lineloop;
                }
            }
        }
        counts.push(queue.iter().rev().fold(0, |accum, x| match x {
            '(' => accum * 5 + 1,
            '[' => accum * 5 + 2,
            '{' => accum * 5 + 3,
            '<' => accum * 5 + 4,
            _ => accum
        }));
    }
    counts.sort();
    counts[counts.len() / 2]
}

fn main() {
    println!("Part 1: {}", part_1("input.txt"));
    println!("Part 2: {}", part_2("input.txt"));
}
