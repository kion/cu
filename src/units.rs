#[derive(Default)]
pub struct Unit {
    pub name: &'static str,
    pub abbr: &'static str,
    pub aliases: &'static [&'static str],
    pub ratios: Option<Vec<(&'static str, f64)>>,
    pub formula: Option<fn(u: &Unit, v: f64) -> Result<f64, String>>
}

lazy_static! {
    pub static ref UNITS: Vec<(&'static str, Vec<Unit>)> = vec![
        ("AREA", vec![
            Unit {
                name: "Square Meter",
                abbr: "m²",
                aliases: &["m2", "sq m", "square meter", "square meters", "square metre", "square metres"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Square Kilometer",
                abbr: "km²",
                aliases: &["km2", "sq km", "square kilometer", "square kilometers", "square kilometre", "square kilometres"],
                ratios: Some(vec![("", 1e+6)]),
                ..Default::default()
            },
            Unit {
                name: "Square Mile",
                abbr: "mi²",
                aliases: &["mi2", "sq mi", "square mile", "square miles"],
                ratios: Some(vec![("", 2.59e+6)]),
                ..Default::default()
            },
            Unit {
                name: "Square Yard",
                abbr: "yd²",
                aliases: &["yd2", "sq yd", "square yard", "square yards"],
                ratios: Some(vec![("", 0.836127)]),
                ..Default::default()
            },
            Unit {
                name: "Square Foot",
                abbr: "ft²",
                aliases: &["ft2", "sq ft", "square foot", "square feet"],
                ratios: Some(vec![("", 0.092903)]),
                ..Default::default()
            },
            Unit {
                name: "Square Inch",
                abbr: "in²",
                aliases: &["in2", "sq in", "square inch", "square inches"],
                ratios: Some(vec![("", 0.00064516)]),
                ..Default::default()
            },
            Unit {
                name: "Hectare",
                abbr: "ha",
                aliases: &["hectare", "hectares"],
                ratios: Some(vec![("", 10000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Acre",
                abbr: "a",
                aliases: &["acre", "acres"],
                ratios: Some(vec![("", 4046.86)]),
                ..Default::default()
            },
        ]),
        ("DIGITAL STORAGE", vec![
            Unit {
                name: "Bit",
                abbr: "b",
                aliases: &["bit", "bits"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kilobit",
                abbr: "kb",
                aliases: &["kbit", "kbits", "kilobit", "kilobits"],
                ratios: Some(vec![("", 1000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kibibit",
                abbr: "Kib",
                aliases: &["kibit", "kibits", "kibibit", "kibibits"],
                ratios: Some(vec![("", 1024.0)]),
                ..Default::default()
            },
            Unit {
                name: "Megabit",
                abbr: "Mb",
                aliases: &["mbit", "mbits", "megabit", "megabits"],
                ratios: Some(vec![("", 1e+6)]),
                ..Default::default()
            },
            Unit {
                name: "Mebibit",
                abbr: "Mib",
                aliases: &["mibit", "mibits", "mebibit", "mebibits"],
                ratios: Some(vec![("", 1048576.0)]),
                ..Default::default()
            },
            Unit {
                name: "Gigabit",
                abbr: "Gb",
                aliases: &["gbit", "gbits", "gigabit", "gigabits"],
                ratios: Some(vec![("", 1e+9)]),
                ..Default::default()
            },
            Unit {
                name: "Gibibit",
                abbr: "Gib",
                aliases: &["gibit", "gibits", "gibibit", "gibibits"],
                ratios: Some(vec![("", 1073741824.0)]),
                ..Default::default()
            },
            Unit {
                name: "Terabit",
                abbr: "Tb",
                aliases: &["tbit", "tbits", "terabit", "terabits"],
                ratios: Some(vec![("", 1e+12)]),
                ..Default::default()
            },
            Unit {
                name: "Tebibit",
                abbr: "Tib",
                aliases: &["tibit", "tibits", "tebibit", "tebibits"],
                ratios: Some(vec![("", 1099511627776.0)]),
                ..Default::default()
            },
            Unit {
                name: "Petabit",
                abbr: "Pb",
                aliases: &["pbit", "pbits", "petabit", "petabits"],
                ratios: Some(vec![("", 1e+15)]),
                ..Default::default()
            },
            Unit {
                name: "Pebibit",
                abbr: "Pib",
                aliases: &["pibit", "pibits", "pebibit", "pebibits"],
                ratios: Some(vec![("", 1125899906842624.0)]),
                ..Default::default()
            },
            Unit {
                name: "Byte",
                abbr: "B",
                aliases: &["byte", "bytes"],
                ratios: Some(vec![("", 8.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kilobyte",
                abbr: "kB",
                aliases: &["kbyte", "kbytes", "kilobyte", "kilobytes"],
                ratios: Some(vec![("", 8000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kibibyte",
                abbr: "KiB",
                aliases: &["kibyte", "kibytes", "kibibyte", "kibibytes"],
                ratios: Some(vec![("", 8192.0)]),
                ..Default::default()
            },
            Unit {
                name: "Megabyte",
                abbr: "MB",
                aliases: &["mbyte", "mbytes", "megabyte", "megabytes"],
                ratios: Some(vec![("", 8e+6)]),
                ..Default::default()
            },
            Unit {
                name: "Mebibyte",
                abbr: "MiB",
                aliases: &["mibyte", "mibytes", "mebibyte", "mebibytes"],
                ratios: Some(vec![("", 8388608.0)]),
                ..Default::default()
            },
            Unit {
                name: "Gigabyte",
                abbr: "GB",
                aliases: &["gbyte", "gbytes", "gigabyte", "gigabytes"],
                ratios: Some(vec![("", 8e+9)]),
                ..Default::default()
            },
            Unit {
                name: "Gibibyte",
                abbr: "GiB",
                aliases: &["gibyte", "gibytes", "gibibyte", "gibibytes"],
                ratios: Some(vec![("", 8589934592.0)]),
                ..Default::default()
            },
            Unit {
                name: "Terabyte",
                abbr: "TB",
                aliases: &["tbyte", "tbytes", "terabyte", "terabytes"],
                ratios: Some(vec![("", 8e+12)]),
                ..Default::default()
            },
            Unit {
                name: "Tebibyte",
                abbr: "TiB",
                aliases: &["tibyte", "tibytes", "tebibyte", "tebibytes"],
                ratios: Some(vec![("", 8796093022208.0)]),
                ..Default::default()
            },
            Unit {
                name: "Petabyte",
                abbr: "PB",
                aliases: &["pbyte", "pbytes", "petabyte", "petabytes"],
                ratios: Some(vec![("", 8e+15)]),
                ..Default::default()
            },
            Unit {
                name: "Pebibyte",
                abbr: "PiB",
                aliases: &["pibyte", "pibytes", "pebibyte", "pebibytes"],
                ratios: Some(vec![("", 9007199254740992.0)]),
                ..Default::default()
            },
        ]),
        ("ENERGY", vec![
            Unit {
                name: "Joule",
                abbr: "J",
                aliases: &["joule", "joules"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kilojoule",
                abbr: "kJ",
                aliases: &["kilojoule", "kilojoules"],
                ratios: Some(vec![("", 1000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Calorie",
                abbr: "cal",
                aliases: &["cals", "calorie", "calories"],
                ratios: Some(vec![("", 4.184)]),
                ..Default::default()
            },
            Unit {
                name: "Kilocalorie",
                abbr: "kcal",
                aliases: &["kcals", "kilocalorie", "kilocalories"],
                ratios: Some(vec![("", 4184.0)]),
                ..Default::default()
            },
            Unit {
                name: "Watt-hour",
                abbr: "W⋅h",
                aliases: &["wh", "whs", "watt-hour", "watt-hours", "watt hour", "watt hours"],
                ratios: Some(vec![("", 3600.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kilowatt-hour",
                abbr: "kW⋅h",
                aliases: &["kwh", "kwhs", "kilowatt-hour", "kilowatt-hours", "kilowatt hour", "kilowatt hours"],
                ratios: Some(vec![("", 3600000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Electronvolt",
                abbr: "eV",
                aliases: &["evs", "electronvolt", "electronvolts", "electron-volt", "electron-volts", "electron volt", "electron volts"],
                ratios: Some(vec![("", 1.6022e-19)]),
                ..Default::default()
            },
            Unit {
                name: "British Thermal Unit",
                abbr: "Btu",
                aliases: &["btus", "british thermal unit", "british thermal units"],
                ratios: Some(vec![("", 1055.06)]),
                ..Default::default()
            },
            Unit {
                name: "Therm",
                abbr: "thm",
                aliases: &["thms", "therm", "therms"],
                ratios: Some(vec![("", 1.055e+8)]),
                ..Default::default()
            },
            Unit {
                name: "Foot-Pound Force",
                abbr: "ft⋅lbf",
                aliases: &["ftlbf", "ftlbfs", "ftlb", "ftlbs", "foot-pound force", "foot pound force", "foot-pound", "foot pound"],
                ratios: Some(vec![("", 1.35582)]),
                ..Default::default()
            },
        ]),
        ("FREQUENCY", vec![
            Unit {
                name: "Hertz",
                abbr: "Hz",
                aliases: &["hertz"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kilohertz",
                abbr: "kHz",
                aliases: &["kilohertz"],
                ratios: Some(vec![("", 1000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Megahertz",
                abbr: "MHz",
                aliases: &["megahertz"],
                ratios: Some(vec![("", 1e+6)]),
                ..Default::default()
            },
            Unit {
                name: "Gigahertz",
                abbr: "GHz",
                aliases: &["gigahertz"],
                ratios: Some(vec![("", 1e+9)]),
                ..Default::default()
            },
        ]),
        ("LENGTH", vec![
            Unit {
                name: "Meter",
                abbr: "m",
                aliases: &["meter", "meters", "metre", "metres"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Kilometer",
                abbr: "km",
                aliases: &["kilometer", "kilometers", "kilometre", "kilometres"],
                ratios: Some(vec![("", 1000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Centimeter",
                abbr: "cm",
                aliases: &["centimeter", "centimeters", "centimetre", "centimetres"],
                ratios: Some(vec![("", 0.01)]),
                ..Default::default()
            },
            Unit {
                name: "Millimeter",
                abbr: "mm",
                aliases: &["millimeter", "millimeters", "millimetre", "millimetres"],
                ratios: Some(vec![("", 0.001)]),
                ..Default::default()
            },
            Unit {
                name: "Micrometer",
                abbr: "μm",
                aliases: &["micrometer", "micrometers", "micrometre", "micrometres"],
                ratios: Some(vec![("", 1e-6)]),
                ..Default::default()
            },
            Unit {
                name: "Nanometer",
                abbr: "nm",
                aliases: &["nanometer", "nanometers", "nanometre", "nanometres"],
                ratios: Some(vec![("", 1e-9)]),
                ..Default::default()
            },
            Unit {
                name: "Mile",
                abbr: "mi",
                aliases: &["mile", "miles"],
                ratios: Some(vec![("", 1609.34)]),
                ..Default::default()
            },
            Unit {
                name: "Nautical Mile",
                abbr: "nmi",
                aliases: &["nautical mile", "nautical miles"],
                ratios: Some(vec![("", 1852.0)]),
                ..Default::default()
            },
            Unit {
                name: "Yard",
                abbr: "yd",
                aliases: &["yard", "yards"],
                ratios: Some(vec![("", 0.9144)]),
                ..Default::default()
            },
            Unit {
                name: "Foot",
                abbr: "ft",
                aliases: &["foot", "feet"],
                ratios: Some(vec![("", 0.3048)]),
                ..Default::default()
            },
            Unit {
                name: "Inch",
                abbr: "″",
                aliases: &["\"", "in", "inch", "inches"],
                ratios: Some(vec![("", 0.0254)]),
                ..Default::default()
            },
        ]),
        ("MASS", vec![
            Unit {
                name: "Kilogram",
                abbr: "kg",
                aliases: &["kilogram", "kilograms"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Tonne • Metric / Imperial / US",
                abbr: "t",
                aliases: &["tonne", "tonnes"],
                ratios: Some(vec![
                    ("Metric", 1000.0),
                    ("Imperial", 1016.05),
                    ("US", 907.185)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Gram",
                abbr: "gm",
                aliases: &["gram", "grams"],
                ratios: Some(vec![("", 0.001)]),
                ..Default::default()
            },
            Unit {
                name: "Milligram",
                abbr: "mg",
                aliases: &["milligram", "milligrams"],
                ratios: Some(vec![("", 1e-6)]),
                ..Default::default()
            },
            Unit {
                name: "Microgram",
                abbr: "µg",
                aliases: &["microgram", "micrograms"],
                ratios: Some(vec![("", 1e-9)]),
                ..Default::default()
            },
            Unit {
                name: "Stone",
                abbr: "st",
                aliases: &["stone", "stones"],
                ratios: Some(vec![("", 6.35029)]),
                ..Default::default()
            },
            Unit {
                name: "Pound",
                abbr: "lb",
                aliases: &["pound", "pounds"],
                ratios: Some(vec![("", 0.453592)]),
                ..Default::default()
            },
            Unit {
                name: "Ounce",
                abbr: "oz",
                aliases: &["ounce", "ounces"],
                ratios: Some(vec![("", 0.0283495)]),
                ..Default::default()
            },
        ]),
        ("PLANE ANGLE", vec![
            Unit {
                name: "Radian",
                abbr: "rad",
                aliases: &["rads", "r", "radian", "radians"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Degree",
                abbr: "°",
                aliases: &["d", "degree", "degrees"],
                ratios: Some(vec![("", 0.0174533)]),
                ..Default::default()
            },
            Unit {
                name: "Gradian",
                abbr: "ᵍ",
                aliases: &["grad", "grads", "gradian", "gradians", "gr", "grs", "grd", "grds", "gon", "gons", "grade", "grades"],
                ratios: Some(vec![("", 0.015708)]),
                ..Default::default()
            },
            Unit {
                name: "Milliradian",
                abbr: "mrad",
                aliases: &["mrads", "mr", "mrs", "mil", "mils", "milliradian", "milliradians"],
                ratios: Some(vec![("", 0.001)]),
                ..Default::default()
            },
            Unit {
                name: "Minute of Arc",
                abbr: "′",
                aliases: &["'", "minute of arc", "minutes of arc", "minute arc", "minutes arc", "arc minute", "arc minutes", "arcminute", "arcminutes", "arcmin", "arcmins", "ma", "am"],
                ratios: Some(vec![("", 0.000290888)]),
                ..Default::default()
            },
            Unit {
                name: "Second of Arc",
                abbr: "″",
                aliases: &["\"", "second of arc", "seconds of arc", "second arc", "seconds arc", "arc second", "arc seconds", "arcsecond", "arcseconds", "arcsec", "arcsecs", "sa", "as"],
                ratios: Some(vec![("", 4.8481e-6)]),
                ..Default::default()
            },
        ]),
        ("PRESSURE", vec![
            Unit {
                name: "Pascal",
                abbr: "Pa",
                aliases: &["pascal", "pascals"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Bar",
                abbr: "bar",
                aliases: &["bars"],
                ratios: Some(vec![("", 100000.0)]),
                ..Default::default()
            },
            Unit {
                name: "Pound-Force per Square Inch",
                abbr: "psi",
                aliases: &["psis", "lbf/in2", "pound-force per square inch"],
                ratios: Some(vec![("", 6894.76)]),
                ..Default::default()
            },
            Unit {
                name: "Standard Atmosphere",
                abbr: "atm",
                aliases: &["atms", "standard atmosphere", "standard atmospheres"],
                ratios: Some(vec![("", 101325.0)]),
                ..Default::default()
            },
            Unit {
                name: "Torr",
                abbr: "Torr",
                aliases: &["torrs"],
                ratios: Some(vec![("", 133.322)]),
                ..Default::default()
            },
        ]),
        ("TEMPERATURE", vec![
            Unit {
                name: "Kelvin",
                abbr: "K",
                aliases: &["k", "kelvin"],
                ratios: None,
                formula: Some(|u: &Unit, v: f64| -> Result<f64, String> {
                    match u.abbr {
                        "°F" => return Ok((v - 32.0) * 5.0/9.0 + 273.15),
                        "°C" => return Ok(v + 273.15),
                        _ => return Err(u.abbr.to_string())
                    }
                })
            },
            Unit {
                name: "Celsius",
                abbr: "°C",
                aliases: &["c", "celsius"],
                ratios: None,
                formula: Some(|u: &Unit, v: f64| -> Result<f64, String> {
                    match u.abbr {
                        "K" => return Ok(v - 273.15),
                        "°F" => return Ok((v - 32.0) * 5.0/9.0),
                        _ => return Err(u.abbr.to_string())
                    }
                })
            },
            Unit {
                name: "Fahrenheit",
                abbr: "°F",
                aliases: &["f", "fahrenheit"],
                ratios: None,
                formula: Some(|u: &Unit, v: f64| -> Result<f64, String> {
                    match u.abbr {
                        "K" => return Ok((v - 273.15) * 9.0/5.0 + 32.0),
                        "°C" => return Ok(v * 9.0/5.0 + 32.0),
                        _ => return Err(u.abbr.to_string())
                    }
                })
            },
        ]),
        ("TIME", vec![
            Unit {
                name: "Second",
                abbr: "s",
                aliases: &["sec", "second", "seconds"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Nanosecond",
                abbr: "ns",
                aliases: &["nanosecond", "nanoseconds"],
                ratios: Some(vec![("", 1e-9)]),
                ..Default::default()
            },
            Unit {
                name: "Microsecond",
                abbr: "μs",
                aliases: &["microsecond", "microseconds"],
                ratios: Some(vec![("", 1e-6)]),
                ..Default::default()
            },
            Unit {
                name: "Millisecond",
                abbr: "ms",
                aliases: &["millisecond", "milliseconds"],
                ratios: Some(vec![("", 0.001)]),
                ..Default::default()
            },
            Unit {
                name: "Minute",
                abbr: "min",
                aliases: &["m", "mins", "minute", "minutes"],
                ratios: Some(vec![("", 60.0)]),
                ..Default::default()
            },
            Unit {
                name: "Hour",
                abbr: "hr",
                aliases: &["h", "hrs", "hour", "hours"],
                ratios: Some(vec![("", 3600.0)]),
                ..Default::default()
            },
            Unit {
                name: "Day",
                abbr: "d",
                aliases: &["day", "days"],
                ratios: Some(vec![("", 86400.0)]),
                ..Default::default()
            },
            Unit {
                name: "Week",
                abbr: "wk",
                aliases: &["w", "week", "weeks"],
                ratios: Some(vec![("", 604800.0)]),
                ..Default::default()
            },
            Unit {
                name: "Month",
                abbr: "mth",
                aliases: &["m", "month", "months"],
                ratios: Some(vec![("", 2629746.0)]),
                ..Default::default()
            },
            Unit {
                name: "Year",
                abbr: "yr",
                aliases: &["y", "yrs", "year", "years"],
                ratios: Some(vec![("", 31556952.0)]),
                ..Default::default()
            },
            Unit {
                name: "Decade",
                abbr: "dec",
                aliases: &["d", "decs", "decade", "decades"],
                ratios: Some(vec![("", 315569520.0)]),
                ..Default::default()
            },
            Unit {
                name: "Century",
                abbr: "cent",
                aliases: &["c", "century", "centuries"],
                ratios: Some(vec![("", 3155695200.0)]),
                ..Default::default()
            },
        ]),
        ("VOLUME", vec![
            Unit {
                name: "Cubic Meter",
                abbr: "m³",
                aliases: &["m3", "meter3", "meters3", "metre3", "metres3", "cubic meter", "cubic meters", "cubic metre", "cubic metres"],
                ratios: Some(vec![("", 1.0)]),
                ..Default::default()
            },
            Unit {
                name: "Liter",
                abbr: "l",
                aliases: &["liter", "liters", "litre", "litres"],
                ratios: Some(vec![("", 0.001)]),
                ..Default::default()
            },
            Unit {
                name: "Milliliter",
                abbr: "ml",
                aliases: &["milliliter", "milliliters", "millilitre", "millilitres"],
                ratios: Some(vec![("", 1e-6)]),
                ..Default::default()
            },
            Unit {
                name: "Gallon • Imperial / US liquid",
                abbr: "gal",
                aliases: &["gallon", "gallons"],
                ratios: Some(vec![
                    ("Imperial", 0.00454609),
                    ("US liquid", 0.00378541)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Quart • Imperial / US liquid",
                abbr: "qt",
                aliases: &["quart", "quarts"],
                ratios: Some(vec![
                    ("Imperial", 0.00113652),
                    ("US liquid", 0.000946353)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Pint • Imperial / US liquid",
                abbr: "pt",
                aliases: &["pint", "pints"],
                ratios: Some(vec![
                    ("Imperial", 0.000568261),
                    ("US liquid", 0.000473176)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Cup • Imperial / US legal",
                abbr: "c",
                aliases: &["cup", "cups"],
                ratios: Some(vec![
                    ("Imperial", 0.000284131),
                    ("US legal", 0.00024)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Fluid Ounce • Imperial / US",
                abbr: "fl oz",
                aliases: &["floz", "fluid ounce", "fluid ounces", "oz", "ounce", "ounces"],
                ratios: Some(vec![
                    ("Imperial", 2.8413e-5),
                    ("US", 2.9574e-5)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Tablespoon • Imperial / US",
                abbr: "tbsp",
                aliases: &["tablespoon", "tablespoons"],
                ratios: Some(vec![
                    ("Imperial", 1.7758e-5),
                    ("US", 1.4787e-5)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Teaspoon • Imperial / US",
                abbr: "tsp",
                aliases: &["teaspoon", "teaspoons"],
                ratios: Some(vec![
                    ("Imperial", 5.9194e-6),
                    ("US", 4.9289e-6)
                ]),
                ..Default::default()
            },
            Unit {
                name: "Cubic Foot",
                abbr: "ft³",
                aliases: &["ft3", "cu ft", "cubic foot", "cubic feet"],
                ratios: Some(vec![("", 0.0283168)]),
                ..Default::default()
            },
            Unit {
                name: "Cubic Inch",
                abbr: "in³",
                aliases: &["in3", "cu in", "cubic inch", "cubic inches"],
                ratios: Some(vec![("", 1.6387e-5)]),
                ..Default::default()
            },
        ]),
    ];
}
