use crate::prelude::*;

// Taken and adapted from StyLua under the MPL-2.0 license
// https://github.com/JohnnyMorganz/StyLua/blob/7efc7cbd91a4b8235ea4e8c6b07b7028fc1534b7/src/verify_ast.rs#L129-L147
pub fn transform_number(token: &tokenizer::TokenReference) -> Expr {
	let text = match token.token_type() {
		tokenizer::TokenType::Number { text } => text,
		_ => return skip("transform_number token was not TokenType::Number", token),
	};
	let text = text.replace('_', "");
	let number = match text.as_str().parse::<f64>() {
		Ok(num) => num,
		// Try parsing as Hex (0x)
		Err(_) => match i32::from_str_radix(&text[2..], 16) {
			Ok(num) => num.into(),
			// Try parsing as binary (0b)
			Err(_) => match i32::from_str_radix(&text.as_str()[2..], 2) {
				Ok(num) => num.into(),
				// Will have been full_moon tokenizer error
				Err(_) => unreachable!(),
			},
		},
	};
	Expr::Lit(Lit::Num(Number {
		span: Default::default(),
		value: number,
	}))
}
