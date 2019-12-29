//! Has all the String Parsers

use nom::{
	branch::alt,
	bytes::complete::is_not,
	character::complete::char,
	sequence::*,
	IResult,
};

use super::types::{
	StringType,
	TokenKind,
};

/// Get a complete normal string as StringType
pub fn normal_string(input: &str) -> IResult<&str, StringType> {
	let (input, parsed) = delimited(char('"'), is_not("\""), char('"'))(input)?;

	return Ok((input, StringType::NormalString(parsed.to_owned())));
}

// Get a complete Template String as StringType
pub fn template_string(input: &str) -> IResult<&str, StringType> {
	let (input, parsed) = delimited(char('`'), is_not("`"), char('`'))(input)?;
	return Ok((input, StringType::TemplateString(parsed.to_owned())));
}

// Get any string as TokenKind
pub fn any_string(input: &str) -> IResult<&str, TokenKind> {
	let (input, parsed) = alt((normal_string, template_string))(input)?;

	return Ok((input, TokenKind::FString(parsed)));
}

#[cfg(test)]
mod tests {
	use super::{
		any_string,
		normal_string,
		template_string,
	};
	use super::{
		StringType,
		TokenKind,
	};

	#[test]
	/// Test for "normal_string"
	fn test_normal_string() {
		assert_eq!(
			normal_string("\"Hello World\"").unwrap(),
			("", StringType::NormalString("Hello World".to_owned()))
		);
	}

	#[test]
	/// Test for "template_string"
	fn test_template_string() {
		assert_eq!(
			template_string("`Hello World`").unwrap(),
			("", StringType::TemplateString("Hello World".to_owned()))
		);
	}

	#[test]
	/// Test for "any_string"
	fn test_any_string() {
		assert_eq!(
			any_string("\"Hello World\"").unwrap(),
			(
				"",
				TokenKind::FString(StringType::NormalString("Hello World".to_owned()))
			)
		);

		assert_eq!(
			any_string("`Hello World`").unwrap(),
			(
				"",
				TokenKind::FString(StringType::TemplateString("Hello World".to_owned()))
			)
		);
	}
}
