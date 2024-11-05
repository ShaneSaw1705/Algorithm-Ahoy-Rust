use rustpython_vm as vm;
use rustpython_vm::builtins::PyBaseExceptionRef;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_code(code: &str) -> Result<String, JsValue> {
    // Enter the interpreter without the standard library
    vm::Interpreter::without_stdlib(Default::default())
        .enter(
            |vm: &vm::VirtualMachine| -> Result<String, PyBaseExceptionRef> {
                let scope = vm.new_scope_with_builtins();

                let code_obj = vm
                    .compile(code, vm::compiler::Mode::Exec, "<embedded>".to_owned())
                    .map_err(|err| vm.new_syntax_error(&err, Some(code)))?;

                vm.run_code_obj(code_obj, scope)?;

                Ok("Code executed successfully.".to_string())
            },
        )
        .map_err(|err: PyBaseExceptionRef| {
            JsValue::from_str(&format!("Error during execution: {:?}", err))
        })
}
