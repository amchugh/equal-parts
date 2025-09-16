/// A trait for splitting owned collections into approximately equal owned parts.
///
/// This trait is similar to [`EqualParts`] but consumes the collection and produces
/// owned parts instead of borrowed slices. This is useful when you need to move
/// the data instead of just referencing it.
///
/// # Examples
///
/// Basic usage with Vec:
///
/// ```
/// use equal_parts::IntoEqualParts;
///
/// let data = vec![1, 2, 3, 4, 5, 6];
/// let parts: Vec<Vec<i32>> = data.into_equal_parts(3).collect();
/// assert_eq!(parts, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
/// ```
///
/// Handling uneven divisions:
///
/// ```
/// use equal_parts::IntoEqualParts;
///
/// let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let mut iter = data.into_equal_parts(4);
/// // First parts get extra elements when the division isn't even.
/// assert_eq!(iter.next(), Some(vec![1, 2, 3]));
/// assert_eq!(iter.next(), Some(vec![4, 5, 6]));
/// // The size of the parts only changes once, and only by one element.
/// assert_eq!(iter.next(), Some(vec![7, 8]));
/// assert_eq!(iter.next(), Some(vec![9, 10]));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IntoEqualParts {
    /// The type of items yielded by the iterator.
    type Item;

    /// The iterator type returned by [`into_equal_parts`](Self::into_equal_parts).
    type IntoIter: Iterator<Item = Self::Item>;

    /// Splits the collection into approximately equal owned parts.
    ///
    /// Consumes the collection and returns an iterator that yields each part
    /// as a separate owned item. The parts will be as equal in size as possible,
    /// with larger parts appearing first when the total length doesn't divide evenly.
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
    /// use equal_parts::IntoEqualParts;
    ///
    /// let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    /// let mut parts = data.into_equal_parts(3);
    ///
    /// assert_eq!(parts.next(), Some(vec![1, 2, 3]));
    /// assert_eq!(parts.next(), Some(vec![4, 5, 6]));
    /// assert_eq!(parts.next(), Some(vec![7, 8]));
    /// assert_eq!(parts.next(), None);
    /// ```
    fn into_equal_parts(self, num_parts: usize) -> Self::IntoIter;
}
