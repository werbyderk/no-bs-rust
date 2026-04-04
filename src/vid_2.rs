#![allow(dead_code)]
// **START EDITS HERE**
fn add_5(x: u32) -> u32 {
    // Return x plus 5
}

fn add_3_subtract_2(x: u32) -> u32 {
    // Add 3 to x, then subtract 2 from x, and return the value. Use shadowing.
}

fn add_2_mod_2(x: u32) -> u32 {
    // Add 2 to x, then mod 2, return the value. Use a mutable variable in your solution.
}

fn five_four_three() -> (u8, u8, u8) {
    // return a tuple with the values 5, 4, 3; all of which are unsigned 8 bit integers. it's as easy as you think.
}
// ** END EDITS HERE **

#[cfg(test)]
pub mod tests {
    #[test]
    fn add_5() {
        let x = 3;
        let x = super::add_5(x);
        assert_eq!(x, 8);
    }

    #[test]
    fn add_3_subtract_2() {
        let x = 8;
        let x = super::add_3_subtract_2(x);
        assert_eq!(x, 9);
    }

    #[test]
    fn add_2_mod_2() {
        let x = 9;
        let x = super::add_2_mod_2(x);
        assert_eq!(x, 1);
    }

    #[test]
    fn five_four_three() {
        assert_eq!(super::five_four_three(), (5, 4, 3)); // note that we did not need to annotate 5,4,3 here (or in your solution) with u8
    }
}
