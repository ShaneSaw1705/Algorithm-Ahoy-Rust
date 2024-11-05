use rlua::Lua;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_code(code: &str) -> Result<String, JsValue> {
    let lua = Lua::new();
    match lua.context(|ctx| ctx.load(code).eval::<String>()) {
        Ok(result) => Ok(result),
        Err(err) => Err(JsValue::from_str(&format!("Error: {}", err))),
    }
}
