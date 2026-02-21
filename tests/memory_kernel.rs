use rupture_engine::model::memory::MemoryKernel;

#[test]
fn test_weights_sum_to_one() {
    for k in [10, 50, 100, 200] {
        for alpha in [0.3, 0.5, 0.65, 0.9] {
            let kernel = MemoryKernel::new(k, alpha);
            let sum: f64 = kernel.weights().iter().sum();
            assert!(
                (sum - 1.0).abs() < 1e-12,
                "Weights do not sum to 1 for K={}, alpha={}",
                k,
                alpha
            );
        }
    }
}

#[test]
fn test_weights_are_monotone_decreasing() {
    let kernel = MemoryKernel::new(200, 0.65);
    let w = kernel.weights();
    for i in 1..w.len() {
        assert!(
            w[i] <= w[i - 1],
            "Weight at index {} ({}) exceeds weight at index {} ({})",
            i,
            w[i],
            i - 1,
            w[i - 1]
        );
    }
}

#[test]
fn test_constant_input_converges() {
    let mut kernel = MemoryKernel::new(50, 0.5);
    let mut strain = 0.0;
    for _ in 0..100 {
        strain = kernel.push_and_accumulate(1.0);
    }
    assert!(
        (strain - 1.0).abs() < 1e-10,
        "Constant input should produce strain of 1.0 but got {}",
        strain
    );
}

#[test]
fn test_zero_input_gives_zero_strain() {
    let mut kernel = MemoryKernel::new(20, 0.65);
    for _ in 0..30 {
        let s = kernel.push_and_accumulate(0.0);
        assert!(s.abs() < 1e-15);
    }
}

#[test]
fn test_impulse_response_decays() {
    let mut kernel = MemoryKernel::new(50, 0.65);
    // Push a single impulse, then zeros.
    let s0 = kernel.push_and_accumulate(10.0);
    assert!(s0 > 0.0);
    let mut prev = s0;
    for _ in 0..49 {
        let s = kernel.push_and_accumulate(0.0);
        assert!(s <= prev, "Strain should decay after impulse");
        prev = s;
    }
}
