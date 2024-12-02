use super::super::shared::types::ActivityLevel;

pub struct MetabolicBodyWeight {
    pub value: Kilogram,
}

impl MetabolicBodyWeight {
    fn new(body_weight: Kilogram) -> Self {
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

trait NewRecommendedIntake<T> {
    fn new_recommended_intake(
        metabolic_bw: &MetabolicBodyWeight,
        activity_level: ActivityLevel,
    ) -> T;
}

trait NewRecommendedNutrientIntake<T> {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> T;
}

pub struct Intake {
    daily_kcal: Kcal,
    nutrients: Nutrients,
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

pub struct Nutrients {
    protein: Gram,
    fat: Gram,
    amino_acids: AminoAcids,
    fatty_acids: FattyAcids,
    minerals: Minerals,
    vitamins: Vitamins,
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

pub struct AminoAcids {
    arginine: Gram,
    histidine: Gram,
    isoleucine: Gram,
    leucine: Gram,
    lysine: Gram,
    methionine: Gram,
    cystine: Gram,
    phenylalanine: Gram,
    tyrosine: Gram,
    threonine: Gram,
    tryptophan: Gram,
    valine: Gram,
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

pub struct FattyAcids {
    linoleic_acid: Gram,
}

impl NewRecommendedNutrientIntake<Self> for FattyAcids {
    fn new_recommended_nutrient_intake(metabolic_bw: &MetabolicBodyWeight) -> FattyAcids {
        return FattyAcids {
            linoleic_acid: metabolic_bw.mul_f32(0.36),
        };
    }
}

pub struct Minerals {
    calcium: Gram,
    phosphorus: Gram,
    potassium: Gram,
    sodium: Gram,
    chloride: Gram,
    magnesium: Gram,
    copper: Milligram,
    iodine: Milligram,
    iron: Milligram,
    manganese: Milligram,
    selenium: Microgram,
    zinc: Milligram,
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

pub struct Vitamins {
    vit_a: IU,
    vit_d: IU,
    vit_e: IU,
    vit_b1: Milligram,
    vit_b2: Milligram,
    vit_b5: Milligram,
    vit_b6: Milligram,
    vit_b12: Microgram,
    vit_b3: Milligram,
    vit_b9: Microgram,
    choline: Milligram,
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

trait FromValue {
    fn from_value(value: f32) -> Self;
}

pub struct IU {
    value: f32,
}

pub struct Kilogram {
    value: f32,
}

pub struct Gram {
    value: f32,
}

pub struct Microgram {
    value: f32,
}

pub struct Milligram {
    value: f32,
}

pub struct Kcal {
    value: f32,
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
