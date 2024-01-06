

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut rem = x;
    let mut list: Vec<_> = vec![];

    while rem > 0 {
        list.push(rem % 10);
        rem /= 10;
    }
    
    match list.iter().cmp(list.iter().rev()) {
        std::cmp::Ordering::Equal => true,
        _ => false
    }
}

pub fn is_palindrome_1(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut min = 1;
    let mut max = 1;
    let mut tmp = x / 10;

    while tmp > 0 {
        max *= 10;
        tmp /= 10;
    }
    

    while min < max {
        let m = x / min % 10;
        let mm = x / max % 10;

        if m != mm {
            return false;
        }

        min *= 10;
        max /= 10;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    
    #[test]
    fn test_palindrome() {
        let palindrome = 12321;
        let result = is_palindrome(palindrome);

        assert!(result);
    }

    #[test]
    fn test_palindrome_1() {
        let palindrome = 1874994781;
        let result = is_palindrome_1(palindrome);

        assert!(result);
    }
}