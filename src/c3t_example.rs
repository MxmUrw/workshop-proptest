use std::{cmp::{max, min}, collections::BTreeMap, ops::Range};
use proptest::{arbitrary::arbitrary, collection::{btree_map, vec}, prelude::*};

/// A compact representation of a subset of u32.
/// 
/// Values in the set are represented by ranges, e.g.
/// [(0,4), (8,10)] means that all values 0..4 and 8..10 are in the set.
/// 
/// The ranges should not be empty (i.e. a key-value pair such as (5,5) is not allowed).
/// They also should not be intersecting (i.e. [(0,2), (1,4)] contains all values in 0..2 and in 1..4,
/// which is effectively just 0..4. Thus it should be always represented in it's most minimal form, as [(0,4)])
/// 
/// Since a BTreeMap is ordered by keys, the entries are ordered automatically.
struct CompactSet {
    ranges: BTreeMap<u32,u32>
}

impl CompactSet {
    pub fn new() -> Self {
        CompactSet { ranges: Default::default() }
    }

    pub fn insert(&mut self, range: Range<u32>) {
        //
        // implement this!
        //
        todo!()
    }

    pub fn contains(&self, query: Range<u32>) -> bool {
        self.ranges.iter().any(|(start, end)| start <= &query.start && end >= &query.end)
    }
}

// This function implements a custom strategy for generating a range.
//
// Given `x: impl Strategy<Value = A>` and `y: impl Strategy<Value = B>`,
// the tuple `(x, y)` is a strategy for generating tuple values, ie. `(x, y): Strategy<Value = (A, B)>`.
//
// Also, we use the `prop_map` method to turn a strategy for tuples into a strategy for ranges.
fn generate_range() -> impl Strategy<Value = Range<u32>> {
    (0..255u32, 0..255u32).prop_map(|(a,b)| a..b)
}

proptest! {
    #[test]
    fn test_insertion(range in generate_range()) {
        let mut set = CompactSet::new();

        // call the `insert` method on the generated range
        set.insert(range);

        // ---------------------------
        // ensure that state is valid

        // ranges are nonempty
        for (a,b) in &set.ranges {
            assert!(a < b, "[ranges are nonempty] current state is: {:?}", set.ranges);
        }

        // ranges are nonintersecting
        for (first, second) in set.ranges.iter().zip(set.ranges.iter().skip(1)) {
            assert!(first.1 < second.0, "[ranges are non intersecting] current state is: {:?}", set.ranges);
        }
    }
}

