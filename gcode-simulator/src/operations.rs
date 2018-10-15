use core::convert::TryFrom;
use core::fmt::{self, Display, Formatter};
use gcode::Gcode;
use state::State;

pub trait Operation {
    fn state_after(&self, seconds: f32, initial_state: State) -> State;
    fn duration(&self, initial_state: &State) -> f32;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConversionError {
    /// The `Gcode` had the wrong major number.
    IncorrectMajorNumber {
        found: usize,
        expected: &'static [u32],
    },
    /// An argument contained an invalid value.
    InvalidArgument {
        letter: char,
        value: f32,
        message: &'static str,
    },
    /// One or more arguments were missing.
    MissingArguments { expected: &'static [char] },
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConversionError::MissingArguments { expected } => {
                write!(f, "expected one of the following arguments: ")?;
                write!(f, "{}", expected[0])?;
                for arg in expected.into_iter().skip(1) {
                    write!(f, ", {}", arg)?;
                }

                Ok(())
            }
            ConversionError::InvalidArgument {
                letter,
                value,
                message,
            } => write!(
                f,
                "The \"{}\" argument has an invalid value of {}, {}",
                letter, value, message
            ),
            ConversionError::IncorrectMajorNumber { found, expected } => {
                write!(f, "The major number {} isn't valid, expected ", found)?;
                if expected.len() > 1 {
                    write!(f, "one of ")?;
                }
                write!(f, "{}", expected[0])?;
                for arg in expected.into_iter().skip(1) {
                    write!(f, ", {}", arg)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dwell {
    /// The number of seconds to wait for.
    pub duration: f32,
}

impl Dwell {
    pub fn new(duration: f32) -> Dwell {
        Dwell { duration }
    }
}

impl Operation for Dwell {
    fn state_after(&self, seconds: f32, initial_state: State) -> State {
        debug_assert!(seconds <= self.duration);
        initial_state
    }

    fn duration(&self, _initial_state: &State) -> f32 {
        self.duration
    }
}

impl TryFrom<Gcode> for Dwell {
    type Error = ConversionError;

    fn try_from(other: Gcode) -> Result<Self, Self::Error> {
        const VALIDATION_MSG: &str = "Dwell times must be positive";

        if other.major_number() != 4 {
            return Err(ConversionError::IncorrectMajorNumber {
                found: other.major_number(),
                expected: &[4],
            });
        }

        if let Some(h) = other.value_for('H') {
            if h >= 0.0 {
                Ok(Dwell::new(h / 1000.0))
            } else {
                Err(ConversionError::InvalidArgument {
                    letter: 'H',
                    value: h,
                    message: VALIDATION_MSG,
                })
            }
        } else if let Some(p) = other.value_for('P') {
            if p >= 0.0 {
                Ok(Dwell::new(p))
            } else {
                Err(ConversionError::InvalidArgument {
                    letter: 'P',
                    value: p,
                    message: VALIDATION_MSG,
                })
            }
        } else {
            Err(ConversionError::MissingArguments {
                expected: &['H', 'P'],
            })
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LinearInterpolate {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub feed_rate: Option<f32>,
}

impl TryFrom<Gcode> for LinearInterpolate {
    type Error = ConversionError;

    fn try_from(other: Gcode) -> Result<Self, Self::Error> {
        let maj = other.major_number();
        if maj != 1 && maj != 0 {
            return Err(ConversionError::IncorrectMajorNumber {
                found: maj,
                expected: &[0, 1],
            });
        }

        let x = other.value_for('X');
        let y = other.value_for('Y');

        if x.is_none() && y.is_none() {
            return Err(ConversionError::MissingArguments {
                expected: &['X', 'Y'],
            });
        }

        let feed_rate = other.value_for('F');

        if let Some(f) = feed_rate {
            if f <= 0.0 {
                return Err(ConversionError::InvalidArgument {
                    letter: 'F',
                    value: f,
                    message: "Feed rate must always be a positive number",
                });
            }
        }

        Ok(LinearInterpolate { x, y, feed_rate })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_dwell() {
        let inputs = vec![
            (
                "G4",
                Err(ConversionError::MissingArguments {
                    expected: &['H', 'P'],
                }),
            ),
            (
                "G00",
                Err(ConversionError::IncorrectMajorNumber {
                    found: 0,
                    expected: &[4],
                }),
            ),
            ("G4 H5", Ok(Dwell::new(5e-3))),
            ("G4 P5", Ok(Dwell::new(5.0))),
            (
                "G4 P-50",
                Err(ConversionError::InvalidArgument {
                    letter: 'P',
                    value: -50.0,
                    message: "Dwell times must be positive",
                }),
            ),
        ];

        for (src, should_be) in inputs {
            let raw = gcode::parse(src).next().unwrap();
            let got = Dwell::try_from(raw);

            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn parse_a_linear_interpolate() {
        let inputs = vec![
            (
                "G00 X50.0",
                Ok(LinearInterpolate {
                    x: Some(50.0),
                    y: None,
                    feed_rate: None,
                }),
            ),
            (
                "G90",
                Err(ConversionError::IncorrectMajorNumber {
                    found: 90,
                    expected: &[0, 1],
                }),
            ),
            (
                "G01 X5 Y-20.5 F10000",
                Ok(LinearInterpolate {
                    x: Some(5.0),
                    y: Some(-20.5),
                    feed_rate: Some(10000.0),
                }),
            ),
            (
                "G01 Y-20.5",
                Ok(LinearInterpolate {
                    x: None,
                    y: Some(-20.5),
                    feed_rate: None,
                }),
            ),
            (
                "G01 F10000",
                Err(ConversionError::MissingArguments {
                    expected: &['X', 'Y'],
                }),
            ),
            (
                "G01 X5 Y-20.5 F-10000",
                Err(ConversionError::InvalidArgument {
                    letter: 'F',
                    value: -10000.0,
                    message: "Feed rate must always be a positive number",
                }),
            ),
        ];

        for (src, should_be) in inputs {
            let raw = gcode::parse(src).next().unwrap();
            let got = LinearInterpolate::try_from(raw);

            assert_eq!(got, should_be);
        }
    }
}
