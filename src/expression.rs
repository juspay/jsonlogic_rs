use crate::operators::Operator;
use crate::Data;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Ambiguous;

pub type PartialResult = Result<Value, Ambiguous>;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Constant(&'a Value),
    Computed(Operator, Vec<Expression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn from_json(json: &Value) -> Result<Expression, String> {
        if !json.is_object() {
            return Ok(Expression::Constant(&json));
        }

        let object = json.as_object().unwrap();
        // If this object has more than one key-value pair, we will return it as is. This replicates
        // the behaviour of the javascript implementation.
        if object.len() != 1 {
            return Ok(Expression::Constant(json));
        }

        let entry: Vec<(&String, &serde_json::Value)> = object.iter().collect();
        let &(operator_key, value) = entry.get(0).unwrap();
        let operator = Operator::from_str(operator_key)
            .ok_or_else(|| format!("Unrecognized operation {}", operator_key))?;

        let arguments: Vec<_> = match value {
            Value::Array(arr) => arr.iter().map(|expr| Expression::from_json(expr)).collect(),
            // Interpret as an empty array.
            Value::Null => Ok(vec![]),
            // If the value is not an array we can only assume that this is a shorthand.
            _ => Expression::from_json(value).and_then(|expr| Ok(vec![expr])),
        }?;

        Ok(Expression::Computed(operator, arguments))
    }

    /// Computes the expression and returns value it evaluates to.
    pub fn compute(&self, data: &Data) -> Value {
        match self {
            Expression::Constant(value) => (*value).clone(),
            Expression::Computed(operator, args) => operator.compute(args, data),
        }
    }

    pub fn partial_compute(&self, data: &Data) -> PartialResult {
        match self {
            Expression::Constant(value) => Ok((*value).clone()),
            Expression::Computed(operator, args) => operator.partial_compute(args, data),
        }
    }

    /// Returns a set that contains all variable names that occure in this expression and its child
    /// expressions. Errors if a variable operator
    ///
    /// - has not a string as its argument (TODO: numbers are ok for when data is an array)
    /// - has a non static argument
    ///
    /// While the latter is valid for computation, it is currently not implemented to analyze the
    /// variable name for that.
    pub fn get_variable_names(&self) -> Result<HashSet<String>, String> {
        let mut variable_names: HashSet<String> = HashSet::new();

        self.insert_var_names(&mut variable_names)?;
        Ok(variable_names)
    }

    fn insert_var_names(&self, names: &mut HashSet<String>) -> Result<(), String> {
        match self {
            Expression::Constant(_) => Ok(()),
            Expression::Computed(operator, args) => {
                if let Operator::Variable = operator {
                    let first_expr = args
                        .get(0)
                        .ok_or("found Variable operator without arguments")?;
                    if let Expression::Constant(name_value) = first_expr {
                        let name = name_value
                            .as_str()
                            .ok_or("found Variable operator with non string argument")?;
                        names.insert(name.to_owned());
                        return Ok(());
                    } else {
                        return Err(String::from(
                            "found Variable operator with non static argument",
                        ));
                    }
                }

                // For all other operations analyze the arguments recursive.
                args.iter()
                    .map(|expr| expr.insert_var_names(names))
                    .collect()
            }
        }
    }

    /// Returns a set that contains all variable names and values that occur in this expression and its child
    /// expressions. Errors if a variable operator
    ///
    /// - has not a string as its argument (TODO: numbers are ok for when data is an array)
    /// - has a non static argument
    ///
    pub fn get_variable_names_and_values(
        &self,
    ) -> Result<HashSet<(String, String, Value)>, String> {
        let mut variable_names: HashSet<(String, String, Value)> = HashSet::new();

        self.insert_var_names_and_values(&mut variable_names)?;
        Ok(variable_names)
    }

    fn insert_var_names_and_values(
        &self,
        variable_names: &mut HashSet<(String, String, Value)>,
    ) -> Result<Value, String> {
        match self {
            Expression::Constant(a) => Ok((**a).clone()),
            Expression::Computed(operator, args) => {
                match operator {
                    Operator::Equal
                    | Operator::StrictEqual
                    | Operator::Negation
                    | Operator::DoubleNegation
                    | Operator::In
                    | Operator::NotEqual
                    | Operator::LessEqualThan
                    | Operator::LessThan
                    | Operator::GreaterEqualThan
                    | Operator::GreaterThan
                    | Operator::JuspayVerEq
                    | Operator::JuspayVerGt
                    | Operator::JuspayVerLt
                    | Operator::JuspayVerGtEq
                    | Operator::JuspayVerLtEq => {
                        let op = format!("{:?}", operator);
                        let present_value = args
                            .last()
                            .ok_or("Found variable operator without value")?
                            .insert_var_names_and_values(variable_names)?;
                        if let Expression::Computed(Operator::Variable, next_arg) =
                            args.get(0).ok_or("Equality operator without arguments")?
                        {
                            let first_expr = next_arg
                                .get(0)
                                .ok_or("found Variable operator without arguments")?;
                            match first_expr {
                                Expression::Constant(name) => variable_names.insert((
                                    name.as_str()
                                        .ok_or("found Variable operator with non string argument")?
                                        .to_owned(),
                                    op,
                                    present_value.clone(),
                                )),
                                _ => {
                                    return Err(String::from(
                                        "found Variable operator with non static argument",
                                    ))
                                }
                            };
                        }
                        Ok(present_value)
                    }
                    _ => {
                        // For all other operations analyze the arguments recursive.
                        args.iter()
                            .map(|expr| expr.insert_var_names_and_values(variable_names))
                            .collect()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expression::*;
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_to_ast() {
        assert_eq!(
            Expression::from_json(&json!({ "==": null })).unwrap(),
            Expression::Computed(Operator::Equal, vec![])
        );

        assert_eq!(
            Expression::from_json(&json!({ "==": [] })).unwrap(),
            Expression::Computed(Operator::Equal, vec![])
        );

        assert_eq!(
            Expression::from_json(&json!({ "==": [1] })).unwrap(),
            Expression::Computed(Operator::Equal, vec![Constant(&json!(1))])
        );

        assert_eq!(
            Expression::from_json(&json!({ "==": [1, 2] })).unwrap(),
            Expression::Computed(
                Operator::Equal,
                vec![Constant(&json!(1)), Constant(&json!(2))]
            )
        );

        assert_eq!(
            Expression::from_json(&json!({"!=": [5, 2]})).unwrap(),
            Expression::Computed(
                Operator::NotEqual,
                vec![Constant(&json!(5)), Constant(&json!(2))]
            )
        );

        assert_eq!(
            Expression::from_json(&json!({"var": ["foo"]})).unwrap(),
            Expression::Computed(Operator::Variable, vec![Constant(&json!("foo"))])
        );

        assert_eq!(
            Expression::from_json(&json!({"==": [{"var": ["foo"]}, "foo"]})).unwrap(),
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(Operator::Variable, vec![Constant(&json!("foo"))]),
                    Expression::Constant(&json!("foo"))
                ]
            )
        );
    }

    #[test]
    fn get_variable_names_error() {
        assert_eq!(
            Expression::Computed(
                Operator::Variable,
                vec![Expression::Computed(
                    Operator::Variable,
                    vec![Expression::Constant(&json!("foo"))]
                )]
            )
            .get_variable_names(),
            Err(String::from(
                "found Variable operator with non static argument"
            ))
        );

        assert_eq!(
            Expression::Computed(Operator::Variable, vec![Expression::Constant(&json!(1))])
                .get_variable_names(),
            Err(String::from(
                "found Variable operator with non string argument"
            ))
        );

        assert_eq!(
            Expression::Computed(Operator::Variable, vec![]).get_variable_names(),
            Err(String::from("found Variable operator without arguments"))
        );
    }

    #[test]
    fn get_variable_names() {
        assert_eq!(
            Expression::Constant(&json!("foo")).get_variable_names(),
            Ok(HashSet::new())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Variable,
                vec![Expression::Constant(&json!("foo"))]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Constant(&json!("a value")),
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    )
                ]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    ),
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    )
                ]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("bar"))]
                    ),
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Constant(&json!("foo"))]
                    )
                ]
            )
            .get_variable_names(),
            Ok(["foo".to_owned(), "bar".to_owned()]
                .iter()
                .cloned()
                .collect::<HashSet<_>>())
        );
    }

    #[test]
    fn get_variable_names_and_values_error() {
        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(
                        Operator::Variable,
                        vec![Expression::Computed(
                            Operator::Variable,
                            vec![Expression::Constant(&json!("foo"))]
                        )]
                    ),
                    Expression::Constant(&json!("bar"))
                ]
            )
            .get_variable_names_and_values(),
            Err(String::from(
                "found Variable operator with non static argument"
            ))
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(Operator::Variable, vec![Expression::Constant(&json!(1))]),
                    Expression::Constant(&json!("bar"))
                ]
            )
            .get_variable_names_and_values(),
            Err(String::from(
                "found Variable operator with non string argument"
            ))
        );

        assert_eq!(
            Expression::Computed(
                Operator::Equal,
                vec![
                    Expression::Computed(Operator::Variable, vec![]),
                    Expression::Constant(&json!("bar"))
                ]
            )
            .get_variable_names_and_values(),
            Err(String::from("found Variable operator without arguments"))
        );
    }

    #[test]
    fn get_variable_names_and_values() {
        assert_eq!(
            Expression::Constant(&json!("foo")).get_variable_names_and_values(),
            Ok(HashSet::new())
        );

        assert_eq!(
            Expression::Computed(
                Operator::Variable,
                vec![Expression::Constant(&json!("foo"))]
            )
            .get_variable_names(),
            Ok(["foo".to_owned()].iter().cloned().collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::And,
                vec![Expression::Computed(
                    Operator::Equal,
                    vec![
                        Expression::Computed(
                            Operator::Variable,
                            vec![Expression::Constant(&json!("foo"))]
                        ),
                        Expression::Constant(&json!(2))
                    ]
                )]
            )
            .get_variable_names_and_values(),
            Ok([("foo".to_owned(), "Equal".to_owned(), json!(2))]
                .iter()
                .cloned()
                .collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::And,
                vec![
                    Expression::Computed(
                        Operator::Equal,
                        vec![
                            Expression::Computed(
                                Operator::Variable,
                                vec![Expression::Constant(&json!("foo"))]
                            ),
                            Expression::Constant(&json!(2))
                        ]
                    ),
                    Expression::Computed(
                        Operator::Equal,
                        vec![
                            Expression::Computed(
                                Operator::Variable,
                                vec![Expression::Constant(&json!("foo"))]
                            ),
                            Expression::Constant(&json!(2))
                        ]
                    )
                ]
            )
            .get_variable_names_and_values(),
            Ok([("foo".to_owned(), "Equal".to_owned(), json!(2))]
                .iter()
                .cloned()
                .collect::<HashSet<_>>())
        );

        assert_eq!(
            Expression::Computed(
                Operator::And,
                vec![
                    Expression::Computed(
                        Operator::Equal,
                        vec![
                            Expression::Computed(
                                Operator::Variable,
                                vec![Expression::Constant(&json!("foo"))]
                            ),
                            Expression::Constant(&json!(2))
                        ]
                    ),
                    Expression::Computed(
                        Operator::Equal,
                        vec![
                            Expression::Computed(
                                Operator::Variable,
                                vec![Expression::Constant(&json!("bar"))]
                            ),
                            Expression::Constant(&json!(5))
                        ]
                    )
                ]
            )
            .get_variable_names_and_values(),
            Ok([
                ("foo".to_owned(), "Equal".to_owned(), json!(2)),
                ("bar".to_owned(), "Equal".to_owned(), json!(5))
            ]
            .iter()
            .cloned()
            .collect::<HashSet<_>>())
        );
    }

    mod compute {
        use super::*;

        #[test]
        fn constant_expression() {
            assert_eq!(Constant(&json!(1)).compute(&Data::empty()), json!(1));
        }

        #[test]
        fn equal() {
            assert_eq!(
                Computed(Operator::Equal, vec![]).compute(&Data::empty()),
                json!(true)
            );
            assert_eq!(
                Computed(Operator::Equal, vec![Constant(&json!(null))]).compute(&Data::empty()),
                json!(true)
            );
            assert_eq!(
                Computed(
                    Operator::Equal,
                    vec![Constant(&json!(1)), Constant(&json!(1))]
                )
                .compute(&Data::empty()),
                json!(true)
            );
            assert_eq!(
                Computed(
                    Operator::Equal,
                    vec![Constant(&json!(1)), Constant(&json!(2))]
                )
                .compute(&Data::empty()),
                json!(false)
            );
        }
    }
}
