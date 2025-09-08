/// A trait for splitting collections into approximately equal parts.
///
/// This trait provides functionality to divide a collection into a specified number
/// of parts, where each part contains roughly the same number of elements. When the
/// total number of elements doesn't divide evenly, some parts will be one element
/// larger than others.
///
/// # Examples
///
/// Basic usage with slices:
///
/// ```
/// use equal_parts::EqualParts;
///
/// let data = vec![1, 2, 3, 4, 5, 6];
/// let parts: Vec<&[i32]> = data.equal_parts(3).collect();
/// assert_eq!(parts, vec![&[1, 2], &[3, 4], &[5, 6]]);
/// ```
///
/// Handling uneven divisions:
///
/// ```
/// use equal_parts::EqualParts;
///
/// let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let mut iter = data.equal_parts(4);
/// // First parts get extra elements when the division isn't even.
/// assert_eq!(iter.next(), Some([1, 2, 3].as_slice()));
/// assert_eq!(iter.next(), Some([4, 5, 6].as_slice()));
/// // The size of the parts only changes once, and only by one element.
/// assert_eq!(iter.next(), Some([7, 8].as_slice()));
/// assert_eq!(iter.next(), Some([9, 10].as_slice()));
/// assert_eq!(iter.next(), None);
/// ```
///
/// When there are fewer elements than requested parts:
///
/// ```
/// use equal_parts::EqualParts;
///
/// let data = [1, 2];
/// let parts: Vec<&[i32]> = data.as_slice().equal_parts(5).collect();
/// // Only returns as many parts as there are elements
/// assert_eq!(parts, vec![&[1], &[2]]);
/// ```
pub trait EqualParts {
    /// The type of items yielded by the iterator.
    type Item;

    /// The iterator type returned by [`equal_parts`](Self::equal_parts).
    type Iter: Iterator<Item = Self::Item>;

    /// Splits the collection into approximately equal parts.
    ///
    /// Returns an iterator that yields each part as a separate item. The parts
    /// will be as equal in size as possible, with larger parts appearing first
    /// when the total length doesn't divide evenly.
    ///
    /// # Arguments
    ///
    /// * `num_parts` - The number of parts to split the collection into.
    ///   Must be greater than 0.
    ///
    /// # Panics
    ///
    /// Panics if `num_parts` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use equal_parts::EqualParts;
    ///
    /// let data = [1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut parts = data.as_slice().equal_parts(3);
    ///
    /// assert_eq!(parts.next(), Some([1, 2, 3].as_slice()));
    /// assert_eq!(parts.next(), Some([4, 5, 6].as_slice()));
    /// assert_eq!(parts.next(), Some([7, 8].as_slice()));
    /// assert_eq!(parts.next(), None);
    /// ```
    fn equal_parts(self, num_parts: usize) -> Self::Iter;
}

/// Iterator that yields approximately equal parts of a slice.
///
/// This iterator is created by calling [`equal_parts`](EqualParts::equal_parts) on a slice.
/// It yields each part as a `&[T]` slice reference.
///
/// The iterator ensures that:
/// - All parts have roughly the same size
/// - When the total length doesn't divide evenly, larger parts come first
/// - The iterator stops when all elements have been consumed
///
/// # Examples
///
/// ```
/// use equal_parts::EqualParts;
///
/// let data = [1, 2, 3, 4, 5, 6, 7];
/// let mut iter = data.as_slice().equal_parts(3);
///
/// assert_eq!(iter.next(), Some([1, 2, 3].as_slice()));
/// assert_eq!(iter.next(), Some([4, 5].as_slice()));
/// assert_eq!(iter.next(), Some([6, 7].as_slice()));
/// assert_eq!(iter.next(), None);
/// ```
pub struct EqualPartsIter<'a, T> {
    data: &'a [T],
    part_size: usize,
    full_parts_left: usize,
}

impl<'a, T> Iterator for EqualPartsIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            let split_point = if self.full_parts_left > 0 {
                self.full_parts_left -= 1;
                self.part_size
            } else {
                self.part_size - 1
            };
            let (chunk, rest) = self.data.split_at(split_point);
            self.data = rest;
            Some(chunk)
        }
    }
}

impl<'a, T> EqualParts for &'a [T] {
    type Item = &'a [T];
    type Iter = EqualPartsIter<'a, T>;

    fn equal_parts(self, num_parts: usize) -> Self::Iter {
        let part_size = self.len().div_ceil(num_parts);
        let small_part_count = part_size * num_parts - self.len();
        EqualPartsIter {
            data: self,
            part_size,
            full_parts_left: num_parts - small_part_count,
        }
    }
}

impl<'a, T> EqualParts for &'a Vec<T> {
    type Item = &'a [T];
    type Iter = EqualPartsIter<'a, T>;

    fn equal_parts(self, num_parts: usize) -> Self::Iter {
        self.as_slice().equal_parts(num_parts)
    }
}

#[cfg(test)]
mod tests {
    use super::EqualParts;

    #[test]
    fn simple_equal_parts() {
        let data: &[i32] = &[1, 2, 3, 4, 5, 6];
        let mut parts = data.equal_parts(3);
        assert_eq!(parts.next(), Some([1, 2].as_slice()));
        assert_eq!(parts.next(), Some([3, 4].as_slice()));
        assert_eq!(parts.next(), Some([5, 6].as_slice()));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn uneven_equal_parts() {
        let data: &[i32] = &[1, 2, 3, 4, 5, 6, 7];
        let mut parts = data.equal_parts(3);
        assert_eq!(parts.next(), Some([1, 2, 3].as_slice()));
        assert_eq!(parts.next(), Some([4, 5].as_slice()));
        assert_eq!(parts.next(), Some([6, 7].as_slice()));
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn not_enough_parts() {
        let data: &[i32] = &[1, 2];
        let mut parts = data.equal_parts(3);
        assert_eq!(parts.next(), Some([1].as_slice()));
        assert_eq!(parts.next(), Some([2].as_slice()));
        assert_eq!(parts.next(), None);
    }

    #[test]
    #[should_panic]
    fn panics_with_zero_parts() {
        let data: &[i32] = &[1, 2, 3];
        let _ = data.equal_parts(0);
    }

    #[test]
    fn works_on_vec() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let mut parts = data.equal_parts(3);
        assert_eq!(parts.next(), Some([1, 2].as_slice()));
        assert_eq!(parts.next(), Some([3, 4].as_slice()));
        assert_eq!(parts.next(), Some([5, 6].as_slice()));
        assert_eq!(parts.next(), None);
    }
}
