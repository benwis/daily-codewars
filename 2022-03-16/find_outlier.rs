fn find_outlier(values: &[i32]) -> i32 {
    if values[0] % 2 == values[1] % 2 {
        //If both even or odd, set search target
        let target = match values[0].abs() % 2 {
            0 => 1,
            1 => 0,
            _ => panic!("x.abs()%2 shouldn't return anything other than 0 or 1!"),
        };
        values
            .iter()
            .find(|&&x| x.abs() % 2 == target)
            .unwrap()
            .to_owned()
    } else {
        //If the numbers are mixed, the value is present within, compare to third
        if values[0] % 2 == values[2] % 2 {
            values[1]
        } else {
            values[0]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
