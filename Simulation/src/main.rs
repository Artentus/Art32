#[cfg(test)]
mod tests;

macro_rules! import {
    ($name:literal) => {{
        const MODULE_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/", $name, ".json"));

        let mut builder = gsim::SimulatorBuilder::default();
        let importer =
            gsim::import::yosys::YosysModuleImporter::from_json_str(MODULE_JSON).unwrap();
        let ports = builder.import_module(&importer).unwrap();

        (builder.build(), ports)
    }};
    ($name:literal, $vcd:expr) => {{
        const MODULE_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/", $name, ".json"));

        let mut builder = gsim::SimulatorBuilder::default();
        let importer =
            gsim::import::yosys::YosysModuleImporter::from_json_str(MODULE_JSON).unwrap();
        let ports = builder.import_module(&importer).unwrap();

        (
            builder
                .build_with_trace($vcd, gsim::Timescale::default())
                .unwrap(),
            ports,
        )
    }};
}

use import;

macro_rules! module {
    (@DECL
        $struct_name:ident ;
        $($field:ident),* ;
    ) => {
        struct $struct_name {
            sim: std::cell::RefCell<gsim::Simulator>,
            $($field: gsim::WireId,)*
        }
    };
    (@DECL
        $struct_name:ident ;
        $($field:ident),* ;
        in $next:ident, $($t:tt)*
    ) => {
        module!(@DECL $struct_name ; $($field,)* $next ; $($t)*);
    };
    (@DECL
        $struct_name:ident ;
        $($field:ident),* ;
        out $next:ident, $($t:tt)*
    ) => {
        module!(@DECL $struct_name ; $($field,)* $next ; $($t)*);
    };
    (@CTOR
        $ports:ident ;
    ) => { };
    (@CTOR
        $ports:ident ;
        in $next:ident, $($t:tt)*
    ) => {
        let $next = $ports.inputs[stringify!($next)];
        module!(@CTOR $ports ; $($t)*);
    };
    (@CTOR
        $ports:ident ;
        out $next:ident, $($t:tt)*
    ) => {
        let $next = $ports.outputs[stringify!($next)];
        module!(@CTOR $ports ; $($t)*);
    };
    (@RET
        $struct_name:ident ;
        $sim:ident ;
        $($field:ident),* ;
    ) => {
        $struct_name {
            sim: std::cell::RefCell::new($sim),
            $($field,)*
        }
    };
    (@RET
        $struct_name:ident ;
        $sim:ident ;
        $($field:ident),* ;
        in $next:ident, $($t:tt)*
    ) => {
        module!(@RET $struct_name ; $sim ; $($field,)* $next ; $($t)*)
    };
    (@RET
        $struct_name:ident ;
        $sim:ident ;
        $($field:ident),* ;
        out $next:ident, $($t:tt)*
    ) => {
        module!(@RET $struct_name ; $sim ; $($field,)* $next ; $($t)*)
    };
    ($global_name:ident : $struct_name:ident = $file_name:literal { $($t:tt)* }) => {
        module!(@DECL $struct_name ; ; $($t)*);

        thread_local! {
            static $global_name: $struct_name = {
                let (sim, ports) = crate::import!($file_name);
                module!(@CTOR ports ; $($t)*);
                module!(@RET $struct_name ; sim ; ; $($t)*)
            };
        }
    };
}

use module;

fn main() {}
