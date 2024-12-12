use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> miette::Result<String> {
    // xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

    // Regex expression from part 1
    // let re = Regex::new(r"mul\((?<num1>[0-9]{1,3}),(?<num2>[0-9]{1,3})\)").unwrap();
    let mut total = 0;

    for captures in re.captures_iter(input) {
        let num1: i32 = captures["num1"].parse().unwrap();
        let num2: i32 = captures["num2"].parse().unwrap();
        total += num1 * num2;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        // Matches should be:
        // mul(2,4) -> 2*4=8
        // mul(5,5) -> 5*5=25
        // mul(11,8) -> 11*8=88
        // mul(8,5) -> 8*5=40
        // Sum = 8 + 25 + 88 + 40 = 161

        assert_eq!("161", process(input)?);
        Ok(())
    }
}
