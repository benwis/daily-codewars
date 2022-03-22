fn prime_factors(n: i64) -> String {
    let mut num = n;
    let mut current = 2;
    let mut res = String::new();
    while num > 1 {
        let mut count = 0;
        while num % current == 0 {
            num /= current;
            count += 1;
        }
        match count {
            0 => (),
            1 => res.push_str(&format!("({})", current)),
            _ => res.push_str(&format!("({}**{})", current, count)),
        }
        current += 1;
    }
    res
}
fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
}
