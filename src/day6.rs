use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize), Dir) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start_pos = (0, 0);
    let mut start_dir = Dir::Up;

    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            match cell {
                '^' => {
                    start_pos = (i, j);
                    start_dir = Dir::Up;
                }
                'v' => {
                    start_pos = (i, j);
                    start_dir = Dir::Down;
                }
                '<' => {
                    start_pos = (i, j);
                    start_dir = Dir::Left;
                }
                '>' => {
                    start_pos = (i, j);
                    start_dir = Dir::Right;
                }
                _ => {}
            }
        }
    }

    (map, start_pos, start_dir)
}

#[aoc(day6, part1)]
fn part1(input: &(Vec<Vec<char>>, (usize, usize), Dir)) -> usize {
    let (map, position, direction) = input;
    simulate_count_visited(map, *position, *direction)
}

#[aoc(day6, part2)]
fn part2(input: &(Vec<Vec<char>>, (usize, usize), Dir)) -> usize {
    let (original_map, (sx, sy), sdir) = input;

    let mut map = original_map.clone();
    // Replace the guard symbol with '.' for consistent processing
    map[*sx][*sy] = '.';

    let rows = map.len();
    let cols = map[0].len();

    let mut count = 0;

    for x in 0..rows {
        for y in 0..cols {
            // Can't place obstruction at the guard's start position
            if (x, y) == (*sx, *sy) {
                continue;
            }
            // Only place on empty cells
            if map[x][y] == '.' {
                // Temporarily place the obstruction
                map[x][y] = '#';

                if causes_loop(&map, (*sx, *sy), *sdir) {
                    count += 1;
                }

                // Restore cell
                map[x][y] = '.';
            }
        }
    }

    count
}

/// Simulates the guard's patrol until leaving the map. Returns the number of visited positions.
fn simulate_count_visited(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Dir,
) -> usize {
    let mut visited = HashSet::new();
    let mut position = start_pos;
    let mut direction = start_dir;

    visited.insert(position);

    loop {
        let (x, y) = position;
        let (dx, dy) = dir_to_delta(direction);

        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // Check if out of bounds
        if nx < 0 || ny < 0 || nx as usize >= map.len() || ny as usize >= map[0].len() {
            // Guard leaves the map
            break;
        }

        let nx_u = nx as usize;
        let ny_u = ny as usize;

        if map[nx_u][ny_u] == '#' {
            // Obstacle ahead, turn right
            direction = turn_right(direction);
        } else {
            // Move forward
            position = (nx_u, ny_u);
            visited.insert(position);
        }
    }

    visited.len()
}

/// Checks if placing a single new obstruction at the chosen location causes the guard to enter a loop.
fn causes_loop(map: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: Dir) -> bool {
    let mut position = start_pos;
    let mut direction = start_dir;

    let mut seen_states = HashSet::new();
    seen_states.insert((position, direction));

    loop {
        let (x, y) = position;
        let (dx, dy) = dir_to_delta(direction);

        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // Check if out of bounds
        if nx < 0 || ny < 0 || nx as usize >= map.len() || ny as usize >= map[0].len() {
            // Guard leaves the map, no loop
            return false;
        }

        let nx_u = nx as usize;
        let ny_u = ny as usize;

        if map[nx_u][ny_u] == '#' {
            // Obstacle ahead, turn right
            direction = turn_right(direction);
        } else {
            // Move forward
            position = (nx_u, ny_u);
        }

        let state = (position, direction);
        if seen_states.contains(&state) {
            // We have seen this state before, so it's a loop
            return true;
        }
        seen_states.insert(state);
    }
}

fn dir_to_delta(d: Dir) -> (isize, isize) {
    match d {
        Dir::Up => (-1, 0),
        Dir::Right => (0, 1),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
    }
}

fn turn_right(d: Dir) -> Dir {
    match d {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_part1() {
        let input = indoc! {
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        };
        assert_eq!(part1(&parse(input)), 41);
    }

    #[test]
    fn example_part2() {
        let input = indoc! {
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        };
        // According to the problem statement, there are 6 positions where a new obstruction causes a loop
        assert_eq!(part2(&parse(input)), 6);
    }
}
