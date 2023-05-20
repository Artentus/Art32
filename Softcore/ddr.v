module DDR (
    input wire [1:0] d_in,
    output wire d_out,

    input wire reset,
    input wire clk
);

    `ifdef SIM

    reg [1:0] d_reg;
    always @(posedge clk) d_reg <= d_in;

    assign d_out = clk ? d_reg[0] : d_reg[1];

    `else

    ODDRX1F ddr (
        .SCLK(clk),
        .RST(reset),
        .D0(d_in[0]),
        .D1(d_in[1]), 
        .Q(d_out)
    );

    `endif

endmodule
