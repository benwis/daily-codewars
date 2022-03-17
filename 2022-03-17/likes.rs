fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]).to_string(),
        2 => format!("{} and {} like this", names[0], names[1]).to_string(),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]).to_string(),
        _ => format!(
            "{}, {} and {} others like this",
            names[0],
            names[1],
            names.len() - 2
        )
        .to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}
