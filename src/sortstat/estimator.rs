use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Complexity {
    Undefined,
    LogN,
    N,
    NLogN,
    NSquared,
    Exp,
}

impl fmt::Display for Complexity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Complexity::Undefined => write!(f, "Undefined"),
            Complexity::LogN => write!(f, "O(log n)"),
            Complexity::N => write!(f, "O(n)"),
            Complexity::NLogN => write!(f, "O(n*log n)"),
            Complexity::NSquared => write!(f, "O(n^2)"),
            Complexity::Exp => write!(f, "O(e^n)"),
        }
    }
}

pub struct Estimator {
    initial_size: usize,
    initial_duration: i128,
    weights: HashMap<Complexity, f64>,
}

impl Estimator {
    pub fn new(initial_size: usize, initial_duration: i128) -> Estimator {
        let mut weights: HashMap<Complexity, f64> = HashMap::with_capacity(5);
        weights.insert(Complexity::LogN, 0f64);
        weights.insert(Complexity::N, 0f64);
        weights.insert(Complexity::NLogN, 0f64);
        weights.insert(Complexity::NSquared, 0f64);
        weights.insert(Complexity::Exp, 0f64);
        Estimator {
            initial_size,
            initial_duration,
            weights,
        }
    }

    fn nlogn(x: f64) -> f64 {
        x * x.ln()
    }

    pub fn ingest(&mut self, size: usize, duration: i128) {

        /* log n */
        let k = (self.initial_duration as f64) / (self.initial_size as f64).ln();
        let predicted_duration = k * (size as f64).ln();
        self.weights.insert(
            Complexity::LogN,
            *self.weights.get(&Complexity::LogN).unwrap()
                + (predicted_duration - duration as f64).powf(2f64),
        );

        /* n */
        let k = (self.initial_duration as f64) / (self.initial_size as f64);
        let predicted_duration = k * (size as f64);
        self.weights.insert(
            Complexity::N,
            *self.weights.get(&Complexity::LogN).unwrap()
                + (predicted_duration - duration as f64).powf(2f64),
        );

        /* n^2 */
        let k = (self.initial_duration as f64) / (self.initial_size as f64).powf(2.0);
        let predicted_duration = k * (size as f64).powf(2f64);
        self.weights.insert(
            Complexity::NSquared,
            *self.weights.get(&Complexity::NSquared).unwrap()
                + (predicted_duration - duration as f64).powf(2f64),
        );

        /* n log n */
        let k = (self.initial_duration as f64) / Estimator::nlogn(self.initial_size as f64);
        let predicted_duration = k * Estimator::nlogn(size as f64);
        self.weights.insert(
            Complexity::NLogN,
            *self.weights.get(&Complexity::NLogN).unwrap()
                + (predicted_duration - duration as f64).powf(2f64),
        );

        /* exp */
        let k = (self.initial_duration as f64) / std::f64::consts::E.powf(self.initial_size as f64);
        let predicted_duration = k * std::f64::consts::E.powf(size as f64);
        self.weights.insert(
            Complexity::Exp,
            *self.weights.get(&Complexity::NLogN).unwrap()
                + (predicted_duration - duration as f64).powf(2f64),
        );
    }

    pub fn get_closest_complexity(&mut self) -> Complexity {
        let mut closest_complexity: Complexity = Complexity::Undefined;
        let mut minimal_weight: f64 = f64::MAX;
        for (complexity, weight) in self.weights.clone() {
            // println!("{} {}", complexity, weight);
            if weight < minimal_weight {
                minimal_weight = weight;
                closest_complexity = complexity;
            }
        }

        closest_complexity
    }
}
