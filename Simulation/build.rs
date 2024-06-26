use glob::glob;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let yosys_path = env::var("YOSYS_PATH").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
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
        yosys_input_files.push('"');
        yosys_input_files.push_str(verilog_file.as_os_str().to_str().unwrap());
        yosys_input_files.push('"');
    }

    for test_file in glob("./tests/*.qrz").unwrap().map(Result::unwrap) {
        let mut sv_file = out_dir.clone();
        if !sv_file.ends_with('/') {
            sv_file.push('/');
        }
        sv_file.push_str(test_file.file_stem().unwrap().to_str().unwrap());
        sv_file.push_str(".sv");

        let mut json_file = out_dir.clone();
        if !json_file.ends_with('/') {
            json_file.push('/');
        }
        json_file.push_str(test_file.file_stem().unwrap().to_str().unwrap());
        json_file.push_str(".json");

        if Path::try_exists(json_file.as_ref()).unwrap() {
            std::fs::remove_file(&json_file).unwrap();
        }

        #[cfg(target_os = "linux")]
        let mut quartz_cmd = Command::new("./quartz");
        #[cfg(target_os = "windows")]
        let mut quartz_cmd = Command::new("./quartz.exe");

        quartz_cmd
            .arg("-o")
            .arg(&sv_file)
            .arg(test_file.to_str().unwrap())
            .args(&quartz_files);

        println!("\u{001b}[36m[RUN]\u{001b}[0m {quartz_cmd:?}");
        let quartz_output = quartz_cmd.output().unwrap();

        if quartz_output.stdout.len() > 0 {
            println!(
                "\u{001b}[36m[OUT]\u{001b}[0m {}",
                String::from_utf8_lossy(&quartz_output.stdout)
            );
        }

        if !quartz_output.status.success() {
            println!(
                "\u{001b}[31m[ERR]\u{001b}[0m Quartz exit code: {}\n{}",
                quartz_output.status.code().unwrap_or(0),
                String::from_utf8_lossy(&quartz_output.stderr),
            );
            panic!();
        }

        let yosys_commands = format!("read_verilog -sv \"{sv_file}\"; read_verilog -DSIM {yosys_input_files}; synth -top Top -flatten -noalumacc -nordff -run begin:fine; hierarchy -check; check; write_json \"{json_file}\"");

        let mut yosys_cmd = Command::new(&yosys_path);
        yosys_cmd.arg("-p").arg(yosys_commands);

        println!("\u{001b}[36m[RUN]\u{001b}[0m {yosys_cmd:?}");
        let yosys_output = yosys_cmd.output().unwrap();

        if yosys_output.stdout.len() > 0 {
            println!(
                "\u{001b}[36m[OUT]\u{001b}[0m {}",
                String::from_utf8_lossy(&yosys_output.stdout)
            );
        }

        if !yosys_output.status.success() {
            println!(
                "\u{001b}[31m[ERR]\u{001b}[0m Yosys exit code: {}\n{}",
                yosys_output.status.code().unwrap_or(0),
                String::from_utf8_lossy(&yosys_output.stderr),
            );
            panic!();
        }
    }
}
