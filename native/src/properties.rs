pub extern crate style;
pub extern crate cssparser;
pub extern crate style_traits;
pub extern crate servo_url;

use style::properties::longhands::background_size;
use style::properties::longhands::{
    background_attachment, background_clip, background_color, background_image,
};
use style::properties::longhands::{
    background_origin, background_position_x, background_position_y, background_repeat,
};
use style::properties::shorthands::background;

use style_traits::CssWriter;


use cssparser::{Parser, ParserInput};
use style::context::QuirksMode;
use style::parser::ParserContext;
use style::stylesheets::{CssRuleType, Origin};
use style_traits::{ParseError, ParsingMode, ToCss};

pub fn parse_background(input: &str) -> style::properties::shorthands::background::Longhands {
    return parse(background::parse_value, input).unwrap();
}

fn parse<'a, T, F>(f: F, s: &'a str) -> Result<T, ParseError<'a>>
where
    F: for<'t> Fn(&ParserContext, &mut Parser<'a, 't>) -> Result<T, ParseError<'a>>,
{
    let mut input = ParserInput::new(s);
    parse_input(f, &mut input)
}

fn parse_input<'i: 't, 't, T, F>(f: F, input: &'t mut ParserInput<'i>) -> Result<T, ParseError<'i>>
where
    F: Fn(&ParserContext, &mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
{
    let url = ::servo_url::ServoUrl::parse("http://localhost").unwrap();
    let context = ParserContext::new(
        Origin::Author,
        &url,
        Some(CssRuleType::Style),
        ParsingMode::DEFAULT,
        QuirksMode::NoQuirks,
        None,
        None,
    );
    let mut parser = Parser::new(input);
    f(&context, &mut parser)
}
