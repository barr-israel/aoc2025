use advent_of_code::util::fast_parse;

advent_of_code::solution!(1);

const DIAL_SIZE: i32 = 100;
pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeroes = 0;
    let mut input = input.as_bytes();
    while !input.is_empty() {
        let (num, remainder): (i32, _) = fast_parse(&input[1..]);
        let virtual_dial = if input[0] == b'L' {
            dial - num
        } else {
            dial + num
        };
        input = &remainder[1..];
        dial = virtual_dial.rem_euclid(DIAL_SIZE);
        if dial == 0 {
            zeroes += 1;
        }
    }
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeroes = 0u64;
    let mut input = input.as_bytes();
    while !input.is_empty() {
        let (num, remainder): (i32, _) = fast_parse(&input[1..]);
        let virtual_dial = if input[0] == b'L' {
            dial - num
        } else {
            dial + num
        };
        input = &remainder[1..];
        let mut passes = (virtual_dial / DIAL_SIZE).abs();
        if virtual_dial <= 0 && dial > 0 {
            passes += 1;
        }
        dial = virtual_dial.rem_euclid(DIAL_SIZE);
        zeroes += passes as u64;
    }
    Some(zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
