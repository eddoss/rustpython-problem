use rustpython::vm::{pymodule, VirtualMachine, PyRef, builtins::PyModule};

#[pymodule]
pub mod _module {
    use rustpython_vm::common::lock::PyMutex;
    use rustpython::vm::{
        builtins::PyStr,
        PyPayload,
        pymodule,
        pyclass,
        VirtualMachine,
    };
    use rustpython_vm::builtins::PyStrRef;
    use rustpython_vm::function::PySetterValue;

    #[pyattr]
    #[pyclass(name = "Foo")]
    #[derive(Debug, PyPayload)]
    struct Foo {
        name: PyMutex<String>,
    }

    #[pyclass]
    impl Foo {
        #[pygetset]
        fn name(&self) -> PyStr {
            PyStr::from(self.name.lock().as_str())
        }

        #[pygetset(setter)]
        pub fn set_name(&self, value: PySetterValue<PyStrRef>, _: &VirtualMachine) {
            let mut n = self.name.lock();
            *n = value.unwrap().as_str().to_string();
        }

    }
}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    _module::make_module(vm)
}