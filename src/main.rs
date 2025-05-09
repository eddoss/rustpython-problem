use std::borrow::Cow::Borrowed;
use ahash::HashMapExt;
use rustpython::InterpreterConfig;
use rustpython::vm::{
    pymodule,
    stdlib::StdlibMap, 
    convert::ToPyResult
};

fn main() {
    // Create stdlib and emplace a framework to this.
    let mut stdlib = StdlibMap::new();
    stdlib.insert(Borrowed("problem"), Box::new(_module::make_module));

    // Create an interpreter with embedded stdlib.
    let interpreter = InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| {
            for (name, module) in stdlib {
                vm.add_native_module(name, module);
            }
        }))
        .interpreter();

    // Append the script directory to sys.path
    interpreter.enter(|vm| {
        let unimake = vm.new_pyobj(".");
        vm.insert_sys_path(unimake)
            .expect("failed to add '.' to sys.path");
    });

    // Execute ./main.py
    interpreter.enter(|vm| match vm.import("main", 0) {
        Ok(_) => {}
        Err(e) => {
            let text = e.to_pyresult(vm).unwrap().str(vm).unwrap().to_string();
            println!("{text}")
        }
    });
}

#[pymodule]
pub mod _module {
    use rustpython::vm::{
        FromArgs,
        VirtualMachine,
        PyPayload,
        PyRef,
        PyResult,
        types::{Constructor, DefaultConstructor, Initializer},
        builtins::{PyStr, PyList, PyListRef},
        pyclass,
        common::lock::PyRwLock,
    };

    #[pyattr]
    #[pyclass(name = "Foo")]
    #[derive(Debug, PyPayload)]
    pub struct Foo {
        pub items: PyRwLock<PyList>,
    }

    impl Default for Foo {
        fn default() -> Self {
            Foo {
                items: PyRwLock::new(PyList::default()),
            }
        }
    }

    impl DefaultConstructor for Foo {}

    #[derive(FromArgs, Debug)]
    pub struct FooInitArgs {
        #[pyarg(any, default)]
        items: Option<Vec<String>>,
    }

    impl Initializer for Foo {
        type Args = FooInitArgs;

        fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
            match args.items {
                // Allow only strings
                None => *zelf.items.write() = PyList::default(),
                Some(strings) => {
                    let refs: Vec<_> = strings
                        .iter()
                        .map(|s| PyStr::from(s.as_str()).into_pyobject(vm))
                        .collect();
                    *zelf.items.write() = PyList::from(refs);
                }
            }
            Ok(())
        }
    }

    #[pyclass(with(Constructor, Initializer))]
    impl Foo {
        #[pygetset]
        pub fn items(&self, vm: &VirtualMachine) -> PyListRef {
            vm.ctx
                .new_list(self.items.write().borrow_vec().to_vec())
                .into()
        }

        #[pygetset(setter)]
        pub fn set_items(&self, value: PyListRef, vm: &VirtualMachine) -> PyResult<()> {
            value.borrow_vec().iter().for_each(|v| {
                match v.downcast_ref::<PyStr>() {
                    Some(_) => {}
                    None => {
                        panic!("Expect strings, but given {:?}", v);
                    }
                }
            });
            *self.items.write() = PyList::from(value.borrow_vec().to_vec());
            Ok(())
        }
    }
}
