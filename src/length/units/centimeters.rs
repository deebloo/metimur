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
fn should_convert_to_centemeters() {
    assert_eq!(Length::CM(25.4), Length::In(1.).as_centimeters());
    assert_eq!(Length::CM(304.8), Length::Ft(1.).as_centimeters());
    assert_eq!(Length::CM(914.4), Length::Yd(1.).as_centimeters());
    assert_eq!(Length::CM(1.609e+6), Length::Mi(1.).as_centimeters());
    assert_eq!(Length::CM(1.), Length::MM(10.).as_centimeters());
    assert_eq!(Length::CM(100.), Length::M(1.).as_centimeters());
    assert_eq!(Length::CM(100000.), Length::KM(1.).as_centimeters());
}
