#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, Throw};
use neon::js::{JsNumber, JsArray, JsValue, Object};
use neon::mem::Handle;

fn add(call: Call) -> JsResult<JsNumber> {
    let a = call.arguments
        .require(call.scope, 0)? // neon::vm::Arguments::require
        .check::<JsNumber>()?    // neon::mem::Handle::check
        .value();                // neon::js::JsNumber::value
    let b = call.arguments
        .require(call.scope, 1)?
        .check::<JsNumber>()?
        .value();
    Ok(JsNumber::new(call.scope, a + b))
}

fn max(call: Call) -> JsResult<JsNumber> {
    let nums: Vec<f64> = call.arguments
        .require(call.scope, 0)?
        .check::<JsArray>()?
        .to_vec(call.scope)?
        .into_iter()
        .map(|x: Handle<JsValue>| -> f64 { x.check::<JsNumber>().unwrap().value() })
        .collect();

    if nums.len() == 0 {
        return Err(Throw);
    }

    let mut max = nums[0];
    for x in nums {
        if x > max {
            max = x;
        }
    }
    Ok(JsNumber::new(call.scope, max))
}

fn larger_than(call: Call) -> JsResult<JsArray> {
    let num = call.arguments
        .require(call.scope, 0)?
        .check::<JsNumber>()?
        .value();
    let nums: Vec<f64> = call.arguments
        .require(call.scope, 1)?
        .check::<JsArray>()?
        .to_vec(call.scope)?
        .into_iter()
        .map(|x: Handle<JsValue>| -> f64 { x.check::<JsNumber>().unwrap().value() })
        .collect();
    let filtered = nums.into_iter().filter(|x| x > &num).collect::<Vec<f64>>();

    // Create JsArray to return
    let arr: Handle<JsArray> = JsArray::new(call.scope, filtered.len() as u32);
    for i in 0..filtered.len() {
        arr.set(i as u32, JsNumber::new(call.scope, filtered[i])).unwrap(); // neon::js::Key::set
    }
    Ok(arr)
}

register_module!(m, {
    m.export("add", add)?;
    m.export("max", max)?;
    m.export("larger_than", larger_than)?;
    Ok(())
});
