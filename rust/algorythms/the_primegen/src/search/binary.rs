//! Contains an [implementation][method@Search::binary_search]
//! of the [BinarySearch] trait for the [Search] struct

use super::{Search, units::FoundItem};


pub trait BinarySearch {
    /// A specific, generic type of item
    /// in a given slice/array where the search is to be done
    type Element;

    /// Binary search implementation
    /// 
    /// Takes a reference to the target element.
    /// Calculates the middle element position index
    /// and compares if it's the right element we are searching for
    /// 
    /// Returns: [FoundItem]
    /// 
    fn binary_search(
        &self,
        wanted: &Self::Element
    ) -> Option<FoundItem<Self::Element>>;
}


impl<'a, T> BinarySearch for Search<'a, T>
where T: PartialEq + Ord + std::ops::Add<T> +  std::fmt::Debug
{
    /// A specific type of a given slice/array item
    type Element = T;

    /// Binary Search implementation
    /// 
    /// This implementation utilizes a `loop` for the implementation.
    /// This is a deliberate choice to avoid problems with stack overflows,
    /// due to poor [tail-call][tco] optimization in Rust.
    /// 
    /// On each loop iteration a check is performed to make a decision,
    /// whether a searched value matches the current needle
    /// (pointer to a value at the middle of split).
    /// 
    /// If it's not then the next iteration is performed with some internal
    /// values (index, neddle, offset) being adjusted to split each time / 2.
    /// 
    /// There are other conditions that are taken into account such, as 
    /// the case when the needle index crosses a given slice(array) boundaries
    /// 
    /// Returns:\
    /// - [Option]<[`FoundItem`]> in case a match found\
    /// - otherwise [None]
    /// 
    /// [tco]: https://en.wikipedia.org/wiki/Tail_call
    /// 
    fn binary_search(
        &self,
        wanted: &Self::Element
    ) -> Option<FoundItem<Self::Element>>
    {
        assert!(self.haystack.len() != 0);

        // offset - is the shear position from start of the array
        // half - is the added to the shift to reach the middle element
        let (mut offset, mut half) = (0, self.haystack.len());

        use utils::{OperationResult as Res, compare};
        loop {
            match compare(&self.haystack, &wanted, &mut offset, &mut half) {
                Res::Break => break None,
                Res::Found(item) => break Some(item),
                Res::Continue => {},
            }
        }
    }
}


/// Extra functions and types that implement granular solutions for specific tasks
/// 
mod utils {
    use std::cmp::Ordering::*;
    use super::{FoundItem, };

    pub enum OperationResult<'a, T> {
        Found(FoundItem<'a, T>),
        Continue,
        Break,
    }
    use OperationResult::*;


    /// Performs comparesment operation for a given state of the binary search.
    /// 
    /// Performs edge-case checks, such as:\
    /// - stack size (if there is nothing to look for - why bother?)
    /// 
    /// # Arguments
    /// * `stack` - A reference to the original array the search is performed on
    /// * `wanted` - a reference to the item we search for
    /// * `offset` - current offset from the beginning of the `slice`
    /// * `half` - an index shift to reach the middle item within current offset
    pub fn compare<'a, T>(
        slice: &'a [T],
        wanted: &T,
        offset: &mut usize,
        half: &mut usize,
    ) -> OperationResult<'a, T>
    where
        T: PartialEq + Ord + std::ops::Add<T> + std::fmt::Debug
    {
        let len = slice.len();
        if len == 0 {
            return Break;
        }
        
        if *half == 0 { *half = 2 };
        *half /= 2;
        
        let mid_index = *offset + *half;
        let current = &slice[mid_index];
        match (current.cmp(wanted), mid_index) {
            (Equal, _)   => { Found(FoundItem(mid_index, current)) }
            (_, mid) if (mid == 0 || mid >= len) => { Break }
            (Less, _)    => {
                *offset += *half;
                Continue
            }
            (Greater, _) => Continue,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_binary_search_ordered() {
        let stack_ordered = [1,2,3,4,5,6,7,8,9,10];
        let search = Search::with_slice(&stack_ordered);
        
        for (idx, el) in stack_ordered.iter().enumerate() {
            assert_eq!(
                search.binary_search(&stack_ordered[idx]),
                Some(FoundItem(idx, el))
            );
        }
    }
}