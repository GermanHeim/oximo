# oximo

oximo is a Rust algebraic modeling library for mathematical optimization. Build LP and MILP models with a clean builder API, then solve them with bundled or commercial solvers.

> Support for nonlinear programming (NLP) and mixed-integer nonlinear programming (MINLP) is planned.

```rust
use oximo::prelude::*;
use oximo::solvers::Highs;

let m = Model::new("transport");
let x = m.var("x").lb(0.0).build();
let y = m.var("y").lb(0.0).ub(4.0).build();

m.constraint("c1", (x + 2.0 * y).le(14.0));
m.constraint("c2", (3.0 * x - y).ge(0.0));
m.constraint("c3", (x - y).le(2.0));
m.maximize(3.0 * x + 4.0 * y);

let result = Highs.solve(&m, &HighsOptions::default())?;
println!("obj = {:?}", result.objective);   // 34.0
println!("x   = {:?}", result.value_of(x)); // 6.0
println!("y   = {:?}", result.value_of(y)); // 4.0
```

## Features

| Feature  | What it adds                                      | Default |
|----------|---------------------------------------------------|---------|
| `highs`  | HiGHS LP/MILP solver (bundled, no install)        | yes     |
| `io`     | MPS and LP file writers                           | yes     |
| `gurobi` | Gurobi LP/MILP solver (requires licensed install) | no      |
| `gams`   | GAMS solver bridge (requires GAMS on PATH)        | no      |

```toml
[dependencies]
oximo = "0.1"                                      # HiGHS + MPS/LP writers
oximo = { version = "0.1", features = ["gurobi"] } # add Gurobi
oximo = { version = "0.1", features = ["gams"] }   # add GAMS backend
```

## Building models

### Variables

```rust
let m = Model::new("my_model");

let x = m.var("x").lb(0.0).build();               // continuous, x >= 0
let y = m.var("y").lb(0.0).ub(10.0).build();      // continuous, 0 <= y <= 10
let z = m.var("z").lb(f64::NEG_INFINITY).build(); // free variable
let b = m.var("b").binary().build();              // binary {0, 1}
let n = m.var("n").lb(0.0).integer().build();     // general integer
```

### Constraints and objectives

Expressions are built with standard Rust operators. Scalar multiplication, addition, and subtraction all work out of the box:

```rust
m.constraint("cap", (2.0 * x + 3.0 * y).le(100.0));
m.constraint("demand", x.ge(5.0));
m.constraint("balance", (x - y).eq(0.0));

m.minimize(3.0 * x + 5.0 * y);
// or
m.maximize(x + 2.0 * y);
```

### Indexed variables

```rust
use oximo::core::Set;

let items = Set::range(0..5);
let xs = m.indexed_var("x", &items); // creates x[0] ... x[4]

// Access individual variables
let x0 = xs.get(&0).unwrap();
```

### Summing over iterators

```rust
let total = sum(xs.iter().zip(weights.iter()).map(|(x, w)| *w * *x));
m.constraint("cap", total.le(capacity));
```

## Solving

All backends implement the `Solver` trait:

```rust
pub trait Solver {
    fn solve(&mut self, model: &Model, opts: &Self::Options) -> Result<SolverResult, SolverError>;
}
```

### HiGHS (default)

No install required, HiGHS is compiled from source via the `highs` crate.

```rust
use oximo::prelude::*;
use oximo::solvers::Highs;

let result = Highs.solve(&m, &HighsOptions::default()
    .time_limit(Duration::from_secs(60))
    .threads(4)
    .mip_gap(0.01)
    .method(HighsMethod::Ipm))?;
```

### Gurobi

Requires a licensed Gurobi install and `GUROBI_HOME` set. See [`crates/oximo-gurobi/README.md`](crates/oximo-gurobi/README.md).

```rust
use oximo::prelude::*;
use oximo::solvers::Gurobi;

let result = Gurobi.solve(&m, &GurobiOptions::default()
    .time_limit(Duration::from_secs(120))
    .mip_focus(1)
    .seed(101))?;
```

### GAMS

Requires GAMS on `PATH`. Supports solving models via GAMS solvers (CPLEX, BARON, etc.). See [`crates/oximo-gams/README.md`](crates/oximo-gams/README.md).

```rust
use oximo::prelude::*;
use oximo::solvers::Gams;

let result = Gams.solve(&m, &GamsOptions::default())?;
```

## Reading results

```rust
let result = Highs.solve(&m, &HighsOptions::default())?;

match result.status {
    SolverStatus::Optimal => println!("optimal: {}", result.objective.unwrap()),
    SolverStatus::Infeasible => println!("infeasible"),
    SolverStatus::TimeLimit => println!("time limit, best = {:?}", result.objective),
    _ => {}
}

// Variable values
let x_val = result.value_of(x); // Option<f64>

// Constraint duals (LP only)
let dual = result.dual.get(&constraint_id);

// Reduced costs
let rc = result.reduced_costs.get(&x.id);
```

## Model export

With the `io` feature (default):

```rust
use oximo::io;

let mps = io::to_mps_string(&m)?;
let lp  = io::to_lp_string(&m)?;

io::write_mps(&m, "model.mps")?;
io::write_lp(&m, "model.lp")?;
```

## Workspace layout

| Crate          | Role                                                  |
|----------------|-------------------------------------------------------|
| `oximo`        | Umbrella crate                                        |
| `oximo-expr`   | Arena-allocated expression tree                       |
| `oximo-core`   | `Model`, `Variable`, `Constraint`, `Objective`, `Set` |
| `oximo-solver` | `Solver` trait, `SolverResult`, `SolverOptions`       |
| `oximo-io`     | MPS and LP writers                                    |
| `oximo-highs`  | HiGHS backend                                         |
| `oximo-gurobi` | Gurobi backend                                        |
| `oximo-gams`   | GAMS writer and backend                               |

## Requirements

- Gurobi feature: Gurobi, `GUROBI_HOME` set, valid license
- GAMS feature: GAMS on `PATH`, valid license

## License

MIT OR Apache-2.0
