#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2_part1(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false; // Reports with fewer than 2 levels cannot meet the criteria
    }

    // Check differences between adjacent levels
    let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    // Determine if the sequence is monotonic
    let is_increasing = diffs.iter().all(|&d| d > 0);
    let is_decreasing = diffs.iter().all(|&d| d < 0);

    // Check if all differences are valid (between 1 and 3)
    let valid_diffs = diffs.iter().all(|&d| d.abs() >= 1 && d.abs() <= 3);

    (is_increasing || is_decreasing) && valid_diffs
}

#[aoc(day2, part2)]
fn day2_part2(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    // Check if the report is safe without modifications
    if is_safe(report) {
        return true;
    }

    // Check if removing one level makes it safe
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i); // Remove the i-th level
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const SAMPLE: &str = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        let reports = parse(SAMPLE);
        assert_eq!(day2_part1(&reports), 2);
    }

    #[test]
    fn part2_example() {
        const SAMPLE: &str = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        let reports = parse(SAMPLE);
        assert_eq!(day2_part2(&reports), 4);
    }
}
