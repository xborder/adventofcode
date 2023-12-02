const INPUT: &str = include_str!("./1.txt");
const NUMBERS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const NUMBERS_TXT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn text_to_digit(text: &str) -> Option<u32> {
    match text {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn find_pos(line: &str, numbers: &Vec<&str>) -> u32 {
    let mut pos = numbers
        .iter()
        .filter_map(|tn| {
            let pos = line
                .match_indices(tn)
                .filter_map(|(i, _)| Some((text_to_digit(tn).unwrap(), i)))
                .collect::<Vec<(u32, usize)>>();
            Some(pos)
        })
        .flatten()
        .collect::<Vec<(u32, usize)>>();

    pos.sort_by(|(_, p1), (_, p2)| p1.cmp(p2));

    let first = *pos.first().and_then(|(n, _)| Some(n)).unwrap_or_else(|| &0);
    let last = *pos.last().and_then(|(n, _)| Some(n)).unwrap_or_else(|| &0);
    10 * first + last
}

fn calibration(input: &str, numbers: Vec<&str>) -> u32 {
    input.lines().map(|s| find_pos(s, &numbers)).sum()
}

fn main() {
    println!("part 1: {}", calibration(INPUT, NUMBERS.to_vec()));
    println!(
        "part 2: {}",
        calibration(INPUT, [&NUMBERS[..], &NUMBERS_TXT[..]].concat())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(calibration(input, NUMBERS.to_vec()), 142);
    }

    #[test]
    fn part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(
            calibration(input, [&NUMBERS[..], &NUMBERS_TXT[..]].concat()),
            281
        );
    }
}
