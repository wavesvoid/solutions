fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

/* YOUR CODE GOES HERE */
fn trim_spaces(input: &str) -> &str {
    if input.is_empty() {
        return input;
    }
    
    let mut first_sym_pos: usize = 0;
    let mut last_sym_pos: usize = input.len();
    let mut first_met = false;
    let mut last_met = false;
    let walker = input.as_bytes();

    while first_sym_pos < last_sym_pos {
        if walker[first_sym_pos] != b' ' {
            first_met = true;
        }
        
        if !first_met { first_sym_pos += 1; }
        if !last_met  { last_sym_pos -= 1;  }

        if walker[last_sym_pos] != b' ' {
            last_met = true;
        }
        
        // When whitespace boundary reached from both sides
        if first_met && last_met {
            break;
        }
    }

    &input[first_sym_pos..last_sym_pos + 1]
}