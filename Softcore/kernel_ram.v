module KernelRam (
    input wire [12:0] instr_addr_in,
    output reg [31:0] instr_out,

    input wire [12:0] data_addr_in,
    input wire [31:0] data_in,
    output reg [31:0] data_out,
    input wire [3:0] data_byte_enable,
    input wire data_write,
    
    input wire clk
);

    (* no_rw_check *)
    reg [31:0] mem [(1<<13)-1:0];
    initial $readmemh("/home/mathis/test.hex", mem);

    // Instruction Port
    always @(posedge clk) instr_out = mem[instr_addr_in];

    // Data Port
    always @(posedge clk) begin
        if (data_write) begin
            if (data_byte_enable[0]) mem[data_addr_in][ 7: 0] <= data_in[ 7: 0];
            if (data_byte_enable[1]) mem[data_addr_in][15: 8] <= data_in[15: 8];
            if (data_byte_enable[2]) mem[data_addr_in][23:16] <= data_in[23:16];
            if (data_byte_enable[3]) mem[data_addr_in][31:24] <= data_in[31:24];
        end else begin
            data_out <= mem[data_addr_in];
        end
    end

endmodule
