//! Integration test for the BARON backend's multi-solution support.
//!
//! Shells out to a BARON installation, so compiled and run only with
//! `--features baron`:
//! ```
//! cargo test -p oximo --features baron --test solve_baron
//! ```

#![cfg(feature = "baron")]

use std::time::Duration;

use oximo::BaronOptions;
use oximo::prelude::*;
use oximo::solvers::Baron;

#[test]
fn baron_enumerates_multiple_solutions() {
    // A MILP with many feasible solutions. With `NumSol > 1`
    // BARON enumerates distinct solutions, which the backend parses into the
    // result's solution pool (best first).
    let m = Model::new("multi");
    let items = Set::range(0..4usize);
    let x = m.indexed_var("x", &items).binary().build();
    m.constraint("cap", sum_over(&items, |i: usize| x[i]).le(2.0));
    m.maximize(sum_over(&items, |i: usize| x[i]));

    let opts = BaronOptions::default().num_sol(10).time_limit(Duration::from_secs(60));
    let r = Baron::new().solve(&m, &opts).unwrap();
    assert_eq!(r.status, SolverStatus::Optimal);
    assert!(r.result_count() > 1, "expected multiple solutions, got {}", r.result_count());

    assert!((r.objective().unwrap() - 2.0).abs() < 1e-4, "best obj={:?}", r.objective());
    for s in &r.solutions {
        let chosen: f64 = (0..4).filter_map(|i| s.value_of_idx(&x, i)).sum();
        assert!(chosen <= 2.0 + 1e-4, "infeasible point: sum={chosen}");
    }
}

#[test]
fn baron_lp_duals_and_reduced_costs() {
    // min x + 2y  s.t.  x + y >= 5,  x, y >= 0
    // Optimal: (x, y) = (5, 0), obj 5, dual of c = 1, rc(x) = 0, rc(y) = 1.
    let m = Model::new("lp_dual");
    let x = m.var("x").lb(0.0).build();
    let y = m.var("y").lb(0.0).build();
    let cap = m.constraint("cap", (x + y).ge(5.0));
    m.minimize(x + 2.0 * y);

    let opts = BaronOptions::default().want_dual(true).time_limit(Duration::from_secs(30));
    let result = Baron::new().solve(&m, &opts).unwrap();
    assert_eq!(result.status, SolverStatus::Optimal);
    assert!((result.objective().unwrap() - 5.0).abs() < 1e-6);
    assert!((result.value_of(x).unwrap() - 5.0).abs() < 1e-6);

    let dual = result.dual_of(cap).expect("dual missing for cap");
    assert!((dual - 1.0).abs() < 1e-6, "dual={dual}");

    let rc = |v: Expr<'_>| result.reduced_costs.get(&v.var_id().unwrap()).copied();
    let rcx = rc(x).expect("reduced cost missing for x");
    let rcy = rc(y).expect("reduced cost missing for y");
    assert!(rcx.abs() < 1e-6, "reduced_cost(x)={rcx}");
    assert!((rcy - 1.0).abs() < 1e-6, "reduced_cost(y)={rcy}");
}

#[test]
fn baron_milp_duals_at_best_point() {
    // max 2a + 3b  s.t.  a + b <= 1,  a, b binary.
    // Optimal: (0, 1), obj 3.
    let m = Model::new("milp_dual");
    let a = m.var("a").binary().build();
    let b = m.var("b").binary().build();
    let cap = m.constraint("cap", (a + b).le(1.0));
    m.maximize(2.0 * a + 3.0 * b);

    let opts = BaronOptions::default().want_dual(true).time_limit(Duration::from_secs(30));
    let result = Baron::new().solve(&m, &opts).unwrap();
    assert_eq!(result.status, SolverStatus::Optimal);
    assert!((result.objective().unwrap() - 3.0).abs() < 1e-6);
    assert!(result.dual_of(cap).is_some(), "dual missing for cap");
    assert!(!result.reduced_costs.is_empty(), "reduced costs missing");
}
