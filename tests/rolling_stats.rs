use rupture_engine::stats::{mad, median, quantile, RollingWindow};

#[test]
fn test_rolling_window_eviction() {
    let mut w = RollingWindow::new(3);
    w.push(1.0);
    w.push(2.0);
    w.push(3.0);
    assert!(w.is_full());
    assert_eq!(w.len(), 3);

    w.push(4.0);
    assert_eq!(w.len(), 3);
    let snap = w.sorted_snapshot();
    assert_eq!(snap, vec![2.0, 3.0, 4.0]);
}

#[test]
fn test_median_on_known_values() {
    let mut v = vec![5.0, 1.0, 3.0, 2.0, 4.0];
    assert_eq!(median(&mut v), 3.0);
}

#[test]
fn test_mad_on_known_values() {
    // [1, 2, 3, 4, 5]: median = 3, deviations = [2, 1, 0, 1, 2], MAD = 1.
    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!((mad(&v) - 1.0).abs() < 1e-12);
}

#[test]
fn test_quantile_at_boundaries() {
    let v = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    assert!((quantile(&v, 0.0) - 10.0).abs() < 1e-12);
    assert!((quantile(&v, 1.0) - 50.0).abs() < 1e-12);
    assert!((quantile(&v, 0.5) - 30.0).abs() < 1e-12);
}

#[test]
fn test_quantile_95() {
    let v: Vec<f64> = (0..100).map(|i| i as f64).collect();
    let q95 = quantile(&v, 0.95);
    assert!((q95 - 94.05).abs() < 0.1);
}
