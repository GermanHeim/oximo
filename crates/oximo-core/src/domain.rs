/// The domain of a variable, which determines the type of values it can take.
///
/// Real: any real number.
/// Integer: any integer.
/// Binary: 0 or 1.
/// SemiContinuous: either 0 or any value >= threshold.
/// SemiInteger: either 0 or any integer >= threshold.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Domain {
    #[default]
    Real,
    Integer,
    Binary,
    SemiContinuous {
        threshold: f64,
    },
    SemiInteger {
        threshold: f64,
    },
}

impl Domain {
    /// Whether this domain is integer-valued (Integer, Binary, SemiInteger)
    pub fn is_integer(self) -> bool {
        matches!(self, Self::Integer | Self::Binary | Self::SemiInteger { .. })
    }
}
