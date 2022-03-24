fn perimeter(n: u64) -> u64 {
    let mut sides: Vec<u64> = Vec::from([0, 1]);

    for i in 2..n + 2 {
        sides.push(sides[(i - 1) as usize] + sides[(i - 2) as usize]);
    }
    sides.iter().sum::<u64>() * 4
}
fn dotest(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
    dotest(5, 80);
    dotest(7, 216);
    dotest(20, 114624);
    dotest(30, 14098308);
}
