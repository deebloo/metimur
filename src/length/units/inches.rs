use crate::Length;

impl Length {
    pub fn as_inches(&self) -> Self {
        match self {
            Self::In(_) => *self,
            Self::Ft(val) => Self::In(val * 12.),
            Self::Yd(val) => Self::In(val * 36.),
            Self::Mi(val) => Self::In(val * 63360.),
            Self::MM(val) => Self::In(val / 25.4),
            Self::CM(val) => Self::In(val / 304.8), // TODO
            Self::M(val) => Self::In(val / 304.8),  // TODO
            Self::KM(val) => Self::In(val / 304.8), // TODO
        }
    }
}

#[test]
fn should_convert() {
    assert_eq!(Length::In(12.), Length::Ft(1.).as_inches());
    assert_eq!(Length::In(36.), Length::Yd(1.).as_inches());
    assert_eq!(Length::In(63360.), Length::Mi(1.).as_inches());
    assert_eq!(Length::In(1.), Length::MM(25.4).as_inches());
}
