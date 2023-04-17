pub fn process_part1(input: &str) -> String {
    let answer = input
        .split("\n\n")
        .map(|food| {
            food.lines()
                .map(|item| item.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    answer.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut sorted: Vec<u32> = input
        .split("\n\n")
        .map(|food| {
            food.lines()
                .map(|item| item.trim().parse::<u32>().unwrap())
                .sum::<u32>()}
        )
        .collect::<_>();
    
    sorted.sort_by(|a, b| b.cmp(a));
    let answer: u32 = sorted
        .into_iter()
        .take(3)
        .sum();

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, String::from("24000"));
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, String::from("45000"));
    }
}
