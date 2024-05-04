use rustpython::vm::{builtins::PyModule, pymodule, PyRef, VirtualMachine};
use rustpython_vm::AsObject;
use rustpython_vm::function::PySetterValue;

mod foo;

#[pymodule(name = "framework")]
mod _module { use super::pymodule; }

fn nest(vm: &VirtualMachine, root: &PyRef<PyModule>, child: PyRef<PyModule>, name: &str) {
    let child = PySetterValue::Assign(child.into());
    let name = vm.new_pyobj(name).as_interned_str(vm).unwrap();
    root.as_object().generic_setattr(name, child, vm)
        .expect("Failed to nest framework module");
}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    let root = _module::make_module(vm);
    let foo = foo::module(vm);
    nest(vm, &root, foo, "foo");
    return root;
}
