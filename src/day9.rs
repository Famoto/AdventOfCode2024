use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<i32> {
    let chars: Vec<char> = input.trim().chars().collect();
    let mut disk = Vec::with_capacity(chars.len() * 9);
    let mut file_id = 0;
    let mut is_file = true;

    let mut i = 0;
    while i < chars.len() {
        let length = chars[i].to_digit(10).unwrap() as usize;

        if length > 0 {
            let fill_val = if is_file { file_id as i32 } else { -1 };
            disk.extend(std::iter::repeat(fill_val).take(length));
            if is_file {
                file_id += 1;
            }
        }
        is_file = !is_file;
        i += 1;
    }

    disk
}

#[aoc(day9, part1)]
fn day9_part1(disk: &Vec<i32>) -> i64 {
    let mut disk = disk.clone();
    // Simulate moves exactly as puzzle states:
    // Move the rightmost file block after the leftmost free space until stable.
    let mut L = 0;
    let mut R = disk.len().saturating_sub(1);

    while L < R {
        while L < disk.len() && disk[L] != -1 {
            L += 1;
        }
        while R > L && (disk[R] == -1 || R <= L) {
            R -= 1;
        }

        if L < R && disk[L] == -1 && disk[R] != -1 && R > L {
            disk[L] = disk[R];
            disk[R] = -1;
            L += 1;
            if R > 0 {
                R -= 1;
            }
        } else {
            break;
        }
    }

    let mut checksum = 0i64;
    for (i, &block) in disk.iter().enumerate() {
        if block != -1 {
            checksum += (i as i64) * (block as i64);
        }
    }
    checksum
}

#[aoc(day9, part2)]
fn day9_part2(disk: &Vec<i32>) -> i64 {
    let mut disk = disk.clone();

    let files = find_files(&disk);
    let mut file_list: Vec<(i32, usize, usize)> =
        files.iter().map(|(&fid, &(s, e))| (fid, s, e)).collect();
    file_list.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    // Get free runs sorted by their start index
    let mut free_runs = find_free_runs(&disk);
    // Sort by start to replicate the exact left-to-right selection
    free_runs.sort_unstable_by_key(|&(start, _end)| start);

    for &(fid, start, end) in &file_list {
        let length = end - start + 1;
        if let Some((run_idx, alloc_start)) = find_leftmost_suitable_run(&free_runs, length, start)
        {
            // Move the file
            for i in start..=end {
                disk[i] = -1;
            }
            for i in 0..length {
                disk[alloc_start + i] = fid;
            }

            // Update the free run
            let (rstart, rend) = free_runs[run_idx];
            let alloc_end = alloc_start + length - 1;
            if alloc_start == rstart && alloc_end == rend {
                // Entire run consumed
                free_runs.remove(run_idx);
            } else if alloc_start == rstart {
                // Front portion of the run used
                free_runs[run_idx].0 = alloc_end + 1;
            } else if alloc_end == rend {
                // End portion of the run used
                free_runs[run_idx].1 = alloc_start - 1;
            } else {
                // Run is split into two smaller runs: one before alloc_start and one after alloc_end
                let left_run = (rstart, alloc_start - 1);
                let right_run = (alloc_end + 1, rend);
                // Replace current run with left portion and insert the right portion after
                free_runs[run_idx] = left_run;
                free_runs.insert(run_idx + 1, right_run);
            }
        }
    }

    let mut checksum = 0i64;
    for (i, &block) in disk.iter().enumerate() {
        if block != -1 {
            checksum += (i as i64) * (block as i64);
        }
    }
    checksum
}

/// Find all files and their start/end indices
fn find_files(disk: &Vec<i32>) -> HashMap<i32, (usize, usize)> {
    let mut files: HashMap<i32, (usize, usize)> = HashMap::new();
    for (i, &b) in disk.iter().enumerate() {
        if b >= 0 {
            files
                .entry(b)
                .and_modify(|(_, old_end)| *old_end = i)
                .or_insert((i, i));
        }
    }
    files
}

/// Identify all contiguous free intervals (-1)
fn find_free_runs(disk: &Vec<i32>) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut in_run = false;
    let mut start = 0;
    for (i, &val) in disk.iter().enumerate() {
        if val == -1 {
            if !in_run {
                in_run = true;
                start = i;
            }
        } else {
            if in_run {
                in_run = false;
                runs.push((start, i - 1));
            }
        }
    }
    if in_run {
        runs.push((start, disk.len() - 1));
    }
    runs
}

/// Find a span of free space of at least `length` blocks that ends before `limit_pos`.
/// This must replicate the original logic: scan from left to right for the first suitable run.
fn find_leftmost_suitable_run(
    runs: &[(usize, usize)],
    length: usize,
    limit_pos: usize,
) -> Option<(usize, usize)> {
    let end_search = if limit_pos == 0 { 0 } else { limit_pos - 1 };

    for (idx, &(rstart, rend)) in runs.iter().enumerate() {
        if rend < limit_pos {
            let rlen = rend - rstart + 1;
            if rlen >= length {
                // Allocate from the front (just like original code)
                return Some((idx, rstart));
            }
        }
        if rstart > end_search {
            // Since runs are sorted by start and this run starts beyond limit_pos-1, no need to continue
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() {
        let disk = parse("2333133121414131402");
        let result = day9_part1(&disk);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_example_part2() {
        let disk = parse("2333133121414131402");
        let result = day9_part2(&disk);
        assert_eq!(result, 2858);
    }
}
