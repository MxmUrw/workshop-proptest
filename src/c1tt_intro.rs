use proptest::{collection::vec, prelude::*};


// This is a function which should return the sum of `a` and `b`,
// but it gives a wrong result for values over 50.
fn sum(a: u32, b: u32) -> u32 {

    // this is the bug
    if a + b > 50 {
        return a + b + 1;
    } 

    a + b
}

// For testing we use the `proptest!` macro, inside of which we write our tests.
proptest! {

    #[test]
    fn test_sum(a in 0..100u32, b in 0..100u32) {
        assert_eq!(sum(a, b), a + b);
    }


    #[test]
    fn test_sum_full(a in any::<u32>(), b in any::<u32>()) {
        assert_eq!(sum(a, b), a + b);
    }

}