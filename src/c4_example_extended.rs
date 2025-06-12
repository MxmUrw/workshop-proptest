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
        if !range.is_empty() && !self.contains(range.clone()) {
            let start_range = self.ranges.iter().find(|(a,b)| **a < range.start && **b >= range.start).map(|(a,b)| (*a, *b));
            let contained = self.ranges.iter().filter(|(a,b)| range.start <= **a && range.end >= **b).map(|(a,b)| (*a, *b)).collect::<Vec<_>>();
            let end_range = self.ranges.iter().find(|(a,b)| **a <= range.end && **b > range.end).map(|(a,b)| (*a, *b));

            let start = start_range.map(|(a,b)| min(a, range.start)).unwrap_or(range.start);
            let end = end_range.map(|(a,b)| max(b, range.end)).unwrap_or(range.end);

            contained.iter().for_each(|(a,_)| { self.ranges.remove(a); });
            start_range.inspect(|(a,b)| { self.ranges.remove(a); });
            end_range.inspect(|(a,b)| { self.ranges.remove(a); });
            self.ranges.insert(start, end);
        }
    }

    pub fn delete(&mut self, range: Range<u32>) {
        //
        // also implement this!
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

// This function implements a custom strategy for generating a range.
fn generate_range() -> impl Strategy<Value = Range<u32>> {
    (0..255u32, 0..255u32).prop_map(|(a,b)| a..b)
}

// We can use the `prop_oneof!` macro to build a strategy that randomly
// chooses one strategy out of a given list. This way we can generate
// enum cases.
fn generate_input() -> impl Strategy<Value = Input> {
    prop_oneof![
        generate_range().prop_map(Input::Insert),
        generate_range().prop_map(Input::Delete)
    ] 
}

// An enum type that describes all possible inputs we want to test.
#[derive(Debug, Clone)]
enum Input {
    Insert(Range<u32>),
    Delete(Range<u32>)
}

proptest! {
    #[test]
    fn test_insertion(inputs in vec(generate_input(), 0..10)) {
        let mut set = CompactSet::new();

        // call either the insert or the delete method depending
        // on input
        for input in inputs {
            match input {
                Input::Insert(range) => set.insert(range),
                Input::Delete(range) => set.delete(range),
            }
        }

        // ensure that the compactset is in a valid state.
        set.validate();
    }
}

