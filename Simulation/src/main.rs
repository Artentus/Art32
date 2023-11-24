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

fn main() {}
