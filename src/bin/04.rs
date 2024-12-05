advent_of_code::solution!(4);

/// Read input into a 2D Vector
/// iterate over the list, when an 'X' is found check each direction for a match
/// simple bounds checking is done at the beginning of each directional check
pub fn part_one(input: &str) -> Option<u32> {
    let chart: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let (width, height) = (chart[0].len(), chart.len());
    let mut sum = 0;

    for (y, line) in chart.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                // North
                if y >= 3
                    && chart[y - 1][x] == 'M'
                    && chart[y - 2][x] == 'A'
                    && chart[y - 3][x] == 'S'
                {
                    sum += 1;
                }
                // North East
                if y >= 3
                    && x <= width - 4
                    && chart[y - 1][x + 1] == 'M'
                    && chart[y - 2][x + 2] == 'A'
                    && chart[y - 3][x + 3] == 'S'
                {
                    sum += 1;
                }
                // East
                if x <= width - 4
                    && chart[y][x + 1] == 'M'
                    && chart[y][x + 2] == 'A'
                    && chart[y][x + 3] == 'S'
                {
                    sum += 1;
                }

                // South East
                if y <= height - 4
                    && x <= width - 4
                    && chart[y + 1][x + 1] == 'M'
                    && chart[y + 2][x + 2] == 'A'
                    && chart[y + 3][x + 3] == 'S'
                {
                    sum += 1;
                }
                // South
                if y <= height - 4
                    && chart[y + 1][x] == 'M'
                    && chart[y + 2][x] == 'A'
                    && chart[y + 3][x] == 'S'
                {
                    sum += 1;
                }
                // South West
                if y <= height - 4
                    && x >= 3
                    && chart[y + 1][x - 1] == 'M'
                    && chart[y + 2][x - 2] == 'A'
                    && chart[y + 3][x - 3] == 'S'
                {
                    sum += 1;
                }
                // West
                if x >= 3
                    && chart[y][x - 1] == 'M'
                    && chart[y][x - 2] == 'A'
                    && chart[y][x - 3] == 'S'
                {
                    sum += 1;
                }
                // North West
                if y >= 3
                    && x >= 3
                    && chart[y - 1][x - 1] == 'M'
                    && chart[y - 2][x - 2] == 'A'
                    && chart[y - 3][x - 3] == 'S'
                {
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

/// Parse the input into a 2D vector
/// loop over, finding any 'A' and then check for diagnols
pub fn part_two(input: &str) -> Option<u32> {
    let chart: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let (width, height) = (chart[0].len(), chart.len());
    let mut sum = 0;

    // skip 1 and take 2 less than the height/width for each iter because
    // the actual edges do not apply
    // Find and 'A', check if both diagnols are a match
    for (y, line) in chart.iter().enumerate().skip(1).take(height - 2) {
        for (x, c) in line.iter().enumerate().skip(1).take(width - 2) {
            if *c == 'A'
                && ((chart[y - 1][x - 1] == 'M' && chart[y + 1][x + 1] == 'S')
                    || (chart[y + 1][x + 1] == 'M' && chart[y - 1][x - 1] == 'S'))
                && ((chart[y - 1][x + 1] == 'M' && chart[y + 1][x - 1] == 'S')
                    || (chart[y + 1][x - 1] == 'M' && chart[y - 1][x + 1] == 'S'))
            {
                sum += 1;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
