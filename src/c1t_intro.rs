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

// For testing we use the `proptest!` macro, inside of which we write our tests.
proptest! {

    // The test functions have arguments, for which proptest is going to generate random values.
    // Instead of `a: u32` we write `a in STRATEGY`, where STRATEGY describes a way to generate
    // random values of a given type. The simplest strategy for numeric types is a range `x..y` which
    // is going to be sampled uniformly.
    #[test]
    fn test_sum(a in 0..100u32, b in 0..100u32) {
        assert_eq!(sum(a, b), a + b);
    }

    // We can also generate values in the full u32 range by using `any::<u32>()`.
    #[test]
    fn test_sum_full(a in any::<u32>(), b in any::<u32>()) {
        assert_eq!(sum(a, b), a + b);
    }

    // When proptest finds a combination of input values which make the test panic,
    // it will rerun the test multiple times trying to find a "minimal" combination of inputs
    // that make it fail.

}