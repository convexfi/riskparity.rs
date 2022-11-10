pub mod vanilla {
    // implement risk parity algo by Choi 2022
    pub fn compute_riskparity_ccd_choi(
        cov: &ndarray::Array2<f64>,
        budget: &ndarray::Array1<f64>,
        maxiter: i32,
        tol: f64,
    ) -> ndarray::Array1<f64> {
        let n = budget.len();
        let volatility = cov.diag().mapv(f64::sqrt);
        let invvol = ndarray::Array2::from_diag(&volatility.mapv(|a| 1.0 / a));
        let corr = (invvol.dot(cov)).dot(&invvol.t());
        let mut adj = corr.to_owned();
        for i in 0..n {
            adj[[i, i]] = 0.0;
        }
        let mut wk = ndarray::Array1::<f64>::ones(n);
        wk = wk.mapv(|a| a / corr.sum());
        for _k in 0..maxiter {
            for i in 0..n {
                let a = 0.5 * ((adj.slice(ndarray::s![.., i]).to_owned() * wk.clone()).sum());
                wk[i] = f64::sqrt(a * a + budget[i]) - a;
            }
            if (wk.clone() * (corr.dot(&wk)) - budget)
                .mapv(f64::abs)
                .mean()
                < Some(tol)
            {
                break;
            }
        }
        let w = wk / volatility;
        w.mapv(|a| a / w.sum())
    }
}
