use crate::Temp;

impl Temp {
    pub fn as_c(&self) -> Self {
        match self {
            Self::C(val) => Self::C(*val),
            Self::F(val) => {
                let raw = (val - 32.) * (5. / 9.);

                Temp::C(raw)
            }
            Self::K(val) => Temp::C(val - 273.15),
        }
    }
}
