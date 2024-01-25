//! Implementations of different search algorythms
//! 
//! Provides additional items for easier API interaction.
//! See [Search's structure methods][1] for more
//! 
//! [1]: self::Search

pub mod linear;



/// Provides different search methods for different algorythms
/// 
/// Stores a Cow type to immutably borrow the stack from the user.
/// In case some transformations are needed it gets converted into owned type.
///
#[derive(Debug)]
pub struct Search<'a, T>
where T: PartialEq {
    haystack: &'a [T],
}

impl<'a, T> Search<'a, T>
where T: PartialEq
{
    pub fn new(stack: &'a [T]) -> Self {
        Self { haystack: stack }
    }
}

