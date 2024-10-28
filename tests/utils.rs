use convers::utils::units::{Length, LengthUnit, Unit};

#[test]
fn check_length_unit() {
    let length_in_meters = Length {
        value: 1.0,
        unit: LengthUnit::Meter,
    };

    let length_in_centimeters = Length {
        value: 100.0,
        unit: LengthUnit::Centimeter,
    };

    let length_in_millimeters = Length {
        value: 1000.0,
        unit: LengthUnit::Millimeter,
    };

    let length_in_kilometers = Length {
        value: 0.001,
        unit: LengthUnit::Kilometer,
    };

    let length_in_decimeters = Length {
        value: 10.0,
        unit: LengthUnit::Decimeter,
    };

    let length_in_hectometers = Length {
        value: 0.01,
        unit: LengthUnit::Hectometer,
    };

    let length_in_decameters = Length {
        value: 0.1,
        unit: LengthUnit::Decameter,
    };

    let length_in_str = "250m";
    let predicated_length_from_str = Length {
        value: 250.0,
        unit: LengthUnit::Meter,
    };

    assert_eq!(
        predicated_length_from_str,
        *Length::from_str(&length_in_str.to_string()).unwrap()
    );
    assert_eq!(length_in_meters.to_base(), 1.0);
    assert_eq!(length_in_centimeters.to_base(), 1.0);
    assert_eq!(length_in_millimeters.to_base(), 1.0);
    assert_eq!(length_in_kilometers.to_base(), 1.0);
    assert_eq!(length_in_decimeters.to_base(), 1.0);
    assert_eq!(length_in_hectometers.to_base(), 1.0);
    assert_eq!(length_in_decameters.to_base(), 1.0);
}
