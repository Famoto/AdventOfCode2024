#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let target: usize = parts[0].trim().parse().unwrap();
            let numbers: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (target, numbers)
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(equations: &[(usize, Vec<usize>)]) -> usize {
    equations
        .iter()
        .filter(|(target, numbers)| can_form_target(*target, numbers))
        .map(|(target, _)| target)
        .sum()
}

#[aoc(day7, part2)]
fn part2(equations: &[(usize, Vec<usize>)]) -> usize {
    equations
        .iter()
        .filter(|(target, numbers)| can_form_target_with_concat(*target, numbers))
        .map(|(target, _)| target)
        .sum()
}

/// Check if we can form the target by inserting '+' or '*' between the numbers
fn can_form_target(target: usize, numbers: &[usize]) -> bool {
    if numbers.len() == 1 {
        // If there's only one number, check if it matches directly
        return numbers[0] == target;
    }

    let n = numbers.len();
    let combinations = 1 << (n - 1); // 2^(n-1) for + and *

    for mask in 0..combinations {
        let mut result = numbers[0];
        for i in 1..n {
            let op = (mask >> (i - 1)) & 1;
            if op == 0 {
                // '+'
                result = result + numbers[i];
            } else {
                // '*'
                result = result * numbers[i];
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

/// Check if we can form the target by inserting '+', '*', or '||' between the numbers
fn can_form_target_with_concat(target: usize, numbers: &[usize]) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    let n = numbers.len();
    let combinations = 3usize.pow((n - 1) as u32); // 3^(n-1) for +, *, and ||

    for mut mask in 0..combinations {
        let mut result = numbers[0];

        for i in 1..n {
            let op = mask % 3;
            mask /= 3;

            let next_val = numbers[i];
            match op {
                0 => {
                    // '+'
                    result = result + next_val;
                }
                1 => {
                    // '*'
                    result = result * next_val;
                }
                2 => {
                    // '||' concatenation
                    result = concat_numbers(result, next_val);
                }
                _ => unreachable!(),
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

fn concat_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        // concatenating zero: need one digit
        return a * 10;
    }

    let digits = num_digits(b);
    a * 10usize.pow(digits as u32) + b
}

fn num_digits(mut x: usize) -> usize {
    if x == 0 {
        return 1;
    }
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_part1() {
        let input = indoc! {
        "190: 10 19
         3267: 81 40 27
         83: 17 5
         156: 15 6
         7290: 6 8 6 15
         161011: 16 10 13
         192: 17 8 14
         21037: 9 7 18 13
         292: 11 6 16 20"
        };
        assert_eq!(part1(&parse(input)), 3749);
    }

    #[test]
    fn example_part2() {
        let input = indoc! {
        "190: 10 19
         3267: 81 40 27
         83: 17 5
         156: 15 6
         7290: 6 8 6 15
         161011: 16 10 13
         192: 17 8 14
         21037: 9 7 18 13
         292: 11 6 16 20"
        };

        // With concatenation considered, three more equations become possible,
        // making the total 11387.
        assert_eq!(part2(&parse(input)), 11387);
    }
}
