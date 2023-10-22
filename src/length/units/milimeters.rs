use crate::Length;

impl Length {
    pub fn as_milimeters(&self) -> Self {
        match self {
            Self::In(val) => Self::MM(val * 25.4),
            Self::Ft(val) => Self::MM(val * 304.8),
            Self::Yd(val) => Self::MM(val * 914.4),
            Self::Mi(val) => Self::MM(val * 1.609e+6),
            Self::MM(_) => *self,
            Self::CM(val) => Self::MM(val * 10.),
            Self::M(val) => Self::MM(val * 1000.),
            Self::KM(val) => Self::MM(val * 1e+6),
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
