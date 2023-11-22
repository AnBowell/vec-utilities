use vec_utilities::Stats;

#[test]
fn test_mean() {
    let test_vec = vec![2.0, 2.0, 2.0];

    let mean = test_vec.mean();

    assert_eq!(mean.unwrap(), 2.0);

    let test_vec = vec![1, 2];

    let mean = test_vec.mean();

    assert_eq!(mean.unwrap(), 1); // Cast truncates and doesn't round

    let test_vec = vec![1, 2, 3];

    let mean = test_vec.mean();

    assert_eq!(mean.unwrap(), 2);
}

#[test]
fn test_nan_mean() {
    let test_vec = vec![2.0, 2.0, 2.0, f64::NAN];

    let mean = test_vec.nan_mean();

    assert_eq!(mean.unwrap(), 2.0);
}

#[test]
fn test_median() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let median = test_vec.median();

    assert_eq!(median.unwrap(), 3.0);
}

#[test]
fn test_nan_median() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, f64::NAN];

    let median = test_vec.nan_median();

    assert_eq!(median.unwrap(), 3.0);
}

#[test]
fn test_mode() {
    let test_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0];

    let mode = test_vec.mode();

    assert_eq!(mode.unwrap(), 5.0);

    let test_vec = vec![1, 2, 3, 4, 5, 5, 5];

    let mode = test_vec.mode();

    assert_eq!(mode.unwrap(), 5);
}

#[test]
fn test_nan_mode() {
    let test_vec = vec![
        1.0,
        2.0,
        3.0,
        4.0,
        5.0,
        5.0,
        5.0,
        f64::NAN,
        f64::NAN,
        f64::NAN,
        f64::NAN,
    ];

    let mode = test_vec.nan_mode();

    assert_eq!(mode.unwrap(), 5.0);
}

#[test]
fn test_variance() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let var = test_vec.variance();

    assert_eq!(var.unwrap(), 4.0);
}

#[test]
fn test_std() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

    let std = test_vec.std();

    assert_eq!(std.unwrap(), 2.0);
}

#[test]
fn test_nan_variance() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f64::NAN];

    let var = test_vec.nan_variance();

    assert_eq!(var.unwrap(), 4.0);
}

#[test]
fn test_nan_std() {
    let test_vec = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0, f64::NAN];

    let std = test_vec.nan_std();

    assert_eq!(std.unwrap(), 2.0);
}

#[test]
fn huge_mean_test() {
    println!("One over max vec size: {}", (u16::MAX as usize) + 1);
    let test_vec = vec![1.0; (u16::MAX as usize) + 1];

    let mean = test_vec.mean();

    assert_ne!(mean.unwrap(), 1.0);
}
