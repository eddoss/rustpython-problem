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
    use rustpython_vm::builtins::{PyStrRef, PyTypeRef};
    use rustpython_vm::function::{FuncArgs, PySetterValue};
    use rustpython_vm::{Py, PyResult};
    use rustpython_vm::types::{Constructor, Unconstructible, Representable, DefaultConstructor};

    #[pyattr]
    #[pyclass(name = "Foo")]
    #[derive(Debug, PyPayload, Default)]
    struct Foo {
        name: PyRwLock<String>,
    }

    #[pyclass(with(DefaultConstructor, Representable))]
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

    impl Unconstructible for Foo {}
    impl DefaultConstructor for Foo {}

    // CODE BELLOW DOES NOT COMPILES
    // note: conflicting implementation in crate `rustpython_vm`:
    //       - impl<T> Constructor for T
    //       where T: Unconstructible;
    // impl Constructor for Foo {
    //     type Args = FuncArgs;
    //
    //     fn py_new(cls: PyTypeRef, _args: Self::Args, vm: &VirtualMachine) -> PyResult {
    //         Foo::default().into_ref_with_type(vm, cls).map(Into::into)
    //     }
    // }

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
}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    _module::make_module(vm)
}