use arboard::Clipboard;
use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{
    query_manager::{ListEntry, QueryParser},
    unit_calc_parser::{
        lexer::{get_units, lex},
        parser::{UnitCalculation, parse_unit_conversion},
        unit_number_parser::superscript,
    },
};

#[derive(Clone)]
pub struct UnitCalcParser {}
impl Default for UnitCalcParser {
    fn default() -> Self {
        Self {}
    }
}
#[async_trait]
impl QueryParser for UnitCalcParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>)->Option<()> {
        let len = query.len();
        let (text, priority) = match execute_unit_str(query) {
            Ok(v) => {
                (v, (len as f32))
            },
            Err(e) => (format!("error: {e}"),-1.0),
        };
        let text2 = text.clone();
        resopnse
            .send(ListEntry {
                layout_fn: Box::new(move |ui| {
                    ui.label(format!("{}", &text));
                }),
                execute: Some(Box::new(move || {
                    Clipboard::new().unwrap().set_text(&text2).unwrap();
                })),
                priority: priority,
            })
            .await.ok()?;
        None
    }
}
pub fn execute_unit_str(input: String) -> Result<String, String> {
    let units = get_units();
    let tokens = lex(input, &units).ok_or("lexing failed!".to_string())?;
    let ast = parse_unit_conversion(tokens)?;
    let (un, mut u, tu) = ast.execute()?;
    let mut exponent = 1;
    if u.is_none()&&tu.is_none() {
        let mut best_score = f64::NEG_INFINITY;
        for unit in units {
            let cleaned = unit.si.cleaned();
            if let Some(log) = un.log(&cleaned) {
                let nun = cleaned.pow_i64(log);
                let unit_number = format!(
                    "{:.5}",
                    UnitCalculation::Div(
                        Box::new(UnitCalculation::Number(un.clone())),
                        Box::new(UnitCalculation::Number(nun.clone()))
                    )
                    .execute()
                    .unwrap()
                    .num
                )
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();
                let mut score = (-(unit_number.len() as f64)
                    - (unit_number.len() as f64
                        - unit_number
                            .chars()
                            .enumerate()
                            .find(|x| x.1 == '.')
                            .unwrap_or((unit_number.to_string().len(), '.'))
                            .0 as f64))
                    + (unit.priority as f64);
                if log != 1 {
                    score -= 0.1;
                }
                if score > best_score && unit_number != "0".to_string() {
                    u = Some(unit);
                    best_score = score;
                    exponent = log;
                }
            }
        }
    }
    if let Some(u) = u {
        let unum=UnitCalculation::Div(
                Box::new(UnitCalculation::Number(un.clone())),
                Box::new(UnitCalculation::Number(u.si.pow_i64(exponent).clone()))
            )
            .execute()
            .unwrap();
        if unum.units.len()>0{
            return Err("incompatible target unit".to_string());
        }
        let unit_number = format!(
            "{:.5}",
            unum
            .num
        )
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
    let mut uname = u.plural;
    if unit_number == "1".to_string() {
            uname = u.name;
        }
        if exponent == 1 {
            Ok(format!("{} {}", unit_number, uname))
        } else {
            Ok(format!(
                "{} {}{}",
                unit_number,
                uname,
                superscript(exponent.to_string())
            ))
        }
    }else if let Some(tu)=tu{
        let unum=UnitCalculation::Div(
                Box::new(UnitCalculation::Number(un.clone())),
                Box::new(UnitCalculation::Number(tu.0.clone()))
            )
            .execute()
            .unwrap();
        if unum.units.len()>0{
            return Err("incompatible target unit".to_string());
        }
        let unit_number = format!(
            "{:.5}",
            unum
            .num
        )
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
        Ok(format!("{unit_number} {}",tu.1))
    } else {
        Ok(un.to_string())
    }
}
