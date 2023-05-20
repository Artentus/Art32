module Pll (
    input wire clk25,

    output wire clk200,
    output wire clk80,
    output wire clk40,
    output wire locked
);

    `ifdef SIM

    reg [0:0] counter_200;
    reg [2:0] counter_80;
    reg [3:0] counter_40;

    initial begin
        counter_200 = 0;
        counter_80 = 0;
        counter_40  = 0;
    end

    reg clk200_reg;
    reg clk80_reg;
    reg clk40_reg;

    initial begin
        clk200_reg = 0;
        clk80_reg = 0;
        clk40_reg  = 0;
    end

    always @(posedge clk25) begin
        if (counter_200 == 0) clk200_reg <= ~clk200_reg;
        if (counter_80  == 0) clk80_reg  <= ~clk80_reg;
        if (counter_40  == 0) clk40_reg  <= ~clk40_reg;

        if (counter_200 == 1) counter_200 <= 0;
        else                  counter_200 <= counter_200 + 1;

        if (counter_80 == 4) counter_80 <= 0;
        else                 counter_80 <= counter_80 + 1;

        if (counter_40 == 9) counter_40 <= 0;
        else                 counter_40 <= counter_40 + 1;
    end

    assign clk200 = clk200_reg;
    assign clk80  = clk80_reg;
    assign clk40  = clk40_reg;

    assign locked = 1'b1;

    `else

    defparam pll0.CLKI_DIV = 1;
    defparam pll0.FEEDBK_PATH = "CLKOP";
    defparam pll0.CLKFB_DIV = 8;
    defparam pll0.PLL_LOCK_MODE = 2;

    defparam pll0.PLLRST_ENA = "DISABLED";
    defparam pll0.INTFB_WAKE = "DISABLED";
    defparam pll0.STDBY_ENABLE = "DISABLED";
    defparam pll0.DPHASE_SOURCE = "DISABLED";

    defparam pll0.CLKOP_ENABLE = "ENABLED";
    defparam pll0.CLKOP_DIV = 4;
    defparam pll0.CLKOP_CPHASE = 3;
    defparam pll0.CLKOP_FPHASE = 0;
    defparam pll0.OUTDIVIDER_MUXA = "DIVA";
    defparam pll0.CLKOP_TRIM_DELAY = 0;
    defparam pll0.CLKOP_TRIM_POL = "FALLING";

    defparam pll0.CLKOS_ENABLE = "ENABLED";
    defparam pll0.CLKOS_DIV = 10;
    defparam pll0.CLKOS_CPHASE = 9;
    defparam pll0.CLKOS_FPHASE = 0;
    defparam pll0.OUTDIVIDER_MUXB = "DIVB";
    defparam pll0.CLKOS_TRIM_DELAY = 0;
    defparam pll0.CLKOS_TRIM_POL = "FALLING";

    defparam pll0.CLKOS2_ENABLE = "ENABLED";
    defparam pll0.CLKOS2_DIV = 20;
    defparam pll0.CLKOS2_CPHASE = 19;
    defparam pll0.CLKOS2_FPHASE = 0;
    defparam pll0.OUTDIVIDER_MUXC = "DIVC";

    defparam pll0.CLKOS3_ENABLE = "DISABLED";
    defparam pll0.CLKOS3_DIV = 1;
    defparam pll0.CLKOS3_CPHASE = 0;
    defparam pll0.CLKOS3_FPHASE = 0;
    defparam pll0.OUTDIVIDER_MUXD = "DIVD";

    (* FREQUENCY_PIN_CLKI="25.000000" *)
    (* FREQUENCY_PIN_CLKOP="200.000000" *)
    (* FREQUENCY_PIN_CLKOS="80.000000" *)
    (* FREQUENCY_PIN_CLKOS2="40.000000" *)
    (* ICP_CURRENT="7" *)
    (* LPF_RESISTOR="16" *)
    EHXPLLL pll0 (
        .CLKI(clk25),
        .CLKFB(clk200),
        .PHASESEL1(1'b0), 
        .PHASESEL0(1'b0),
        .PHASEDIR(1'b0),
        .PHASESTEP(1'b0), 
        .PHASELOADREG(1'b0),
        .STDBY(1'b0),
        .PLLWAKESYNC(1'b0), 
        .RST(1'b0),
        .ENCLKOP(1'b0),
        .ENCLKOS(1'b0),
        .ENCLKOS2(1'b0), 
        .ENCLKOS3(1'b0),
        .CLKOP(clk200),
        .CLKOS(clk80),
        .CLKOS2(clk40), 
        .CLKOS3(),
        .LOCK(locked),
        .INTLOCK(),
        .REFCLK(),
        .CLKINTFB()
    );

    `endif

endmodule
