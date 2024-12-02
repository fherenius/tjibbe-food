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
    weight: f32,
    activity_level: ActivityLevel,
}
