use crate::Length;

impl Length {
    pub fn as_feet(&self) -> Self {
        match self {
            Self::In(val) => Self::Ft(val / 12.),
            Self::Ft(_) => *self,
            Self::Yd(val) => Self::Ft(val * 3.),
            Self::Mi(val) => Self::Ft(val * 5280.),
            Self::MM(val) => Self::Ft(val / 304.8),
            Self::CM(val) => Self::Ft(val / 30.48),
            Self::M(val) => Self::Ft(val * 3.281),
            Self::KM(val) => Self::Ft(val * 3281.),
        }
    }
}

#[test]
fn should_convert_to_feet() {
    assert_eq!(Length::Ft(1.), Length::In(12.).as_feet());
    assert_eq!(Length::Ft(3.), Length::Yd(1.).as_feet());
    assert_eq!(Length::Ft(5280.), Length::Mi(1.).as_feet());
    assert_eq!(Length::Ft(1.), Length::MM(304.8).as_feet());
    assert_eq!(Length::Ft(1.), Length::CM(30.48).as_feet());
    assert_eq!(Length::Ft(3.281), Length::M(1.).as_feet());
    assert_eq!(Length::Ft(3281.), Length::KM(1.).as_feet());
}
