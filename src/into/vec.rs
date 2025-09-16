use crate::into::into_equal_parts::IntoEqualParts;

/// Iterator that yields approximately equal owned parts of a Vec.
///
/// This iterator is created by calling [`into_equal_parts`](IntoEqualParts::into_equal_parts)
/// on a Vec. It yields each part as an owned `Vec<T>`.
///
/// The iterator ensures that:
/// - All parts have roughly the same size
/// - When the total length doesn't divide evenly, larger parts come first
/// - The iterator stops when all elements have been consumed
///
/// # Examples
///
/// ```
/// use equal_parts::IntoEqualParts;
///
/// let data = vec![1, 2, 3, 4, 5, 6, 7];
/// let mut iter = data.into_equal_parts(3);
///
/// assert_eq!(iter.next(), Some(vec![1, 2, 3]));
/// assert_eq!(iter.next(), Some(vec![4, 5]));
/// assert_eq!(iter.next(), Some(vec![6, 7]));
/// assert_eq!(iter.next(), None);
/// ```
pub struct IntoEqualPartsIter<T> {
    data: Vec<T>,
    part_size: usize,
    full_parts_left: usize,
}

impl<T> Iterator for IntoEqualPartsIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let chunk_size = if self.full_parts_left > 0 {
            self.full_parts_left -= 1;
            self.part_size
        } else {
            self.part_size - 1
        };
        debug_assert!(chunk_size <= self.data.len());

        Some(self.data.drain(0..chunk_size).collect())
    }
}

impl<T> IntoEqualParts for Vec<T> {
    type Item = Vec<T>;
    type IntoIter = IntoEqualPartsIter<T>;

    fn into_equal_parts(self, num_parts: usize) -> Self::IntoIter {
        assert!(num_parts > 0, "Number of parts must be greater than 0");

        let part_size = self.len().div_ceil(num_parts);
        let small_part_count = part_size * num_parts - self.len();

        IntoEqualPartsIter {
            data: self,
            part_size,
            full_parts_left: num_parts - small_part_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IntoEqualParts;

    #[test]
    fn into_simple_equal_parts() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let mut parts = data.into_equal_parts(3);
        assert_eq!(parts.next(), Some(vec![1, 2]));
        assert_eq!(parts.next(), Some(vec![3, 4]));
        assert_eq!(parts.next(), Some(vec![5, 6]));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn into_uneven_equal_parts() {
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let mut parts = data.into_equal_parts(3);
        assert_eq!(parts.next(), Some(vec![1, 2, 3]));
        assert_eq!(parts.next(), Some(vec![4, 5]));
        assert_eq!(parts.next(), Some(vec![6, 7]));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn into_not_enough_parts() {
        let data = vec![1, 2];
        let mut parts = data.into_equal_parts(3);
        assert_eq!(parts.next(), Some(vec![1]));
        assert_eq!(parts.next(), Some(vec![2]));
        assert_eq!(parts.next(), None);
    }

    #[test]
    #[should_panic]
    fn into_panics_with_zero_parts() {
        let data = vec![1, 2, 3];
        let _ = data.into_equal_parts(0);
    }

    #[test]
    fn into_empty_vec() {
        let data: Vec<i32> = vec![];
        let mut parts = data.into_equal_parts(3);
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn into_single_element() {
        let data = vec![42];
        let mut parts = data.into_equal_parts(3);
        assert_eq!(parts.next(), Some(vec![42]));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn into_one_part() {
        let data = vec![1, 2, 3, 4, 5];
        let mut parts = data.into_equal_parts(1);
        assert_eq!(parts.next(), Some(vec![1, 2, 3, 4, 5]));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn into_collect_all_parts() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let parts: Vec<Vec<i32>> = data.into_equal_parts(4).collect();
        assert_eq!(
            parts,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8], vec![9, 10]]
        );
    }

    #[test]
    fn into_works_with_strings() {
        let data = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let parts: Vec<Vec<String>> = data.into_equal_parts(2).collect();
        assert_eq!(
            parts,
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "d".to_string()]
            ]
        );
    }
}
