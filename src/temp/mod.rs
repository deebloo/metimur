mod into_temp;
mod ops;
mod units;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Temp {
    C(f64),
    F(f64),
    K(f64),
}

impl Temp {
    pub fn round(&self) -> Self {
        match self {
            Self::C(val) => Self::C(round(*val)),
            Self::F(val) => Self::F(round(*val)),
            Self::K(val) => Self::K(round(*val)),
        }
    }
}

fn round(val: f64) -> f64 {
    (val * 1000.).round() / 1000.
}

impl Into<f64> for Temp {
    fn into(self) -> f64 {
        match self {
            Self::C(val) => val,
            Self::F(val) => val,
            Self::K(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestData(Vec<Conversion>);

    #[derive(Debug, Serialize, Deserialize)]
    struct Conversion {
        celsius: f64,
        fahrenheit: f64,
        kelvin: f64,
    }

    #[test]
    fn should_convert_correctly() {
        let data_string = fs::read_to_string("data/temps.json").unwrap();
        let data = serde_json::from_str::<TestData>(data_string.as_str()).unwrap();

        for entry in data.0 {
            // convert to celcius
            assert_eq!(Temp::F(entry.fahrenheit).as_c(), Temp::C(entry.celsius));
            assert_eq!(Temp::K(entry.kelvin).as_c(), Temp::C(entry.celsius));

            // convert to farenheit
            assert_eq!(Temp::C(entry.celsius).as_f(), Temp::F(entry.fahrenheit));
            assert_eq!(Temp::K(entry.kelvin).as_f(), Temp::F(entry.fahrenheit));

            // convert to kelvin
            assert_eq!(Temp::C(entry.celsius).as_k(), Temp::K(entry.kelvin));
            assert_eq!(Temp::F(entry.fahrenheit).as_k(), Temp::K(entry.kelvin));
        }
    }
}
