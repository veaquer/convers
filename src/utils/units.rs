use super::CONV_ERROR;

#[derive(Debug, PartialEq)]
pub enum Unit {
    Meter,
    Centimeter,
    Millimeter,
    Kilometer,
    Decimeter,
    Hectometer,
    Decameter,
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    Ton,
    Pound,
    Ounce,
    Ampere,
    Milliampere,
    Volt,
    Millivolt,
    Watt,
    Kilowatt,
    Joule,
    Kilojoule,
    Second,
    Minute,
    Hour,
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
    Pixel,
    Rem,
    Em,
}

#[derive(Debug, PartialEq)]
pub struct Measurement {
    pub value: f64,
    pub unit: Unit,
}

impl Measurement {
    /// That function is used to convert Unit to default base unit from SI system.
    pub fn to_base(&self) -> f64 {
        match self.unit {
            Unit::Meter => self.value,
            Unit::Centimeter => self.value / 100.0,
            Unit::Millimeter => self.value / 1000.0,
            Unit::Kilometer => self.value * 1000.0,
            Unit::Decimeter => self.value / 10.0,
            Unit::Hectometer => self.value * 100.0,
            Unit::Decameter => self.value * 10.0,
            Unit::Kilogram => self.value,
            Unit::Gram => self.value / 1000.0,
            Unit::Milligram => self.value / 1_000_000.0,
            Unit::Microgram => self.value / 1_000_000_000.0,
            Unit::Ton => self.value * 1000.0,
            Unit::Pound => self.value * 0.453592,
            Unit::Ounce => self.value * 0.0283495,
            Unit::Ampere => self.value,
            Unit::Milliampere => self.value / 1000.0,
            Unit::Volt => self.value,
            Unit::Millivolt => self.value / 1000.0,
            Unit::Watt => self.value,
            Unit::Kilowatt => self.value * 1000.0,
            Unit::Joule => self.value,
            Unit::Kilojoule => self.value * 1000.0,
            Unit::Second => self.value,
            Unit::Minute => self.value * 60.0,
            Unit::Hour => self.value * 3600.0,
            Unit::Byte => self.value,
            Unit::Kilobyte => self.value * 1024.0,
            Unit::Megabyte => self.value * 1024.0 * 1024.0,
            Unit::Gigabyte => self.value * 1024.0 * 1024.0 * 1024.0,
            Unit::Terabyte => self.value * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            Unit::Petabyte => self.value * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            Unit::Pixel => self.value,
            Unit::Rem => self.value * 16.0, // Assuming 1 rem = 16 pixels
            Unit::Em => self.value * 16.0,  // Assuming 1 em = 16 pixels
        }
    }
    // That function is used to convert Measurement to other unit.
    pub fn to_other(&self, target_unit: Unit) -> Self {
        let base_value = self.to_base();
        let target_value = match target_unit {
            Unit::Meter => base_value,
            Unit::Centimeter => base_value * 100.0,
            Unit::Millimeter => base_value * 1000.0,
            Unit::Kilometer => base_value / 1000.0,
            Unit::Decimeter => base_value * 10.0,
            Unit::Hectometer => base_value / 100.0,
            Unit::Decameter => base_value / 10.0,
            Unit::Kilogram => base_value,
            Unit::Gram => base_value * 1000.0,
            Unit::Milligram => base_value * 1_000_000.0,
            Unit::Microgram => base_value * 1_000_000_000.0,
            Unit::Ton => base_value / 1000.0,
            Unit::Pound => base_value / 0.453592,
            Unit::Ounce => base_value / 0.0283495,
            Unit::Ampere => base_value,
            Unit::Milliampere => base_value * 1000.0,
            Unit::Volt => base_value,
            Unit::Millivolt => base_value * 1000.0,
            Unit::Watt => base_value,
            Unit::Kilowatt => base_value / 1000.0,
            Unit::Joule => base_value,
            Unit::Kilojoule => base_value / 1000.0,
            Unit::Second => base_value,
            Unit::Minute => base_value / 60.0,
            Unit::Hour => base_value / 3600.0,
            Unit::Byte => base_value,
            Unit::Kilobyte => base_value / 1024.0,
            Unit::Megabyte => base_value / (1024.0 * 1024.0),
            Unit::Gigabyte => base_value / (1024.0 * 1024.0 * 1024.0),
            Unit::Terabyte => base_value / (1024.0 * 1024.0 * 1024.0 * 1024.0),
            Unit::Petabyte => base_value / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0),
            Unit::Pixel => base_value,
            Unit::Rem => base_value / 16.0, // Assuming 1 rem = 16 pixels
            Unit::Em => base_value / 16.0,  // Assuming 1 em = 16 pixels
        };
        Measurement {
            value: target_value,
            unit: target_unit,
        }
    }
    /// Returns String formated value and unit.
    pub fn txt(&self) -> String {
        format!("{} {:?}", self.value, self.unit)
    }

    /// Returns is measurement in str valid.
    pub fn is_this(query: &String) -> bool {
        if let Some(_) = Measurement::from_str(query) {
            true
        } else {
            false
        }
    }
    /// Converts String query to Measurement.
    /// Example: `1m to cm` returns `Measurement { value: 100.0, unit: Unit::Centimeter }` (don't forget that's wrapped in Result).
    pub fn convert(query: &String) -> Result<Self, String> {
        let parts: Vec<&str> = query.split_whitespace().collect();
        let conv_sign_index = match parts.iter().position(|&x| x == "to" || x == ":") {
            Some(i) => i,
            None => return Err(CONV_ERROR.to_string()),
        };
        if parts.len() != conv_sign_index + 2 {
            return Err(CONV_ERROR.to_string());
        }
        let from_part = parts[..conv_sign_index].join(" ");
        let to_part = parts[conv_sign_index + 1..].join(" ");
        let from = match Measurement::from_str(&from_part) {
            Some(m) => m,
            None => return Err(CONV_ERROR.to_string()),
        };
        let to = match Measurement::from_str(&to_part) {
            Some(m) => m,
            None => return Err("Invalid conversion query: error parsing to part".to_string()),
        };
        Ok(from.to_other(to.unit))
    }

    /// Formats &String to Measurement.
    pub fn from_str(query: &String) -> Option<Box<Self>> {
        let mut new_length = Box::new(Measurement {
            value: 0.,
            unit: Unit::Meter,
        });
        let val_str = query
            .chars()
            .take_while(|c| c.is_digit(10) || c == &'.')
            .collect::<String>();
        match val_str.parse::<f64>() {
            Ok(v) => new_length.value = v,
            Err(_) => {}
        };
        let unit_part = query.chars().skip(val_str.len()).collect::<String>();
        if unit_part.is_empty() {
            return None;
        }
        match unit_part.as_str() {
            "m" => new_length.unit = Unit::Meter,
            "cm" => new_length.unit = Unit::Centimeter,
            "mm" => new_length.unit = Unit::Millimeter,
            "km" => new_length.unit = Unit::Kilometer,
            "dcm" => new_length.unit = Unit::Decimeter,
            "hm" => new_length.unit = Unit::Hectometer,
            "dm" => new_length.unit = Unit::Decameter,
            "kg" => new_length.unit = Unit::Kilogram,
            "g" => new_length.unit = Unit::Gram,
            "mg" => new_length.unit = Unit::Milligram,
            "µg" => new_length.unit = Unit::Microgram,
            "t" => new_length.unit = Unit::Ton,
            "lb" => new_length.unit = Unit::Pound,
            "oz" => new_length.unit = Unit::Ounce,
            "A" => new_length.unit = Unit::Ampere,
            "mA" => new_length.unit = Unit::Milliampere,
            "V" => new_length.unit = Unit::Volt,
            "mV" => new_length.unit = Unit::Millivolt,
            "W" => new_length.unit = Unit::Watt,
            "kW" => new_length.unit = Unit::Kilowatt,
            "J" => new_length.unit = Unit::Joule,
            "kJ" => new_length.unit = Unit::Kilojoule,
            "s" => new_length.unit = Unit::Second,
            "min" => new_length.unit = Unit::Minute,
            "h" => new_length.unit = Unit::Hour,
            "B" => new_length.unit = Unit::Byte,
            "KB" => new_length.unit = Unit::Kilobyte,
            "MB" => new_length.unit = Unit::Megabyte,
            "GB" => new_length.unit = Unit::Gigabyte,
            "TB" => new_length.unit = Unit::Terabyte,
            "PB" => new_length.unit = Unit::Petabyte,
            "px" => new_length.unit = Unit::Pixel,
            "rem" => new_length.unit = Unit::Rem,
            "em" => new_length.unit = Unit::Em,
            _ => return None,
        }
        Some(new_length)
    }
}
