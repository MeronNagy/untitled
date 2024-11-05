fn main() {
    println!("{}", greet());
}

pub fn greet() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }
}
