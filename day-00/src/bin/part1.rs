fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output); // Prints output to console
}

fn part1(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("11000\n13000");
        assert_eq!(result, 24000);
    }
}
