fn descending_order(x: u64) -> u64 {
    //Convert to char to break into digits
    let mut digits: Vec<u64> = x
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as u64))
        .collect();
    //Sort by descending order
    digits.sort_by(|a, b| b.cmp(a));
    //Collect back into a u64
    digits.iter().fold(0, |acc, elem| acc * 10 + elem)
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
