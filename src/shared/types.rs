use std::fmt;
use strum::EnumIter;

#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum Age {
    Puppy,
    Adult,
    Senior,
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum ActivityLevel {
    Sedentary,
    Moderate,
    Active,
    High,
    Extreme,
}

impl fmt::Display for ActivityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Inputs {
    age: Age,
    weight: f32,
    activity_level: ActivityLevel,
}
