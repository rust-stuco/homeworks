// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)

#[test]
fn whats_that() {

    let mut vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

#[cfg(test)]
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}