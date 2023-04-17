pub fn process_part1(input: &str) -> u32 {
    // String containing to messages which have the same lenght
    // One character is duplicated which is the identifier
    // a - z -> 1 - 26 / A - Z -> 27 - 52
    // sum of all identifiers
    let anwswer = input
        .lines()
        .map(|s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            let mut hit: u32 = 0;
            for ch in s1.chars() {
                if s2.contains(ch) {
                    hit = match ch.is_ascii_uppercase() {
                        true => ch as u32 - 38,
                        false => ch as u32 - 96,
                    };
                    break;
                } else {
                }
            }
            hit
        })
        .sum();
    anwswer
}

pub fn process_part2(input: &str) -> u32 {
    let mut count = 0;
    let groups: Vec<Vec<&str>> = input.lines().into_iter().fold(Vec::new(), |mut acc, x| {
        if count == 3 || acc.is_empty() {
            count = 0;
            acc.push(Vec::new());
        }
        count += 1;
        acc.last_mut().unwrap().push(x);
        acc
    });
    let mut hit: u32 = 0;
    for group in groups {
        for ch in group[0].chars() {
            if group[1].contains(ch) && group[2].contains(ch) {
                hit += match ch.is_ascii_uppercase() {
                    true => ch as u32 - 38,
                    false => ch as u32 - 96,
                };
                break;
            }
        }
    }
    hit
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 70);
    }
}
