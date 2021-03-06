use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::statevector::StateVector;

/// Map classical registers with values and number of outcomes.
pub type Histogram = HashMap<String, Vec<(u64, usize)>>;

/// Represent the result of a simulation.
///
/// API functions such as [`simulate()`] or [`simulate_with_shots()`] return
/// `Computation` instances.
///
/// # Examples:
///
/// See [`simulate()`] or [`simulate_with_shots()`] for an example of generating
/// a `Computation` instance.
///
/// [`simulate()`]: ./fn.simulate.html
/// [`simulate_with_shots()`]: ./fn.simulate_with_shots.html
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Computation {
    statevector: StateVector,
    memory: HashMap<String, u64>,
    probabilities: Vec<f64>,
    histogram: Option<Histogram>,
}

impl Computation {
    /// Create a new computation.
    ///
    /// Probabilities are computed from the state-vector.
    pub fn new(
        memory: HashMap<String, u64>,
        statevector: StateVector,
        histogram: Option<Histogram>,
    ) -> Self {
        Computation {
            probabilities: statevector.probabilities(),
            statevector,
            memory,
            histogram,
        }
    }

    /// Return the statevector of the quantum system.
    pub fn statevector(&self) -> &StateVector {
        &self.statevector
    }

    /// Return an associative map with classical names and the classical outcomes.
    pub fn memory(&self) -> &HashMap<String, u64> {
        &self.memory
    }

    /// Return the probabilities associated with the state-vector.
    pub fn probabilities(&self) -> &[f64] {
        &self.probabilities
    }

    /// Return the histogram when simulating with several shots.
    pub fn histogram(&self) -> &Option<Histogram> {
        &self.histogram
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HistogramBuilder {
    histogram: Histogram,
}

impl HistogramBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, memory: &HashMap<String, u64>) {
        for (key, current_value) in memory {
            if !self.histogram.contains_key(key) {
                self.histogram.insert(key.clone(), Vec::new());
            }
            let values = self.histogram.get_mut(key).expect("get values for key");
            match values.binary_search_by_key(&current_value, |(v, _)| v) {
                Err(idx) => values.insert(idx, (*current_value, 1)),
                Ok(found) => values[found].1 += 1,
            }
        }
    }

    pub fn histogram(self) -> Histogram {
        self.histogram
    }
}

#[cfg(test)]
mod test {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn test_histogram_builder_empty_histogram() {
        let builder = HistogramBuilder::new();
        let histogram = builder.histogram();
        assert_eq!(histogram, HashMap::new());
    }

    #[test]
    fn test_histogram_builder_one_update() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), 1)]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![("a".into(), vec![(1, 1)])])
        );
    }

    #[test]
    fn test_histogram_builder_couple_of_updates() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), 1)]));
        builder.update(&HashMap::from_iter(vec![("a".into(), 1)]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![("a".into(), vec![(1, 2)])])
        );
    }

    #[test]
    fn test_histogram_builder_couple_of_registers() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), 1)]));
        builder.update(&HashMap::from_iter(vec![("b".into(), 1)]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![("a".into(), vec![(1, 1)]), ("b".into(), vec![(1, 1)])])
        );
    }

    #[test]
    fn test_histogram_builder_different_values() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), 5)]));
        builder.update(&HashMap::from_iter(vec![("b".into(), 4)]));
        builder.update(&HashMap::from_iter(vec![("a".into(), 3)]));
        builder.update(&HashMap::from_iter(vec![("b".into(), 2)]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![
                ("a".into(), vec![(3, 1), (5, 1)]),
                ("b".into(), vec![(2, 1), (4, 1)])
            ])
        );
    }

    #[test]
    fn test_histogram_builder_different_repeated_values() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), 5)]));
        builder.update(&HashMap::from_iter(vec![("b".into(), 4)]));
        builder.update(&HashMap::from_iter(vec![("a".into(), 5)]));
        builder.update(&HashMap::from_iter(vec![("b".into(), 2)]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![
                ("a".into(), vec![(5, 2)]),
                ("b".into(), vec![(2, 1), (4, 1)])
            ])
        );
    }
}
