fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("{}", reverse_string("Gabriel"));
}