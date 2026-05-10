use highs::Model as HighsModel;
use oximo_solver::{OptionValue, SolverOptions};

/// Translate generic [`SolverOptions`] into HiGHS' string-keyed configuration.
/// Unknown keys are silently ignored to keep backends interchangeable.
///
/// TODO: We should probably have a more structured way to represent options in the future,
/// but this is good enough for now.
/// HiGHS has a lot of options and we don't want to have to mirror all of them in our own API immediately.
///
/// TODO: Create a warning when the user tries to set an option that is not supported by the backend?
pub(crate) fn apply(model: &mut HighsModel, opts: &SolverOptions) {
    for (k, v) in &opts.entries {
        match (k.as_str(), v) {
            ("time_limit", OptionValue::Float(t)) => model.set_option("time_limit", *t),
            #[allow(clippy::cast_precision_loss)]
            ("time_limit", OptionValue::Int(t)) => model.set_option("time_limit", *t as f64),
            ("threads", OptionValue::Int(n)) => {
                model.set_option("threads", i32::try_from(*n).unwrap_or(i32::MAX));
            }
            ("mip_gap", OptionValue::Float(g)) => model.set_option("mip_rel_gap", *g),
            ("verbose", OptionValue::Bool(b)) => {
                model.set_option("output_flag", *b);
                model.set_option("log_to_console", *b);
            }
            ("presolve", OptionValue::Str(s)) => model.set_option("presolve", s.as_str()),
            ("presolve", OptionValue::Bool(b)) => {
                model.set_option("presolve", if *b { "on" } else { "off" });
            }
            ("solver", OptionValue::Str(s)) => model.set_option("solver", s.as_str()),
            ("parallel", OptionValue::Bool(b)) => {
                model.set_option("parallel", if *b { "on" } else { "off" });
            }
            _ => {}
        }
    }
}
