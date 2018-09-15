#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::prelude::*;

fn threading_hint(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(JsNumber::new(&mut cx, num_cpus::get() as f64))
}

register_module!(mut cx, {
    cx.export_function("threading_hint", threading_hint)
});
