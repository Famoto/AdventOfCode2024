#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect()) // Convert each line into a vector of characters
        .collect() // Collect all lines into a Vec<Vec<char>>
}
#[aoc(day4, part1)]
fn day4_part1(grid: &[Vec<char>]) -> usize {
    // Directions represented as (dx, dy)
    let directions = [
        (0, 1),  // Horizontal right
        (0, -1), // Horizontal left
        (1, 0),  // Vertical down
        (-1, 0), // Vertical up
        (1, 1),  // Diagonal ↘
        (1, -1), // Diagonal ↙
        (-1, 1), // Diagonal ↗
        (-1, -1), // Diagonal ↖
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let target = ['X', 'M', 'A', 'S'];
    let word_length = target.len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            // Try each direction
            for &(dx, dy) in &directions {
                let mut found = true;
                for i in 0..word_length {
                    let nr = r as isize + i as isize * dx;
                    let nc = c as isize + i as isize * dy;
                    // Check bounds and match characters
                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != target[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}


#[aoc(day4, part2)]
fn day4_part2(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Check for X-MAS centered at (i, j)
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            // Extract diagonals
            let diag1 = (grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]); // Top-left to bottom-right
            let diag2 = (grid[i + 1][j - 1], grid[i][j], grid[i - 1][j + 1]); // Bottom-left to top-right

            // Check both diagonals for MAS or SAM
            if is_xmas(diag1, diag2) {
                count += 1;
            }
        }
    }

    count
}

fn is_xmas(diag1: (char, char, char), diag2: (char, char, char)) -> bool {
    // Check if the diagonals form an X-MAS
    let mas = ('M', 'A', 'S');
    let sam = ('S', 'A', 'M');
    (diag1 == mas || diag1 == sam) && (diag2 == mas || diag2 == sam)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        const SAMPLE: &str =
        {"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"};


        assert_eq!(day4_part1(parse(&SAMPLE).borrow()), 18);
    }

    fn part1_example() {
        const SAMPLE: &str =
            {"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"};


        assert_eq!(day4_part1(parse(&SAMPLE).borrow()), 9);
    }
}
