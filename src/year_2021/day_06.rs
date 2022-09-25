use std::collections::VecDeque;

fn lantern_fish_total(input: &str, days: u64) -> u64 {
    let mut counts = VecDeque::from([0; 9]);
    for num in input.split(',') {
        let timer = num.parse().unwrap();
        counts[timer] += 1;
    }
    for _ in 0..days {
        counts[7] += counts[0];
        counts.rotate_left(1);
    }
    counts.iter().sum()
}

pub fn part_1(input: &str) -> u64 {
    lantern_fish_total(input, 80)
}

pub fn part_2(input: &str) -> u64 {
    lantern_fish_total(input, 256)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    static EXAMPLE: &str = "3,4,3,1,2";

    static INPUT: &str = include_str!("testdata/day_06.txt");

    #[test_case(EXAMPLE, 5934)]
    #[test_case(INPUT, 389726)]
    fn part_1(input: &str, actual: u64) {
        assert_eq!(super::part_1(input), actual);
    }

    #[test_case(EXAMPLE, 26984457539)]
    #[test_case(INPUT, 1743335992042)]
    fn part_2(input: &str, actual: u64) {
        assert_eq!(super::part_2(input), actual);
    }
}
