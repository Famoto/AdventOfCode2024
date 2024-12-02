#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if let Some((l, r)) = line.split_once(' ') {
            left.push(l.trim().parse::<u32>().unwrap());
            right.push(r.trim().parse::<u32>().unwrap());
        }
    }

    (left, right)
}
#[aoc(day1, part1)]
fn day1_part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {

    let mut left = left.clone();
    let mut right = right.clone();

    // Sort both lists
    left.sort_unstable();
    right.sort_unstable();

    // Calculate the total distance
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (*l as i32 - *r as i32).abs() as u32)
        .sum()
}
#[aoc(day1, part2)]
fn day1_part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    // Create a frequency map for the right list
    let max_value = 1900000; // Adjust this based on the expected input range

    // Use an array to count frequencies instead of a HashMap
    let mut frequency = vec![0u32; max_value + 1];
    for &num in right {
        frequency[num as usize] += 1;
    }

    // Calculate the similarity score
    left.iter()
        .map(|&num| num * frequency[num as usize])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        const SAMPLE: &str = indoc!
        {"3   4
        4   3
        2   5
        1   3
        3   9
        3   3"};
        assert_eq!(day1_part1(&parse(SAMPLE)), 11);
    }

#[test]
fn part2_example() {
    const SAMPLE: &str = indoc!
    {"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"};
      assert_eq!(day1_part2(&parse(SAMPLE)), 31);
  }
}
