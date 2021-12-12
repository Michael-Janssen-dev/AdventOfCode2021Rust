use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs;
use std::rc::Rc;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 10);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(handle_input("test.txt")), 36);
}

struct Cave<'a> {
    is_big: bool,
    connections: RefCell<Vec<Rc<Cave<'a>>>>,
    name: String,
}

impl Debug for Cave<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {:?}", self.name, self.connections)
    }
}

type CaveSystem<'a> = HashMap<String, Rc<Cave<'a>>>;

fn handle_input(path: &str) -> CaveSystem {
    let mut caves: CaveSystem = HashMap::new();
    let file = fs::read_to_string(path).unwrap();
    file.lines().for_each(|connection| {
        let (cave_1, cave_2) = connection.split_once("-").unwrap();
        if !caves.contains_key(cave_1) {
            caves.insert(
                cave_1.to_string(),
                Rc::new(Cave {
                    is_big: cave_1.chars().next().unwrap().is_uppercase(),
                    connections: RefCell::from(Vec::new()),
                    name: cave_1.to_string(),
                }),
            );
        }
        if !caves.contains_key(cave_2) {
            caves.insert(
                cave_2.to_string(),
                Rc::new(Cave {
                    is_big: cave_2.chars().next().unwrap().is_uppercase(),
                    connections: RefCell::from(Vec::new()),
                    name: cave_2.to_string(),
                }),
            );
        }
        caves
            .get(cave_1)
            .unwrap()
            .connections
            .borrow_mut()
            .push(Rc::clone(caves.get(cave_2).unwrap()));
        caves
            .get(cave_2)
            .unwrap()
            .connections
            .borrow_mut()
            .push(Rc::clone(caves.get(cave_1).unwrap()));
    });
    caves
}

fn part_1(caves: CaveSystem) -> usize {
    let mut count = 0;
    let mut queue: Vec<(Rc<Cave>, Vec<String>)> = Vec::new();
    queue.push((caves.get("start").unwrap().clone(), Vec::new()));

    while !queue.is_empty() {
        let (cave, mut visited) = queue.pop().unwrap();
        
        visited.push(cave.name.clone());
        if cave.name == "end" {
            count += 1;
            continue;
        }
        for connection in cave.connections.borrow().iter() {
            if connection.is_big || !visited.iter().any(|v| v == &connection.name) {
                queue.push((connection.clone(), visited.clone()));
            }
        }
    }
    count
}

fn part_2(caves: CaveSystem) -> usize {
    let mut count = 0;
    let mut queue: Vec<(Rc<Cave>, Vec<String>, bool)> = Vec::new();
    queue.push((caves.get("start").unwrap().clone(), Vec::new(), false));
    while !queue.is_empty() {
        let (cave, mut visited, mut twice) = queue.pop().unwrap();
        
        if !cave.is_big && visited.contains(&cave.name) {
            twice = true;
        }
        visited.push(cave.name.clone());

        if cave.name == "end" {
            count += 1;
            continue;
        }

        for connection in cave.connections.borrow().iter() {
            if connection.is_big || (twice && !visited.iter().any(|v| v == &connection.name)) || (!twice && connection.name != "start") {
                queue.push((connection.clone(), visited.clone(), twice));
            }
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("input.txt")));
    println!("Part 2: {}", part_2(handle_input("input.txt")));
}
