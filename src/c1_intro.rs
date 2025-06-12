use proptest::{collection::vec, prelude::*};

// This is a function which should return the sum of `a` and `b`,
// but it gives a wrong result for values over 50.
fn sum(a: u32, b: u32) -> u32 {
    if a + b > 50 {
        a + b + 1
    } else {
        a + b
    }
}