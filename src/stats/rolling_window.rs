use std::collections::VecDeque;

/// A simple fixed-capacity rolling window backed by a VecDeque.
/// On each push, if the window is full, the oldest element is evicted.
#[derive(Debug, Clone)]
pub struct RollingWindow {
    buf: VecDeque<f64>,
    capacity: usize,
}

impl RollingWindow {
    /// Create a new rolling window with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Push a value into the window, evicting the oldest if full.
    pub fn push(&mut self, value: f64) {
        if self.buf.len() == self.capacity {
            self.buf.pop_front();
        }
        self.buf.push_back(value);
    }

    /// Return whether the window has reached its full capacity.
    pub fn is_full(&self) -> bool {
        self.buf.len() == self.capacity
    }

    /// Return the number of elements currently in the window.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Return a sorted copy of the current window contents.
    pub fn sorted_snapshot(&self) -> Vec<f64> {
        let mut v: Vec<f64> = self.buf.iter().copied().collect();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    /// Return a copy of the current window contents in insertion order.
    pub fn as_slice(&self) -> Vec<f64> {
        self.buf.iter().copied().collect()
    }
}
