fn parse(code: &str) -> Vec<i32> {
    let mut res = Vec::new();
    let mut val = 0;
    for c in code.chars() {
        match c {
            'i' => val += 1,
            'd' => val -= 1,
            's' => val *= val,
            'o' => res.push(val),
            _ => { /* ignore */ }
        }
    }
    res
}
