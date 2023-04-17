use std::{ops::RangeInclusive};

pub fn process_part1(input: &str) -> u32 {
    let assignments = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(",").expect("Can not split schedules");
            let (fstart, fend) = first.split_once("-").expect("Can not parse into u32");
            let (sstart, send) = second.split_once("-").expect("Can not parse into u32");

            let fschedule = fstart.parse::<u32>().unwrap()..=fend.parse::<u32>().unwrap();
            let sschedule = sstart.parse::<u32>().unwrap()..=send.parse::<u32>().unwrap();

            (fschedule, sschedule)
        })
        .collect::<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>();

    let answer: u32 = assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a
                .clone()
                .into_iter()
                .all(|num| range_b.contains(&num));

            let b_contains_a = range_b
                .clone()
                .into_iter()
                .all(|num| range_a.contains(&num));

            a_contains_b || b_contains_a
        })
        .count()
        .try_into()
        .unwrap();
    answer
}

pub fn process_part2(input: &str) -> u32 {
    let assignments = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(",").expect("Can not split schedules");
            let (fstart, fend) = first.split_once("-").expect("Can not parse into u32");
            let (sstart, send) = second.split_once("-").expect("Can not parse into u32");

            let fschedule = fstart.parse::<u32>().unwrap()..=fend.parse::<u32>().unwrap();
            let sschedule = sstart.parse::<u32>().unwrap()..=send.parse::<u32>().unwrap();

            (fschedule, sschedule)
        })
        .collect::<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>();

        let answer: u32 = assignments
        .iter()
        .filter(|(range_a, range_b)| {
            let a_contains_b = range_a
                .clone()
                .into_iter()
                .any(|num| range_b.contains(&num));

            let b_contains_a = range_b
                .clone()
                .into_iter()
                .any(|num| range_a.contains(&num));

            a_contains_b || b_contains_a
        })
        .count()
        .try_into()
        .unwrap();
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 4);
    }
}
