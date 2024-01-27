//! Implementations of different search algorythms
//! 
//! Provides additional items for easier API interaction.
//! See [Search's structure methods][1] for more
//! 
//! [1]: self::Search

pub mod linear;
pub mod binary;



/// Provides different search methods for different algorythms
/// 
/// Stores a a reference to the original stack we are searching the value in.
///
/// # Examples
/// ```rust
/// use alg::search::{Search, units::FoundItem, linear::LinearSearch};
/// 
/// # fn main() {
/// let slice = [1,2,3,4,5,6,7,8,9];
/// let search = Search::with_slice(&slice);
/// 
/// assert_eq!(Some(FoundItem(3, &slice[3])), search.linear_search(&slice[3]));
/// # }
/// ```
/// 
#[derive(Debug, Default, PartialEq)]
pub struct Search<'a, T>
where T: PartialEq {
    haystack: &'a [T],
}

impl<'a, T> Search<'a, T>
where T: PartialEq
{
    pub fn new() -> Self {
        Self { haystack: & [] }
    }

    /// Create a new `Search` struct from a given array slice reference
    pub fn with_slice(slice: &'a [T]) -> Self {
        Self { haystack: slice }
    }

    /// Replace the internal reference to the buffer
    /// 
    pub fn replace_stack(&mut self, new_stack: &'a [T]) {
        self.haystack = new_stack;
    }
}


pub mod units {
    /// This structure is returned as a result of search
    /// 
    /// It contains metadata about the stack/array element in case it's found
    /// 
    #[derive(Debug, PartialEq)]
    pub struct FoundItem<'a, T>(
        /// And index of the found element in the given stack/array
        pub usize,
        /// A reference to the found element in the given stack/array
        pub &'a T
    );
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_default_initialization() {
        let search: Search<i32> = Default::default();
        assert_eq!(Search { haystack: &[] }, search);
    }

    #[test]
    fn test_new_from_slice() {
        let new_stack = [2, 3, 45, 5, 5];
        let mut search = Search::with_slice(&new_stack);
        assert_eq!(Search { haystack: &new_stack }, search);
    }

    #[test]
    fn test_stack_replace() {
        let mut search: Search<i32> = Default::default();
        let new_stack = [2, 3, 45, 5, 5];

        search.replace_stack(&new_stack);
        assert_eq!(Search { haystack: &new_stack }, search);
    }
}