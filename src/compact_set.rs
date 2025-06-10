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
    
    /// This function verifies that the compact set conforms to its validity requirements.
    pub fn validate(&self) {
        // ranges are nonempty
        for (a,b) in &self.ranges {
            assert!(a < b, "[ranges are nonempty] current state is: {:?}", self.ranges);
        }

        // ranges are nonintersecting
        for (first, second) in self.ranges.iter().zip(self.ranges.iter().skip(1)) {
            assert!(first.1 < second.0, "[ranges are non intersecting] current state is: {:?}", self.ranges);
        }
    }
}

fn generate_ranges() -> impl Strategy<Value = Vec<Range<u32>>> {
    vec((0..255u32, 0..255u32).prop_map(|(a,b)| a..b), 0..20)
}

proptest! {
    #[test]
    fn test_insertion(ranges in generate_ranges()) {
        let mut set = CompactSet::new();
        for range in ranges {
            set.insert(range);
        }

        set.validate();
    }
}

