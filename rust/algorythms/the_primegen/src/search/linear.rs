//! Contains an [implementation][method@Search::linear_search] of [LinearSearch] trait
//! for the [`Search`] struct

use super::{Search, units::FoundItem};


pub trait LinearSearch {
    type Element;

    /// Implementation of the linear search
    /// 
    /// Takes a reference to the target element.
    /// Returns: [element pack][pack]
    /// 
    /// [pack]: FoundItem
    /// 
    fn linear_search(
        &self,
        needle: &Self::Element
    ) -> Option<FoundItem<Self::Element>>;
}


impl<'a, T> LinearSearch for Search<'a, T>
where T: PartialEq
{
    type Element = T;

    fn linear_search(
        &self,
        needle: &Self::Element
    ) -> Option<FoundItem<Self::Element>>
    {
        for (idx, elem) in self.haystack.iter().enumerate() {
            if elem == needle {
                return Some(FoundItem(idx, elem))
            }
        }
        None
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    mod units {
        #[derive(Debug, PartialEq)]
        // Special struct representing array element
        pub struct Number(pub String);
    }


    #[test]
    fn test_references() {
        let mut some_array = vec![];
        
        for num in 1..20 {
            some_array.push(units::Number(num.to_string()));
        }

        let search = Search::with_slice(&some_array.as_slice().as_ref());
        let found = search.linear_search(&units::Number(3.to_string())).unwrap();
        
        assert_eq!(&units::Number(3.to_string()), found.1);
        assert_eq!(FoundItem(2, &units::Number(3.to_string())), found);
    }
}