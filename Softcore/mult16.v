module Mult16 (SignA, SignB, A, B, P);
    input wire SignA;
    input wire SignB;
    input wire [15:0] A;
    input wire [15:0] B;
    output wire [31:0] P;

    `ifdef SIM

    wire [31:0] A32 = SignA ? { {16{A[15]}}, A } : { 16'h0, A };
    wire [31:0] B32 = SignB ? { {16{B[15]}}, B } : { 16'h0, B };
    assign P = A32 * B32;

    `else

    wire mult16_mult_direct_out_1_35;
    wire mult16_mult_direct_out_1_34;
    wire mult16_mult_direct_out_1_33;
    wire mult16_mult_direct_out_1_32;
    wire mult16_mult_direct_out_1_31;
    wire mult16_mult_direct_out_1_30;
    wire mult16_mult_direct_out_1_29;
    wire mult16_mult_direct_out_1_28;
    wire mult16_mult_direct_out_1_27;
    wire mult16_mult_direct_out_1_26;
    wire mult16_mult_direct_out_1_25;
    wire mult16_mult_direct_out_1_24;
    wire mult16_mult_direct_out_1_23;
    wire mult16_mult_direct_out_1_22;
    wire mult16_mult_direct_out_1_21;
    wire mult16_mult_direct_out_1_20;
    wire mult16_mult_direct_out_1_19;
    wire mult16_mult_direct_out_1_18;
    wire mult16_mult_direct_out_1_17;
    wire mult16_mult_direct_out_1_16;
    wire mult16_mult_direct_out_1_15;
    wire mult16_mult_direct_out_1_14;
    wire mult16_mult_direct_out_1_13;
    wire mult16_mult_direct_out_1_12;
    wire mult16_mult_direct_out_1_11;
    wire mult16_mult_direct_out_1_10;
    wire mult16_mult_direct_out_1_9;
    wire mult16_mult_direct_out_1_8;
    wire mult16_mult_direct_out_1_7;
    wire mult16_mult_direct_out_1_6;
    wire mult16_mult_direct_out_1_5;
    wire mult16_mult_direct_out_1_4;
    wire mult16_mult_direct_out_1_3;
    wire mult16_mult_direct_out_1_2;
    wire mult16_mult_direct_out_1_1;
    wire mult16_mult_direct_out_1_0;

    defparam dsp_mult_0.CLK3_DIV = "ENABLED" ;
    defparam dsp_mult_0.CLK2_DIV = "ENABLED" ;
    defparam dsp_mult_0.CLK1_DIV = "ENABLED" ;
    defparam dsp_mult_0.CLK0_DIV = "ENABLED" ;
    defparam dsp_mult_0.HIGHSPEED_CLK = "NONE" ;
    defparam dsp_mult_0.REG_INPUTC_RST = "RST0" ;
    defparam dsp_mult_0.REG_INPUTC_CE = "CE0" ;
    defparam dsp_mult_0.REG_INPUTC_CLK = "NONE" ;
    defparam dsp_mult_0.SOURCEB_MODE = "B_SHIFT" ;
    defparam dsp_mult_0.MULT_BYPASS = "DISABLED" ;
    defparam dsp_mult_0.CAS_MATCH_REG = "FALSE" ;
    defparam dsp_mult_0.RESETMODE = "SYNC" ;
    defparam dsp_mult_0.GSR = "ENABLED" ;
    defparam dsp_mult_0.REG_OUTPUT_RST = "RST0" ;
    defparam dsp_mult_0.REG_OUTPUT_CE = "CE0" ;
    defparam dsp_mult_0.REG_OUTPUT_CLK = "NONE" ;
    defparam dsp_mult_0.REG_PIPELINE_RST = "RST0" ;
    defparam dsp_mult_0.REG_PIPELINE_CE = "CE0" ;
    defparam dsp_mult_0.REG_PIPELINE_CLK = "NONE" ;
    defparam dsp_mult_0.REG_INPUTB_RST = "RST0" ;
    defparam dsp_mult_0.REG_INPUTB_CE = "CE0" ;
    defparam dsp_mult_0.REG_INPUTB_CLK = "NONE" ;
    defparam dsp_mult_0.REG_INPUTA_RST = "RST0" ;
    defparam dsp_mult_0.REG_INPUTA_CE = "CE0" ;
    defparam dsp_mult_0.REG_INPUTA_CLK = "NONE" ;
    MULT18X18D dsp_mult_0 (.A17(A[15]), .A16(A[14]), .A15(A[13]), .A14(A[12]),
        .A13(A[11]), .A12(A[10]), .A11(A[9]), .A10(A[8]), .A9(A[7]), .A8(A[6]),
        .A7(A[5]), .A6(A[4]), .A5(A[3]), .A4(A[2]), .A3(A[1]), .A2(A[0]),
        .A1(1'b0), .A0(1'b0), .B17(B[15]), .B16(B[14]), .B15(B[13]),
        .B14(B[12]), .B13(B[11]), .B12(B[10]), .B11(B[9]), .B10(B[8]), .B9(B[7]),
        .B8(B[6]), .B7(B[5]), .B6(B[4]), .B5(B[3]), .B4(B[2]), .B3(B[1]),
        .B2(B[0]), .B1(1'b0), .B0(1'b0), .C17(1'b0), .C16(1'b0),
        .C15(1'b0), .C14(1'b0), .C13(1'b0), .C12(1'b0),
        .C11(1'b0), .C10(1'b0), .C9(1'b0), .C8(1'b0),
        .C7(1'b0), .C6(1'b0), .C5(1'b0), .C4(1'b0),
        .C3(1'b0), .C2(1'b0), .C1(1'b0), .C0(1'b0),
        .SIGNEDA(SignA), .SIGNEDB(SignB), .SOURCEA(1'b0), .SOURCEB(1'b0),
        .CE0(1'b1), .CE1(1'b1), .CE2(1'b1), .CE3(1'b1),
        .CLK0(1'b0), .CLK1(1'b0), .CLK2(1'b0), .CLK3(1'b0),
        .RST0(1'b0), .RST1(1'b0), .RST2(1'b0), .RST3(1'b0),
        .P35(mult16_mult_direct_out_1_35),
        .P34(mult16_mult_direct_out_1_34),
        .P33(mult16_mult_direct_out_1_33),
        .P32(mult16_mult_direct_out_1_32),
        .P31(mult16_mult_direct_out_1_31),
        .P30(mult16_mult_direct_out_1_30),
        .P29(mult16_mult_direct_out_1_29),
        .P28(mult16_mult_direct_out_1_28),
        .P27(mult16_mult_direct_out_1_27),
        .P26(mult16_mult_direct_out_1_26),
        .P25(mult16_mult_direct_out_1_25),
        .P24(mult16_mult_direct_out_1_24),
        .P23(mult16_mult_direct_out_1_23),
        .P22(mult16_mult_direct_out_1_22),
        .P21(mult16_mult_direct_out_1_21),
        .P20(mult16_mult_direct_out_1_20),
        .P19(mult16_mult_direct_out_1_19),
        .P18(mult16_mult_direct_out_1_18),
        .P17(mult16_mult_direct_out_1_17),
        .P16(mult16_mult_direct_out_1_16),
        .P15(mult16_mult_direct_out_1_15),
        .P14(mult16_mult_direct_out_1_14),
        .P13(mult16_mult_direct_out_1_13),
        .P12(mult16_mult_direct_out_1_12),
        .P11(mult16_mult_direct_out_1_11),
        .P10(mult16_mult_direct_out_1_10),
        .P9(mult16_mult_direct_out_1_9),
        .P8(mult16_mult_direct_out_1_8),
        .P7(mult16_mult_direct_out_1_7),
        .P6(mult16_mult_direct_out_1_6),
        .P5(mult16_mult_direct_out_1_5),
        .P4(mult16_mult_direct_out_1_4),
        .P3(mult16_mult_direct_out_1_3),
        .P2(mult16_mult_direct_out_1_2),
        .P1(mult16_mult_direct_out_1_1),
        .P0(mult16_mult_direct_out_1_0));

    assign P[31] = mult16_mult_direct_out_1_35;
    assign P[30] = mult16_mult_direct_out_1_34;
    assign P[29] = mult16_mult_direct_out_1_33;
    assign P[28] = mult16_mult_direct_out_1_32;
    assign P[27] = mult16_mult_direct_out_1_31;
    assign P[26] = mult16_mult_direct_out_1_30;
    assign P[25] = mult16_mult_direct_out_1_29;
    assign P[24] = mult16_mult_direct_out_1_28;
    assign P[23] = mult16_mult_direct_out_1_27;
    assign P[22] = mult16_mult_direct_out_1_26;
    assign P[21] = mult16_mult_direct_out_1_25;
    assign P[20] = mult16_mult_direct_out_1_24;
    assign P[19] = mult16_mult_direct_out_1_23;
    assign P[18] = mult16_mult_direct_out_1_22;
    assign P[17] = mult16_mult_direct_out_1_21;
    assign P[16] = mult16_mult_direct_out_1_20;
    assign P[15] = mult16_mult_direct_out_1_19;
    assign P[14] = mult16_mult_direct_out_1_18;
    assign P[13] = mult16_mult_direct_out_1_17;
    assign P[12] = mult16_mult_direct_out_1_16;
    assign P[11] = mult16_mult_direct_out_1_15;
    assign P[10] = mult16_mult_direct_out_1_14;
    assign P[9] = mult16_mult_direct_out_1_13;
    assign P[8] = mult16_mult_direct_out_1_12;
    assign P[7] = mult16_mult_direct_out_1_11;
    assign P[6] = mult16_mult_direct_out_1_10;
    assign P[5] = mult16_mult_direct_out_1_9;
    assign P[4] = mult16_mult_direct_out_1_8;
    assign P[3] = mult16_mult_direct_out_1_7;
    assign P[2] = mult16_mult_direct_out_1_6;
    assign P[1] = mult16_mult_direct_out_1_5;
    assign P[0] = mult16_mult_direct_out_1_4;

    `endif

endmodule
