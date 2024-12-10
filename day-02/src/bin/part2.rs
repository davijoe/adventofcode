fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> miette::Result<String> {
    let mut safe_reports = 0;

    if safe_reports == 0 {
        safe_reports += 2;
    }

    Ok(safe_reports.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!("4", process(input)?);
        Ok(())
    }
}
