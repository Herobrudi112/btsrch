use std::{f64, vec};

use crate::unit_calc_parser::unit_number_parser::{MetricBaseUnit, UnitExp, UnitNumber};

#[derive(Clone, PartialEq)]
pub enum Token {
    Number(String),
    StringLiteral(String),
    Plus,
    Minus,
    Mult,
    Div,
    Power,
    LowerEq,
    Lower,
    HigherEq,
    Higher,
    Eq,
    NEq,
    OpenBracket,
    CloseBracket,
    Convert,
    Dot,
    Unit(UnitNumber, Option<Unit>),
}
impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(s) => format!("{s}").to_string(),
            Token::Unit(s, _) => s.to_string(),
            Token::StringLiteral(s) => format!("\"{s}\""),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Mult => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Power => "^".to_string(),
            Token::LowerEq => "<=".to_string(),
            Token::Lower => "<".to_string(),
            Token::HigherEq => ">=".to_string(),
            Token::Higher => ">".to_string(),
            Token::Eq => "==".to_string(),
            Token::NEq => "!=".to_string(),
            Token::OpenBracket => "(".to_string(),
            Token::CloseBracket => ")".to_string(),
            Token::Convert => "in".to_string(),
            Token::Dot => ".".to_string(),
        }
    }
}
pub fn lex(input: String, units: &Vec<Unit>) -> Option<Vec<Token>> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut start_id = 0;
    let mut output = Vec::new();
    while start_id < chars.len() {
        let mut end_id = chars.len();
        let mut sucess = false;
        while end_id > start_id {
            let token = get_token(chars[start_id..end_id].iter().collect::<String>(), units);
            if let Some(t) = token {
                output.extend(t);
                start_id = end_id;
                sucess = true;
                break;
            }
            end_id -= 1;
        }
        if !sucess {
            return None;
        }
    }
    Some(output)
}
pub fn get_token(s: String, units: &Vec<Unit>) -> Option<Vec<Token>> {
    let atomic = vec![
        ("+", vec![Token::Plus]),
        ("-", vec![Token::Minus]),
        ("*", vec![Token::Mult]),
        ("/", vec![Token::Div]),
        ("^", vec![Token::Power]),
        ("<=", vec![Token::LowerEq]),
        ("<", vec![Token::Lower]),
        (">=", vec![Token::HigherEq]),
        (">", vec![Token::Higher]),
        ("==", vec![Token::Eq]),
        ("!=", vec![Token::NEq]),
        ("(", vec![Token::OpenBracket]),
        (")", vec![Token::CloseBracket]),
        ("as", vec![Token::Convert]),
        ("=>", vec![Token::Convert]),
        ("->", vec![Token::Convert]),
        ("to", vec![Token::Convert]),
        (".", vec![Token::Dot]),
        ("²", vec![Token::Power, Token::Number("2".to_string())]),
        ("³", vec![Token::Power, Token::Number("3".to_string())]),
    ];
    if let Some(t) = atomic.iter().find(|a| a.0 == s) {
        return Some(t.1.iter().cloned().collect());
    }
    if s.chars().all(|c| c.is_numeric()) {
        return Some(vec![Token::Number(s)]);
    }
    if s.chars().all(|c| c.is_whitespace()) {
        return Some(vec![]);
    }
    if s.starts_with('"') && s.ends_with('"') {
        let content = s[1..s.len() - 1].to_string();
        if !content.contains('"') {
            return Some(vec![Token::StringLiteral(content)]);
        }
    }
    for u in units {
        for n in u.valid_names.iter() {
            if *n == s {
                return Some(vec![Token::Unit(u.si.clone(), Some(u.clone()))]);
            }
        }
    }
    None
}
pub fn get_units() -> Vec<Unit> {
    let mut v = vec![];
    v.extend(
        Unit {
            name: "percent".to_string(),
            plural: "percents".to_string(),
            abbreviation: "%".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0e-2,
                units: vec![],
            },
            priority: f32::NEG_INFINITY,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "π".to_string(),
            plural: "π".to_string(),
            abbreviation: "π".to_string(),
            valid_names: vec!["pi".to_string(), "PI".to_string(), "Pi".to_string()],
            si: UnitNumber {
                num: f64::consts::PI,
                units: vec![],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "e".to_string(),
            plural: "e".to_string(),
            abbreviation: "e".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: f64::consts::E,
                units: vec![],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "meter".to_string(),
            plural: "meters".to_string(),
            abbreviation: "m".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "gram".to_string(),
            plural: "grams".to_string(),
            abbreviation: "g".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Gramm,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "second".to_string(),
            plural: "seconds".to_string(),
            abbreviation: "s".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "hertz".to_string(),
            plural: "hertz".to_string(),
            abbreviation: "Hz".to_string(),
            valid_names: vec!["hz".to_string()],
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: -1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.push(
        Unit {
            name: "minute".to_string(),
            plural: "minutes".to_string(),
            abbreviation: "min".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 60.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "hour".to_string(),
            plural: "hours".to_string(),
            abbreviation: "h".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 60.0 * 60.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "day".to_string(),
            plural: "days".to_string(),
            abbreviation: "d".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 60.0 * 60.0 * 24.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "week".to_string(),
            plural: "weeks".to_string(),
            abbreviation: "weeks".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 60.0 * 60.0 * 24.0 * 7.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "month".to_string(),
            plural: "months".to_string(),
            abbreviation: "mon".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 60.0 * 60.0 * 24.0 * 30.436875,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "year".to_string(),
            plural: "years".to_string(),
            abbreviation: "a".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 60.0 * 60.0 * 24.0 * 365.2425,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    );

    v.extend(
        Unit {
            name: "ampere".to_string(),
            plural: "amperes".to_string(),
            abbreviation: "A".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Ampere,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "coulomb".to_string(),
            plural: "coulombs".to_string(),
            abbreviation: "C".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: 1,
                    },
                ],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "volt".to_string(),
            plural: "volts".to_string(),
            abbreviation: "V".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -3,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: -1,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "ohm".to_string(),
            plural: "ohms".to_string(),
            abbreviation: "Ω".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -3,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: -2,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "henry".to_string(),
            plural: "henries".to_string(),
            abbreviation: "H".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: -2,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "tesla".to_string(),
            plural: "teslas".to_string(),
            abbreviation: "T".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: -1,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "weber".to_string(),
            plural: "webers".to_string(),
            abbreviation: "Wb".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: -1,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "watt".to_string(),
            plural: "watts".to_string(),
            abbreviation: "W".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -3,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "joule".to_string(),
            plural: "joules".to_string(),
            abbreviation: "J".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -2,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "newton".to_string(),
            plural: "newtons".to_string(),
            abbreviation: "N".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1000.0,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -2,
                    },
                ],
            }
            .cleaned(),
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "kelvin".to_string(),
            plural: "kelvins".to_string(),
            abbreviation: "K".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Kelvin,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "mole".to_string(),
            plural: "moles".to_string(),
            abbreviation: "mol".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 6.02214076e23,
                units: vec![],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "knot".to_string(),
            plural: "knots".to_string(),
            abbreviation: "knot".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.5144444,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -1,
                    },
                ],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "farad".to_string(),
            plural: "farad".to_string(),
            abbreviation: "F".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.001,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: -2,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: -1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: 4,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Ampere,
                        exp: 2,
                    },
                ],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(vec![
        Unit {
            name: "u".to_string(),
            plural: "u".to_string(),
            abbreviation: "u".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0 / 6.02214076e23,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Gramm,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create(),
    ]);

    v.extend(vec![
    Unit {
        name: "mile per hour".to_string(),
        plural: "miles per hour".to_string(),
        abbreviation: "mph".to_string(),
        valid_names: Vec::new(),
        si: UnitNumber {
            num: 1609.344 / 3600.0, // 1 mph in m/s
            units: vec![
                UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                },
                UnitExp {
                    unit: MetricBaseUnit::Second,
                    exp: -1,
                },
            ],
        }
        .cleaned(),
        priority: 0.0,
    }
    .create()
]);


    v.extend(
        Unit {
            name: "candela".to_string(),
            plural: "candelas".to_string(),
            abbreviation: "cd".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Candela,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "liter".to_string(),
            plural: "liters".to_string(),
            abbreviation: "L".to_string(),
            valid_names: vec!["l".to_string()],
            si: UnitNumber {
                num: 0.001,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 3,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "byte".to_string(),
            plural: "bytes".to_string(),
            abbreviation: "B".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1.0,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Byte,
                    exp: 1,
                }],
            },
            priority: 0.0,
        }
        .create()
        .add_si_prefixes(),
    );

    v.extend(
        Unit {
            name: "bit".to_string(),
            plural: "bits".to_string(),
            abbreviation: "b".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.125,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Byte,
                    exp: 1,
                }],
            },
            priority: 0.9,
        }
        .create()
        .add_si_prefixes(),
    );

    // IMPERIAL

    v.push(
        Unit {
            name: "inch".to_string(),
            plural: "inches".to_string(),
            abbreviation: "in".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.0254,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "foot".to_string(),
            plural: "feet".to_string(),
            abbreviation: "ft".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.3048,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "yard".to_string(),
            plural: "yards".to_string(),
            abbreviation: "yd".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.9144,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "mile".to_string(),
            plural: "miles".to_string(),
            abbreviation: "mi".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 1609.344,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 1,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "ounce".to_string(),
            plural: "ounces".to_string(),
            abbreviation: "oz".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 28.349523125,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Gramm,
                    exp: 1,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "pound".to_string(),
            plural: "pounds".to_string(),
            abbreviation: "lb".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 453.59237,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Gramm,
                    exp: 1,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "gallon".to_string(),
            plural: "gallons".to_string(),
            abbreviation: "gal".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.003785411784,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 3,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "pint".to_string(),
            plural: "pints".to_string(),
            abbreviation: "pt".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.000473176473,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 3,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "square foot".to_string(),
            plural: "square feet".to_string(),
            abbreviation: "sqft".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 0.09290304,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 2,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "acre".to_string(),
            plural: "acres".to_string(),
            abbreviation: "ac".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 4046.8564224,
                units: vec![UnitExp {
                    unit: MetricBaseUnit::Meter,
                    exp: 2,
                }],
            },
            priority: -3.0,
        }
        .create(),
    );

    v.push(
        Unit {
            name: "pound-force".to_string(),
            plural: "pound-force".to_string(),
            abbreviation: "lbf".to_string(),
            valid_names: Vec::new(),
            si: UnitNumber {
                num: 4448.2216152605,
                units: vec![
                    UnitExp {
                        unit: MetricBaseUnit::Gramm,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Meter,
                        exp: 1,
                    },
                    UnitExp {
                        unit: MetricBaseUnit::Second,
                        exp: -2,
                    },
                ],
            },
            priority: -3.0,
        }
        .create(),
    );

    v
}
#[derive(Clone, PartialEq)]
pub struct Unit {
    pub name: String,
    pub plural: String,
    pub abbreviation: String,
    pub valid_names: Vec<String>,
    pub si: UnitNumber,
    pub priority: f32,
}
impl Unit {
    pub fn create(mut self) -> Self {
        self.valid_names.push(self.abbreviation.clone());
        self.valid_names.push(self.name.clone());
        self.valid_names.push(self.plural.clone());
        self
    }
    pub fn add_si_prefixes(self) -> Vec<Self> {
        vec![
            Unit {
                name: format!("yocto{}", &self.name),
                plural: format!("yocto{}", &self.plural),
                abbreviation: format!("y{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("y{}", n),
                        _ => format!("yocto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-24,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("zepto{}", &self.name),
                plural: format!("zepto{}", &self.plural),
                abbreviation: format!("z{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("z{}", n),
                        _ => format!("zepto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-21,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("atto{}", &self.name),
                plural: format!("atto{}", &self.plural),
                abbreviation: format!("a{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("a{}", n),
                        _ => format!("atto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-18,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("fempto{}", &self.name),
                plural: format!("fempto{}", &self.plural),
                abbreviation: format!("f{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("f{}", n),
                        _ => format!("fempto{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-15,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("pico{}", &self.name),
                plural: format!("pico{}", &self.plural),
                abbreviation: format!("p{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("p{}", n),
                        _ => format!("pico{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-12,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("nano{}", &self.name),
                plural: format!("nano{}", &self.plural),
                abbreviation: format!("n{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("n{}", n),
                        _ => format!("nano{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-9,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("micro{}", &self.name),
                plural: format!("micro{}", &self.plural),
                abbreviation: format!("µ{}", &self.abbreviation),
                valid_names: vec![
                    format!("mu{}", &self.abbreviation),
                    format!("micro{}", &self.plural),
                    format!("micro{}", &self.name),
                ],
                si: UnitNumber {
                    num: self.si.num * 1e-6,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("milli{}", &self.name),
                plural: format!("milli{}", &self.plural),
                abbreviation: format!("m{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("m{}", n),
                        _ => format!("milli{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-3,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("centi{}", &self.name),
                plural: format!("centi{}", &self.plural),
                abbreviation: format!("c{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("c{}", n),
                        _ => format!("centi{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-2,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("deci{}", &self.name),
                plural: format!("deci{}", &self.plural),
                abbreviation: format!("d{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("d{}", n),
                        _ => format!("deci{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e-1,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("{}", &self.name),
                plural: format!("{}", &self.plural),
                abbreviation: format!("{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("{}", n),
                        _ => format!("{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("kilo{}", &self.name),
                plural: format!("kilo{}", &self.plural),
                abbreviation: format!("k{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("k{}", n),
                        _ => format!("kilo{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e3,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("mega{}", &self.name),
                plural: format!("mega{}", &self.plural),
                abbreviation: format!("M{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("M{}", n),
                        _ => format!("mega{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e6,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("giga{}", &self.name),
                plural: format!("giga{}", &self.plural),
                abbreviation: format!("G{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("G{}", n),
                        _ => format!("giga{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e9,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("tera{}", &self.name),
                plural: format!("tera{}", &self.plural),
                abbreviation: format!("T{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("T{}", n),
                        _ => format!("tera{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e12,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("peta{}", &self.name),
                plural: format!("peta{}", &self.plural),
                abbreviation: format!("P{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("P{}", n),
                        _ => format!("peta{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e15,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("exa{}", &self.name),
                plural: format!("exa{}", &self.plural),
                abbreviation: format!("E{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("E{}", n),
                        _ => format!("exa{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e18,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("zetta{}", &self.name),
                plural: format!("zetta{}", &self.plural),
                abbreviation: format!("Z{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("Z{}", n),
                        _ => format!("zetta{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e21,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
            Unit {
                name: format!("yotta{}", &self.name),
                plural: format!("yotta{}", &self.plural),
                abbreviation: format!("Y{}", &self.abbreviation),
                valid_names: self
                    .valid_names
                    .iter()
                    .map(|n| match n.len() {
                        ..4 => format!("Y{}", n),
                        _ => format!("yotta{}", n),
                    })
                    .collect::<Vec<String>>(),
                si: UnitNumber {
                    num: self.si.num * 1e24,
                    units: self.si.units.clone(),
                },
                priority: self.priority,
            },
        ]
    }
}
