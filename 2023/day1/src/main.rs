fn day1() {
    let input = std::fs::read_to_string("./1.txt").expect("failed to read file");

    let sum: u32 = input
        .lines()
        .map(|s| {
            let numbers = s
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let first = *numbers.first().unwrap_or_else(|| &0);
            let last = *numbers.last().unwrap_or_else(|| &0);
            10 * first + last
        })
        .sum();

    println!("{}", sum);
}

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

fn day2() {
    let text_numbers = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let input = std::fs::read_to_string("./1.txt").expect("failed to read file");
    let sum: u32 = input
        .lines()
        .map(|s| {
            let mut pos = text_numbers
                .iter()
                .filter_map(|tn| {
                    let pos = s.match_indices(tn);
                    Some(
                        pos.filter_map(|(i, _)| Some((text_to_digit(tn).unwrap(), i)))
                            .collect::<Vec<(u32, usize)>>(),
                    )
                })
                .flatten()
                .collect::<Vec<(u32, usize)>>();
            pos.sort_by(|(_, p1), (_, p2)| p1.cmp(p2));
            let first = *pos.first().and_then(|(n, _)| Some(n)).unwrap_or_else(|| &0);
            let last = *pos.last().and_then(|(n, _)| Some(n)).unwrap_or_else(|| &0);
            10 * first + last
        })
        .sum();
    println!("{}", sum);
}

fn main() {
    day1();
    day2();
}
