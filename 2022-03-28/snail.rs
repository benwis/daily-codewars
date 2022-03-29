fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut in_matrix = matrix.to_owned();
    let mut out: Vec<i32> = Vec::new();

    if in_matrix.len() == 0 {
        return out;
    }

    let y_len = in_matrix[0].len();

    if y_len == 0 {
        return out;
    }

    while in_matrix.len() > 1 {
        //add all items in first row to out
        out.extend(&in_matrix[0]);
        in_matrix.remove(0);
        //add all last items minus first and last and remove them
        let last_items: Vec<i32> = in_matrix.iter_mut().map(|x| x.pop().unwrap()).collect();
        out.extend(&last_items);
        //add all of last row to out in reverse order
        let mut last_row: Vec<i32> = in_matrix.last().unwrap().to_vec();
        last_row.reverse();
        out.extend(&last_row);
        in_matrix.pop();
        //add all first items to out except first and last in reverse order
        let first_items: Vec<i32> = in_matrix.iter().rev().map(|x| x[0]).collect();
        out.extend(&first_items);
        //Remove first items from remaining vecs
        let _rem: Vec<i32> = in_matrix.iter_mut().map(|x| x.remove(0)).collect();
    }
    //Each vec should contain one item
    if in_matrix.len() > 0 {
        out.extend(&in_matrix[0]);
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
