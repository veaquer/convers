use convers::{
    convert::magic_convert,
    utils::{
        translate::{Translator, TranslatorError},
        units::{Measurement, Unit},
    },
};

#[test]
fn check_length_unit() {
    let length_in_meters = Measurement {
        value: 1.0,
        unit: Unit::Meter,
    };

    let length_in_centimeters = Measurement {
        value: 100.0,
        unit: Unit::Centimeter,
    };

    let length_in_millimeters = Measurement {
        value: 1000.0,
        unit: Unit::Millimeter,
    };

    let length_in_kilometers = Measurement {
        value: 0.001,
        unit: Unit::Kilometer,
    };

    let length_in_decimeters = Measurement {
        value: 10.0,
        unit: Unit::Decimeter,
    };

    let length_in_hectometers = Measurement {
        value: 0.01,
        unit: Unit::Hectometer,
    };

    let length_in_decameters = Measurement {
        value: 0.1,
        unit: Unit::Decameter,
    };

    let length_in_str = "250m";
    let predicated_length_from_str = Measurement {
        value: 250.0,
        unit: Unit::Meter,
    };

    assert_eq!(
        predicated_length_from_str,
        *Measurement::from_str(&length_in_str.to_string()).unwrap()
    );
    assert_eq!(length_in_meters.to_base(), 1.0);
    assert_eq!(length_in_centimeters.to_base(), 1.0);
    assert_eq!(length_in_millimeters.to_base(), 1.0);
    assert_eq!(length_in_kilometers.to_base(), 1.0);
    assert_eq!(length_in_decimeters.to_base(), 1.0);
    assert_eq!(length_in_hectometers.to_base(), 1.0);
    assert_eq!(length_in_decameters.to_base(), 1.0);
}

#[test]
fn check_convert() -> Result<(), String> {
    let response = Measurement::convert(&String::from("250m to km"))?;
    println!("Result: {}", response.txt());
    Ok(())
}

#[tokio::test]
async fn check_convert_async() -> Result<(), TranslatorError> {
    let tr = Translator::new();
    let response = tr.translate("en", "ru", "no way").await?;
    assert_eq!(response, "\n [ en -> ru ] \n\n ни за что");
    Ok(())
}

#[tokio::test]
async fn check_magic_convert_async() -> Result<(), TranslatorError> {
    let response = magic_convert(&String::from("250m:km")).await?;
    assert_eq!(response, "0.25 Kilometer");
    let tr_response = magic_convert(&String::from("en to ru no way")).await?;
    assert_eq!(tr_response, "\n [ en -> ru ] \n\n ни за что");
    Ok(())
}
