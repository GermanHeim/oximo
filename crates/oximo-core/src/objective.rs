use oximo_expr::ExprId;

/// Whether the model minimizes or maximizes its objective.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ObjectiveSense {
    Minimize,
    Maximize,
}

/// The model's objective: an expression to optimize and the direction.
#[derive(Clone, Debug)]
pub struct Objective {
    /// Root node of the objective expression in the model's [`ExprArena`].
    pub expr: ExprId,
    pub sense: ObjectiveSense,
}
