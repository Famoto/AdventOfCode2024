use regex::Regex;

#[aoc(day3, part1)]
fn day3_part1(input: &str) -> i32 {
    // Define a regular expression to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all matches, parse the numbers, and calculate their product
    re.captures_iter(input)
        .map(|caps| {
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            x * y
        })
        .sum()
}

#[aoc(day3, part2)]
fn day3_part2(input: &str) -> i32 {
    // Define a regular expression to match instructions
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    // Initialize state variables
    let mut enabled = true;
    let mut total = 0;

    // Process each match in order
    for caps in re.captures_iter(input) {
        if caps.get(1).is_some() {
            // Handle mul(X,Y) when enabled
            if enabled {
                let x: i32 = caps[2].parse().unwrap();
                let y: i32 = caps[3].parse().unwrap();
                total += x * y;
            }
        } else if caps.get(4).is_some() {
            // Handle do(): enable multiplications
            enabled = true;
        } else if caps.get(5).is_some() {
            // Handle don't(): disable multiplications
            enabled = false;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const INPUT: &str = indoc! {"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "};
        assert_eq!(day3_part1(INPUT), 161); // 2*4 + 5*5 + 11*8 + 8*5 = 161
    }

    #[test]
    fn part2_example() {
        const INPUT: &str = indoc! {"
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "};
        assert_eq!(day3_part2(INPUT), 48); // 2*4 + 8*5 = 48
    }
}
