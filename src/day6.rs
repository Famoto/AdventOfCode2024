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
fn day6_part1(input: &(Vec<Vec<char>>, (usize, usize), Dir)) -> usize {
    let (map, position, direction) = input;
    simulate_count_visited(map, *position, *direction)
}

#[aoc(day6, part2)]
fn day6_part2(input: &(Vec<Vec<char>>, (usize, usize), Dir)) -> usize {
    let (original_map, (sx, sy), sdir) = input;

    let mut map = original_map.clone();
    map[*sx][*sy] = '.'; // Replace the guard symbol with '.'

    let mut candidates = find_candidate_cells_fast(&map, (*sx, *sy), *sdir);

    // Use a counter to keep track of valid obstructions
    let mut count = 0;

    for &(x, y) in &candidates {
        if (x, y) == (*sx, *sy) {
            continue; // Skip the guard's start position
        }

        // Temporarily place the obstruction
        let original = map[x][y];
        map[x][y] = '#';

        if causes_loop(&map, (*sx, *sy), *sdir) {
            count += 1;
        }

        // Restore cell
        map[x][y] = original;
    }

    count
}

fn find_candidate_cells_fast(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Dir,
) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    let mut position = start_pos;
    let mut direction = start_dir;

    loop {
        let (x, y) = position;
        let (dx, dy) = dir_to_delta(direction);

        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 || nx as usize >= map.len() || ny as usize >= map[0].len() {
            break; // Out of bounds
        }

        let nx_u = nx as usize;
        let ny_u = ny as usize;

        if map[nx_u][ny_u] == '#' {
            direction = turn_right(direction); // Obstacle ahead, turn right
        } else {
            if !visited[nx_u][ny_u] && map[nx_u][ny_u] == '.' {
                candidates.push((nx_u, ny_u));
                visited[nx_u][ny_u] = true;
            }
            position = (nx_u, ny_u);
        }
    }

    candidates
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
fn causes_loop(map: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: Dir) -> bool {
    let mut slow = (start_pos, start_dir);
    let mut fast = match advance(map, start_pos, start_dir) {
        Some(state) => state,
        None => return false, // If the guard leaves immediately, no loop
    };

    // Advance `fast` one more step
    fast = match advance(map, fast.0, fast.1) {
        Some(state) => state,
        None => return false, // If `fast` goes out of bounds, no loop
    };

    while let Some(fast_state) = advance(map, fast.0, fast.1) {
        // Advance `slow` by one step
        slow = match advance(map, slow.0, slow.1) {
            Some(state) => state,
            None => return false, // If `slow` goes out of bounds, no loop
        };

        // Advance `fast` one more step
        fast = fast_state;
        fast = match advance(map, fast.0, fast.1) {
            Some(state) => state,
            None => return false, // If `fast` goes out of bounds, no loop
        };

        if slow == fast {
            return true; // Loop detected
        }
    }

    false
}

/// Advance one step in the simulation
fn advance(
    map: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: Dir,
) -> Option<((usize, usize), Dir)> {
    let (x, y) = position;
    let (dx, dy) = dir_to_delta(direction);

    let nx = x as isize + dx;
    let ny = y as isize + dy;

    if nx < 0 || ny < 0 || nx as usize >= map.len() || ny as usize >= map[0].len() {
        return None; // Out of bounds
    }

    let nx_u = nx as usize;
    let ny_u = ny as usize;

    if map[nx_u][ny_u] == '#' {
        Some((position, turn_right(direction)))
    } else {
        Some(((nx_u, ny_u), direction))
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
        assert_eq!(day6_part1(&parse(input)), 41);
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
        assert_eq!(day6_part2(&parse(input)), 6);
    }
}
