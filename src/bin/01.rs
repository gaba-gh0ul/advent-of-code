advent_of_code::solution!(1);
use std::collections::{hash_map, HashMap};

/// Create a vector to store left and right values, iterate over the lines pushing each value
/// sort each vector with sort_unstable
/// Zip the lists together, mapping to the greater value minus the lesser value and then sum the
/// result
pub fn part_one(input: &str) -> Option<u32> {
    let line_count = input.lines().count();
    let mut left: Vec<u32> = Vec::with_capacity(line_count);
    let mut right: Vec<u32> = Vec::with_capacity(line_count);

    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| if l > r { l - r } else { r - l })
            .sum::<u32>(),
    )
}

/// Create a vector to store left values and a hashmap tp store right values,
/// iterate over the lines pushing each value
/// Iterate over the left list and map each value to the product of its value and the associated key
/// from right list, or 0 if there is no key, value pair, then sum the result
pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.lines().count();
    let mut left: Vec<u32> = Vec::with_capacity(line_count);
    let mut right: HashMap<u32, u32> = HashMap::with_capacity(line_count);

    input.lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        let r = r.parse::<u32>().unwrap();
        left.push(l.parse::<u32>().unwrap());
        match right.get(&r) {
            Some(value) => right.insert(r, *value + 1),
            None => right.insert(r, 1),
        };
    });

    Some(
        left.iter()
            .map(|l| match right.get(&l) {
                Some(value) => *value * l,
                None => 0,
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
