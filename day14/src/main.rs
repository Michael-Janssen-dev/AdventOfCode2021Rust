use std::collections::HashMap;
use std::fs;

type Pair = (char, char);
type Template = HashMap<Pair, u64>;
type Rules = HashMap<Pair, char>;
type Input = (Template, Rules, char, char);

fn handle_input(path: &str) -> Input {
    let file = fs::read_to_string(path).unwrap();
    let (template, rules) = file.split_once("\r\n\r\n").unwrap();
    let mut template_map = HashMap::new();
    template.chars().enumerate().take(template.chars().count() - 1).for_each(|(i, x)| {*template_map.entry((x, template.chars().nth(i + 1).unwrap())).or_insert(0) += 1;});
    let rules_map = rules.lines().map(|r| r.split_once(" -> ").unwrap()).map(|(k, v)| ((k.chars().nth(0).unwrap(), k.chars().nth(1).unwrap()), v.chars().next().unwrap())).collect();
    return (template_map, rules_map, template.chars().nth(0).unwrap(), template.chars().nth(template.chars().count() - 1).unwrap())
}

fn step(template: &Template, rules: &Rules) -> Template {
    let mut result = HashMap::new();
    for ((a, b), v) in template {
        let n = rules.get(&(*a, *b));
        if let Some(x) = n {
            *result.entry((*a, *x)).or_insert(0) += v;
            *result.entry((*x, *b)).or_insert(0) += v;
        } else {
            *result.entry((*a, *b)).or_insert(0) += v;
        }
        
    }
    result
}

fn count(template: Template, first: char, last: char) -> u64 {
    let mut c: HashMap<char, u64> = HashMap::new();
    for ((a, b), v) in template {
        *c.entry(a).or_insert(0) += v;
        *c.entry(b).or_insert(0) += v;
    }
    *c.get_mut(&first).unwrap() += 1;
    *c.get_mut(&last).unwrap() += 1;
    let max = c.values().max().unwrap() / 2;
    let min = c.values().min().unwrap() / 2;

    max - min
}

fn part_1((template, rules, first, last): Input) -> u64 {
    let final_template = (0..10).fold(template, |r, _| step(&r, &rules));
    count(final_template, first, last)
}

fn part_2((template, rules, first, last): Input) -> u64 {
    let final_template = (0..40).fold(template, |r, _| step(&r, &rules));
    count(final_template, first, last)
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 1588);
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("input.txt")));
    println!("Part 2: {}", part_2(handle_input("input.txt")));
}
