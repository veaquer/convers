#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Meter,
    Centimeter,
    Millimeter,
    Kilometer,
    Decimeter,
    Hectometer,
    Decameter,
}

pub trait Unit {
    fn to_base(&self) -> f64;
    fn to_other(&self, target_unit: LengthUnit) -> Self;
    fn txt(&self) -> String;
    fn from_str(query: &String) -> Option<Box<Self>>;
    fn is_this(query: &String) -> bool;
}

#[derive(Debug, PartialEq)]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit,
}

impl Unit for Length {
    fn to_base(&self) -> f64 {
        match self.unit {
            LengthUnit::Meter => self.value,
            LengthUnit::Centimeter => self.value / 100.0,
            LengthUnit::Millimeter => self.value / 1000.0,
            LengthUnit::Kilometer => self.value * 1000.0,
            LengthUnit::Decimeter => self.value / 10.0,
            LengthUnit::Hectometer => self.value * 100.0,
            LengthUnit::Decameter => self.value * 10.0,
        }
    }

    fn to_other(&self, target_unit: LengthUnit) -> Self {
        let base_value = self.to_base();
        let target_value = match target_unit {
            LengthUnit::Meter => base_value,
            LengthUnit::Centimeter => base_value * 100.0,
            LengthUnit::Millimeter => base_value * 1000.0,
            LengthUnit::Kilometer => base_value / 1000.0,
            LengthUnit::Decimeter => base_value * 10.0,
            LengthUnit::Hectometer => base_value / 100.0,
            LengthUnit::Decameter => base_value / 10.0,
        };
        Length {
            value: target_value,
            unit: target_unit,
        }
    }

    fn txt(&self) -> String {
        format!("{} {:?}", self.value, self.unit)
    }

    fn is_this(query: &String) -> bool {
        if let Some(_) = Length::from_str(query) {
            true
        } else {
            false
        }
    }

    fn from_str(query: &String) -> Option<Box<Self>> {
        let mut new_length = Box::new(Length {
            value: 0.,
            unit: LengthUnit::Meter,
        });
        let val_str = query
            .chars()
            .take_while(|c| c.is_digit(10) || c == &'.')
            .collect::<String>();
        match val_str.parse::<f64>() {
            Ok(v) => new_length.value = v,
            Err(_) => return None,
        };
        let unit_part = query.chars().skip(val_str.len()).collect::<String>();
        if unit_part.is_empty() {
            return None;
        }
        match unit_part.as_str() {
            "m" => new_length.unit = LengthUnit::Meter,
            "cm" => new_length.unit = LengthUnit::Centimeter,
            "mm" => new_length.unit = LengthUnit::Millimeter,
            "km" => new_length.unit = LengthUnit::Kilometer,
            "dcm" => new_length.unit = LengthUnit::Decimeter,
            "hm" => new_length.unit = LengthUnit::Hectometer,
            "dm" => new_length.unit = LengthUnit::Decameter,
            _ => return None,
        }
        Some(new_length)
    }
}
