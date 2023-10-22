use crate::Length;

impl Length {
    pub fn as_centimeters(&self) -> Self {
        match self {
            Self::In(val) => Self::CM(val * 25.4),
            Self::Ft(val) => Self::CM(val * 304.8),
            Self::Yd(val) => Self::CM(val * 914.4),
            Self::Mi(val) => Self::CM(val * 1.609e+6),
            Self::MM(val) => Self::CM(val / 10.),
            Self::CM(_) => *self,
            Self::M(val) => Self::CM(val * 100.),
            Self::KM(val) => Self::CM(val * 100000.),
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
