use crate::Length;

impl Length {
    pub fn as_kilometers(&self) -> Self {
        match self {
            Self::In(val) => Self::KM(val * 25.4),
            Self::Ft(val) => Self::KM(val * 304.8),
            Self::Yd(val) => Self::KM(val * 914.4),
            Self::Mi(val) => Self::KM(val * 1.609e+6),
            Self::MM(val) => Self::KM(val / 1e+6),
            Self::CM(val) => Self::KM(val / 100000.),
            Self::M(val) => Self::KM(val / 1000.),
            Self::KM(_) => *self,
        }
    }
}

#[test]
fn should_convert_to_milimeters() {
    assert_eq!(Length::MM(25.4), Length::In(1.).as_milimeters());
    assert_eq!(Length::MM(304.8), Length::Ft(1.).as_milimeters());
    assert_eq!(Length::MM(914.4), Length::Yd(1.).as_milimeters());
    assert_eq!(Length::MM(1.609e+6), Length::Mi(1.).as_milimeters());
}
