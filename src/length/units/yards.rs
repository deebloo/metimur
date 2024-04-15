use crate::Length;

impl Length {
    pub fn as_yards(&self) -> Self {
        match self {
            Self::In(val) => Self::Yd(val / 36.),
            Self::Ft(val) => Self::Yd(val / 3.),
            Self::Yd(_) => *self,
            Self::Mi(val) => Self::Yd(val * 1760.),
            Self::MM(val) => Self::Yd(val / 914.4),
            Self::CM(val) => Self::Yd(val / 304.8), // TODO
            Self::M(val) => Self::Yd(val / 304.8),  // TODO
            Self::KM(val) => Self::Yd(val / 304.8), // TODO
        }
    }
}

#[test]
fn should_convert_to_feet() {
    assert_eq!(Length::Yd(1.), Length::In(36.).as_yards());
    assert_eq!(Length::Yd(1.), Length::Ft(3.).as_yards());
    assert_eq!(Length::Yd(1760.), Length::Mi(1.).as_yards());
    assert_eq!(Length::Yd(1.), Length::MM(914.4).as_yards());
}
