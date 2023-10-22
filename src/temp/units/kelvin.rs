use crate::Temp;

impl Temp {
    pub fn as_k(&self) -> Self {
        match self {
            Self::C(val) => Self::K(val + 273.15),
            Self::F(_) => {
                let c: f64 = self.as_c().into();

                Temp::K(c + 273.15)
            }
            Self::K(val) => Self::K(*val),
        }
    }
}
