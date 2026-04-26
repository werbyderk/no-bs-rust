#![allow(dead_code)]
static FOO_BAR: [char; 7] = ['F', 'O', 'O', ' ', 'B', 'A', 'R'];
static BAR_BAZ: [char; 7] = ['B', 'A', 'R', ' ', 'B', 'A', 'Z'];
const FIVE: u32 = 5;

// Here, we are giving left and right two different lifetimes. In other words, a can outlive b and vice versa
pub struct Comparison<'a, 'b> {
    left: &'a str,
    right: &'b str,
}

impl<'a, 'b> Comparison<'a, 'b> {
    // write an implementation get_left(&self) on Comparison that returns a reference to self.left even if self.right is not
    // a valid reference
    // hint: fn get_left(&self) -> ...? what are we returning and its lifetime?
    fn get_left(&self) -> &'a str {
        self.left
    }
}

// write a function foo_bar_n(n: u32, my_c_arr: &'static [char]) that returns the first n characters
// of the static character array my_c_arr
// remember string slices are indexed by byte. in this very specific case,
// that is OK since we are dealing only with ASCII characters
fn foo_bar_n(n: usize, my_c_arr: &'static [char]) -> &'static [char] {
    &my_c_arr[..n]
}

#[cfg(test)]
pub mod tests {
    use crate::vid_9::{BAR_BAZ, FOO_BAR};

    use super::Comparison;

    #[test]
    fn get_left() {
        let l = "foo";
        let result;

        {
            let r = "bar";
            let c = Comparison { left: l, right: r };
            result = c.get_left();
        }

        assert_eq!(&result, &l);
    }

    #[test]
    fn foo_bar_n() {
        let foo = super::foo_bar_n(3, &FOO_BAR);
        let bar = super::foo_bar_n(3, &BAR_BAZ);
        let foo_bar = super::foo_bar_n(7, &FOO_BAR);
        assert_eq!(foo, ['F', 'O', 'O']);
        assert_eq!(bar, ['B', 'A', 'R']);
        assert_eq!(foo_bar, ['F', 'O', 'O', ' ', 'B', 'A', 'R'])
    }
}
