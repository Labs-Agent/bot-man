pub fn main_middleman(inferred_json:String) -> String {
    let mut response = String::new();
    response.push_str(&inferred_json);
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_middleman() {
        let inferred_json = String::from("Hello, World!");
        let expected = String::from("Hello, World!");
        let result = main_middleman(inferred_json);
        assert_eq!(result, expected);
    }
}