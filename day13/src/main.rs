use std::fs;

#[test]
fn test_part_1() {
    assert_eq!(part_1(handle_input("test.txt")), 17);
}

enum FoldDirection {
    Up(usize),
    Left(usize),
}

type Coor = (usize, usize);

fn fold(dots: Vec<Coor>, fold_line: &FoldDirection) -> Vec<Coor> {
    match fold_line {
        FoldDirection::Left(m) => {
            let mut new_dots: Vec<Coor> = dots
                .iter()
                .map(|(x, y)| if *x < *m { (*x, *y) } else { (*m - (*x - *m), *y) })
                .collect();
            new_dots.sort_unstable();
            new_dots.dedup();
            return new_dots
        }
        FoldDirection::Up(m) => {
            let mut new_dots: Vec<Coor> = dots
                .iter()
                .map(|(x, y)| if *y < *m { (*x, *y) } else { (*x, *m - (*y - *m)) })
                .collect();
            new_dots.sort_unstable();
            new_dots.dedup();
            return new_dots;
        }
    }
}

type Input = (Vec<Coor>, Vec<FoldDirection>);

fn handle_input(path: &str) -> Input {
    let file = fs::read_to_string(path).unwrap();
    let (dot_places, folds) = file.split_once("\r\n\r\n").unwrap();
    (
        dot_places
            .lines()
            .map(|x| {
                (
                    x.split_once(",").unwrap().0.parse().unwrap(),
                    x.split_once(",").unwrap().1.parse().unwrap(),
                )
            })
            .collect(),
        folds
            .lines()
            .map(|x| match x.chars().nth(11).unwrap() {
                'x' => {
                    return FoldDirection::Left(
                        x.chars().skip(13).collect::<String>().parse().unwrap(),
                    )
                }
                'y' => {
                    return FoldDirection::Up(
                        x.chars().skip(13).collect::<String>().parse().unwrap(),
                    )
                }
                _ => return FoldDirection::Up(0),
            })
            .collect(),
    )
}

fn part_1(input: Input) -> usize {
    let dots = fold(input.0, input.1.first().unwrap());
    dots.len()
}

fn part_2(input: Input){
    let dots = input.1.iter().fold(input.0, |accum, x| fold(accum, x));
    let mx = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
    let my = dots.iter().max_by_key(|(_, y)| y).unwrap().1;
    (0..=my).for_each(|y| {(0..=mx).for_each(|x| if dots.contains(&(x, y)) {print!("#")} else {print!(".")}); println!("")});
}

fn main() {
    println!("Part 1: {}", part_1(handle_input("input.txt")));
    part_2(handle_input("input.txt"));
}
