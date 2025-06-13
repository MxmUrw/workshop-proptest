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
