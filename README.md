# cu

A simple, fast and accurate unit conversion utility.

Written in Rust, with minimal dependencies.

![demo](assets/cu-demo.gif)

### Usage

```
<sv> <su> =|to <tu> [:<dp>]
```

* `<sv>` _(required)_ - the value to convert
* `<su>` _(required)_ - the unit of the value to convert
* `<tu>` _(required)_ - the unit to convert the value into
* `<dp>` _(optional)_ - the decimal places precision for the conversion result: either an integer _(implicit/default value is 2)_ or an `*` to use max precision

Use one of the following parameters to get the usage output from the app itself:

`--help, -help, -h, help`

### Supported units

```
-------------------------------------------
| AREA                                    |
-------------------------------------------
| Square Meter                   | m²     |
| Square Kilometer               | km²    |
| Square Mile                    | mi²    |
| Square Yard                    | yd²    |
| Square Foot                    | ft²    |
| Square Inch                    | in²    |
| Hectare                        | ha     |
| Acre                           | a      |
-------------------------------------------
| DIGITAL STORAGE                         |
-------------------------------------------
| Bit                            | b      |
| Kilobit                        | kb     |
| Kibibit                        | Kib    |
| Megabit                        | Mb     |
| Mebibit                        | Mib    |
| Gigabit                        | Gb     |
| Gibibit                        | Gib    |
| Terabit                        | Tb     |
| Tebibit                        | Tib    |
| Petabit                        | Pb     |
| Pebibit                        | Pib    |
| Byte                           | B      |
| Kilobyte                       | kB     |
| Kibibyte                       | KiB    |
| Megabyte                       | MB     |
| Mebibyte                       | MiB    |
| Gigabyte                       | GB     |
| Gibibyte                       | GiB    |
| Terabyte                       | TB     |
| Tebibyte                       | TiB    |
| Petabyte                       | PB     |
| Pebibyte                       | PiB    |
-------------------------------------------
| ENERGY                                  |
-------------------------------------------
| Joule                          | J      |
| Kilojoule                      | kJ     |
| Calorie                        | cal    |
| Kilocalorie                    | kcal   |
| Watt-hour                      | W⋅h    |
| Kilowatt-hour                  | kW⋅h   |
| Electronvolt                   | eV     |
| British Thermal Unit           | Btu    |
| Therm                          | thm    |
| Foot-Pound Force               | ft⋅lbf |
-------------------------------------------
| FREQUENCY                               |
-------------------------------------------
| Hertz                          | Hz     |
| Kilohertz                      | kHz    |
| Megahertz                      | MHz    |
| Gigahertz                      | GHz    |
-------------------------------------------
| LENGTH                                  |
-------------------------------------------
| Meter                          | m      |
| Kilometer                      | km     |
| Centimeter                     | cm     |
| Millimeter                     | mm     |
| Micrometer                     | μm     |
| Nanometer                      | nm     |
| Mile                           | mi     |
| Nautical Mile                  | nmi    |
| Yard                           | yd     |
| Foot                           | ft     |
| Inch                           | ″      |
-------------------------------------------
| MASS                                    |
-------------------------------------------
| Kilogram                       | kg     |
| Tonne • Metric / Imperial / US | t      |
| Gram                           | gm     |
| Milligram                      | mg     |
| Microgram                      | µg     |
| Stone                          | st     |
| Pound                          | lb     |
| Ounce                          | oz     |
-------------------------------------------
| PLANE ANGLE                             |
-------------------------------------------
| Radian                         | rad    |
| Degree                         | °      |
| Gradian                        | ᵍ      |
| Milliradian                    | mrad   |
| Minute of Arc                  | ′      |
| Second of Arc                  | ″      |
-------------------------------------------
| PRESSURE                                |
-------------------------------------------
| Pascal                         | Pa     |
| Bar                            | bar    |
| Pound-Force per Square Inch    | psi    |
| Standard Atmosphere            | atm    |
| Torr                           | Torr   |
-------------------------------------------
| TEMPERATURE                             |
-------------------------------------------
| Kelvin                         | K      |
| Celsius                        | °C     |
| Fahrenheit                     | °F     |
-------------------------------------------
| TIME                                    |
-------------------------------------------
| Second                         | s      |
| Nanosecond                     | ns     |
| Microsecond                    | μs     |
| Millisecond                    | ms     |
| Minute                         | min    |
| Hour                           | hr     |
| Day                            | d      |
| Week                           | wk     |
| Month                          | mth    |
| Year                           | yr     |
| Decade                         | dec    |
| Century                        | cent   |
-------------------------------------------
| VOLUME                                  |
-------------------------------------------
| Cubic Meter                    | m³     |
| Liter                          | l      |
| Milliliter                     | ml     |
| Gallon • Imperial / US liquid  | gal    |
| Quart • Imperial / US liquid   | qt     |
| Pint • Imperial / US liquid    | pt     |
| Cup • Imperial / US legal      | c      |
| Fluid Ounce • Imperial / US    | fl oz  |
| Tablespoon • Imperial / US     | tbsp   |
| Teaspoon • Imperial / US       | tsp    |
| Cubic Foot                     | ft³    |
| Cubic Inch                     | in³    |
-------------------------------------------

```

Use one of the following parameters to get the supported units output from the app itself:

`--help units, -help units, -hu, help units`

### Version

Use one of the following parameters to check the app version:

`--version, -version, -v, version`
