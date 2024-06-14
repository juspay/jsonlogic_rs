use regex::Regex;
use serde_json::{json, Value};

use super::{logic, Data, Expression, PartialResult};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return json!(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return json!(false),
    };

    let mut pattern = logic::coerce_to_str(&b);
    if let Some(Value::String(mut flags)) = args.get(2).map(|arg| arg.compute(data)) {
        if let Some(g) = flags.find("g") {
            flags.remove(g);
        };
        if flags.len() > 0 {
            pattern = format!("(?{}){}", flags, pattern);
        }
    }

    match Regex::new(&pattern) {
        Ok(re) => {
            let text = &logic::coerce_to_str(&a);
            serde_json::to_value(re.is_match(text)).unwrap_or(json!(false))
        }
        Err(_) => json!(false),
    }
}

// early returns on finding either Ambiguous arg
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let a = match args.get(0) {
        Some(arg) => arg.partial_compute(data)?,
        None => return Ok(json!(false)),
    };

    let b = match args.get(1) {
        Some(arg) => arg.partial_compute(data)?,
        None => return Ok(json!(false)),
    };

    let mut pattern = logic::coerce_to_str(&b);
    if let Some(Value::String(mut flags)) = args
        .get(2)
        .map(|arg| arg.partial_compute(data))
        .transpose()?
    {
        if let Some(g) = flags.find("g") {
            flags.remove(g);
        };
        if flags.len() > 0 {
            pattern = format!("(?{}){}", flags, pattern);
        }
    }

    match Regex::new(&pattern) {
        Ok(re) => {
            let text = &logic::coerce_to_str(&a);
            Ok(serde_json::to_value(re.is_match(text)).unwrap_or(json!(false)))
        }
        Err(_) => Ok(json!(false)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn regex_result() {
        assert_eq!(compute_const!(), json!(false));
        assert_eq!(compute_const!(json!("test")), json!(false));
        assert_eq!(compute_const!(json!("test"), json!("test")), json!(true));
        assert_eq!(compute_const!(json!("super"), json!("test")), json!(false));
        assert_eq!(
            compute_const!(json!("TESTabctest"), json!("test"), json!("i")),
            json!(true)
        );
        assert_eq!(
            compute_const!(json!("abctestabctest"), json!("test"), json!("")),
            json!(true)
        );
        assert_eq!(
            compute_const!(json!("testabctest"), json!("(test)abc")),
            json!(true)
        );
        assert_eq!(
            compute_const!(json!("abctestabctest"), json!("^test")),
            json!(false)
        );
        assert_eq!(
            compute_const!(json!("testabctest"), json!("^test")),
            json!(true)
        );
    }
}
