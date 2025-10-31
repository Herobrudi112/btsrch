#[derive(Clone, PartialEq)]
pub struct UnitNumber {
    pub num: f64,
    pub units: Vec<UnitExp>,
}
impl ToString for UnitNumber {
    fn to_string(&self) -> String {
        let upos = self
            .units
            .clone()
            .iter()
            .filter(|a| a.exp > 0)
            .map(|a| unit_exp_to_superscript_exp(a))
            .collect::<Vec<String>>()
            .join("");
        let udiv = self
            .units
            .clone()
            .iter()
            .filter(|a| a.exp < 0)
            .map(|a| {
                unit_exp_to_superscript_exp(&UnitExp {
                    exp: -a.exp,
                    unit: a.unit.clone(),
                })
            })
            .collect::<Vec<String>>()
            .join("");
        if self.num==1.0{
            if upos.len() == 0 {
                if udiv.len() == 0 {
                    format!("")
                } else {
                    format!("1/{udiv}")
                }
            } else {
                if udiv.len() == 0 {
                    format!("{upos}")
                } else {
                    format!("{upos}/{udiv}")
                }
            }
        }else{
            if upos.len() == 0 {
                if udiv.len() == 0 {
                    format!("{}",self.num)
                } else {
                    format!("{} 1/{udiv}",self.num)
                }
            } else {
                if udiv.len() == 0 {
                    format!("{} {upos}",self.num)
                } else {
                    format!("{} {upos}/{udiv}",self.num)
                }
            }
        }
    }
}
pub fn unit_exp_to_superscript_exp(input: &UnitExp) -> String {
    format!(
        "{}{}",
        input.unit.to_string(),
        if input.exp == 1 {
            String::new()
        } else {
            superscript(input.exp.to_string())
        }
    )
}
pub fn superscript(input: String) -> String {
    input
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '-' => '⁻',
            _ => panic!(),
        })
        .collect::<String>()
}
#[derive(Clone, PartialEq)]
pub struct UnitExp {
    pub exp: i64,
    pub unit: MetricBaseUnit,
}
impl ToString for UnitExp {
    fn to_string(&self) -> String {
        format!("{}^{}", self.unit.to_string(), self.exp)
    }
}
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum MetricBaseUnit {
    Meter,
    Gramm,
    Second,
    Ampere,
    Kelvin,
    Candela,
    Byte,
}
impl ToString for MetricBaseUnit {
    fn to_string(&self) -> String {
        match self {
            MetricBaseUnit::Meter => "m",
            MetricBaseUnit::Gramm => "g",
            MetricBaseUnit::Second => "s",
            MetricBaseUnit::Ampere => "A",
            MetricBaseUnit::Kelvin => "°K",
            MetricBaseUnit::Candela => "cd",
            MetricBaseUnit::Byte => "B",
        }
        .to_string()
    }
}
