use crate::prelude::*;

pub fn transform_table_constructor(args: &lua_ast::TableConstructor) -> Expr {
	let is_array = args
		.fields()
		.iter()
		.all(|field| matches!(field, lua_ast::Field::NoKey(_)));
	let is_object = args
		.fields()
		.iter()
		.all(|field| matches!(field, lua_ast::Field::NameKey { .. }));
	if is_object {
		Expr::Object(ObjectLit {
			span: Default::default(),
			props: args
				.fields()
				.iter()
				.map(|field| {
					if let lua_ast::Field::NameKey {
						key,
						value,
						equal: _,
					} = field
					{
						PropOrSpread::Prop(boxed(Prop::KeyValue(KeyValueProp {
							key: PropName::Str(make_string(&key.token().to_string())),
							value: boxed(transform_expression(value)),
						})))
					} else {
						unreachable!("Checked to be all NameKey variant earlier")
					}
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
					if let lua_ast::Field::NoKey(mut expression) = field.clone() {
						Some(ExprOrSpread {
							// Detect usage of `unpack` in table constructor
							// This needs to convert to the `...` spread operator in TS
							spread: if let lua_ast::Expression::Value {
								value,
								type_assertion: _,
							} = expression.clone()
							{
								if let lua_ast::Value::FunctionCall(function_call) = *value.clone()
								{
									if let lua_ast::Prefix::Name(token) = function_call.prefix() {
										if token.token().to_string() == "unpack" {
											let call = function_call.suffixes().next().unwrap();
											if let lua_ast::Suffix::Call(
												lua_ast::Call::AnonymousCall(function_args),
											) = call
											{
												expression = match function_args {
													lua_ast::FunctionArgs::Parentheses {
														arguments,
														parentheses: _,
													} => arguments.iter().next().unwrap().clone(),
													lua_ast::FunctionArgs::TableConstructor(
														table_constructor,
													) => lua_ast::Expression::Value {
														value: boxed(
															lua_ast::Value::TableConstructor(
																table_constructor.clone(),
															),
														),
														type_assertion: None,
													},
													_ => unreachable!("Unknown function args type"),
												};
												Some(Default::default())
											} else {
												unreachable!()
											}
										} else {
											None
										}
									} else {
										None
									}
								} else {
									None
								}
							} else {
								None
							},
							expr: boxed(transform_expression(&expression)),
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
