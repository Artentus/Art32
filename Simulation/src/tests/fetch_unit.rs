use super::run_test;
use crate::import;

#[test]
fn fetch_unit() {
    let (mut sim, ports) = import!("fetch_unit");

    let instruction_address = ports.outputs["instruction_address"];
    let request_instruction_data = ports.outputs["request_instruction_data"];
    let instruction_data = ports.inputs["instruction_data"];

    let instruction = ports.outputs["instruction"];
    let instruction_16bit = ports.outputs["instruction_16bit"];
    let instruction_valid = ports.outputs["instruction_valid"];
    let program_counter = ports.outputs["program_counter"];

    let advance = ports.inputs["advance"];
    let jump_target = ports.inputs["jump_target"];
    let do_jump = ports.inputs["do_jump"];

    let enable = ports.inputs["enable"];
    let reset = ports.inputs["reset"];
    let clk = ports.inputs["clk"];

    run_test!(sim => {
        enable <= false;
        reset <= true;
        clk <= false;

        advance <= false;
        do_jump <= false;

        assert instruction_address == 0;
        assert request_instruction_data == false;
        assert instruction == 0;
        assert instruction_valid == false;
        assert program_counter == 0;

        posedge clk;
        negedge clk;

        assert instruction_address == 0;
        assert request_instruction_data == false;
        assert instruction == 0;
        assert instruction_valid == false;
        assert program_counter == 0;

        reset <= false;
        posedge clk;
        negedge clk;

        assert instruction_address == 0;
        assert instruction_valid == false;
        assert program_counter == 0;

        enable <= true;
        instruction_data <= 0;
        jump_target <= 0;

        assert instruction_address == 0;
        assert request_instruction_data == true;
        assert instruction_valid == false;
        assert program_counter == 0;

        posedge clk;
        negedge clk;

        assert instruction_address == 1;
        assert request_instruction_data == false;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 0;

        posedge clk;
        negedge clk;

        assert instruction_address == 1;
        assert request_instruction_data == false;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 0;

        advance <= true;

        assert instruction_address == 1;
        assert request_instruction_data == true;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 0;

        posedge clk;
        negedge clk;

        assert instruction_address == 2;
        assert request_instruction_data == false;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 1;

        posedge clk;
        negedge clk;

        assert instruction_address == 2;
        assert request_instruction_data == true;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 2;

        advance <= false;

        assert instruction_address == 2;
        assert request_instruction_data == false;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 2;

        jump_target <= 0x55;
        do_jump <= true;

        assert instruction_address == (0x55 >> 1);
        assert request_instruction_data == true;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 2;

        posedge clk;
        negedge clk;

        do_jump <= false;

        assert instruction_address == (0x56 >> 1);
        assert request_instruction_data == true;
        assert instruction == 0;
        assert instruction_16bit == true;
        assert instruction_valid == true;
        assert program_counter == 0x55;
    });
}
