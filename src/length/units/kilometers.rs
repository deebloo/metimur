use crate::Length;

impl Length {
    pub fn as_kilometers(&self) -> Self {
        match self {
            Self::In(val) => Self::KM(val / 39370.),
            Self::Ft(val) => Self::KM(val / 3281.),
            Self::Yd(val) => Self::KM(val / 1094.),
            Self::Mi(val) => Self::KM(val * 1.609),
            Self::MM(val) => Self::KM(val / 1e+6),
            Self::CM(val) => Self::KM(val / 100000.),
            Self::M(val) => Self::KM(val / 1000.),
            Self::KM(_) => *self,
        }
    }
}

#[test]
fn should_convert_to_kilometers() {
    assert_eq!(Length::KM(1.), Length::In(39370.).as_kilometers());
    assert_eq!(Length::KM(1.), Length::Ft(3281.).as_kilometers());
    assert_eq!(Length::KM(1.609), Length::Mi(1.).as_kilometers());
    assert_eq!(Length::KM(1.), Length::MM(1e+6).as_kilometers());
    assert_eq!(Length::KM(1.), Length::CM(100000.).as_kilometers());
    assert_eq!(Length::KM(1.), Length::M(1000.).as_kilometers());
}
