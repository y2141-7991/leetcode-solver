use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};
struct Solution;

type Arrivals = BinaryHeap<Reverse<ArrivalWrapper>>;
type Departures = BinaryHeap<Reverse<DepartureWrapper>>;
type Chairs = BinaryHeap<Reverse<i32>>;

impl Solution {
    const EXPECT_TIMES_DIMENSIONS: &'static str =
        "Elements in `times` should be contain exactly two elements.";

    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        // Initial setup.
        let mut arrivals = Self::generate_arrivals(times);
        let mut departures = Departures::new();
        let mut old_chairs = Chairs::new();
        let mut new_chair = 1i32;
        let mut departing = match arrivals.pop() {
            Some(Reverse(ArrivalWrapper(friend))) => Some(DepartureWrapper { friend, chair: 0 }),
            None => panic!("Should have at least one friend arriving"),
        };

        // Early exit if needed.
        if departing
            .expect("Should have at least one friend arriving")
            .friend
            .id
            == target_friend
        {
            return 0;
        }

        // Main loop.
        while let Some(Reverse(ArrivalWrapper(arriving))) = arrivals.pop() {
            // Clear friends that depart before the current arrival
            // and reclaim chairs.
            while let Some(DepartureWrapper { friend, chair }) = departing {
                if arriving.arrival < friend.departure {
                    break;
                }
                old_chairs.push(Reverse(chair));
                departing = departures.pop().map(|rev| rev.0);
            }
            // Get the chair the new arrival will sit in.
            let chair = match old_chairs.pop() {
                Some(Reverse(chair)) => chair,
                None => {
                    let increment = new_chair + 1;
                    std::mem::replace(&mut new_chair, increment)
                }
            };
            // Return the answer if the arrival is the target friend.
            if arriving.id == target_friend {
                return chair;
            }
            // Otherwise push the friend onto the awaiting departures.
            departures.push(Reverse(DepartureWrapper {
                friend: arriving,
                chair,
            }));
            // Update the next friend to depart.
            if let Some(might_swap) = departing.as_mut() {
                let mut heap_min = departures
                    .peek_mut()
                    .expect("There should be a value on the departures heap");
                if heap_min.0 < *might_swap {
                    std::mem::swap(&mut heap_min.0, might_swap);
                }
            } else {
                departing = departures.pop().map(|rev| rev.0);
            }
        }
        panic!("The target friend never arrived.");
    }

    fn generate_arrivals(times: Vec<Vec<i32>>) -> Arrivals {
        let mut heap = BinaryHeap::with_capacity(times.len());
        for (id, vector) in (0i32..).zip(times) {
            let [arrival, departure] =
                <[i32; 2]>::try_from(vector).expect(Self::EXPECT_TIMES_DIMENSIONS);
            heap.push(Reverse(ArrivalWrapper(Friend {
                id,
                arrival,
                departure,
            })));
        }
        heap
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Friend {
    id: i32,
    arrival: i32,
    departure: i32,
}

#[derive(Debug, Default, Clone, Copy, Hash)]
struct ArrivalWrapper(Friend);

impl PartialEq for ArrivalWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for ArrivalWrapper {}

impl PartialOrd for ArrivalWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArrivalWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0.arrival, self.0.id).cmp(&(other.0.arrival, other.0.id))
    }
}

#[derive(Debug, Default, Clone, Copy, Hash)]
struct DepartureWrapper {
    friend: Friend,
    chair: i32,
}

impl PartialEq for DepartureWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for DepartureWrapper {}

impl PartialOrd for DepartureWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DepartureWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.friend.departure, self.friend.id).cmp(&(other.friend.departure, other.friend.id))
    }
}

fn main() {}
