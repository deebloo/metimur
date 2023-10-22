use crate::Length;

impl Length {
    pub fn as_feet(&self) -> Self {
        match self {
            Self::In(val) => Self::Ft(val / 12.),
            Self::Ft(_) => *self,
            Self::Yd(val) => Self::Ft(val * 3.),
            Self::Mi(val) => Self::Ft(val * 5280.),
            Self::MM(val) => Self::Ft(val / 304.8),
            Self::CM(val) => Self::Ft(val / 304.8), // TODO
            Self::M(val) => Self::Ft(val / 304.8),  // TODO
            Self::KM(val) => Self::Ft(val / 304.8), // TODO
        }
    }
}

#[test]
fn should_convert_to_feet() {
    assert_eq!(Length::Ft(1.), Length::In(12.).as_feet());
    assert_eq!(Length::Ft(3.), Length::Yd(1.).as_feet());
    assert_eq!(Length::Ft(5280.), Length::Mi(1.).as_feet());
    assert_eq!(Length::Ft(1.), Length::MM(304.8).as_feet());
}
