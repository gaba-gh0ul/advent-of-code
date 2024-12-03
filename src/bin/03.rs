use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re_1 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_2 = Regex::new(r"[0-9]+").unwrap();
    Some(
        input
            .lines()
            .map(|l| -> u32 {
                re_1.find_iter(l)
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|r| {
                        re_2.find_iter(r)
                            .fold(1, |acc, v| acc * v.as_str().parse::<u32>().unwrap())
                    })
                    .sum()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
