

/// Designates a unit value to be substracted from current Roman numerical
/// 
/// Used to represent amount that will be substracted from current number
/// in case a special numeral combination rule is met 
/// (like IV, IX, XL, XC, etc...)
struct Substract(i32);

/// Designates current Roman numeral that is being summed
/// 
/// This ZST represents an integer number in correspondance to Roman numeral
/// that will be added to common integer sum
struct Increase(i32);


#[derive(PartialEq, Debug)]
enum RomaNum { I, V, X, L, C, D, M, }

struct ResultContext {
    num: Option<RomaNum>, // Roman numeral
    count: u8, // tracks amount of time specific numeral met in a row
    result: i32, // final result as decimal
}


///! This function does validation, edge-case substraction, result modification
/// 
/// - validation: not all cases are valid,
///     i.e. repetition of the same token more than 3 times is an error
/// - edge-case substraction: not all cases are straight-forward
///     i.e. IV - is interpreted as 4, and so on - this is "edge-case"
/// - result modification: we pass whole context as parameter
///     and all the parameters are calculated inside of this function,
///     Roman numeral is matched outside and then is passed as Enum
/// 
fn repeat(context: &mut ResultContext,
          num: RomaNum,
          prev: Option<RomaNum>,
          add: Increase,
          sub: Substract)
{
    // If combination of current with previous numeral is an edge-case
    // substract needed amount from result
    context.result += add.0;

    // count previously added value too
    // so if the I was before V, we need to sub 2
    // to eliminate previous appendage step
    if context.num == prev {
        context.result -= sub.0 * 2; // mul is used to clarify the code
        // we could just pass a correct number parameter from start
    }

    // Update numeral repetition context info or reset
    if context.num.as_ref() == Some(&num) {
        if context.count >= 3 {
            panic!("The same numeral cannot be put more than 3 times in row");
        }
        context.count += 1;
    } else {
        context.num = Some(num);
        context.count = 1;
    }
}


pub fn roman_to_int(s: impl AsRef<str>) -> i32 {
    let mut context = ResultContext {
        num: None,
        count: 0u8,
        result: 0,
    };
    let mut bytes = s.as_ref().as_bytes().iter();
    let mut byte;

    loop {
        if let Some(b) = bytes.next() { byte = b; } else {
            return context.result;
        };
        match byte {
            b'I' => repeat(&mut context, RomaNum::I, None, Increase(1), Substract(0)),
            b'V' => repeat(&mut context,
                           RomaNum::V,
                           Some(RomaNum::I), Increase(5), Substract(1)),
            b'X' => repeat(&mut context,
                           RomaNum::X,
                           Some(RomaNum::I), Increase(10), Substract(1)),
            b'L' => repeat(&mut context,
                           RomaNum::L,
                           Some(RomaNum::X), Increase(50), Substract(10)),
            b'C' => repeat(&mut context,
                           RomaNum::C,
                           Some(RomaNum::X), Increase(100), Substract(10)),
            b'D' => repeat(&mut context,
                           RomaNum::D,
                           Some(RomaNum::C), Increase(500), Substract(100)),
            b'M' => repeat(&mut context,
                           RomaNum::M,
                           Some(RomaNum::C), Increase(1000), Substract(100)),
            _    => panic!("Unknown numerical value."),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    #[should_panic(expected = "The same numeral cannot be put more than 3 times in row")]
    fn test_repeat_numeral_limit() {
        roman_to_int("IIIIVVIIXXXX");
    }

    #[test]
    #[should_panic(expected = "The same numeral cannot be put more than 3 times in row")]
    fn test_failstates() {
        roman_to_int("IIIIV");
    }

    #[test]
    #[should_panic(expected = "Unknown numerical value.")]
    fn test_bad_character() {
        roman_to_int("IIIVVI9IXXXX");
    }

    #[test]
    fn test_correct_cases() {
        let cases = [
            ("III", 3),
            ("LVIII", 58),
            ("MCMXCIV", 1994),
            ("XVIII", 18),
            ("XXXVIII", 38),
            ("XLIV", 44),
            ("XLIX", 49),
            ("LXXI", 71),
        ];

        for (input, expected) in cases {
            assert_eq!(roman_to_int(input), expected);
        }
    }
}