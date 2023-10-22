use crate::Length;

impl Length {
    pub fn as_miles(&self) -> Self {
        match self {
            Self::In(val) => Self::Mi(val / 63360.),
            Self::Ft(val) => Self::Mi(val / 5280.),
            Self::Yd(val) => Self::Mi(val / 1760.),
            Self::Mi(_) => *self,
            Self::MM(val) => Self::Mi(val / 1.609e+6),
            Self::CM(val) => Self::Mi(val / 304.8), // TODO
            Self::M(val) => Self::Mi(val / 304.8),  // TODO
            Self::KM(val) => Self::Mi(val / 304.8), // TODO
        }
    }
}

#[test]
fn should_convert_to_feet() {
    assert_eq!(Length::Mi(1.), Length::In(63360.).as_miles());
    assert_eq!(Length::Mi(1.), Length::Ft(5280.).as_miles());
    assert_eq!(Length::Mi(1.), Length::Yd(1760.).as_miles());
    assert_eq!(Length::Mi(1.), Length::MM(1.609e+6).as_miles());
}
