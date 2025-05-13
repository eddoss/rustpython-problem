use ahash::HashMapExt;
use rustpython::InterpreterConfig;
use rustpython::vm::{pymodule, stdlib::StdlibMap};
use std::borrow::Cow::Borrowed;

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
        Err(e) => vm.print_exception(e),
    });
}

#[pymodule]
pub mod _module {
    use rustpython::vm::{
        FromArgs, PyPayload, PyRef, PyResult, VirtualMachine,
        builtins::{PyList, PyListRef, PyStr},
        class::StaticType,
        common::lock::PyRwLock,
        pyclass,
        types::{Constructor, DefaultConstructor, Initializer},
        AsObject,
    };

    #[pyattr]
    #[pyclass(name = "Foo")]
    #[derive(Debug, PyPayload)]
    pub struct Foo {
        pub items: PyRwLock<PyListRef>,
    }

    impl Default for Foo {
        fn default() -> Self {
            Foo { items: PyRwLock::new(PyListRef::new_ref(PyList::default(), PyList::create_static_type(), None)) }
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
            *zelf.items.write() = vm.ctx.new_list(vec![]);
            if let Some(entries) = args.items {
                let references: Vec<_> = entries
                    .into_iter()
                    .map(|s| PyStr::from(s).into_pyobject(vm))
                    .collect();
                *zelf.items.write() = vm.ctx.new_list(references);
            }
            Ok(())
        }
    }

    #[pyclass(with(Constructor, Initializer))]
    impl Foo {
        #[pygetset]
        fn items(&self, _: &VirtualMachine) -> PyListRef {
            self.items.read().clone()
        }

        #[pygetset(setter)]
        fn set_items(&self, value: PyListRef, vm: &VirtualMachine) -> PyResult<()> {
            if value.borrow_vec().iter().all(|v|v.fast_isinstance(&*PyStr::create_static_type())) {
                return Err(vm.new_type_error("expected list[str]".to_string()));
            }
            *self.items.write() = value;
            Ok(())
        }
    }
}
