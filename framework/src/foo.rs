use rustpython::vm::{pymodule, VirtualMachine, PyRef, builtins::PyModule};

#[pymodule]
pub mod _module {
    use rustpython_vm::common::lock::PyRwLock;
    use rustpython::vm::{
        builtins::PyStr,
        PyPayload,
        pymodule,
        pyclass,
        VirtualMachine,
    };
    use rustpython_vm::builtins::{PyStrRef};
    use rustpython_vm::function::{PySetterValue};
    use rustpython_vm::{Py, PyResult};
    use rustpython_vm::types::{Constructor, Unconstructible, Representable};

    #[pyattr]
    #[pyclass(name = "Foo")]
    #[derive(Debug, PyPayload, Default)]
    pub struct Foo {
        name: PyRwLock<String>,
    }


    // CODE BELLOW DOES NOT COMPILE
    // note: conflicting implementation in crate `rustpython_vm`:
    //       - impl<T> Constructor for T
    //       where T: Unconstructible;

    // impl Constructor for Foo {
    //     type Args = FuncArgs;
    //
    //     fn py_new(cls: PyTypeRef, _args: Self::Args, vm: &VirtualMachine) -> PyResult {
    //         Err(vm.new_type_error(format!("cannot create {} instances", cls.slot_name())))
    //     }
    // }

    impl Unconstructible for Foo {}

    impl Representable for Foo {
        #[inline]
        fn repr(_zelf: &Py<Self>, vm: &VirtualMachine) -> PyResult<PyStrRef> {
            const REPR: &str = "<Foo object at .. >";
            Ok(vm.ctx.intern_str(REPR).to_owned())
        }

        #[cold]
        fn repr_str(_zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            unreachable!("use repr instead")
        }
    }


    #[pyclass(with(Constructor))]
    impl Foo {
        #[pygetset]
        fn name(&self) -> PyStr {
            PyStr::from(self.name.read().as_str())
        }

        #[pygetset(setter)]
        pub fn set_name(&self, value: PySetterValue<PyStrRef>, _: &VirtualMachine) {
            let mut n = self.name.write();
            *n = value.unwrap().as_str().to_string();
        }
    }

}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    _module::make_module(vm)
}