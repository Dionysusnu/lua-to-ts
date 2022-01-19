use crate::prelude::*;

pub fn transform_string(number: &tokenizer::TokenReference) -> Expr {
	let content = if let tokenizer::TokenType::StringLiteral {
		literal,
		multi_line: _,
		quote_type: _,
	} = number.token().token_type()
	{
		literal.as_str()
	} else {
		unreachable!("Must be tring literal");
	};
	Expr::Lit(Lit::Str(make_string(content)))
}
