use std::collections::BTreeSet;

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    l.iter()
        .flat_map(|&x| x.abs().get_prime_factor())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|p: i64| (p, l.iter().cloned().filter(|x| x % p == 0).sum()))
        .collect()
}

trait PrimeFactor {
    fn get_prime_factor(&self) -> Vec<i64>;
    fn is_prime(&self) -> bool;
}

impl PrimeFactor for i64 {
    fn get_prime_factor(&self) -> Vec<i64> {
        (1..self + 1)
            .into_iter()
            .filter(|&x| self % x == 0 && x.is_prime())
            .collect::<Vec<i64>>()
    }

    fn is_prime(&self) -> bool {
        *self != 0 && *self != 1 && (2..).take_while(|i| i * i <= *self).all(|i| *self % i != 0)
    }
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    testing(
        vec![15, 21, 24, 30, 45],
        vec![(2, 54), (3, 135), (5, 90), (7, 21)],
    );
}
