# oximo-highs

HiGHS LP/MILP backend for [oximo](https://github.com/germanheim/oximo).

Wraps the [`highs`](https://crates.io/crates/highs) crate (HiGHS bundled, no separate install required). Supports `LP` and `MILP` model kinds.

## Usage

Most users should depend on the umbrella `oximo` crate with the `highs` feature (enabled by default):

```toml
[dependencies]
oximo = "0.1"
```

To use this crate directly:

```toml
[dependencies]
oximo-highs = "0.1"
oximo-core   = "0.1"
oximo-solver = "0.1"
```

## Quick example

```rust
use oximo_core::prelude::*;
use oximo_highs::{Highs, HighsOptions};
use oximo_solver::Solver;

let m = Model::new("toy");
let x = m.var("x").lb(0.0).build();
let y = m.var("y").lb(0.0).ub(4.0).build();
m.constraint("c1", (x + 2.0 * y).le(14.0));
m.constraint("c2", (3.0 * x - y).ge(0.0));
m.maximize(3.0 * x + 4.0 * y);

let result = Highs.solve(&m, &HighsOptions::default()).unwrap();
assert_eq!(result.status, SolverStatus::Optimal);
println!("obj = {}", result.objective.unwrap()); // 34.0
println!("x   = {}", result.value_of(x).unwrap()); // 6.0
println!("y   = {}", result.value_of(y).unwrap()); // 4.0
```

## Options

`HighsOptions` is a typed builder. All methods return `Self` for chaining.

| Method                       | Type       | HiGHS option                    | Default |
|------------------------------|------------|---------------------------------|---------|
| `.time_limit(Duration)`      | universal  | `time_limit`                    | none    |
| `.threads(usize)`            | universal  | `threads`                       | none    |
| `.verbose(bool)`             | universal  | `output_flag`, `log_to_console` | none    |
| `.mip_gap(f64)`              | HiGHS-only | `mip_rel_gap`                   | none    |
| `.presolve(HighsPresolve)`   | HiGHS-only | `presolve`                      | none    |
| `.method(HighsMethod)`       | HiGHS-only | `solver`                        | none    |
| `.parallel(bool)`            | HiGHS-only | `parallel`                      | none    |

`HighsMethod` variants: `Choose` (let HiGHS decide), `Simplex`, `Ipm` (interior-point), `PdLp` (first-order).

`HighsPresolve` variants: `Off`, `On`, `Auto` (`"choose"`).

```rust
use std::time::Duration;
use oximo_highs::{HighsOptions, HighsMethod, HighsPresolve};
use oximo_solver::UniversalOptionsExt;

let opts = HighsOptions::default()
    .time_limit(Duration::from_secs(60))
    .threads(4)
    .mip_gap(0.01)
    .presolve(HighsPresolve::On)
    .method(HighsMethod::Ipm)
    .parallel(true);
```

## Result

`SolverResult` fields populated on `Optimal` or `Feasible`:

- `objective` - objective value (adjusted for any constant term)
- `primal` - variable values, keyed by `VarId`; access via `result.value_of(var)`
- `dual` - constraint duals, keyed by `ConstraintId`
- `reduced_costs` - variable reduced costs, keyed by `VarId`
- `solve_time` - wall time measured around the HiGHS solve call

## License

MIT OR Apache-2.0
