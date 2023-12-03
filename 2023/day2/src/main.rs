use std::i32;

const INPUT: &str = include_str!("./2.txt");
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            let (game, bag) = s.split_once(":").expect(": not found");
            let sums = bag
                .split(";")
                .map(|set| {
                    let cubes = set
                        .split(",")
                        .map(|cube| {
                            // dbg!(cube);
                            let amount: Vec<&str> = cube.trim_matches(' ').split(' ').collect();
                            let index: usize = match *amount.last().unwrap() {
                                "blue" => 0,
                                "red" => 1,
                                "green" => 2,
                                _ => panic!("UNKNOWN COLOR"),
                            };
                            // dbg!(&amount);
                            let number = (*amount).first().unwrap().parse::<i32>().unwrap();
                            (index, number)
                        })
                        .collect::<Vec<(usize, i32)>>();
                    cubes
                })
                .flatten()
                .fold([0; 3], |mut acc, (index, v)| {
                    acc[index] = std::cmp::max(acc[index], v);
                    acc
                });
            sums
        })
        .fold(0, |acc, e| acc + (e[0] * e[1] * e[2]))
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|s| {
            let (game, bag) = s.split_once(":").expect(": not found");
            dbg!(game);
            let index = game.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            let sums = bag.split(";").all(|set| {
                let cubes = set.split(",").all(|cube| {
                    // dbg!(cube);
                    let amount: Vec<&str> = cube.trim_matches(' ').split(' ').collect();
                    let color = *amount.last().unwrap();
                    let number = (*amount).first().unwrap().parse::<i32>().unwrap();

                    (color == "blue" && number <= 14)
                        || (color == "green" && number <= 13)
                        || (color == "red" && number <= 12)
                });
                cubes
            });
            dbg!(sums);
            if sums {
                Some(index)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part1(&input), 8);
        assert_eq!(part2(&input), 2286)
    }
}
