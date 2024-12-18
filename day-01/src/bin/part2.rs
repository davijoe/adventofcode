fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<usize>().unwrap());
        right.push(items.next().unwrap().parse::<usize>().unwrap());
    }

    let result: usize = left
        .iter()
        .map(|number| number * right.iter().filter(|r| &number == r).count())
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!("31", part2(input)?);
        Ok(())
    }
}
