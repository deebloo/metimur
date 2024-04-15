use crate::Temp;

impl Temp {
    pub fn as_f(&self) -> Self {
        match self {
            Self::C(temp) => {
                let raw = (temp * (9. / 5.)) + 32.;

                Temp::F(raw)
            }
            Self::F(val) => Self::F(*val),
            Self::K(temp) => {
                let raw = (temp - 273.15) * (9. / 5.) + 32.;

                Self::F(raw)
            }
        }
    }
}
