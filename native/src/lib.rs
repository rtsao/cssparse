mod properties;

use neon::prelude::*;
use style_traits::{ToCss};
use properties::parse_background;

register_module!(mut cx, {
    cx.export_function("expandShorthand", expand_shorthand_js)
});

fn expand_shorthand_js(mut cx: FunctionContext) -> JsResult<JsObject> {
    // cx.check_argument::<JsString>(0)?;
    // cx.check_argument::<JsNumber>(1)?;

    let property = cx.argument::<JsString>(0)?.value(&mut cx);
    // let value = cx.argument::<JsString>(1)?.value(&mut cx);

    match property.as_str() {
        "background" => expand_background_js(cx),
        _ => panic!("property `{}` not supported", property),
    }
}

fn expand_background_js(mut cx: FunctionContext) -> JsResult<JsObject> {
    // let property = cx.argument::<JsString>(0)?.value(&mut cx);
    let value = cx.argument::<JsString>(1)?.value(&mut cx);

    let val = value.to_string();

    let result = parse_background(&val);

    let object = JsObject::new(&mut cx);

    let background_clip = cx.string(result.background_clip.to_css_string());
    object.set(&mut cx, "background-clip", background_clip).unwrap();

    let background_size = cx.string(result.background_size.to_css_string());
    object.set(&mut cx, "background-size", background_size).unwrap();

    let background_image = cx.string(result.background_image.to_css_string());
    object.set(&mut cx, "background-image", background_image).unwrap();

    let background_color = cx.string(result.background_color.to_css_string());
    object.set(&mut cx, "background-color", background_color).unwrap();

    let background_origin = cx.string(result.background_origin.to_css_string());
    object.set(&mut cx, "background-origin", background_origin).unwrap();

    let background_repeat = cx.string(result.background_repeat.to_css_string());
    object.set(&mut cx, "background-repeat", background_repeat).unwrap();

    let background_attachment = cx.string(result.background_attachment.to_css_string());
    object.set(&mut cx, "background-attachment", background_attachment).unwrap();

    let background_position_x = cx.string(result.background_position_x.to_css_string());
    object.set(&mut cx, "background-position-x", background_position_x).unwrap();

    let background_position_y = cx.string(result.background_position_y.to_css_string());
    object.set(&mut cx, "background-position-y", background_position_y).unwrap();

    Ok(object)
}
