use oximo_expr::ParamId;
use smol_str::SmolStr;

/// Parameter metadata. Parameters are scalars referenced symbolically in
/// expressions and re-bound between solves.
///
/// For now, we support scalar parameters only.
///
/// TODO: Add support for more parameters
#[derive(Clone, Debug)]
pub struct Parameter {
    pub id: ParamId,
    pub name: SmolStr,
    pub value: f64,
}
