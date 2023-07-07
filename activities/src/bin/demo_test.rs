fn all_caps(string: &str) -> String {
    string.to_uppercase()
}

fn main() {
    println!("{}", all_caps("hello"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_caps() {
        let result = all_caps("hello");
        let expected = "HELLO";
        assert_eq!(result, expected);
    }
}
