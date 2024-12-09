use std::collections::{HashMap, HashSet};

#[aoc_generator(day8)]
fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, char)>) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas = Vec::new();

    for (r, row) in map.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch.is_ascii_alphanumeric() {
                antennas.push((r, c, ch));
            }
        }
    }

    (map, antennas)
}

#[aoc(day8, part1)]
fn part1(input: &(Vec<Vec<char>>, Vec<(usize, usize, char)>)) -> usize {
    let (map, antennas) = input;
    let rows = map.len();
    let cols = map[0].len();

    // Group antennas by frequency
    let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for &(r, c, ch) in antennas {
        freq_map.entry(ch).or_default().push((r, c));
    }

    let mut antinodes = HashSet::new();

    // For each frequency group, consider all pairs of antennas
    for (&freq, positions) in &freq_map {
        let n = positions.len();
        for i in 0..n {
            for j in i + 1..n {
                let (px, py) = positions[i];
                let (qx, qy) = positions[j];

                // Antinodes:
                // A1 = 2Q - P
                // A2 = 2P - Q
                let a1x = 2 * qx as isize - px as isize;
                let a1y = 2 * qy as isize - py as isize;
                let a2x = 2 * px as isize - qx as isize;
                let a2y = 2 * py as isize - qy as isize;

                // Check if within bounds
                if a1x >= 0 && a1x < rows as isize && a1y >= 0 && a1y < cols as isize {
                    antinodes.insert((a1x as usize, a1y as usize));
                }
                if a2x >= 0 && a2x < rows as isize && a2y >= 0 && a2y < cols as isize {
                    antinodes.insert((a2x as usize, a2y as usize));
                }
            }
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
fn part2(input: &(Vec<Vec<char>>, Vec<(usize, usize, char)>)) -> usize {
    let (map, antennas) = input;
    let rows = map.len();
    let cols = map[0].len();

    // Group antennas by frequency
    let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for &(r, c, ch) in antennas {
        freq_map.entry(ch).or_default().push((r, c));
    }

    let mut antinodes = HashSet::new();

    for (&freq, positions) in &freq_map {
        // If only one antenna for this freq, it can't form a line with another antenna
        if positions.len() < 2 {
            continue;
        }

        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (px, py) = positions[i];
                let (qx, qy) = positions[j];

                let dx = qx as isize - px as isize;
                let dy = qy as isize - py as isize;
                let g = gcd(dx.abs(), dy.abs());
                let sx = dx / g;
                let sy = dy / g;

                // We now have a step (sx, sy) that represents the smallest move along this line.
                // We want all points (px + k*sx, py + k*sy) within the map.

                // Find the range of k values that keep the point inside the map:
                let (k_min, k_max) = find_k_range(px, py, sx, sy, rows, cols);

                for k in k_min..=k_max {
                    let x = (px as isize + k * sx) as usize;
                    let y = (py as isize + k * sy) as usize;
                    antinodes.insert((x, y));
                }
            }
        }
    }

    antinodes.len()
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Given a starting point (px, py) and step (sx, sy), find the range of integer k
/// such that the line point (px + k*sx, py + k*sy) remains inside [0..rows, 0..cols].
fn find_k_range(
    px: usize,
    py: usize,
    sx: isize,
    sy: isize,
    rows: usize,
    cols: usize,
) -> (isize, isize) {
    let px = px as isize;
    let py = py as isize;
    let (mut k_min_x, mut k_max_x) = (-std::isize::MAX, std::isize::MAX);
    let (mut k_min_y, mut k_max_y) = (-std::isize::MAX, std::isize::MAX);

    // For x dimension
    if sx == 0 {
        // No horizontal movement, check if px in range
        if px < 0 || px >= rows as isize {
            // Line does not intersect map at all
            return (1, -1); // empty range
        }
        // No further k constraints from x
    } else {
        // 0 <= px + k*sx < rows
        // => -px <= k*sx < rows - px
        // Divide by sx considering sign
        let min_val_x = -px;
        let max_val_x = (rows - 1) as isize - px;

        let (local_min_x, local_max_x) = if sx > 0 {
            (ceil_div(min_val_x, sx), floor_div(max_val_x, sx))
        } else {
            // sx < 0
            (ceil_div(max_val_x, sx), floor_div(min_val_x, sx))
        };

        k_min_x = local_min_x;
        k_max_x = local_max_x;
    }

    // For y dimension
    if sy == 0 {
        // No vertical movement, check if py in range
        if py < 0 || py >= cols as isize {
            return (1, -1); // no intersection
        }
        // No further constraints from y
    } else {
        // 0 <= py + k*sy < cols
        // => -py <= k*sy < cols - py
        let min_val_y = -py;
        let max_val_y = (cols - 1) as isize - py;

        let (local_min_y, local_max_y) = if sy > 0 {
            (ceil_div(min_val_y, sy), floor_div(max_val_y, sy))
        } else {
            (ceil_div(max_val_y, sy), floor_div(min_val_y, sy))
        };

        k_min_y = local_min_y;
        k_max_y = local_max_y;
    }

    let k_min = k_min_x.max(k_min_y);
    let k_max = k_max_x.min(k_max_y);

    (k_min, k_max)
}

/// Integer division that rounds up for negative or positive values.
/// ceil_div(a,b) = ceil(a/b)
fn ceil_div(a: isize, b: isize) -> isize {
    if (a ^ b) >= 0 {
        // same sign
        (a + b - 1) / b
    } else {
        // different sign
        a / b
    }
}

/// Integer division that floors the result
fn floor_div(a: isize, b: isize) -> isize {
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    // Test with the provided example
    #[test]
    fn example() {
        let input = indoc! {
        "............
         ........0...
         .....0......
         .......0....
         ....0.......
         ......A.....
         ............
         ............
         ........A...
         .........A..
         ............
         ............"
        };
        assert_eq!(part1(&parse(input)), 14);
    }

    fn example_part2() {
        let input = indoc! {
        "............
         ........0...
         .....0......
         .......0....
         ....0.......
         ......A.....
         ............
         ............
         ........A...
         .........A..
         ............
         ............"
        };
        let (map, antennas) = parse(input);
        assert_eq!(part2(&(map, antennas)), 34);
    }
}
