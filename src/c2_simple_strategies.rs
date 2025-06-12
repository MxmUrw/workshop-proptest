use proptest::{collection::vec, prelude::*};


fn sum_all_below(below: u32, values: Vec<u32>) -> u32 {
    let mut result = 0;
    for x in values {
        if x <= below {
            result += x;
        }
    }
    return result;
}


proptest! {
    #[test]
    fn test_sum_all(below in any::<u32>(), values in vec(any::<u32>(), 0..100)) {
        assert_eq!(
            sum_all_below(below, values.clone()),
            values.iter().filter(|x| **x < below).sum()
        )
    }
}