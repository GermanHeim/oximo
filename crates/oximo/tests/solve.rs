use oximo::prelude::*;
use oximo::solvers::Highs;

#[test]
fn lp_canonical() {
    let m = Model::new("transport");
    let x = m.var("x").lb(0.0).build();
    let y = m.var("y").lb(0.0).ub(4.0).build();
    m.constraint("c1", (x + 2.0 * y).le(14.0));
    m.constraint("c2", (3.0 * x - y).ge(0.0));
    m.constraint("c3", (x - y).le(2.0));
    m.maximize(3.0 * x + 4.0 * y);

    let result = Highs.solve(&m, &SolverOptions::default()).unwrap();
    assert_eq!(result.status, SolverStatus::Optimal);
    assert!((result.objective.unwrap() - 34.0).abs() < 1e-6);
    assert!((result.value_of(x).unwrap() - 6.0).abs() < 1e-6);
    assert!((result.value_of(y).unwrap() - 4.0).abs() < 1e-6);
}

#[test]
fn knapsack_milp() {
    let weights = [3.0, 4.0, 2.0, 5.0, 1.0, 6.0, 7.0, 2.0];
    let values = [10.0, 12.0, 5.0, 14.0, 3.0, 18.0, 22.0, 6.0];

    let m = Model::new("knapsack");
    let xs: Vec<_> = (0..weights.len()).map(|i| m.var(format!("x{i}")).binary().build()).collect();
    let weight_sum = sum(xs.iter().zip(weights.iter()).map(|(x, w)| *w * *x));
    m.constraint("cap", weight_sum.le(15.0));
    m.maximize(sum(xs.iter().zip(values.iter()).map(|(x, v)| *v * *x)));

    let result = Highs.solve(&m, &SolverOptions::default()).unwrap();
    assert_eq!(result.status, SolverStatus::Optimal);
    assert!((result.objective.unwrap() - 47.0).abs() < 1e-6);
}

#[test]
fn infeasible_returns_status() {
    let m = Model::new("infeas");
    let x = m.var("x").lb(0.0).ub(1.0).build();
    m.constraint("c1", x.ge(5.0));
    m.minimize(x);
    let result = Highs.solve(&m, &SolverOptions::default()).unwrap();
    assert_eq!(result.status, SolverStatus::Infeasible);
}

#[cfg(feature = "io")]
#[test]
fn mps_export_round_trips_objective() {
    let m = Model::new("transport");
    let x = m.var("x").lb(0.0).build();
    let y = m.var("y").lb(0.0).ub(4.0).build();
    m.constraint("c1", (x + 2.0 * y).le(14.0));
    m.minimize(3.0 * x + 4.0 * y);

    let s = oximo::io::to_mps_string(&m).unwrap();
    assert!(s.contains("NAME"));
    assert!(s.contains("OBJ"));
    assert!(s.contains("c1"));
    assert!(s.contains("ENDATA"));
}

#[cfg(feature = "io")]
#[test]
fn lp_export_emits_required_sections() {
    let m = Model::new("transport");
    let x = m.var("x").lb(0.0).build();
    let y = m.var("y").lb(0.0).ub(4.0).build();
    m.constraint("c1", (x + 2.0 * y).le(14.0));
    m.constraint("c2", (3.0 * x - y).ge(0.0));
    m.maximize(3.0 * x + 4.0 * y);

    let s = oximo::io::to_lp_string(&m).unwrap();
    assert!(s.contains("Maximize"));
    assert!(s.contains("obj:"));
    assert!(s.contains("Subject To"));
    assert!(s.contains("c1:"));
    assert!(s.contains("c2:"));
    assert!(s.contains("<="));
    assert!(s.contains(">="));
    assert!(s.contains("Bounds"));
    assert!(s.contains("End"));
}

#[cfg(feature = "io")]
#[test]
fn lp_export_lists_binaries_and_integers() {
    let m = Model::new("mixed");
    let _x = m.var("x").binary().build();
    let _y = m.var("y").lb(0.0).ub(10.0).integer().build();
    let z = m.var("z").lb(0.0).build();
    m.minimize(z);

    let s = oximo::io::to_lp_string(&m).unwrap();
    assert!(s.contains("General"));
    assert!(s.contains(" y"));
    assert!(s.contains("Binaries"));
    assert!(s.contains(" x"));
}
