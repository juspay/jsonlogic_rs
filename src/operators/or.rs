use serde_json::Value;

use super::{logic, Ambiguous, Data, Expression, PartialResult};

/// Takes an arbitrary number of arguments. Returns the first truthy argument or the last
/// argument.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let args = args.iter().map(|arg| arg.compute(data));
    let mut last = None;

    for arg in args {
        if logic::is_truthy(&arg) {
            return arg;
        }

        last = Some(arg);
    }

    last.unwrap_or(Value::Null)
}

// for Ambiguous results, returns Ambiguous when no there are no true results
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let args = args.iter().map(|arg| arg.partial_compute(data));
    let mut last = None;

    let mut is_ambiguous = false;

    for arg in args {
        match arg {
            Err(Ambiguous) => is_ambiguous = true,
            Ok(arg) => {
                if logic::is_truthy(&arg) {
                    return Ok(arg);
                }
                last = Some(arg);
            }
        }
    }

    if is_ambiguous {
        return Err(Ambiguous);
    }

    Ok(last.unwrap_or(Value::Null))
}
