use std::error::Error;
use std::fs;

fn parse_rotation(rotation: &str) -> (char, usize) {
    let direction = rotation.chars().next().unwrap();
    let turns = rotation[1..].parse::<usize>().unwrap();
    (direction, turns)
}

fn part_1(rotations: &Vec<(char, usize)>) -> usize {
    let mut count = 0;
    let mut dial: isize = 50;

    for rotation in rotations {
        if rotation.0 == 'L' {
            dial -= rotation.1 as isize;
        } else {
            dial += rotation.1 as isize;
        }

        dial = dial % 100;

        if dial == 0 {
            count += 1;
        }
    }

    count
}

fn part_2(rotations: &Vec<(char, usize)>) -> usize {
    let mut count = 0;
    let mut dial: isize = 50;

    for rotation in rotations {

        let n = rotation.1;

        (0..rotation.1).for_each(|_| {
            if rotation.0 == 'L' {
                dial -= 1;
            } else {
                dial += 1;
            }

            dial = dial % 100;
            if dial == 0 {
                count += 1;
            }
        });
    }

    count

}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input.txt";
    let contents = fs::read_to_string(path)?;
    let rotations: Vec<_> = contents
        .lines()
        .map(|rotation| parse_rotation(rotation))
        .collect();

    // println!("rotations: {:?}", rotations);
    println!("part_1: {:?}", part_1(&rotations));
    println!("part_2: {:?}", part_2(&rotations));

    Ok(())
}
