advent_of_code::solution!(2);

/// Split each line, then filter it if it does not meet the criteria, return the count post filter
/// Within the filter, split whitespace and parse the numbers out, then iterate over a window of
/// size 2, mapping a new set holding a bool (true = ascending, false = descending) and the
/// difference between the two values. Create a bool to represent whether the list should be
/// ascending or descending based on the first result of the window map. Finally, iterate over the
/// window map to ensure all values are either ascending or descending with a difference between 1
/// and 3
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|l| {
                let nums = l
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()[..]
                    .windows(2)
                    .map(|n| -> (bool, u32) {
                        match n[0] > n[1] {
                            true => return (true, n[0] - n[1]),
                            false => return (false, n[1] - n[0]),
                        }
                    })
                    .collect::<Vec<(bool, u32)>>();

                let (descending, _) = nums[0];

                nums.iter()
                    .all(|(a, r)| *a == descending && *r > 0 && *r < 4)
            })
            .count() as u32,
    )
}

/// This is a lazy solution, I essentially redid step 1 but for each line we create every possible
/// variant, that list of numbers with exactly one value dropped, plus the original variant, then
/// test each variant until we find a succesful one, otherwise, filter the line out
/// Finally count the lines remaining after filter
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|l| {
                let nums: Vec<u32> = l
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();

                let nums_len = nums.len();
                let mut variants: Vec<Vec<u32>> = Vec::with_capacity(nums_len);
                variants.push(nums.clone());

                for idx in 0..nums_len {
                    variants.push(nums.clone());
                    variants[idx + 1].remove(idx);
                }

                variants
                    .iter()
                    .find(|s| {
                        let slice_var = &s[..]
                            .windows(2)
                            .map(|n| -> (bool, u32) {
                                match n[0] > n[1] {
                                    true => return (true, n[0] - n[1]),
                                    false => return (false, n[1] - n[0]),
                                }
                            })
                            .collect::<Vec<(bool, u32)>>();

                        let (descending, _) = slice_var[0];

                        slice_var
                            .iter()
                            .all(|(a, r)| *a == descending && *r > 0 && *r < 4)
                    })
                    .is_some()
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
