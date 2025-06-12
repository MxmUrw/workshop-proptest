use proptest::{collection::vec, prelude::*};


// This function has to some all entries of `values` that are below `below`,
// but there's a bug here.
fn sum_all_below(mut below: u32, values: Vec<u32>) -> u32 {
    let mut result = 0;
    for x in values {
        if result > 4000 {
            below += 1;
        }
        if x < below {
            result += x;
        }
    }
    return result;
}

proptest! {
    // There's a `vec(value_strategy, length_strategy)` strategy that
    // generates random vectors with random elements and of random length.
    #[test]
    fn test_sum_all(below in 0..1000u32, values in vec(0..1000u32, 0..20)) {
        assert_eq!(
            sum_all_below(below, values.clone()),
            values.iter().filter(|x| **x < below).sum()
        )
    }
}