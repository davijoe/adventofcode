fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> miette::Result<String> {
    let mut safe_reports = 0;

    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        // [7, 6, 4, 2, 1]
        // [1, 2, 7, 8, 9]
        let is_increasing = report
            .windows(2)
            .all(|lvl| (lvl[0] <= lvl[1]) && (lvl[1] - lvl[0] >= 1 && lvl[1] - lvl[0] <= 3));
        let is_decreasing = report
            .windows(2)
            .all(|lvl| (lvl[0] >= lvl[1]) && (lvl[0] - lvl[1] >= 1 && lvl[0] - lvl[1] <= 3));

        if is_increasing || is_decreasing {
            safe_reports += 1;
        }
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

        assert_eq!("2", process(input)?);
        Ok(())
    }
}
