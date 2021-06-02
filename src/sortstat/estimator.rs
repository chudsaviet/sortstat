use std::collections::HashMap;
use std::f64::consts;
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
            Complexity::NSquared => write!(f, "O(n**2)"),
            Complexity::Exp => write!(f, "O(e**n)"),
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
        //weights.insert(Complexity::Exp, 0f64);
        Estimator {
            initial_size,
            initial_duration,
            weights,
        }
    }

    fn add_weight(
        &mut self,
        complexity: Complexity,
        predicted_coefficient: f64,
        actual_duration: i128,
    ) {
        let predicted_duration = ((actual_duration - self.initial_duration) as f64) * predicted_coefficient;
        let delta = ((actual_duration as f64) - predicted_duration).abs();
        self.weights.insert(complexity, self.weights.get(&complexity).unwrap() + delta.powf(2f64));
    }

    pub fn ingest(&mut self, size: usize, duration: i128) {
        let p: f64 = size as f64 - self.initial_size as f64;
        self.add_weight(Complexity::LogN, p.ln(), duration);
        self.add_weight(Complexity::N, p, duration);
        self.add_weight(Complexity::NLogN, p * p.ln(), duration);
        self.add_weight(Complexity::NSquared, p.powf(2f64), duration);
        //self.add_weight(Complexity::Exp, consts::E.powf(p), duration);
    }

    pub fn get_closest_complexity(&mut self) -> Complexity {
        let mut closest_complexity: Complexity = Complexity::Undefined;
        let mut minimal_weight: f64 = f64::MAX;
        for (complexity, weight) in self.weights.clone() {
            println!("{} {}", complexity, weight);
            if weight < minimal_weight {
                minimal_weight = weight;
                closest_complexity = complexity;
            }
        }

        closest_complexity
    }
}
