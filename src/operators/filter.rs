use serde_json::Value;

use super::{logic, Ambiguous, Data, Expression, PartialResult};

/// You can use `filter` to keep only elements of the array that pass a test. Note, that inside
/// the logic being used to map, var operations are relative to the array element being worked
/// on.
///
/// Also note, the returned array will have contiguous indexes starting at zero (typical for
/// JavaScript, Python and Ruby) it will not preserve the source indexes (making it unlike
/// PHP’s array_filter).
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let arr = match args.get(0).map(|arg| arg.compute(data)) {
        Some(Value::Array(arr)) => arr,
        _ => Vec::with_capacity(0),
    };
    let op = match args.get(1) {
        Some(expr) => expr,
        None => &Expression::Constant(&Value::Null),
    };

    let mut result = Vec::new();
    for elem in arr.iter() {
        let include = op.compute(&Data::from_json(elem));
        if logic::is_truthy(&include) {
            result.push(elem.clone());
        }
    }

    Value::Array(result)
}

// early returns on finding Ambiguous first arg and for Ambiguous results, returns Ambiguous when no there are no false results
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let arr = match args
        .get(0)
        .map(|arg| arg.partial_compute(data))
        .transpose()?
    {
        Some(Value::Array(arr)) => arr,
        _ => Vec::with_capacity(0),
    };
    let op = match args.get(1) {
        Some(expr) => expr,
        None => &Expression::Constant(&Value::Null),
    };

    let mut is_ambiguous = false;

    let mut result = Vec::new();
    for elem in arr.iter() {
        match op.partial_compute(&Data::from_json(elem)) {
            Err(Ambiguous) => is_ambiguous = true,
            Ok(include) if logic::is_truthy(&include) => result.push(elem.clone()),
            _ => (),
        }
    }

    if is_ambiguous {
        return Err(Ambiguous);
    }

    Ok(Value::Array(result))
}
