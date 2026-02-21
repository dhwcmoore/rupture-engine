use std::collections::VecDeque;

/// Power-law (Caputo-like) memory kernel.
/// Weights are w_k = k^(-alpha) for k = 1..K, normalised so they sum to 1.
/// The kernel accumulates strain by convolving recent residuals with these weights.
pub struct MemoryKernel {
    weights: Vec<f64>,
    buffer: VecDeque<f64>,
    capacity: usize,
}

impl MemoryKernel {
    /// Create a new memory kernel with the given capacity K and decay exponent alpha.
    pub fn new(k: usize, alpha: f64) -> Self {
        let raw_weights: Vec<f64> = (1..=k).map(|i| (i as f64).powf(-alpha)).collect();
        let total: f64 = raw_weights.iter().sum();
        let weights: Vec<f64> = raw_weights.iter().map(|w| w / total).collect();

        Self {
            weights,
            buffer: VecDeque::with_capacity(k),
            capacity: k,
        }
    }

    /// Push a new residual value and compute the accumulated strain.
    /// Strain is the dot product of the weight vector with the buffer contents,
    /// where the most recent value receives weight w_1 (the largest weight).
    pub fn push_and_accumulate(&mut self, value: f64) -> f64 {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);

        let _n = self.buffer.len();
        let mut strain = 0.0;
        for (i, &val) in self.buffer.iter().rev().enumerate() {
            if i < self.weights.len() {
                strain += self.weights[i] * val;
            }
        }
        strain
    }

    /// Return a reference to the normalised weights for testing.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weights_sum_to_one() {
        let kernel = MemoryKernel::new(100, 0.65);
        let sum: f64 = kernel.weights().iter().sum();
        assert!((sum - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_weights_monotone_decreasing() {
        let kernel = MemoryKernel::new(50, 0.65);
        for i in 1..kernel.weights().len() {
            assert!(kernel.weights()[i] <= kernel.weights()[i - 1]);
        }
    }

    #[test]
    fn test_constant_input() {
        let mut kernel = MemoryKernel::new(10, 0.5);
        let mut strain = 0.0;
        for _ in 0..20 {
            strain = kernel.push_and_accumulate(1.0);
        }
        // With constant input 1.0 and normalised weights, strain should converge to 1.0.
        assert!((strain - 1.0).abs() < 1e-10);
    }
}
