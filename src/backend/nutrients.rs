use super::super::shared::types::ActivityLevel;
use std::fmt;

#[derive(Clone, Default)]
pub struct MetabolicBodyWeight {
    pub value: Kilogram,
}

impl MetabolicBodyWeight {
    pub fn new(body_weight: Kilogram) -> Self {
        let metabolic_bw = body_weight.value.powf(0.75);
        return MetabolicBodyWeight {
            value: Kilogram {
                value: metabolic_bw,
            },
        };
    }

    fn mul_f32<T: FromValue>(&self, rhs: f32) -> T {
        T::from_value(self.value.value * rhs)
    }
}

pub trait NewRecommendedIntake<T> {
    fn new_recommended_intake(
        metabolic_bw: &MetabolicBodyWeight,
        activity_level: ActivityLevel,
    ) -> T;
}

trait NewRecommendedNutrientIntake<T> {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> T;
}


#[derive(Clone, Default)]
pub struct Intake {
    pub daily_kcal: Kcal,
    pub nutrients: Nutrients,
}

impl NewRecommendedIntake<Self> for Intake {
    fn new_recommended_intake(
        metabolic_bw: &MetabolicBodyWeight,
        activity_level: ActivityLevel,
    ) -> Self {
        return Intake {
            daily_kcal: match activity_level {
                ActivityLevel::Sedentary => metabolic_bw.mul_f32(95.00),
                ActivityLevel::Moderate => metabolic_bw.mul_f32(110.00),
                ActivityLevel::Active => metabolic_bw.mul_f32(125.00),
                ActivityLevel::High => metabolic_bw.mul_f32(162.50),
                ActivityLevel::Extreme => metabolic_bw.mul_f32(1070.00),
            },
            nutrients: Nutrients::new_recommended_nutrient_intake(metabolic_bw),
        };
    }
}


#[derive(Clone, Default)]
pub struct Nutrients {
    pub protein: Gram,
    pub fat: Gram,
    pub amino_acids: AminoAcids,
    pub fatty_acids: FattyAcids,
    pub minerals: Minerals,
    pub vitamins: Vitamins,
}

impl NewRecommendedNutrientIntake<Self> for Nutrients {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> Self {
        return Nutrients {
            protein: metabolic_bw.mul_f32(4.95),
            fat: metabolic_bw.mul_f32(1.51),
            amino_acids: AminoAcids::new_recommended_nutrient_intake(metabolic_bw),
            fatty_acids: FattyAcids::new_recommended_nutrient_intake(metabolic_bw),
            minerals: Minerals::new_recommended_nutrient_intake(metabolic_bw),
            vitamins: Vitamins::new_recommended_nutrient_intake(metabolic_bw),
        };
    }
}


#[derive(Clone, Default)]
pub struct AminoAcids {
    pub arginine: Gram,
    pub histidine: Gram,
    pub isoleucine: Gram,
    pub leucine: Gram,
    pub lysine: Gram,
    pub methionine: Gram,
    pub cystine: Gram,
    pub phenylalanine: Gram,
    pub tyrosine: Gram,
    pub threonine: Gram,
    pub tryptophan: Gram,
    pub valine: Gram,
}

impl NewRecommendedNutrientIntake<Self> for AminoAcids {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> Self {
        return AminoAcids {
            arginine: metabolic_bw.mul_f32(0.14),
            histidine: metabolic_bw.mul_f32(0.06),
            isoleucine: metabolic_bw.mul_f32(0.13),
            leucine: metabolic_bw.mul_f32(0.23),
            lysine: metabolic_bw.mul_f32(0.12),
            methionine: metabolic_bw.mul_f32(0.11),
            cystine: metabolic_bw.mul_f32(0.10),
            phenylalanine: metabolic_bw.mul_f32(0.15),
            tyrosine: metabolic_bw.mul_f32(0.09),
            threonine: metabolic_bw.mul_f32(0.14),
            tryptophan: metabolic_bw.mul_f32(0.05),
            valine: metabolic_bw.mul_f32(0.16),
        };
    }
}

impl<'a> IntoIterator for &'a AminoAcids {
    type Item = (&'static str, &'a dyn fmt::Display);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("Arginine", &self.arginine as &dyn fmt::Display),
            ("Histidine", &self.histidine as &dyn fmt::Display),
            ("Isoleucine", &self.isoleucine as &dyn fmt::Display),
            ("Leucine", &self.leucine as &dyn fmt::Display),
            ("Lysine", &self.lysine as &dyn fmt::Display),
            ("Methionine", &self.methionine as &dyn fmt::Display),
            ("Cystine", &self.cystine as &dyn fmt::Display),
            ("Phenylalanine", &self.phenylalanine as &dyn fmt::Display),
            ("Tyrosine", &self.tyrosine as &dyn fmt::Display),
            ("Threonine", &self.threonine as &dyn fmt::Display),
            ("Tryptophan", &self.tryptophan as &dyn fmt::Display),
            ("Valine", &self.valine as &dyn fmt::Display),
        ].into_iter()
    }
}

#[derive(Clone, Default)]
pub struct FattyAcids {
    pub linoleic_acid: Gram,
}

impl NewRecommendedNutrientIntake<Self> for FattyAcids {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> FattyAcids {
        return FattyAcids {
            linoleic_acid: metabolic_bw.mul_f32(0.36),
        };
    }
}

impl<'a> IntoIterator for &'a FattyAcids {
    type Item = (&'static str, &'a dyn fmt::Display);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("Linoleic Acid", &self.linoleic_acid as &dyn fmt::Display),
        ].into_iter()
    }
}

#[derive(Clone, Default)]
pub struct Minerals {
    pub calcium: Gram,
    pub phosphorus: Gram,
    pub potassium: Gram,
    pub sodium: Gram,
    pub chloride: Gram,
    pub magnesium: Gram,
    pub copper: Milligram,
    pub iodine: Milligram,
    pub iron: Milligram,
    pub manganese: Milligram,
    pub selenium: Microgram,
    pub zinc: Milligram,
}

impl NewRecommendedNutrientIntake<Self> for Minerals {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> Self {
        return Minerals {
            calcium: metabolic_bw.mul_f32(0.14),
            phosphorus: metabolic_bw.mul_f32(0.11),
            potassium: metabolic_bw.mul_f32(0.14),
            sodium: metabolic_bw.mul_f32(0.03),
            chloride: metabolic_bw.mul_f32(0.04),
            magnesium: metabolic_bw.mul_f32(0.02),
            copper: metabolic_bw.mul_f32(0.20),
            iodine: metabolic_bw.mul_f32(0.03),
            iron: metabolic_bw.mul_f32(1.00),
            manganese: metabolic_bw.mul_f32(0.16),
            selenium: metabolic_bw.mul_f32(6.40),
            zinc: metabolic_bw.mul_f32(2.00),
        };
    }
}

impl<'a> IntoIterator for &'a Minerals {
    type Item = (&'static str, &'a dyn fmt::Display);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("Calcium", &self.calcium as &dyn fmt::Display),
            ("Phosphorus", &self.phosphorus as &dyn fmt::Display),
            ("Potassium", &self.potassium as &dyn fmt::Display),
            ("Sodium", &self.sodium as &dyn fmt::Display),
            ("Chloride", &self.chloride as &dyn fmt::Display),
            ("Magnesium", &self.magnesium as &dyn fmt::Display),
            ("Copper", &self.copper as &dyn fmt::Display),
            ("Iodine", &self.iodine as &dyn fmt::Display),
            ("Iron", &self.iron as &dyn fmt::Display),
            ("Manganese", &self.manganese as &dyn fmt::Display),
            ("Selenium", &self.selenium as &dyn fmt::Display),
            ("Zinc", &self.zinc as &dyn fmt::Display),
        ].into_iter()
    }
}

#[derive(Clone, Default)]
pub struct Vitamins {
    pub vit_a: IU,
    pub vit_d: IU,
    pub vit_e: IU,
    pub vit_b1: Milligram,
    pub vit_b2: Milligram,
    pub vit_b5: Milligram,
    pub vit_b6: Milligram,
    pub vit_b12: Microgram,
    pub vit_b3: Milligram,
    pub vit_b9: Microgram,
    pub choline: Milligram,
}

impl NewRecommendedNutrientIntake<Self> for Vitamins {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> Self {
        return Vitamins {
            vit_a: metabolic_bw.mul_f32(167.00),
            vit_d: metabolic_bw.mul_f32(15.20),
            vit_e: metabolic_bw.mul_f32(1.00),
            vit_b1: metabolic_bw.mul_f32(0.06),
            vit_b2: metabolic_bw.mul_f32(0.17),
            vit_b5: metabolic_bw.mul_f32(0.39),
            vit_b6: metabolic_bw.mul_f32(0.04),
            vit_b12: metabolic_bw.mul_f32(0.92),
            vit_b3: metabolic_bw.mul_f32(0.45),
            vit_b9: metabolic_bw.mul_f32(7.10),
            choline: metabolic_bw.mul_f32(45.00),
        };
    }
}

impl<'a> IntoIterator for &'a Vitamins {
    type Item = (&'static str, &'a dyn fmt::Display);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("Vitamin A", &self.vit_a as &dyn fmt::Display),
            ("Vitamin D", &self.vit_d as &dyn fmt::Display),
            ("Vitamin E", &self.vit_e as &dyn fmt::Display),
            ("Vitamin B1", &self.vit_b1 as &dyn fmt::Display),
            ("Vitamin B2", &self.vit_b2 as &dyn fmt::Display),
            ("Vitamin B5", &self.vit_b5 as &dyn fmt::Display),
            ("Vitamin B6", &self.vit_b6 as &dyn fmt::Display),
            ("Vitamin B12", &self.vit_b12 as &dyn fmt::Display),
            ("Vitamin B3", &self.vit_b3 as &dyn fmt::Display),
            ("Vitamin B9", &self.vit_b9 as &dyn fmt::Display),
            ("Choline", &self.choline as &dyn fmt::Display),
        ].into_iter()
    }
}

pub trait FromValue {
    fn from_value(value: f32) -> Self;
}

#[derive(Clone, Default)]
pub struct IU {
    value: f32,
}

#[derive(Clone, Default)]
pub struct Kilogram {
    value: f32,
}

#[derive(Clone, Default)]
pub struct Gram {
    value: f32,
}

#[derive(Clone, Default)]
pub struct Microgram {
    value: f32,
}

#[derive(Clone, Default)]
pub struct Milligram {
    value: f32,
}

#[derive(Clone, Default, Debug)]
pub struct Kcal {
    value: f32,
}

impl fmt::Display for Kcal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} kcal", self.value.round())
    }
}

impl fmt::Display for Gram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} gr", self.value)
    }
}

impl fmt::Display for Milligram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} mg", self.value)
    }
}

impl fmt::Display for Microgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} µg", self.value)
    }
}

impl fmt::Display for IU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} IU", self.value.round())
    }
}

impl FromValue for Kcal {
    fn from_value(value: f32) -> Self {
        Kcal { value }
    }
}

impl FromValue for IU {
    fn from_value(value: f32) -> Self {
        IU { value }
    }
}

impl FromValue for Kilogram {
    fn from_value(value: f32) -> Self {
        Kilogram { value }
    }
}

impl FromValue for Gram {
    fn from_value(value: f32) -> Self {
        Gram { value }
    }
}

impl FromValue for Milligram {
    fn from_value(value: f32) -> Self {
        Milligram { value }
    }
}

impl FromValue for Microgram {
    fn from_value(value: f32) -> Self {
        Microgram { value }
    }
}
