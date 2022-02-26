use crate::prelude::*;

fn get_unpack_data(original_expression: &lua_ast::Expression) -> (Option<()>, Expr) {
	let default = || (None, transform_expression(original_expression));
	let value = match original_expression {
		lua_ast::Expression::Value {
			value,
			type_assertion: _,
		} => value,
		_ => return default(),
	};
	let function_call = match **value {
		lua_ast::Value::FunctionCall(ref function_call) => function_call,
		_ => return default(),
	};
	let token = match function_call.prefix() {
		lua_ast::Prefix::Name(token) => token,
		_ => return default(),
	};
	let call = if token.token().to_string() == "unpack" {
		function_call.suffixes().next().unwrap()
	} else {
		return default();
	};
	let function_args = match call {
		lua_ast::Suffix::Call(lua_ast::Call::AnonymousCall(function_args)) => function_args,
		_ => unreachable!(),
	};
	(
		Some(()),
		match function_args {
			lua_ast::FunctionArgs::Parentheses {
				arguments,
				parentheses: _,
			} => transform_expression(arguments.iter().next().unwrap()),
			lua_ast::FunctionArgs::TableConstructor(table_constructor) => {
				transform_table_constructor(table_constructor)
			}
			_ => skip("Unknown function args type", function_args),
		},
	)
}

pub fn transform_table_constructor(args: &lua_ast::TableConstructor) -> Expr {
	let is_array = args
		.fields()
		.iter()
		.all(|field| matches!(field, lua_ast::Field::NoKey(_)));
	let is_object = args.fields().iter().all(|field| {
		matches!(field, lua_ast::Field::NameKey { .. })
			|| matches!(
				field,
				lua_ast::Field::ExpressionKey {
					key: lua_ast::Expression::Value {
						value,
						type_assertion: _,
					},
					..
				} if matches!(**value, lua_ast::Value::Number(_))
			)
	});
	if is_object {
		Expr::Object(ObjectLit {
			span: Default::default(),
			props: args
				.fields()
				.iter()
				.map(|field| {
					PropOrSpread::Prop(boxed(Prop::KeyValue(
						if let lua_ast::Field::NameKey {
							key,
							value,
							equal: _,
						} = field
						{
							KeyValueProp {
								key: PropName::Str(make_string(&key.token().to_string())),
								value: boxed(transform_expression(value)),
							}
						} else if let lua_ast::Field::ExpressionKey {
							key:
								lua_ast::Expression::Value {
									value: key,
									type_assertion: _,
								},
							value,
							equal: _,
							brackets: _,
						} = field
						{
							let key = if let lua_ast::Value::Number(ref key) = **key {
								if let tokenizer::TokenType::Number { text } = key.token_type() {
									text
								} else {
									unreachable!("Value::Number only has TokenType::Number")
								}
							} else {
								unreachable!("Checked to be Value::Number earlier")
							};
							KeyValueProp {
								key: PropName::Num(Number::from(
									// TODO: See if full_moon provides this without manual parsing
									// BUG: parse will panic on lua syntax like `1_000` or `0b0000`
									key.to_string().parse::<f64>().unwrap(),
								)),
								value: boxed(transform_expression(value)),
							}
						} else {
							unreachable!("Checked to be Field::NameKey earlier")
						},
					)))
				})
				.collect(),
		})
	} else if is_array {
		Expr::Array(ArrayLit {
			span: Default::default(),
			elems: args
				.fields()
				.iter()
				.map(|field| {
					if let lua_ast::Field::NoKey(original_expression) = field {
						let (spread, expression) = get_unpack_data(original_expression);
						Some(ExprOrSpread {
							// Detect usage of `unpack` in table constructor
							// This needs to convert to the `...` spread operator in TS
							spread: spread.map(|()| Default::default()),
							expr: boxed(expression),
						})
					} else {
						unreachable!("Checked to be all NoKey variant earlier")
					}
				})
				.collect(),
		})
	} else {
		skip("Mixed tables do not exist in TS", args)
	}
}
