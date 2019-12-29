use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StringType {
	NormalString(String),
	TemplateString(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FloatType {
	F32(f32),
	F64(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegerType {
	Int8(i8),
	Int16(i16),
	Int32(i32),
	Int64(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokenKind {
	/// > 123456789
	Integer(IntegerType),
	/// > 123.456
	Float(FloatType),

	/// > UNKOWN
	Identifier(String),
	/// > "Hello World"
	FString(StringType),

	/// > (
	OpenParen,
	/// > [
	OpenBracket,
	/// > {
	OpenBrace,

	/// > )
	CloseParen,
	/// > ]
	CloseBracket,
	/// > }
	CloseBrace,

	/// > +
	Plus,
	/// > -
	Minus,
	/// > /
	Slash,
	/// > *
	Asterisk,
	/// > %
	Percent,
	/// > ^
	Carat,

	/// > >
	GreaterThan,
	/// > <
	LowerThan,
	/// > =
	Equals,

	/// > .
	Dot,
	/// > :
	Colon,
	/// > ;
	SemiColon,
}
