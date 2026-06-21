use cathedral_33t::tensor::Tensor;

#[test]
fn test_creation() {
    let t = Tensor::zeros((2, 3));
    assert_eq!(t.shape(), (2, 3));
    assert_eq!(t.sum(), 0.0);
}

#[test]
fn test_matmul() {
    let mut a = Tensor::zeros((2, 2));
    a.data[[0, 0]] = 1.0; a.data[[0, 1]] = 2.0;
    a.data[[1, 0]] = 3.0; a.data[[1, 1]] = 4.0;

    let mut b = Tensor::zeros((2, 2));
    b.data[[0, 0]] = 5.0; b.data[[0, 1]] = 6.0;
    b.data[[1, 0]] = 7.0; b.data[[1, 1]] = 8.0;

    let c = a.matmul(&b);
    assert_eq!(c.shape(), (2, 2));
    assert!((c.data[[0, 0]] - 19.0).abs() < 1e-6);
}
