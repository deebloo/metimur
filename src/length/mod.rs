mod units;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Length {
    In(f64),
    Ft(f64),
    Yd(f64),
    Mi(f64),
    MM(f64),
    CM(f64),
    M(f64),
    KM(f64),
}

impl Length {
    pub fn round(&self) -> Self {
        match self {
            Self::In(val) => Self::In(round(*val)),
            Self::Ft(val) => Self::Ft(round(*val)),
            Self::Yd(val) => Self::Yd(round(*val)),
            Self::Mi(val) => Self::Mi(round(*val)),
            Self::MM(val) => Self::MM(round(*val)),
            Self::CM(val) => Self::CM(round(*val)),
            Self::M(val) => Self::M(round(*val)),
            Self::KM(val) => Self::KM(round(*val)),
        }
    }
}

impl Into<f64> for Length {
    fn into(self) -> f64 {
        match self {
            Self::In(val) => val,
            Self::Ft(val) => val,
            Self::Yd(val) => val,
            Self::Mi(val) => val,
            Self::MM(val) => val,
            Self::CM(val) => val,
            Self::M(val) => val,
            Self::KM(val) => val,
        }
    }
}

pub fn round(val: f64) -> f64 {
    let res = (val * 10000.).round() / 10000.;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_f64() {
        let inch: f64 = Length::In(1.).into();
        let feet: f64 = Length::Ft(2.).into();
        let yards: f64 = Length::Yd(3.).into();

        assert_eq!(inch, 1.);
        assert_eq!(feet, 2.);
        assert_eq!(yards, 3.);
    }
}
