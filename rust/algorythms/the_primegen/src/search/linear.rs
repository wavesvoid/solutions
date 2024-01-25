//! Implements a linear search method for the [Search structure][1]
//! 
//! [1]: `super::Search`

use super::Search;


/// This structure is returned as a result of search
/// 
/// It contains index that represents a found element's position.
/// And a reference to the element itself
#[derive(Debug, PartialEq)]
pub struct ElemPack<'a, T>(pub usize, pub &'a T);


impl<'a, T> Search<'a, T>
where
    T: PartialEq
{
    /// Implementation of the linear search
    /// 
    /// Takes a reference to the target element.
    /// Returns: [element pack][pack]
    /// 
    /// [pack]: ElemPack
    /// 
    pub fn linear(&self, needle: &T) -> Option<ElemPack<T>> {
        for (idx, elem) in self.haystack.iter().enumerate() {
            if elem == needle {
                return Some(ElemPack(idx, elem))
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
        pub struct Number(pub String);  // Special struct representing array element
    }


    #[test]
    fn test_references() {
        let mut some_array = vec![];
        
        for num in 1..20 {
            some_array.push(units::Number(num.to_string()));
        }

        let search = Search::new(&some_array.as_slice().as_ref());
        let found = search.linear(&units::Number(3.to_string())).unwrap();
        
        assert_eq!(&units::Number(3.to_string()), found.1);
        assert_eq!(ElemPack(2, &units::Number(3.to_string())), found);
    }
}