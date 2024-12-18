use std::fmt;
use strum::EnumIter;

#[derive(Debug, Default, PartialEq, Clone, Copy, EnumIter)]
pub enum Age {
    Puppy,
    #[default]
    Adult,
    Senior,
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, EnumIter)]
pub enum ActivityLevel {
    Sedentary,
    #[default]
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

#[derive(Clone, Default, Copy)]
pub struct Inputs {
    pub age: Age,
    pub weight: f32,
    pub activity_level: ActivityLevel,
}
