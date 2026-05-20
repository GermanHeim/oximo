# oximo-io

Model I/O for [oximo](https://github.com/germanheim/oximo): MPS and LP writers.

Converts an oximo [`Model`] to standard text formats for exchanging models with external solvers and tools. Both formats support `LP` and `MILP` only. Nonlinear expressions raise [`IoError::Nonlinear`].

> Support for NL files for nonlinear programming (NLP) and mixed-integer nonlinear programming (MINLP) is planned.

## Usage

Enabled by default via the `io` feature on the umbrella `oximo` crate:

```toml
[dependencies]
oximo = "0.1"  # io is on by default
```

To opt out:

```toml
[dependencies]
oximo = { version = "0.1", default-features = false, features = ["highs"] }
```

To use this crate directly:

```toml
[dependencies]
oximo-io   = "0.1"
oximo-core = "0.1"
```

## Quick example

```rust
use oximo::prelude::*;
use oximo::io::{to_mps_string, to_lp_string};

let m = Model::new("knapsack");
let x = m.var("x").lb(0.0).build();
let y = m.var("y").lb(0.0).ub(4.0).build();

m.constraint("c1", (x + 2.0 * y).le(14.0));
m.constraint("c2", (3.0 * x - y).ge(0.0));
m.maximize(3.0 * x + 4.0 * y);

let mps = to_mps_string(&m)?;
let lp  = to_lp_string(&m)?;
println!("{mps}");
```

## Formats

### MPS

Fixed-format MPS (fixed-column, 10-char field width). Widely supported by commercial and open-source solvers.

| Feature           | Behavior                                                                                                                       |
|-------------------|--------------------------------------------------------------------------------------------------------------------------------|
| Objective row     | Named `OBJ`, maximization models are negated with a `* sense: maximize` comment so re-importers can recover the original sense |
| Integer variables | Wrapped in `INTORG`/`INTEND` markers                                                                                           |
| Bounds            | `FR` (free), `MI`+`UP` (lower=-inf), `LO`/`UP` as needed. Default lb=0 omitted                                                 |
| Constant terms    | Objective constant written to `RHS OBJ`, constraint constants folded into `RHS`                                                |

```rust
use oximo_io::{write_mps, to_mps_string};
use std::fs::File;
use std::io::BufWriter;

// To string
let s = to_mps_string(&model)?;

// To file
let mut f = BufWriter::new(File::create("model.mps")?);
write_mps(&model, &mut f)?;
```

### LP (CPLEX LP format)

Human-readable CPLEX LP format. Sections emitted: header comment, `Minimize`/`Maximize`, `Subject To`, `Bounds` (non-default only), `General`, `Binaries`, `End`.

| Feature            | Behavior                                                          |
|--------------------|-------------------------------------------------------------------|
| Objective sense    | `Minimize` / `Maximize` keyword, no negation needed               |
| Integer variables  | `General` section (integer/semi-integer), `Binaries` section      |
| Bounds             | Free variables declared with `free`; default lb=0, ub=+inf omitted|
| Objective constant | Written as a comment if non-zero                                  |

```rust
use oximo_io::{write_lp, to_lp_string};
use std::fs::File;
use std::io::BufWriter;

// To string
let s = to_lp_string(&model)?;

// To file
let mut f = BufWriter::new(File::create("model.lp")?);
write_lp(&model, &mut f)?;
```

## Errors

All functions return `Result<_, IoError>`:

| Variant                | Cause                                                                                                       |
|------------------------|-------------------------------------------------------------------------------------------------------------|
| `IoError::NoObjective` | Model has no objective set                                                                                  |
| `IoError::Nonlinear`   | Objective or constraint contains nonlinear nodes (`Mul` with two non-constant children, `Pow`, `Sin`, etc.) |
| `IoError::Io(e)`       | Underlying `std::io::Error` from the writer                                                                 |

## License

MIT OR Apache-2.0
