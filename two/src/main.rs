fn do_cmp(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", do_cmp("a", "aaabbb"));
}
