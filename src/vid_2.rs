pub fn video_2_exercises() {
    let x = 3;
    let x = add_5(x);
    assert_eq!(x, 8);

    let x = add_3_subtract_2(x);
    assert_eq!(x, 9);

    let x = add_2_mod_2(x);
    assert_eq!(x, 1);

    assert_eq!(five_four_three(), (5, 4, 3)); // note that we did not need to annotate 5,4,3 here (or in your solution) with u8
}

fn add_5(x: u32) -> u32 {
    // Return x plus 5
    x + 5
}

fn add_3_subtract_2(x: u32) -> u32 {
    // Add 3 to x, then subtract 2 from x, and return the value. Use shadowing.
    let y = x + 3;
    let y = y - 2;
    y
}

fn add_2_mod_2(x: u32) -> u32 {
    // Add 2 to x, then mod 2, return the value. Use a mutable variable in your solution.
    let mut y = x + 2;
    y %= 2;
    y
}

fn five_four_three() -> (u8, u8, u8) {
    // return a tuple with the values 5, 4, 3; all of which are unsigned 8 bit integers. it's as easy as you think.
    (5, 4, 3)
}
