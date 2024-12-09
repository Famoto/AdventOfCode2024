use std::collections::HashMap;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<i32> {
    // Parse the given input line
    // The digits alternate between file-length and free-length
    // First digit: file length (file #0), second digit: free length, third: file (file #1), etc.

    let chars: Vec<char> = input.trim().chars().collect();
    let mut disk = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    let mut i = 0;
    while i < chars.len() {
        let length = chars[i].to_digit(10).unwrap() as usize;

        if length > 0 {
            if is_file {
                // Add 'length' blocks of file_id
                for _ in 0..length {
                    disk.push(file_id as i32);
                }
                file_id += 1;
            } else {
                // Add 'length' blocks of free space (-1)
                for _ in 0..length {
                    disk.push(-1);
                }
            }
        }
        is_file = !is_file;
        i += 1;
    }

    disk
}

#[aoc(day9, part1)]
fn day9_part1(disk: &Vec<i32>) -> i64 {
    // We need to compact the disk according to the rules:
    // Move file blocks one at a time from the end of the disk to the leftmost free space,
    // until no free spaces appear before any file block.
    let mut disk = disk.clone();

    // Keep trying until we can't move any blocks
    loop {
        // Find leftmost free space
        let fpos = match disk.iter().position(|&x| x == -1) {
            Some(pos) => pos,
            None => break, // no free space at all
        };

        // Find the rightmost file block that is to the right of fpos
        // i.e. last file block after fpos
        let rpos = match disk
            .iter()
            .rposition(|&x| x != -1 && disk.iter().take_while(|&&y| y == -1).count() < disk.len())
        {
            Some(pos) if pos > fpos => {
                // Ensure that there is at least one free space before we reach this file block
                // If no free space after we find a file block, no move needed
                if pos > fpos {
                    pos
                } else {
                    break;
                }
            }
            _ => break, // No file block after a free space
        };

        // Move that block
        disk[fpos] = disk[rpos];
        disk[rpos] = -1;

        // Check if we can still move, continue loop
        // The loop continues until stable
    }

    // Compute checksum
    // sum of (index * file_id) where file_id != -1
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

    // Identify the files and their ranges
    let files = find_files(&disk);

    // Sort files by descending file_id
    let mut file_ids: Vec<i32> = files.keys().cloned().collect();
    file_ids.sort_unstable_by(|a, b| b.cmp(a));

    // For each file (descending ID), try to move it
    for &fid in &file_ids {
        let (start, end) = files[&fid];
        let file_length = (end - start + 1) as usize;

        // Try to find a span of free space to the left of 'start' that can fit the file
        // We'll look for a contiguous run of -1 that ends before 'start'
        if let Some((fstart, fend)) = find_free_space(&disk, file_length, start) {
            // Move the file to [fstart..fstart+file_length]
            move_file(&mut disk, fid, start, end, fstart);
        }
    }

    // Compute checksum
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

/// Find a span of free space of at least `length` blocks that ends before `limit_pos`.
/// We'll search from the beginning of the disk up to `limit_pos - 1`.
fn find_free_space(disk: &Vec<i32>, length: usize, limit_pos: usize) -> Option<(usize, usize)> {
    let mut count = 0;
    let mut start = 0;

    let end_search = if limit_pos == 0 { 0 } else { limit_pos - 1 };

    for i in 0..=end_search {
        if disk[i] == -1 {
            if count == 0 {
                start = i;
            }
            count += 1;
            if count >= length {
                // Found a span [start..start+length)
                return Some((start, start + length - 1));
            }
        } else {
            count = 0;
        }
    }

    None
}

/// Move the entire file with ID `fid` currently at [start..end] to [dest_start..dest_start+(end-start+1)]
fn move_file(disk: &mut Vec<i32>, fid: i32, old_start: usize, old_end: usize, dest_start: usize) {
    let length = old_end - old_start + 1;
    // Clear old location
    for i in old_start..=old_end {
        disk[i] = -1;
    }
    // Place file blocks at new location
    for i in 0..length {
        disk[dest_start + i] = fid;
    }
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
        // Given from the problem statement:
        // The first example from above (2333133121414131402)
        // After using the new method of moving whole files,
        // the final checksum given is 2858.
        let disk = parse("2333133121414131402");
        let result = day9_part2(&disk);
        assert_eq!(result, 2858);
    }
}
