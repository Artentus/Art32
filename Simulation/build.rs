use glob::glob;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let yosys_path = std::env::var("YOSYS_PATH").unwrap();

    println!("cargo:rerun-if-changed=../Softcore/");
    println!("cargo:rerun-if-changed=./tests/");

    let quartz_files: Vec<_> = glob("../Softcore/*.qrz")
        .unwrap()
        .map(Result::unwrap)
        .collect();
    let verilog_files: Vec<_> = glob("../Softcore/*.v")
        .unwrap()
        .map(Result::unwrap)
        .collect();

    let mut yosys_input_files = String::new();
    for verilog_file in verilog_files {
        yosys_input_files.push(' ');
        yosys_input_files.push_str(verilog_file.as_os_str().to_str().unwrap());
    }

    for test_file in glob("./tests/*.qrz").unwrap().map(Result::unwrap) {
        let mut sv_file = out_dir.clone();
        sv_file.push('/');
        sv_file.push_str(test_file.file_stem().unwrap().to_str().unwrap());
        sv_file.push_str(".sv");

        let mut json_file = out_dir.clone();
        json_file.push('/');
        json_file.push_str(test_file.file_stem().unwrap().to_str().unwrap());
        json_file.push_str(".json");

        let quartz_output = Command::new("./quartz")
            .arg("-o")
            .arg(&sv_file)
            .arg(test_file.to_str().unwrap())
            .args(&quartz_files)
            .output()
            .unwrap();

        if !quartz_output.status.success() {
            panic!("{}", std::str::from_utf8(&quartz_output.stderr).unwrap());
        }

        let yosys_commands = format!("read_verilog -sv {sv_file}; read_verilog {yosys_input_files}; synth -top Top -flatten -noalumacc -run begin:fine; hierarchy -check; check; write_json {json_file}");
        let yosys_output = Command::new(&yosys_path)
            .arg("-p")
            .arg(yosys_commands)
            .output()
            .unwrap();

        if !yosys_output.status.success() {
            panic!("{}", std::str::from_utf8(&yosys_output.stderr).unwrap());
        }
    }
}
