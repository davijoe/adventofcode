fn main() {
    let s1 = String::from("RUST");
    let len = calculate_s_length(&s1);
}

fn calculate_s_length(s: &String) -> usize {
    s.len()
}
