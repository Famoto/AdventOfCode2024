use std::collections::{HashSet, VecDeque};

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Invalid character") as u8)
                .collect()
        })
        .collect()
}

#[aoc(day10, part1)]
fn day10_part1(map: &Vec<Vec<u8>>) -> usize {
    let rows = map.len();
    if rows == 0 {
        return 0;
    }
    let cols = map[0].len();

    // Identify all trailheads (positions with height 0)
    let mut trailheads = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    let mut sum_scores = 0;

    for &(trail_r, trail_c) in &trailheads {
        // BFS setup
        let mut visited = vec![vec![false; cols]; rows];
        let mut queue = VecDeque::new();
        let mut reachable_nines = HashSet::new();

        // Initialize BFS
        queue.push_back((trail_r, trail_c, 0)); // (row, col, current_height)
        visited[trail_r][trail_c] = true;

        while let Some((r, c, current_height)) = queue.pop_front() {
            if map[r][c] == 9 {
                reachable_nines.insert((r, c));
                continue;
            }

            let next_height = current_height + 1;

            // Explore all four directions
            let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
            for &(dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                // Check bounds
                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    let nr_usize = nr as usize;
                    let nc_usize = nc as usize;
                    if map[nr_usize][nc_usize] == next_height && !visited[nr_usize][nc_usize] {
                        visited[nr_usize][nc_usize] = true;
                        queue.push_back((nr_usize, nc_usize, next_height));
                    }
                }
            }
        }

        sum_scores += reachable_nines.len();
    }

    sum_scores
}

#[aoc(day10, part2)]
fn day10_part2(map: &Vec<Vec<u8>>) -> usize {
    let rows = map.len();
    if rows == 0 {
        return 0;
    }
    let cols = map[0].len();

    let mut trailheads = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    let mut sum_ratings = 0;

    fn count_paths(
        r: usize,
        c: usize,
        map: &Vec<Vec<u8>>,
        memo: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        if map[r][c] == 9 {
            return 1;
        }

        if let Some(count) = memo[r][c] {
            return count;
        }

        let rows = map.len();
        let cols = map[0].len();
        let current_height = map[r][c];
        let mut total_paths = 0;

        let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
        for &(dr, dc) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr_usize = nr as usize;
                let nc_usize = nc as usize;

                if map[nr_usize][nc_usize] == current_height + 1 {
                    total_paths += count_paths(nr_usize, nc_usize, map, memo);
                }
            }
        }

        memo[r][c] = Some(total_paths);
        total_paths
    }

    for &(trail_r, trail_c) in &trailheads {
        let mut local_memo = vec![vec![None; cols]; rows];
        let rating = count_paths(trail_r, trail_c, map, &mut local_memo);
        sum_ratings += rating;
    }

    sum_ratings
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_part1() {
        let input = indoc! {
            "89010123
             78121874
             87430965
             96549874
             45678903
             32019012
             01329801
             10456732"
        };
        let map = parse(input);
        assert_eq!(day10_part1(&map), 36);
    }

    #[test]
    fn example_part2() {
        let input = indoc! {
            "89010123
             78121874
             87430965
             96549874
             45678903
             32019012
             01329801
             10456732"
        };
        let map = parse(input);
        assert_eq!(day10_part2(&map), 81);
    }
}
