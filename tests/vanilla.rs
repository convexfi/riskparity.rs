use riskparity::vanilla::compute_riskparity_ccd_choi;

#[test]
fn test_vanilla_case() {
    let cov = ndarray::arr2(&[
        [1.0, 0.0015, -0.0119],
        [0.0015, 1.0, -0.0308],
        [-0.0119, -0.0308, 1.0],
    ]);
    let budget = ndarray::arr1(&[0.1594, 0.0126, 0.8280]);
    let maxiter = 100;
    let tol = 1e-6;
    let weights = compute_riskparity_ccd_choi(&cov, &budget, maxiter, tol);
    assert_eq!(
        weights,
        ndarray::arr1(&[0.27986280067580155, 0.08774908615114298, 0.6323881131730554])
    );
}
