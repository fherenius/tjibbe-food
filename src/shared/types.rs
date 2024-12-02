use uom::si::energy::kilocalorie;
use uom::si::mass::kilogram;

pub enum Age {
    Puppy,
    Adult,
    Senior,
}

pub enum ActivityLevel {
    Sedentary,
    Moderate,
    Active,
    High,
    Extreme,
}

pub struct Inputs {
    age: Age,
    weight: kilogram,
    activity_level: ActivityLevel,
}

pub struct Output {
    daily_kcal: kilocalorie,
}
