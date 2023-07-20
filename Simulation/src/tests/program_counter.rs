use super::run_test;
use crate::import;

#[test]
fn program_counter() {
    let (mut sim, ports) = import!("program_counter");

    let data_in = ports.inputs["data_in"];
    let inc = ports.inputs["inc"];
    let load = ports.inputs["load"];

    let pc_next = ports.outputs["pc_next"];
    let pc_value = ports.outputs["pc_value"];

    let enable = ports.inputs["enable"];
    let reset = ports.inputs["reset"];
    let clk = ports.inputs["clk"];

    run_test!(sim => {
        assert pc_next == UNDEFINED;
        assert pc_value == UNDEFINED;

        data_in <= 0;
        inc <= 0;
        load <= false;
        enable <= false;
        reset <= false;
        clk <= false;

        assert pc_next == UNDEFINED;
        assert pc_value == UNDEFINED;

        reset <= true;
        assert pc_next == 0;
        assert pc_value == UNDEFINED;

        reset <= false;
        assert pc_next == UNDEFINED;
        assert pc_value == UNDEFINED;

        reset <= true;
        posedge clk;
        assert pc_next == 0;
        assert pc_value == 0;

        reset <= false;
        negedge clk;
        assert pc_next == 0;
        assert pc_value == 0;

        data_in <= 0xAA55;
        enable <= true;
        assert pc_next == 0;
        assert pc_value == 0;

        load <= true;
        assert pc_next == 0xAA55;
        assert pc_value == 0;

        posedge clk;
        assert pc_next == 0xAA55;
        assert pc_value == 0xAA55;

        enable <= false;
        load <= false;
        negedge clk;
        assert pc_next == 0xAA55;
        assert pc_value == 0xAA55;

        inc <= 1;
        assert pc_next == 0xAA55;
        assert pc_value == 0xAA55;

        enable <= true;
        assert pc_next == 0xAA56;
        assert pc_value == 0xAA55;

        inc <= 2;
        assert pc_next == 0xAA57;
        assert pc_value == 0xAA55;

        inc <= 3;
        assert pc_next == 0xAA58;
        assert pc_value == 0xAA55;

        posedge clk;
        assert pc_next == 0xAA5B;
        assert pc_value == 0xAA58;

        negedge clk;
        inc <= 1;
        assert pc_next == 0xAA59;
        assert pc_value == 0xAA58;

        load <= true;
        reset <= true;
        assert pc_next == 0;
        assert pc_value == 0xAA58;

        posedge clk;
        assert pc_next == 0;
        assert pc_value == 0;

        reset <= false;
        negedge clk;
        assert pc_next == 0xAA55;
        assert pc_value == 0;

        load <= false;
        assert pc_next == 1;
        assert pc_value == 0;
    });
}
