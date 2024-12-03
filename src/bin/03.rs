use regex::Regex;

advent_of_code::solution!(3);

/// Parse each line, use a regular expression to extract the number from any matching substrings
/// parse the numbers into u32, multiply them, then add them to the accumulator
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(input.lines().fold(0, |acc, l| {
        acc + re.captures_iter(l).fold(0, |a, c| {
            let (_, [first, second]) = c.extract();
            a + first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap()
        })
    }))
}

/// Similar to step 1, but we cannot find matching strings and captures at the same time
/// create a bool variable called pass to track whether should be processing values
/// first find any substrings which now also include "don't" and "do"
/// iterator over them, match to the string and value of pass
/// if the value is "do", set pass to true, if "don't", set pass to false
pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d+,\d+\)|don\'t|do").unwrap();
    let cg = Regex::new(r"\((\d+),(\d+)\)").unwrap();
    let mut pass = false;

    Some(input.lines().fold(0, |acc, l| {
        acc + re.find_iter(l).fold(0, |a, h| match (pass, h.as_str()) {
            (_, "do") => {
                pass = false;
                a
            }
            (_, "don't") => {
                pass = true;
                a
            }
            (false, _) => {
                a + cg
                    .captures(h.as_str())
                    .map(|c| {
                        let (_, [first, second]) = c.extract();
                        first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap()
                    })
                    .unwrap()
            }
            (true, _) => a,
        })
    }))
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
        assert_eq!(result, Some(48));
    }
}
