use rustpython::InterpreterConfig;
use rustpython::vm::{
    Interpreter, VirtualMachine
};

use rustpython_vm::convert::ToPyResult;


static mut INTERPRETER: Option<Interpreter> = None;

fn init() {
    unsafe {
        INTERPRETER = Some(
            InterpreterConfig::new()
                .init_stdlib()
                .init_hook(Box::new(|vm| {
                    vm.add_native_module("framework", Box::new(framework::module));
                }))
                .interpreter(),
        );
    }
}
fn enter(callback: impl Fn(&VirtualMachine)) {
    unsafe { INTERPRETER.as_mut().unwrap().enter(|vm| callback(vm)); }
}

fn main() {
    init();
    enter(|vm| {
        vm.insert_sys_path(vm.new_pyobj("sample")).expect("failed to add sys.path");
        match vm.import("main", 0) {
            Ok(_) => {}
            Err(e) => {
                let text = e.to_pyresult(vm).unwrap().str(vm).unwrap().to_string();
                println!("{text}")
            }
        }
    })
}
