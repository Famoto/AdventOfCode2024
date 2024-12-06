use std::collections::{HashMap, VecDeque};

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split('|').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates = sections[1]
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

#[aoc(day5, part1)]
fn day5_part1(input: &(Vec<(usize, usize)>, Vec<Vec<usize>>)) -> usize {
    let (rules, updates) = input;

    // Build adjacency list for rules
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(x, y) in rules {
        adj_list.entry(x).or_default().push(y);
    }

    // Iterate through updates and check their validity
    updates
        .iter()
        .filter(|update| {
            // Check if the update is valid
            is_valid_update(update, &adj_list)
        })
        .map(|update| {
            // If valid, calculate and return the middle page number
            update[update.len() / 2]
        })
        .sum()
}

#[aoc(day5, part2)]
fn day5_part2(input: &(Vec<(usize, usize)>, Vec<Vec<usize>>)) -> usize {
    let (rules, updates) = input;

    // Build adjacency list for rules
    let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();

    for &(x, y) in rules {
        adj_list.entry(x).or_default().push(y);
        *in_degree.entry(y).or_default() += 1;
        in_degree.entry(x).or_default();
    }

    // Part 2: Find the middle page numbers of incorrectly ordered updates after correcting them
    updates
        .iter()
        .filter(|update| !is_valid_update(update, &adj_list)) // Only consider invalid updates
        .map(|update| {
            // Correct the order of the update using topological sort
            let ordered_update = reorder_update(update, &adj_list);
            // Return the middle page number of the ordered update
            ordered_update[ordered_update.len() / 2]
        })
        .sum()
}

fn is_valid_update(update: &Vec<usize>, adj_list: &HashMap<usize, Vec<usize>>) -> bool {
    // For each pair of pages in the update, check if the ordering rules are respected
    for (before, after) in adj_list
        .iter()
        .flat_map(|(&before, after)| after.iter().map(move |&after| (before, after)))
    {
        // Check if 'before' appears before 'after' in the update
        if let (Some(i_before), Some(i_after)) = (
            update.iter().position(|&p| p == before),
            update.iter().position(|&p| p == after),
        ) {
            if i_before > i_after {
                return false; // Order is incorrect, return false
            }
        }
    }
    true // All rules are satisfied
}

fn reorder_update(update: &Vec<usize>, adj_list: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    // Build a local in-degree map and adjacency list for the current update
    let mut local_in_degree: HashMap<usize, usize> = HashMap::new();
    let mut local_adj_list: HashMap<usize, Vec<usize>> = HashMap::new();

    for &node in update {
        local_in_degree.insert(node, 0); // Initialize in-degrees
    }

    // Build the local adjacency list and in-degree map
    for &node in update {
        if let Some(neighbors) = adj_list.get(&node) {
            for &neighbor in neighbors {
                if update.contains(&neighbor) {
                    local_in_degree
                        .entry(neighbor)
                        .and_modify(|d| *d += 1)
                        .or_insert(1);
                    local_adj_list.entry(node).or_default().push(neighbor);
                }
            }
        }
    }

    // Initialize the queue with nodes that have in-degree 0
    let mut queue: VecDeque<usize> = local_in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut ordered_update = Vec::new();
    while let Some(current) = queue.pop_front() {
        ordered_update.push(current);
        if let Some(neighbors) = local_adj_list.get(&current) {
            for &neighbor in neighbors {
                if local_in_degree.contains_key(&neighbor) {
                    *local_in_degree.get_mut(&neighbor).unwrap() -= 1;
                    if local_in_degree[&neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Return the correctly ordered update
    ordered_update
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = indoc! {
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        };
        assert_eq!(day5_part1(&parse(input)), 143);
    }
    #[test]
    fn part2_example() {
        let input = indoc! {
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        };
        assert_eq!(day5_part2(&parse(input)), 123);
    }
}
