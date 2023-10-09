mod move_semantics;
mod strings;
mod vecs;

/// Problems inspred by 15-112: https://www.kosbie.net/cmu/spring-22/15-112/notes/hw4.html

/// Take a vector and create a new vector with all of the duplicate elements removed.
/// The elements should be in the same order they appeared, unless removed.
/// You are NOT allowed to use the built-in vector method `dedup()` or any similar methods.
/// Also, ignore the clippy::ptr_arg warning you get.
#[cfg(test)]
fn create_with_removed_duplicates(v: &Vec<i32>) -> Vec<i32> {
    let mut v = v.clone();
    v.sort();

    let mut i = 1;
    while i < v.len() {
        if v[i - 1] == v[i] {
            // SAFETY: we know i is in bounds and also the element at i-1 must exist
            v.remove(i);
        } else {
            i += 1;
        }
    }

    v
}

/// Take a vector and modify it so that all of the duplicate elements are removed.
/// You are NOT allowed to use the built-in vector method `dedup()` or any similar methods.
/// The vector method `sort()` might be useful here.
/// This function should not be much different from the one above
#[cfg(test)]
fn remove_duplicates(v: &mut Vec<i32>) {
    v.sort();

    let mut i = 1;
    while i < v.len() {
        if v[i - 1] == v[i] {
            // SAFETY: we know i is in bounds and also the element at i-1 must exist
            v.remove(i);
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_destructible() {
        let vec1 = vec![3, 5, 3, 3, 6];
        let vec2 = vec![1, 3, 5, 3, 3, 2, 1, 7, 5];
        let vec3 = vec![1, 2, 3, -2];
        let new_vec1 = create_with_removed_duplicates(&vec1);
        let new_vec2 = create_with_removed_duplicates(&vec2);
        let new_vec3 = create_with_removed_duplicates(&vec3);
        assert_eq!(new_vec1, vec![3, 5, 6]);
        assert_eq!(new_vec2, vec![1, 3, 5, 2, 7]);
        assert_eq!(new_vec3, vec![1, 2, 3, -2]);
        // TODO make more tests
    }

    #[test]
    fn test_destructible() {
        let mut vec1 = vec![3, 5, 3, 3, 6];
        let mut vec2 = vec![1, 3, 5, 3, 3, 2, 1, 7, 5];
        let mut vec3 = vec![1, 2, 3, -2];
        remove_duplicates(&mut vec1);
        remove_duplicates(&mut vec2);
        remove_duplicates(&mut vec3);
        assert_eq!(vec1, vec![3, 5, 6]);
        assert_eq!(vec2, vec![1, 3, 5, 2, 7]);
        assert_eq!(vec3, vec![1, 2, 3, -2]);
        // TODO make more tests
    }
}
