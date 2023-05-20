`default_nettype none

typedef struct packed {
    logic d_in;
} InPort__1__Interface;

typedef struct packed {
    logic clk25;
    logic clk200;
    logic clk80;
    logic clk40;
    logic locked;
} Pll__Interface;

typedef struct packed {
    logic[12:0] instr_addr_in;
    logic[31:0] instr_out;
    logic[12:0] data_addr_in;
    logic[31:0] data_in;
    logic[31:0] data_out;
    logic[3:0] data_byte_enable;
    logic data_write;
    logic clk;
} KernelRam__Interface;

typedef struct packed {
    logic[17:0] d_out;
} OutPort__18__Interface;

typedef struct packed {
    logic d_out;
} OutPort__1__Interface;

typedef struct packed {
    logic[15:0] d_in;
    logic[15:0] d_out;
    logic oe;
} InOutPort__16__Interface;

typedef enum logic[2:0] {
    LhsBusSource__Register = 0,
    LhsBusSource__Pc = 1,
    LhsBusSource__Syscall = 2,
    LhsBusSource__Forward3 = 4,
    LhsBusSource__Forward4 = 5,
    LhsBusSource__Forward5 = 6
} LhsBusSource;

typedef enum logic[2:0] {
    RhsBusSource__Register = 0,
    RhsBusSource__Pc = 1,
    RhsBusSource__Immediate = 2,
    RhsBusSource__Forward3 = 4,
    RhsBusSource__Forward4 = 5,
    RhsBusSource__Forward5 = 6
} RhsBusSource;

typedef enum logic[3:0] {
    AluOp__Add = 0,
    AluOp__AddC = 1,
    AluOp__Sub = 2,
    AluOp__SubB = 3,
    AluOp__And = 4,
    AluOp__Or = 5,
    AluOp__Xor = 6,
    AluOp__Shl = 7,
    AluOp__Lsr = 8,
    AluOp__Asr = 9,
    AluOp__Mul = 10,
    AluOp__MulHuu = 11,
    AluOp__MulHss = 12,
    AluOp__MulHus = 13,
    AluOp__Cond = 14,
    AluOp__Nop = 15
} AluOp;

typedef enum logic[3:0] {
    Condition__Never = 0,
    Condition__Carry = 1,
    Condition__Zero = 2,
    Condition__Signed = 3,
    Condition__Overflow = 4,
    Condition__NotCarry = 5,
    Condition__NotZero = 6,
    Condition__NotSigned = 7,
    Condition__NotOverflow = 8,
    Condition__UnsignedLessEqual = 9,
    Condition__UnsignedGreater = 10,
    Condition__SignedLess = 11,
    Condition__SignedGreaterEqual = 12,
    Condition__SignedLessEqual = 13,
    Condition__SignedGreater = 14,
    Condition__Always = 15
} Condition;

typedef enum logic[1:0] {
    MemoryMode__Bits32 = 0,
    MemoryMode__Bits8 = 1,
    MemoryMode__Bits16 = 2,
    MemoryMode__IO = 3
} MemoryMode;

typedef enum logic {
    DataBusSource__Result = 0,
    DataBusSource__Memory = 1
} DataBusSource;

typedef struct packed {
    logic[7:0] data_in;
    logic transmit;
    logic fetch;
    logic[7:0] data_out;
    logic received;
    logic rxd;
    logic txd;
    logic reset;
    logic clk;
} Uart__Interface;

typedef struct packed {
    logic carry;
    logic zero;
    logic sign;
    logic overflow;
} Flags;

typedef enum logic[3:0] {
    OpCode__System = 0,
    OpCode__AluRegReg = 1,
    OpCode__AluRegImm = 2,
    OpCode__Load = 3,
    OpCode__Store = 4,
    OpCode__Branch = 5,
    OpCode__MoveRegReg = 6,
    OpCode__MoveRegImm = 7,
    OpCode__Jump = 8,
    OpCode__UI = 9,
    OpCode__Nop = 15
} OpCode;

typedef enum logic[1:0] {
    SyncState__Front = 0,
    SyncState__Sync = 1,
    SyncState__Back = 2,
    SyncState__Active = 3
} SyncState;

typedef enum logic {
    TmdsMode__Control = 0,
    TmdsMode__Video = 1
} TmdsMode;

typedef struct packed {
    TmdsMode mode;
    logic[1:0] control_data;
    logic[7:0] video_data;
    logic[9:0] encoded;
    logic reset;
    logic pclk;
} TmdsEncoder__Interface;

typedef enum logic[1:0] {
    AdderOp__Add = 0,
    AdderOp__AddC = 1,
    AdderOp__Sub = 2,
    AdderOp__SubB = 3
} AdderOp;

typedef enum logic[1:0] {
    MultOp__MulUU = 0,
    MultOp__MulSS = 3,
    MultOp__MulUS = 1
} MultOp;

typedef struct packed {
    logic SignA;
    logic SignB;
    logic[15:0] A;
    logic[15:0] B;
    logic[31:0] P;
} Mult16__Interface;

typedef struct packed {
    logic[1:0] d_in;
    logic d_out;
    logic reset;
    logic clk;
} DDR__Interface;

typedef struct packed {
    logic[7:0] mem_page;
    logic mem_enable;
    logic mem_write;
    logic k_flag;
    logic kram_write;
    logic sram_write;
    logic vram_write;
    logic[31:0] kram_data_in;
    logic[31:0] sram_data_in;
    logic[31:0] vram_data_in;
    logic[31:0] data_out;
    logic reset;
    logic clk;
} Mmu__Interface;

module Mmu (
    input var logic[7:0] mem_page,
    input var logic mem_enable,
    input var logic mem_write,
    input var logic k_flag,
    output var logic kram_write,
    output var logic sram_write,
    output var logic vram_write,
    input var logic[31:0] kram_data_in,
    input var logic[31:0] sram_data_in,
    input var logic[31:0] vram_data_in,
    output var logic[31:0] data_out,
    input var logic reset,
    input var logic clk
);

var logic[7:0] mem_page_reg;
var logic k_flag_reg;

var logic[31:0] __tmp_0;
var logic[7:0] __tmp_1;
var logic[31:0] __tmp_2;
always_comb begin
    __tmp_1 = mem_page_reg;
    if (k_flag_reg) begin
        __tmp_2 = kram_data_in;
    end else begin
        __tmp_2 = 32'd2863311530;
    end
    if ((__tmp_1) == (8'd0)) begin
        __tmp_0 = __tmp_2;
    end else if ((__tmp_1) == (8'd1)) begin
        __tmp_0 = sram_data_in;
    end else if ((__tmp_1) == (8'd2)) begin
        __tmp_0 = vram_data_in;
    end else begin
        __tmp_0 = 32'd2863311530;
    end
end

always_ff @(posedge clk) begin
    mem_page_reg <= mem_page;
    k_flag_reg <= k_flag;
end

always_comb begin
    kram_write = (((mem_enable) & (mem_write)) & ((mem_page) == (8'd0))) & (k_flag);
    sram_write = ((mem_enable) & (mem_write)) & ((mem_page) == (8'd1));
    vram_write = ((mem_enable) & (mem_write)) & ((mem_page) == (8'd2));
end

always_comb begin
    data_out = __tmp_0;
end


endmodule

typedef struct packed {
    logic ce_n_out;
    logic reset;
    logic clk;
} CE__Interface;

(* keep_hierarchy *)
module CE (
    output var logic ce_n_out,
    input var logic reset,
    input var logic clk
);

(* syn_useioff, ioff_dir="output" *) var logic ce_n;

var logic __tmp_0;
always_comb begin
    if (reset) begin
        __tmp_0 = 1'd1;
    end else begin
        __tmp_0 = 1'd0;
    end
end

always_ff @(posedge clk) begin
    ce_n <= __tmp_0;
end

always_comb begin
    ce_n_out = ce_n;
end


endmodule

typedef struct packed {
    logic oe_n_in;
    logic oe_n_out;
    logic clk;
} OE__Interface;

(* keep_hierarchy *)
module OE (
    input var logic oe_n_in,
    output var logic oe_n_out,
    input var logic clk
);

(* syn_useioff, ioff_dir="output" *) var logic oe_n;

always_ff @(posedge clk) begin
    oe_n <= oe_n_in;
end

always_comb begin
    oe_n_out = oe_n;
end


endmodule

typedef struct packed {
    logic set;
    logic clear;
    logic k_flag_out;
    logic enable;
    logic reset;
    logic clk;
} KernelModeRegister__Interface;

module KernelModeRegister (
    input var logic set,
    input var logic clear,
    output var logic k_flag_out,
    input var logic enable,
    input var logic reset,
    input var logic clk
);

var logic k_flag;

always_ff @(posedge clk) begin
    if (reset) begin
        k_flag <= 1'd1;
    end else if ((enable) & (clear)) begin
        k_flag <= 1'd0;
    end else if ((enable) & (set)) begin
        k_flag <= 1'd1;
    end
end

always_comb begin
    k_flag_out = k_flag;
end


endmodule

typedef struct packed {
    logic[23:0] data_in;
    logic[23:0] data_out;
    logic write;
    logic r_out;
    logic g_out;
    logic b_out;
    logic reset;
    logic clk;
} LedController__Interface;

module LedController (
    input var logic[23:0] data_in,
    output var logic[23:0] data_out,
    input var logic write,
    (* syn_useioff, ioff_dir="output" *) output var logic r_out,
    (* syn_useioff, ioff_dir="output" *) output var logic g_out,
    (* syn_useioff, ioff_dir="output" *) output var logic b_out,
    input var logic reset,
    input var logic clk
);

var logic[7:0] r;
var logic[7:0] g;
var logic[7:0] b;
var logic[15:0] counter;

var logic[23:0] __tmp_0;
var logic[23:0] __tmp_1;
var logic[23:0] __tmp_2;
var logic[15:0] __tmp_3;
var logic[15:0] __tmp_4;
var logic[15:0] __tmp_5;
always_comb begin
    __tmp_0 = data_in;
    __tmp_1 = data_in;
    __tmp_2 = data_in;
    __tmp_3 = counter;
    __tmp_4 = counter;
    __tmp_5 = counter;
end

always_ff @(posedge clk) begin
    if (reset) begin
        r <= 8'd0;
        g <= 8'd0;
        b <= 8'd0;
    end else if (write) begin
        r <= __tmp_0[7:0];
        g <= __tmp_1[15:8];
        b <= __tmp_2[23:16];
    end
end

always_ff @(posedge clk) begin
    counter <= (counter) + (16'd1);
end

always_ff @(posedge clk) begin
    r_out <= (r) > (__tmp_3[15:8]);
    g_out <= (g) > (__tmp_4[15:8]);
    b_out <= (b) > (__tmp_5[15:8]);
end

always_comb begin
    data_out = {{b, g}, r};
end


endmodule

typedef struct packed {
    Flags flags_in;
    logic load;
    Flags flags_out;
    logic enable;
    logic clk;
} FlagRegister__Interface;

module FlagRegister (
    input var Flags flags_in,
    input var logic load,
    output var Flags flags_out,
    input var logic enable,
    input var logic clk
);

var Flags flags;

always_ff @(posedge clk) begin
    if ((enable) & (load)) begin
        flags <= flags_in;
    end
end

always_comb begin
    flags_out = flags;
end


endmodule

typedef struct packed {
    logic[31:0] lhs;
    logic[31:0] rhs;
    logic carry_in;
    AdderOp op;
    logic[31:0] result;
    logic carry_out;
    logic sign;
    logic overflow;
} Adder__32__Interface;

module Adder__32 (
    input var logic[31:0] lhs,
    input var logic[31:0] rhs,
    input var logic carry_in,
    input var AdderOp op,
    output var logic[31:0] result,
    output var logic carry_out,
    output var logic sign,
    output var logic overflow
);

var logic[31:0] rhs_inverted;
var logic carry_in_override;
var logic lhs_sign;
var logic rhs_sign;
var logic[32:0] lhs_full;
var logic[32:0] rhs_full;
var logic[32:0] carry_in_full;
var logic[32:0] sum;

var logic[31:0] __tmp_0;
var logic __tmp_1;
var logic[31:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[32:0] __tmp_4;
var logic[32:0] __tmp_5;
var logic[32:0] __tmp_6;
always_comb begin
    case (op)
        AdderOp__Add, AdderOp__AddC: begin
            __tmp_0 = rhs;
        end
        default: begin
            __tmp_0 = ~(rhs);
        end
    endcase
    case (op)
        AdderOp__Add: begin
            __tmp_1 = 1'd0;
        end
        AdderOp__Sub: begin
            __tmp_1 = 1'd1;
        end
        default: begin
            __tmp_1 = carry_in;
        end
    endcase
    __tmp_2 = lhs;
    __tmp_3 = rhs_inverted;
    __tmp_4 = sum;
    __tmp_5 = sum;
    __tmp_6 = sum;
end

always_comb begin
    rhs_inverted = __tmp_0;
end

always_comb begin
    carry_in_override = __tmp_1;
end

always_comb begin
    lhs_sign = __tmp_2[5'd31];
    rhs_sign = __tmp_3[5'd31];
end

always_comb begin
    lhs_full = {1'd0, lhs};
    rhs_full = {1'd0, rhs_inverted};
    carry_in_full = {32'd0, carry_in_override};
    sum = ((lhs_full) + (rhs_full)) + (carry_in_full);
end

always_comb begin
    result = __tmp_4[31:0];
    carry_out = __tmp_5[6'd32];
    sign = __tmp_6[6'd31];
    overflow = ((lhs_sign) == (rhs_sign)) & ((lhs_sign) != (sign));
end


endmodule

typedef struct packed {
    logic[31:0] lhs;
    logic[31:0] rhs;
    logic[63:0] result;
    MultOp op;
} Mult32__Interface;

module Mult32 (
    input var logic[31:0] lhs,
    input var logic[31:0] rhs,
    output var logic[63:0] result,
    input var MultOp op
);

var logic lhs_sign;
var logic rhs_sign;
var logic[15:0] lhs_l;
var logic[15:0] lhs_h;
var logic[15:0] rhs_l;
var logic[15:0] rhs_h;
var logic[31:0] prod_ll;
var Mult16__Interface mult_ll /*verilator split_var*/;
Mult16 mult_ll__instance (
    .SignA(mult_ll.SignA),
    .SignB(mult_ll.SignB),
    .A(mult_ll.A),
    .B(mult_ll.B),
    .P(mult_ll.P)
);
var logic[31:0] prod_lh;
var Mult16__Interface mult_lh /*verilator split_var*/;
Mult16 mult_lh__instance (
    .SignA(mult_lh.SignA),
    .SignB(mult_lh.SignB),
    .A(mult_lh.A),
    .B(mult_lh.B),
    .P(mult_lh.P)
);
var logic[31:0] prod_hl;
var Mult16__Interface mult_hl /*verilator split_var*/;
Mult16 mult_hl__instance (
    .SignA(mult_hl.SignA),
    .SignB(mult_hl.SignB),
    .A(mult_hl.A),
    .B(mult_hl.B),
    .P(mult_hl.P)
);
var logic[31:0] prod_hh;
var Mult16__Interface mult_hh /*verilator split_var*/;
Mult16 mult_hh__instance (
    .SignA(mult_hh.SignA),
    .SignB(mult_hh.SignB),
    .A(mult_hh.A),
    .B(mult_hh.B),
    .P(mult_hh.P)
);

var logic __tmp_0;
var logic __tmp_1;
var logic[31:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[31:0] __tmp_4;
var logic[31:0] __tmp_5;
always_comb begin
    case (op)
        MultOp__MulUU: begin
            __tmp_0 = 1'd0;
        end
        MultOp__MulSS: begin
            __tmp_0 = 1'd1;
        end
        default: begin
            __tmp_0 = 1'd0;
        end
    endcase
    case (op)
        MultOp__MulUU: begin
            __tmp_1 = 1'd0;
        end
        MultOp__MulSS: begin
            __tmp_1 = 1'd1;
        end
        default: begin
            __tmp_1 = 1'd1;
        end
    endcase
    __tmp_2 = lhs;
    __tmp_3 = lhs;
    __tmp_4 = rhs;
    __tmp_5 = rhs;
end

always_comb begin
    lhs_sign = __tmp_0;
    rhs_sign = __tmp_1;
end

always_comb begin
    lhs_l = __tmp_2[15:0];
    lhs_h = __tmp_3[31:16];
end

always_comb begin
    rhs_l = __tmp_4[15:0];
    rhs_h = __tmp_5[31:16];
end

always_comb begin
    mult_ll.SignA = 1'd0;
    mult_ll.SignB = 1'd0;
    mult_ll.A = lhs_l;
    mult_ll.B = rhs_l;
    prod_ll = mult_ll.P;
end

always_comb begin
    mult_lh.SignA = 1'd0;
    mult_lh.SignB = rhs_sign;
    mult_lh.A = lhs_l;
    mult_lh.B = rhs_h;
    prod_lh = mult_lh.P;
end

always_comb begin
    mult_hl.SignA = lhs_sign;
    mult_hl.SignB = 1'd0;
    mult_hl.A = lhs_h;
    mult_hl.B = rhs_l;
    prod_hl = mult_hl.P;
end

always_comb begin
    mult_hh.SignA = lhs_sign;
    mult_hh.SignB = rhs_sign;
    mult_hh.A = lhs_h;
    mult_hh.B = rhs_h;
    prod_hh = mult_hh.P;
end

always_comb begin
    result = (({prod_hh, prod_ll}) + ($signed({prod_lh, 32'd0}) >>> $signed(64'd16))) + ($signed({prod_hl, 32'd0}) >>> $signed(64'd16));
end


endmodule

typedef struct packed {
    logic[17:0] sram_address_out;
    logic sram_we_n_out;
    logic sram_we_out;
    logic[15:0] sram_a_data_in;
    logic[15:0] sram_a_data_out;
    logic sram_a_ce_n_out;
    logic sram_a_oe_n_out;
    logic sram_a_lb_n_out;
    logic sram_a_ub_n_out;
    logic[15:0] sram_b_data_in;
    logic[15:0] sram_b_data_out;
    logic sram_b_ce_n_out;
    logic sram_b_oe_n_out;
    logic sram_b_lb_n_out;
    logic sram_b_ub_n_out;
    logic[17:0] address_in;
    logic[31:0] data_in;
    logic[31:0] data_out;
    logic write_read_n_in;
    logic[3:0] byte_enable_in;
    logic reset;
    logic clk;
    logic clk2;
} SramInterface__Interface;

(* keep_hierarchy *)
module SramInterface (
    (* syn_useioff, ioff_dir="output" *) output var logic[17:0] sram_address_out,
    (* syn_useioff, ioff_dir="output" *) output var logic sram_we_n_out,
    output var logic sram_we_out,
    input var logic[15:0] sram_a_data_in,
    output var logic[15:0] sram_a_data_out,
    output var logic sram_a_ce_n_out,
    output var logic sram_a_oe_n_out,
    (* syn_useioff, ioff_dir="output" *) output var logic sram_a_lb_n_out,
    (* syn_useioff, ioff_dir="output" *) output var logic sram_a_ub_n_out,
    input var logic[15:0] sram_b_data_in,
    output var logic[15:0] sram_b_data_out,
    output var logic sram_b_ce_n_out,
    output var logic sram_b_oe_n_out,
    (* syn_useioff, ioff_dir="output" *) output var logic sram_b_lb_n_out,
    (* syn_useioff, ioff_dir="output" *) output var logic sram_b_ub_n_out,
    input var logic[17:0] address_in,
    input var logic[31:0] data_in,
    output var logic[31:0] data_out,
    input var logic write_read_n_in,
    input var logic[3:0] byte_enable_in,
    input var logic reset,
    input var logic clk,
    input var logic clk2
);

var logic cycle_parity;
(* syn_useioff, ioff_dir="output" *) var logic[15:0] sram_a_data_out_reg;
(* syn_useioff, ioff_dir="input" *) var logic[15:0] sram_a_data_in_reg;
(* syn_useioff, ioff_dir="output" *) var logic[15:0] sram_b_data_out_reg;
(* syn_useioff, ioff_dir="input" *) var logic[15:0] sram_b_data_in_reg;
var logic sram_we_reg;
var CE__Interface sram_a_ce /*verilator split_var*/;
CE sram_a_ce__instance (
    .ce_n_out(sram_a_ce.ce_n_out),
    .reset(sram_a_ce.reset),
    .clk(sram_a_ce.clk)
);
var CE__Interface sram_b_ce /*verilator split_var*/;
CE sram_b_ce__instance (
    .ce_n_out(sram_b_ce.ce_n_out),
    .reset(sram_b_ce.reset),
    .clk(sram_b_ce.clk)
);
var OE__Interface sram_a_oe /*verilator split_var*/;
OE sram_a_oe__instance (
    .oe_n_in(sram_a_oe.oe_n_in),
    .oe_n_out(sram_a_oe.oe_n_out),
    .clk(sram_a_oe.clk)
);
var OE__Interface sram_b_oe /*verilator split_var*/;
OE sram_b_oe__instance (
    .oe_n_in(sram_b_oe.oe_n_in),
    .oe_n_out(sram_b_oe.oe_n_out),
    .clk(sram_b_oe.clk)
);

var logic __tmp_0;
var logic[3:0] __tmp_1;
var logic[3:0] __tmp_2;
var logic[3:0] __tmp_3;
var logic[3:0] __tmp_4;
var logic[31:0] __tmp_5;
var logic[31:0] __tmp_6;
var logic __tmp_7;
var logic __tmp_8;
always_comb begin
    if (reset) begin
        __tmp_0 = 1'd0;
    end else begin
        __tmp_0 = ~(cycle_parity);
    end
    __tmp_1 = byte_enable_in;
    __tmp_2 = byte_enable_in;
    __tmp_3 = byte_enable_in;
    __tmp_4 = byte_enable_in;
    __tmp_5 = data_in;
    __tmp_6 = data_in;
    if (cycle_parity) begin
        __tmp_7 = write_read_n_in;
    end else begin
        __tmp_7 = 1'd1;
    end
    if (cycle_parity) begin
        __tmp_8 = write_read_n_in;
    end else begin
        __tmp_8 = 1'd1;
    end
end

always_ff @(posedge clk2) begin
    cycle_parity <= __tmp_0;
end

always_ff @(posedge clk2) begin
    if (cycle_parity) begin
        sram_we_n_out <= ~(write_read_n_in);
        sram_a_lb_n_out <= ~(__tmp_1[2'd0]);
        sram_a_ub_n_out <= ~(__tmp_2[2'd1]);
        sram_b_lb_n_out <= ~(__tmp_3[2'd2]);
        sram_b_ub_n_out <= ~(__tmp_4[2'd3]);
    end else begin
        sram_we_n_out <= 1'd1;
        sram_a_lb_n_out <= 1'd1;
        sram_a_ub_n_out <= 1'd1;
        sram_b_lb_n_out <= 1'd1;
        sram_b_ub_n_out <= 1'd1;
    end
end

always_ff @(posedge clk) begin
    sram_address_out <= address_in;
    sram_a_data_out_reg <= __tmp_5[15:0];
    sram_b_data_out_reg <= __tmp_6[31:16];
end

always_ff @(negedge clk) begin
    sram_a_data_in_reg <= sram_a_data_in;
    sram_b_data_in_reg <= sram_b_data_in;
end

always_ff @(posedge clk) begin
    sram_we_reg <= write_read_n_in;
end

always_comb begin
    sram_a_ce.reset = reset;
    sram_a_ce.clk = clk;
    sram_b_ce.reset = reset;
    sram_b_ce.clk = clk;
end

always_comb begin
    sram_a_oe.oe_n_in = __tmp_7;
    sram_b_oe.oe_n_in = __tmp_8;
    sram_a_oe.clk = clk2;
    sram_b_oe.clk = clk2;
end

always_comb begin
    sram_we_out = sram_we_reg;
    sram_a_ce_n_out = sram_a_ce.ce_n_out;
    sram_b_ce_n_out = sram_b_ce.ce_n_out;
    sram_a_oe_n_out = sram_a_oe.oe_n_out;
    sram_b_oe_n_out = sram_b_oe.oe_n_out;
    sram_a_data_out = sram_a_data_out_reg;
    sram_b_data_out = sram_b_data_out_reg;
    data_out = {sram_b_data_in_reg, sram_a_data_in_reg};
end


endmodule

typedef struct packed {
    Flags flags;
    Condition condition;
    logic conditional;
} ConditionUnit__Interface;

module ConditionUnit (
    input var Flags flags,
    input var Condition condition,
    output var logic conditional
);


var logic __tmp_0;
always_comb begin
    case (condition)
        Condition__Never: begin
            __tmp_0 = 1'd0;
        end
        Condition__Carry: begin
            __tmp_0 = flags.carry;
        end
        Condition__Zero: begin
            __tmp_0 = flags.zero;
        end
        Condition__Signed: begin
            __tmp_0 = flags.sign;
        end
        Condition__Overflow: begin
            __tmp_0 = flags.overflow;
        end
        Condition__NotCarry: begin
            __tmp_0 = ~(flags.carry);
        end
        Condition__NotZero: begin
            __tmp_0 = ~(flags.zero);
        end
        Condition__NotSigned: begin
            __tmp_0 = ~(flags.sign);
        end
        Condition__NotOverflow: begin
            __tmp_0 = ~(flags.overflow);
        end
        Condition__UnsignedLessEqual: begin
            __tmp_0 = (~(flags.carry)) | (flags.zero);
        end
        Condition__UnsignedGreater: begin
            __tmp_0 = (flags.carry) & (~(flags.zero));
        end
        Condition__SignedLess: begin
            __tmp_0 = (flags.sign) != (flags.overflow);
        end
        Condition__SignedGreaterEqual: begin
            __tmp_0 = (flags.sign) == (flags.overflow);
        end
        Condition__SignedLessEqual: begin
            __tmp_0 = (flags.zero) | ((flags.sign) != (flags.overflow));
        end
        Condition__SignedGreater: begin
            __tmp_0 = (~(flags.zero)) & ((flags.sign) == (flags.overflow));
        end
        default: begin
            __tmp_0 = 1'd1;
        end
    endcase
end

always_comb begin
    conditional = __tmp_0;
end


endmodule

typedef struct packed {
    logic[4:0] lhs_select;
    logic[4:0] rhs_select;
    logic[4:0] load_select;
    logic[31:0] data_in;
    logic[31:0] lhs_out;
    logic[31:0] rhs_out;
    logic enable;
    logic reset;
    logic clk;
} RegisterFile__32_32__Interface;

module RegisterFile__32_32 (
    input var logic[4:0] lhs_select,
    input var logic[4:0] rhs_select,
    input var logic[4:0] load_select,
    input var logic[31:0] data_in,
    output var logic[31:0] lhs_out,
    output var logic[31:0] rhs_out,
    input var logic enable,
    input var logic reset,
    input var logic clk
);

var logic[31:0] regs[31:0];

var logic[31:0] __tmp_0[31:0];
var logic[31:0] __tmp_1[31:0];
always_comb begin
    __tmp_0 = regs;
    __tmp_1 = regs;
end

always_ff @(posedge clk) begin
    if (reset) begin
        regs[5'd0] <= 32'd0;
    end else if ((enable) & ((load_select) != (5'd0))) begin
        regs[load_select] <= data_in;
    end
end

always_comb begin
    lhs_out = __tmp_0[lhs_select];
    rhs_out = __tmp_1[rhs_select];
end


endmodule

typedef struct packed {
    logic[31:0] unpacked_data_in;
    logic[31:0] unpacked_data_out;
    MemoryMode mode_in;
    logic[1:0] byte_address_in;
    logic sign_extend_in;
    logic write_in;
    logic[31:0] packed_data_in;
    logic[31:0] packed_data_out;
    logic[3:0] byte_enable_out;
    logic[31:0] io_data_in;
    logic[31:0] io_data_out;
    logic io_enable_out;
    logic clk;
} DataSwizzle__Interface;

module DataSwizzle (
    input var logic[31:0] unpacked_data_in,
    output var logic[31:0] unpacked_data_out,
    input var MemoryMode mode_in,
    input var logic[1:0] byte_address_in,
    input var logic sign_extend_in,
    input var logic write_in,
    input var logic[31:0] packed_data_in,
    output var logic[31:0] packed_data_out,
    output var logic[3:0] byte_enable_out,
    input var logic[31:0] io_data_in,
    output var logic[31:0] io_data_out,
    output var logic io_enable_out,
    input var logic clk
);

var logic[7:0] unpacked_data_in_8;
var logic[15:0] unpacked_data_in_16;
var MemoryMode mode_reg_0;
var MemoryMode mode_reg_1;
var logic[1:0] byte_address_reg_0;
var logic[1:0] byte_address_reg_1;
var logic sign_extend_reg_0;
var logic sign_extend_reg_1;
var logic[7:0] packed_data_in_8;
var logic[15:0] packed_data_in_16;

var logic[3:0] __tmp_0;
var logic[3:0] __tmp_1;
var logic[3:0] __tmp_2;
var logic[1:0] __tmp_3;
var logic[3:0] __tmp_4;
var logic __tmp_5;
var logic[1:0] __tmp_6;
var logic[31:0] __tmp_7;
var logic[31:0] __tmp_8;
var logic[31:0] __tmp_9;
var logic[7:0] __tmp_10;
var logic[1:0] __tmp_11;
var logic[31:0] __tmp_12;
var logic[31:0] __tmp_13;
var logic[31:0] __tmp_14;
var logic[31:0] __tmp_15;
var logic[15:0] __tmp_16;
var logic __tmp_17;
var logic[1:0] __tmp_18;
var logic[31:0] __tmp_19;
var logic[31:0] __tmp_20;
var logic[31:0] __tmp_21;
var logic[31:0] __tmp_22;
var logic[31:0] __tmp_23;
always_comb begin
    __tmp_3 = byte_address_in;
    if ((__tmp_3) == (2'd0)) begin
        __tmp_2 = 4'd1;
    end else if ((__tmp_3) == (2'd1)) begin
        __tmp_2 = 4'd2;
    end else if ((__tmp_3) == (2'd2)) begin
        __tmp_2 = 4'd4;
    end else begin
        __tmp_2 = 4'd8;
    end
    __tmp_6 = byte_address_in;
    __tmp_5 = __tmp_6[1'd1];
    if ((__tmp_5) == (1'd0)) begin
        __tmp_4 = 4'd3;
    end else begin
        __tmp_4 = 4'd12;
    end
    case (mode_in)
        MemoryMode__Bits32: begin
            __tmp_1 = 4'd15;
        end
        MemoryMode__Bits8: begin
            __tmp_1 = __tmp_2;
        end
        MemoryMode__Bits16: begin
            __tmp_1 = __tmp_4;
        end
        default: begin
            __tmp_1 = 4'd0;
        end
    endcase
    if (write_in) begin
        __tmp_0 = __tmp_1;
    end else begin
        __tmp_0 = 4'd15;
    end
    __tmp_7 = unpacked_data_in;
    __tmp_8 = unpacked_data_in;
    case (mode_in)
        MemoryMode__Bits32: begin
            __tmp_9 = unpacked_data_in;
        end
        MemoryMode__Bits8: begin
            __tmp_9 = {{{unpacked_data_in_8, unpacked_data_in_8}, unpacked_data_in_8}, unpacked_data_in_8};
        end
        MemoryMode__Bits16: begin
            __tmp_9 = {unpacked_data_in_16, unpacked_data_in_16};
        end
        default: begin
            __tmp_9 = unpacked_data_in;
        end
    endcase
    __tmp_11 = byte_address_reg_1;
    __tmp_12 = packed_data_in;
    __tmp_13 = packed_data_in;
    __tmp_14 = packed_data_in;
    __tmp_15 = packed_data_in;
    if ((__tmp_11) == (2'd0)) begin
        __tmp_10 = __tmp_12[7:0];
    end else if ((__tmp_11) == (2'd1)) begin
        __tmp_10 = __tmp_13[15:8];
    end else if ((__tmp_11) == (2'd2)) begin
        __tmp_10 = __tmp_14[23:16];
    end else begin
        __tmp_10 = __tmp_15[31:24];
    end
    __tmp_18 = byte_address_reg_1;
    __tmp_17 = __tmp_18[1'd1];
    __tmp_19 = packed_data_in;
    __tmp_20 = packed_data_in;
    if ((__tmp_17) == (1'd0)) begin
        __tmp_16 = __tmp_19[15:0];
    end else begin
        __tmp_16 = __tmp_20[31:16];
    end
    if (sign_extend_reg_1) begin
        __tmp_22 = $signed({packed_data_in_8, 24'd0}) >>> $signed(32'd24);
    end else begin
        __tmp_22 = {24'd0, packed_data_in_8};
    end
    if (sign_extend_reg_1) begin
        __tmp_23 = $signed({packed_data_in_16, 16'd0}) >>> $signed(32'd16);
    end else begin
        __tmp_23 = {16'd0, packed_data_in_16};
    end
    case (mode_reg_1)
        MemoryMode__Bits32: begin
            __tmp_21 = packed_data_in;
        end
        MemoryMode__Bits8: begin
            __tmp_21 = __tmp_22;
        end
        MemoryMode__Bits16: begin
            __tmp_21 = __tmp_23;
        end
        default: begin
            __tmp_21 = io_data_in;
        end
    endcase
end

always_ff @(posedge clk) begin
    mode_reg_0 <= mode_in;
    mode_reg_1 <= mode_reg_0;
    byte_address_reg_0 <= byte_address_in;
    byte_address_reg_1 <= byte_address_reg_0;
    sign_extend_reg_0 <= sign_extend_in;
    sign_extend_reg_1 <= sign_extend_reg_0;
end

always_comb begin
    byte_enable_out = __tmp_0;
    io_enable_out = (mode_in) == (MemoryMode__IO);
end

always_comb begin
    unpacked_data_in_8 = __tmp_7[7:0];
    unpacked_data_in_16 = __tmp_8[15:0];
end

always_comb begin
    packed_data_out = __tmp_9;
    io_data_out = unpacked_data_in;
end

always_comb begin
    packed_data_in_8 = __tmp_10;
    packed_data_in_16 = __tmp_16;
end

always_comb begin
    unpacked_data_out = __tmp_21;
end


endmodule

typedef struct packed {
    logic[9:0] r_in;
    logic[9:0] g_in;
    logic[9:0] b_in;
    logic r_out;
    logic g_out;
    logic b_out;
    logic reset;
    logic sclk;
    logic pclk;
} DviSerializer__Interface;

module DviSerializer (
    input var logic[9:0] r_in,
    input var logic[9:0] g_in,
    input var logic[9:0] b_in,
    output var logic r_out,
    output var logic g_out,
    output var logic b_out,
    input var logic reset,
    input var logic sclk,
    input var logic pclk
);

var logic ready;
var logic[9:0] r_reg;
var logic[9:0] g_reg;
var logic[9:0] b_reg;
var logic[2:0] counter;
var logic[1:0] r_ddr_reg;
var logic[1:0] g_ddr_reg;
var logic[1:0] b_ddr_reg;
var DDR__Interface ddr_r /*verilator split_var*/;
DDR ddr_r__instance (
    .d_in(ddr_r.d_in),
    .d_out(ddr_r.d_out),
    .reset(ddr_r.reset),
    .clk(ddr_r.clk)
);
var DDR__Interface ddr_g /*verilator split_var*/;
DDR ddr_g__instance (
    .d_in(ddr_g.d_in),
    .d_out(ddr_g.d_out),
    .reset(ddr_g.reset),
    .clk(ddr_g.clk)
);
var DDR__Interface ddr_b /*verilator split_var*/;
DDR ddr_b__instance (
    .d_in(ddr_b.d_in),
    .d_out(ddr_b.d_out),
    .reset(ddr_b.reset),
    .clk(ddr_b.clk)
);

var logic[2:0] __tmp_0;
var logic[2:0] __tmp_1;
var logic[9:0] __tmp_2;
var logic[9:0] __tmp_3;
var logic[9:0] __tmp_4;
var logic[9:0] __tmp_5;
var logic[9:0] __tmp_6;
var logic[9:0] __tmp_7;
var logic[9:0] __tmp_8;
var logic[9:0] __tmp_9;
var logic[9:0] __tmp_10;
var logic[9:0] __tmp_11;
var logic[9:0] __tmp_12;
var logic[9:0] __tmp_13;
var logic[9:0] __tmp_14;
var logic[9:0] __tmp_15;
var logic[9:0] __tmp_16;
always_comb begin
    if (((reset) | (~(ready))) | ((counter) == (3'd4))) begin
        __tmp_0 = 3'd0;
    end else begin
        __tmp_0 = (counter) + (3'd1);
    end
    __tmp_1 = counter;
    __tmp_2 = r_reg;
    __tmp_3 = g_reg;
    __tmp_4 = b_reg;
    __tmp_5 = r_reg;
    __tmp_6 = g_reg;
    __tmp_7 = b_reg;
    __tmp_8 = r_reg;
    __tmp_9 = g_reg;
    __tmp_10 = b_reg;
    __tmp_11 = r_reg;
    __tmp_12 = g_reg;
    __tmp_13 = b_reg;
    __tmp_14 = r_reg;
    __tmp_15 = g_reg;
    __tmp_16 = b_reg;
end

always_ff @(posedge pclk) begin
    if (reset) begin
        ready <= 1'd0;
        r_reg <= 10'd0;
        g_reg <= 10'd0;
        b_reg <= 10'd0;
    end else begin
        ready <= 1'd1;
        r_reg <= r_in;
        g_reg <= g_in;
        b_reg <= b_in;
    end
end

always_ff @(posedge sclk) begin
    counter <= __tmp_0;
end

always_ff @(posedge sclk) begin
    if ((__tmp_1) == (3'd0)) begin
        r_ddr_reg <= __tmp_2[1:0];
        g_ddr_reg <= __tmp_3[1:0];
        b_ddr_reg <= __tmp_4[1:0];
    end else if ((__tmp_1) == (3'd1)) begin
        r_ddr_reg <= __tmp_5[3:2];
        g_ddr_reg <= __tmp_6[3:2];
        b_ddr_reg <= __tmp_7[3:2];
    end else if ((__tmp_1) == (3'd2)) begin
        r_ddr_reg <= __tmp_8[5:4];
        g_ddr_reg <= __tmp_9[5:4];
        b_ddr_reg <= __tmp_10[5:4];
    end else if ((__tmp_1) == (3'd3)) begin
        r_ddr_reg <= __tmp_11[7:6];
        g_ddr_reg <= __tmp_12[7:6];
        b_ddr_reg <= __tmp_13[7:6];
    end else if ((__tmp_1) == (3'd4)) begin
        r_ddr_reg <= __tmp_14[9:8];
        g_ddr_reg <= __tmp_15[9:8];
        b_ddr_reg <= __tmp_16[9:8];
    end else begin
        r_ddr_reg <= 2'd0;
        g_ddr_reg <= 2'd0;
        b_ddr_reg <= 2'd0;
    end
end

always_comb begin
    ddr_r.d_in = r_ddr_reg;
    r_out = ddr_r.d_out;
    ddr_r.reset = reset;
    ddr_r.clk = sclk;
end

always_comb begin
    ddr_g.d_in = g_ddr_reg;
    g_out = ddr_g.d_out;
    ddr_g.reset = reset;
    ddr_g.clk = sclk;
end

always_comb begin
    ddr_b.d_in = b_ddr_reg;
    b_out = ddr_b.d_out;
    ddr_b.reset = reset;
    ddr_b.clk = sclk;
end


endmodule

typedef struct packed {
    logic[9:0] cpu_address;
    logic[31:0] cpu_data_in;
    logic[31:0] cpu_data_out;
    logic[3:0] cpu_byte_enable;
    logic cpu_write;
    logic[5:0] vdp_palette_index;
    logic[3:0] vdp_color_index;
    logic[23:0] vdp_color_out;
    logic cpu_clk;
    logic vdp_clk;
} PaletteMemory__Interface;

module PaletteMemory (
    input var logic[9:0] cpu_address,
    input var logic[31:0] cpu_data_in,
    output var logic[31:0] cpu_data_out,
    input var logic[3:0] cpu_byte_enable,
    input var logic cpu_write,
    input var logic[5:0] vdp_palette_index,
    input var logic[3:0] vdp_color_index,
    output var logic[23:0] vdp_color_out,
    input var logic cpu_clk,
    input var logic vdp_clk
);

(* no_rw_check *) var logic[23:0] mem[1023:0];

var logic[3:0] __tmp_0;
var logic[31:0] __tmp_1;
var logic[3:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[3:0] __tmp_4;
var logic[31:0] __tmp_5;
var logic[23:0] __tmp_6[1023:0];
var logic[23:0] __tmp_7[1023:0];
always_comb begin
    __tmp_0 = cpu_byte_enable;
    __tmp_1 = cpu_data_in;
    __tmp_2 = cpu_byte_enable;
    __tmp_3 = cpu_data_in;
    __tmp_4 = cpu_byte_enable;
    __tmp_5 = cpu_data_in;
    __tmp_6 = mem;
    __tmp_7 = mem;
end

always_ff @(posedge cpu_clk) begin
    if (cpu_write) begin
        if (__tmp_0[2'd0]) begin
            mem[cpu_address][7:0] <= __tmp_1[7:0];
        end
        if (__tmp_2[2'd1]) begin
            mem[cpu_address][15:8] <= __tmp_3[15:8];
        end
        if (__tmp_4[2'd2]) begin
            mem[cpu_address][23:16] <= __tmp_5[23:16];
        end
    end else begin
        cpu_data_out <= {8'd0, __tmp_6[cpu_address]};
    end
end

always_ff @(posedge vdp_clk) begin
    vdp_color_out <= __tmp_7[{vdp_palette_index, vdp_color_index}];
end


endmodule

typedef struct packed {
    logic[12:0] cpu_address;
    logic[31:0] cpu_data_in;
    logic[31:0] cpu_data_out;
    logic[3:0] cpu_byte_enable;
    logic cpu_write;
    logic[9:0] vdp_bitmap_index;
    logic[2:0] vdp_bitmap_row;
    logic[31:0] vdp_row_data_out;
    logic cpu_clk;
    logic vdp_clk;
} BitmapMemory__Interface;

module BitmapMemory (
    input var logic[12:0] cpu_address,
    input var logic[31:0] cpu_data_in,
    output var logic[31:0] cpu_data_out,
    input var logic[3:0] cpu_byte_enable,
    input var logic cpu_write,
    input var logic[9:0] vdp_bitmap_index,
    input var logic[2:0] vdp_bitmap_row,
    output var logic[31:0] vdp_row_data_out,
    input var logic cpu_clk,
    input var logic vdp_clk
);

(* no_rw_check *) var logic[31:0] mem[8191:0];

var logic[3:0] __tmp_0;
var logic[31:0] __tmp_1;
var logic[3:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[3:0] __tmp_4;
var logic[31:0] __tmp_5;
var logic[3:0] __tmp_6;
var logic[31:0] __tmp_7;
var logic[31:0] __tmp_8[8191:0];
var logic[31:0] __tmp_9[8191:0];
always_comb begin
    __tmp_0 = cpu_byte_enable;
    __tmp_1 = cpu_data_in;
    __tmp_2 = cpu_byte_enable;
    __tmp_3 = cpu_data_in;
    __tmp_4 = cpu_byte_enable;
    __tmp_5 = cpu_data_in;
    __tmp_6 = cpu_byte_enable;
    __tmp_7 = cpu_data_in;
    __tmp_8 = mem;
    __tmp_9 = mem;
end

always_ff @(posedge cpu_clk) begin
    if (cpu_write) begin
        if (__tmp_0[2'd0]) begin
            mem[cpu_address][7:0] <= __tmp_1[7:0];
        end
        if (__tmp_2[2'd1]) begin
            mem[cpu_address][15:8] <= __tmp_3[15:8];
        end
        if (__tmp_4[2'd2]) begin
            mem[cpu_address][23:16] <= __tmp_5[23:16];
        end
        if (__tmp_6[2'd3]) begin
            mem[cpu_address][31:24] <= __tmp_7[31:24];
        end
    end else begin
        cpu_data_out <= __tmp_8[cpu_address];
    end
end

always_ff @(posedge vdp_clk) begin
    vdp_row_data_out <= __tmp_9[{vdp_bitmap_index, vdp_bitmap_row}];
end


endmodule

typedef struct packed {
    logic[7:0] r_in;
    logic[7:0] g_in;
    logic[7:0] b_in;
    TmdsMode mode;
    logic hsync;
    logic vsync;
    logic r_out;
    logic g_out;
    logic b_out;
    logic reset;
    logic sclk;
    logic pclk;
} DviController__Interface;

module DviController (
    input var logic[7:0] r_in,
    input var logic[7:0] g_in,
    input var logic[7:0] b_in,
    input var TmdsMode mode,
    input var logic hsync,
    input var logic vsync,
    output var logic r_out,
    output var logic g_out,
    output var logic b_out,
    input var logic reset,
    input var logic sclk,
    input var logic pclk
);

var logic[7:0] r_reg;
var logic[7:0] g_reg;
var logic[7:0] b_reg;
var TmdsMode mode_reg;
var logic[1:0] control_data_reg;
var TmdsEncoder__Interface b_encoder /*verilator split_var*/;
TmdsEncoder b_encoder__instance (
    .mode(b_encoder.mode),
    .control_data(b_encoder.control_data),
    .video_data(b_encoder.video_data),
    .encoded(b_encoder.encoded),
    .reset(b_encoder.reset),
    .pclk(b_encoder.pclk)
);
var TmdsEncoder__Interface g_encoder /*verilator split_var*/;
TmdsEncoder g_encoder__instance (
    .mode(g_encoder.mode),
    .control_data(g_encoder.control_data),
    .video_data(g_encoder.video_data),
    .encoded(g_encoder.encoded),
    .reset(g_encoder.reset),
    .pclk(g_encoder.pclk)
);
var TmdsEncoder__Interface r_encoder /*verilator split_var*/;
TmdsEncoder r_encoder__instance (
    .mode(r_encoder.mode),
    .control_data(r_encoder.control_data),
    .video_data(r_encoder.video_data),
    .encoded(r_encoder.encoded),
    .reset(r_encoder.reset),
    .pclk(r_encoder.pclk)
);
var DviSerializer__Interface serializer /*verilator split_var*/;
DviSerializer serializer__instance (
    .r_in(serializer.r_in),
    .g_in(serializer.g_in),
    .b_in(serializer.b_in),
    .r_out(serializer.r_out),
    .g_out(serializer.g_out),
    .b_out(serializer.b_out),
    .reset(serializer.reset),
    .sclk(serializer.sclk),
    .pclk(serializer.pclk)
);

always_ff @(posedge pclk) begin
    if (reset) begin
        r_reg <= 8'd0;
        g_reg <= 8'd0;
        b_reg <= 8'd0;
    end else begin
        r_reg <= r_in;
        g_reg <= g_in;
        b_reg <= b_in;
    end
end

always_ff @(posedge pclk) begin
    if (reset) begin
        mode_reg <= TmdsMode__Control;
        control_data_reg <= 2'd0;
    end else begin
        mode_reg <= mode;
        control_data_reg <= {vsync, hsync};
    end
end

always_comb begin
    b_encoder.mode = mode_reg;
    b_encoder.control_data = control_data_reg;
    b_encoder.video_data = b_reg;
    b_encoder.reset = reset;
    b_encoder.pclk = pclk;
end

always_comb begin
    g_encoder.mode = mode_reg;
    g_encoder.control_data = 2'd0;
    g_encoder.video_data = g_reg;
    g_encoder.reset = reset;
    g_encoder.pclk = pclk;
end

always_comb begin
    r_encoder.mode = mode_reg;
    r_encoder.control_data = 2'd0;
    r_encoder.video_data = r_reg;
    r_encoder.reset = reset;
    r_encoder.pclk = pclk;
end

always_comb begin
    serializer.r_in = r_encoder.encoded;
    serializer.g_in = g_encoder.encoded;
    serializer.b_in = b_encoder.encoded;
    r_out = serializer.r_out;
    g_out = serializer.g_out;
    b_out = serializer.b_out;
    serializer.reset = reset;
    serializer.sclk = sclk;
    serializer.pclk = pclk;
end


endmodule

typedef struct packed {
    logic[11:0] cpu_address;
    logic[31:0] cpu_data_in;
    logic[31:0] cpu_data_out;
    logic[3:0] cpu_byte_enable;
    logic cpu_write;
    logic[5:0] vdp_tile_column;
    logic[5:0] vdp_tile_row;
    logic[9:0] vdp_bitmap_index_out;
    logic[5:0] vdp_palette_index_out;
    logic cpu_clk;
    logic vdp_clk;
} TilemapMemory__Interface;

module TilemapMemory (
    input var logic[11:0] cpu_address,
    input var logic[31:0] cpu_data_in,
    output var logic[31:0] cpu_data_out,
    input var logic[3:0] cpu_byte_enable,
    input var logic cpu_write,
    input var logic[5:0] vdp_tile_column,
    input var logic[5:0] vdp_tile_row,
    output var logic[9:0] vdp_bitmap_index_out,
    output var logic[5:0] vdp_palette_index_out,
    input var logic cpu_clk,
    input var logic vdp_clk
);

(* no_rw_check *) var logic[31:0] tiles[4095:0];
var logic[15:0] tile_out;

var logic[3:0] __tmp_0;
var logic[31:0] __tmp_1;
var logic[3:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[3:0] __tmp_4;
var logic[31:0] __tmp_5;
var logic[3:0] __tmp_6;
var logic[31:0] __tmp_7;
var logic[31:0] __tmp_8;
var logic[31:0] __tmp_9[4095:0];
var logic[31:0] __tmp_10;
var logic[31:0] __tmp_11[4095:0];
var logic[15:0] __tmp_12;
var logic[15:0] __tmp_13;
always_comb begin
    __tmp_0 = cpu_byte_enable;
    __tmp_1 = cpu_data_in;
    __tmp_2 = cpu_byte_enable;
    __tmp_3 = cpu_data_in;
    __tmp_4 = cpu_byte_enable;
    __tmp_5 = cpu_data_in;
    __tmp_6 = cpu_byte_enable;
    __tmp_7 = cpu_data_in;
    __tmp_9 = tiles;
    __tmp_8 = __tmp_9[cpu_address];
    __tmp_11 = tiles;
    __tmp_10 = __tmp_11[{vdp_tile_column, vdp_tile_row}];
    __tmp_12 = tile_out;
    __tmp_13 = tile_out;
end

always_ff @(posedge cpu_clk) begin
    if (cpu_write) begin
        if (__tmp_0[2'd0]) begin
            tiles[cpu_address][7:0] <= __tmp_1[7:0];
        end
        if (__tmp_2[2'd1]) begin
            tiles[cpu_address][15:8] <= __tmp_3[15:8];
        end
        if (__tmp_4[2'd2]) begin
            tiles[cpu_address][23:16] <= __tmp_5[23:16];
        end
        if (__tmp_6[2'd3]) begin
            tiles[cpu_address][31:24] <= __tmp_7[31:24];
        end
    end else begin
        cpu_data_out <= {16'd0, __tmp_8[15:0]};
    end
end

always_ff @(posedge vdp_clk) begin
    tile_out <= __tmp_10[15:0];
end

always_comb begin
    vdp_bitmap_index_out = __tmp_12[9:0];
    vdp_palette_index_out = __tmp_13[15:10];
end


endmodule

typedef struct packed {
    logic[21:0] cpu_address;
    logic[31:0] cpu_data_in;
    logic[31:0] cpu_data_out;
    logic[3:0] cpu_byte_enable;
    logic cpu_write;
    logic[9:0] vdp_bitmap_index;
    logic[2:0] vdp_bitmap_row;
    logic[31:0] vdp_row_data_out;
    logic[5:0] vdp_palette_index;
    logic[3:0] vdp_color_index;
    logic[23:0] vdp_color_out;
    logic[5:0] vdp_tile_column;
    logic[5:0] vdp_tile_row;
    logic[9:0] vdp_bitmap_index_out;
    logic[5:0] vdp_palette_index_out;
    logic cpu_clk;
    logic vdp_clk;
} VideoRam__Interface;

module VideoRam (
    input var logic[21:0] cpu_address,
    input var logic[31:0] cpu_data_in,
    output var logic[31:0] cpu_data_out,
    input var logic[3:0] cpu_byte_enable,
    input var logic cpu_write,
    input var logic[9:0] vdp_bitmap_index,
    input var logic[2:0] vdp_bitmap_row,
    output var logic[31:0] vdp_row_data_out,
    input var logic[5:0] vdp_palette_index,
    input var logic[3:0] vdp_color_index,
    output var logic[23:0] vdp_color_out,
    input var logic[5:0] vdp_tile_column,
    input var logic[5:0] vdp_tile_row,
    output var logic[9:0] vdp_bitmap_index_out,
    output var logic[5:0] vdp_palette_index_out,
    input var logic cpu_clk,
    input var logic vdp_clk
);

var BitmapMemory__Interface bitmaps /*verilator split_var*/;
BitmapMemory bitmaps__instance (
    .cpu_address(bitmaps.cpu_address),
    .cpu_data_in(bitmaps.cpu_data_in),
    .cpu_data_out(bitmaps.cpu_data_out),
    .cpu_byte_enable(bitmaps.cpu_byte_enable),
    .cpu_write(bitmaps.cpu_write),
    .vdp_bitmap_index(bitmaps.vdp_bitmap_index),
    .vdp_bitmap_row(bitmaps.vdp_bitmap_row),
    .vdp_row_data_out(bitmaps.vdp_row_data_out),
    .cpu_clk(bitmaps.cpu_clk),
    .vdp_clk(bitmaps.vdp_clk)
);
var PaletteMemory__Interface palettes /*verilator split_var*/;
PaletteMemory palettes__instance (
    .cpu_address(palettes.cpu_address),
    .cpu_data_in(palettes.cpu_data_in),
    .cpu_data_out(palettes.cpu_data_out),
    .cpu_byte_enable(palettes.cpu_byte_enable),
    .cpu_write(palettes.cpu_write),
    .vdp_palette_index(palettes.vdp_palette_index),
    .vdp_color_index(palettes.vdp_color_index),
    .vdp_color_out(palettes.vdp_color_out),
    .cpu_clk(palettes.cpu_clk),
    .vdp_clk(palettes.vdp_clk)
);
var TilemapMemory__Interface tilemap /*verilator split_var*/;
TilemapMemory tilemap__instance (
    .cpu_address(tilemap.cpu_address),
    .cpu_data_in(tilemap.cpu_data_in),
    .cpu_data_out(tilemap.cpu_data_out),
    .cpu_byte_enable(tilemap.cpu_byte_enable),
    .cpu_write(tilemap.cpu_write),
    .vdp_tile_column(tilemap.vdp_tile_column),
    .vdp_tile_row(tilemap.vdp_tile_row),
    .vdp_bitmap_index_out(tilemap.vdp_bitmap_index_out),
    .vdp_palette_index_out(tilemap.vdp_palette_index_out),
    .cpu_clk(tilemap.cpu_clk),
    .vdp_clk(tilemap.vdp_clk)
);
var logic[1:0] select_reg;

var logic[21:0] __tmp_0;
var logic[21:0] __tmp_1;
var logic[21:0] __tmp_2;
var logic[21:0] __tmp_3;
var logic[21:0] __tmp_4;
var logic[21:0] __tmp_5;
var logic[21:0] __tmp_6;
var logic[31:0] __tmp_7;
var logic[1:0] __tmp_8;
always_comb begin
    __tmp_0 = cpu_address;
    __tmp_1 = cpu_address;
    __tmp_2 = cpu_address;
    __tmp_3 = cpu_address;
    __tmp_4 = cpu_address;
    __tmp_5 = cpu_address;
    __tmp_6 = cpu_address;
    __tmp_8 = select_reg;
    if ((__tmp_8) == (2'd0)) begin
        __tmp_7 = bitmaps.cpu_data_out;
    end else if ((__tmp_8) == (2'd1)) begin
        __tmp_7 = palettes.cpu_data_out;
    end else if ((__tmp_8) == (2'd2)) begin
        __tmp_7 = tilemap.cpu_data_out;
    end else begin
        __tmp_7 = 32'd0;
    end
end

always_ff @(posedge cpu_clk) begin
    select_reg <= __tmp_0[21:20];
end

always_comb begin
    bitmaps.cpu_address = __tmp_1[12:0];
    bitmaps.cpu_data_in = cpu_data_in;
    bitmaps.cpu_byte_enable = cpu_byte_enable;
    bitmaps.cpu_write = (cpu_write) & ((__tmp_2[21:20]) == (2'd0));
    bitmaps.vdp_bitmap_index = vdp_bitmap_index;
    bitmaps.vdp_bitmap_row = vdp_bitmap_row;
    vdp_row_data_out = bitmaps.vdp_row_data_out;
    bitmaps.cpu_clk = cpu_clk;
    bitmaps.vdp_clk = vdp_clk;
end

always_comb begin
    palettes.cpu_address = __tmp_3[9:0];
    palettes.cpu_data_in = cpu_data_in;
    palettes.cpu_byte_enable = cpu_byte_enable;
    palettes.cpu_write = (cpu_write) & ((__tmp_4[21:20]) == (2'd1));
    palettes.vdp_palette_index = vdp_palette_index;
    palettes.vdp_color_index = vdp_color_index;
    vdp_color_out = palettes.vdp_color_out;
    palettes.cpu_clk = cpu_clk;
    palettes.vdp_clk = vdp_clk;
end

always_comb begin
    tilemap.cpu_address = __tmp_5[11:0];
    tilemap.cpu_data_in = cpu_data_in;
    tilemap.cpu_byte_enable = cpu_byte_enable;
    tilemap.cpu_write = (cpu_write) & ((__tmp_6[21:20]) == (2'd2));
    tilemap.vdp_tile_column = vdp_tile_column;
    tilemap.vdp_tile_row = vdp_tile_row;
    vdp_bitmap_index_out = tilemap.vdp_bitmap_index_out;
    vdp_palette_index_out = tilemap.vdp_palette_index_out;
    tilemap.cpu_clk = cpu_clk;
    tilemap.vdp_clk = vdp_clk;
end

always_comb begin
    cpu_data_out = __tmp_7;
end


endmodule

typedef struct packed {
    logic[31:0] data_in;
    logic inc;
    logic load;
    logic[31:0] pc_next_out;
    logic[31:0] pc_value_out;
    logic enable;
    logic reset;
    logic clk;
} ProgramCounter__32__Interface;

module ProgramCounter__32 (
    input var logic[31:0] data_in,
    input var logic inc,
    input var logic load,
    output var logic[31:0] pc_next_out,
    output var logic[31:0] pc_value_out,
    input var logic enable,
    input var logic reset,
    input var logic clk
);

var logic[29:0] pc_next;
var logic[29:0] pc_value;

var logic[29:0] __tmp_0;
var logic[31:0] __tmp_1;
always_comb begin
    __tmp_1 = data_in;
    if (reset) begin
        __tmp_0 = 30'd0;
    end else if ((enable) & (load)) begin
        __tmp_0 = __tmp_1[31:2];
    end else if ((enable) & (inc)) begin
        __tmp_0 = (pc_value) + (30'd1);
    end else begin
        __tmp_0 = pc_value;
    end
end

always_ff @(posedge clk) begin
    pc_value <= pc_next;
end

always_comb begin
    pc_next = __tmp_0;
end

always_comb begin
    pc_next_out = {pc_next, 2'd0};
    pc_value_out = {pc_value, 2'd0};
end


endmodule

typedef struct packed {
    logic[31:0] instruction_word;
    OpCode op_code;
    logic[31:0] imm;
} ImmediateDecoder__Interface;

module ImmediateDecoder (
    input var logic[31:0] instruction_word,
    input var OpCode op_code,
    output var logic[31:0] imm
);

var logic[14:0] imm_15;
var logic[21:0] imm_22;
var logic[19:0] uimm_20;

var logic[31:0] __tmp_0;
var logic[31:0] __tmp_1;
var logic[31:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[31:0] __tmp_4;
var logic[31:0] __tmp_5;
var logic[31:0] __tmp_6;
var logic[31:0] __tmp_7;
always_comb begin
    __tmp_0 = instruction_word;
    __tmp_1 = instruction_word;
    __tmp_2 = instruction_word;
    __tmp_3 = instruction_word;
    __tmp_4 = instruction_word;
    __tmp_5 = instruction_word;
    __tmp_6 = instruction_word;
    case (op_code)
        OpCode__System: begin
            __tmp_7 = 32'd0;
        end
        OpCode__Branch: begin
            __tmp_7 = $signed({imm_22, 10'd0}) >>> $signed(32'd10);
        end
        OpCode__UI: begin
            __tmp_7 = {uimm_20, 12'd0};
        end
        default: begin
            __tmp_7 = $signed({imm_15, 17'd0}) >>> $signed(32'd17);
        end
    endcase
end

always_comb begin
    imm_15 = __tmp_0[31:17];
    imm_22 = {{{__tmp_1[5'd31], __tmp_2[18:12]}, __tmp_3[30:19]}, 2'd0};
    uimm_20 = {{__tmp_4[5'd31], __tmp_5[28:12]}, __tmp_6[30:29]};
end

always_comb begin
    imm = __tmp_7;
end


endmodule

typedef struct packed {
    logic[31:0] instruction_word;
    AluOp alu_op_out;
    Condition move_condition_out;
    Condition jump_condition_out;
    logic load_flags_out;
    logic[4:0] reg_lhs_select_out;
    logic[4:0] reg_rhs_select_out;
    logic[4:0] reg_load_select_out;
    LhsBusSource lhs_bus_source_out;
    RhsBusSource rhs_bus_source_out;
    DataBusSource data_bus_source_out;
    logic mem_enable_out;
    MemoryMode mem_mode_out;
    logic mem_sign_ext_out;
    logic mem_write_out;
    logic set_k_flag_out;
    logic clear_k_flag_out;
    logic[31:0] imm_out;
} InstructionDecoder__Interface;

module InstructionDecoder (
    input var logic[31:0] instruction_word,
    output var AluOp alu_op_out,
    output var Condition move_condition_out,
    output var Condition jump_condition_out,
    output var logic load_flags_out,
    output var logic[4:0] reg_lhs_select_out,
    output var logic[4:0] reg_rhs_select_out,
    output var logic[4:0] reg_load_select_out,
    output var LhsBusSource lhs_bus_source_out,
    output var RhsBusSource rhs_bus_source_out,
    output var DataBusSource data_bus_source_out,
    output var logic mem_enable_out,
    output var MemoryMode mem_mode_out,
    output var logic mem_sign_ext_out,
    output var logic mem_write_out,
    output var logic set_k_flag_out,
    output var logic clear_k_flag_out,
    output var logic[31:0] imm_out
);

var AluOp alu_op;
var Condition condition;
var MemoryMode mem_mode;
var OpCode op_code;
var logic[4:0] reg_load;
var logic[4:0] reg_lhs;
var logic[4:0] reg_rhs;
var ImmediateDecoder__Interface imm_decoder /*verilator split_var*/;
ImmediateDecoder imm_decoder__instance (
    .instruction_word(imm_decoder.instruction_word),
    .op_code(imm_decoder.op_code),
    .imm(imm_decoder.imm)
);

var AluOp __tmp_0;
var logic[3:0] __tmp_1;
var logic[31:0] __tmp_2;
var Condition __tmp_3;
var logic[3:0] __tmp_4;
var logic[31:0] __tmp_5;
var MemoryMode __tmp_6;
var logic[1:0] __tmp_7;
var logic[31:0] __tmp_8;
var OpCode __tmp_9;
var logic[2:0] __tmp_10;
var logic[31:0] __tmp_11;
var OpCode __tmp_12;
var logic __tmp_13;
var logic[31:0] __tmp_14;
var OpCode __tmp_15;
var OpCode __tmp_16;
var OpCode __tmp_17;
var logic __tmp_18;
var logic[31:0] __tmp_19;
var OpCode __tmp_20;
var logic __tmp_21;
var logic[31:0] __tmp_22;
var OpCode __tmp_23;
var OpCode __tmp_24;
var logic[31:0] __tmp_25;
var logic[31:0] __tmp_26;
var logic[31:0] __tmp_27;
var logic __tmp_28;
var logic[31:0] __tmp_29;
var logic[31:0] __tmp_30;
var logic __tmp_31;
var logic[31:0] __tmp_32;
always_comb begin
    __tmp_2 = instruction_word;
    __tmp_1 = __tmp_2[6:3];
    if ((__tmp_1) == (4'd0)) begin
        __tmp_0 = AluOp__Add;
    end else if ((__tmp_1) == (4'd1)) begin
        __tmp_0 = AluOp__AddC;
    end else if ((__tmp_1) == (4'd2)) begin
        __tmp_0 = AluOp__Sub;
    end else if ((__tmp_1) == (4'd3)) begin
        __tmp_0 = AluOp__SubB;
    end else if ((__tmp_1) == (4'd4)) begin
        __tmp_0 = AluOp__And;
    end else if ((__tmp_1) == (4'd5)) begin
        __tmp_0 = AluOp__Or;
    end else if ((__tmp_1) == (4'd6)) begin
        __tmp_0 = AluOp__Xor;
    end else if ((__tmp_1) == (4'd7)) begin
        __tmp_0 = AluOp__Shl;
    end else if ((__tmp_1) == (4'd8)) begin
        __tmp_0 = AluOp__Lsr;
    end else if ((__tmp_1) == (4'd9)) begin
        __tmp_0 = AluOp__Asr;
    end else if ((__tmp_1) == (4'd10)) begin
        __tmp_0 = AluOp__Mul;
    end else if ((__tmp_1) == (4'd11)) begin
        __tmp_0 = AluOp__MulHuu;
    end else if ((__tmp_1) == (4'd12)) begin
        __tmp_0 = AluOp__MulHss;
    end else if ((__tmp_1) == (4'd13)) begin
        __tmp_0 = AluOp__MulHus;
    end else if ((__tmp_1) == (4'd14)) begin
        __tmp_0 = AluOp__Cond;
    end else begin
        __tmp_0 = AluOp__Nop;
    end
    __tmp_5 = instruction_word;
    __tmp_4 = __tmp_5[6:3];
    if ((__tmp_4) == (4'd0)) begin
        __tmp_3 = Condition__Never;
    end else if ((__tmp_4) == (4'd1)) begin
        __tmp_3 = Condition__Carry;
    end else if ((__tmp_4) == (4'd2)) begin
        __tmp_3 = Condition__Zero;
    end else if ((__tmp_4) == (4'd3)) begin
        __tmp_3 = Condition__Signed;
    end else if ((__tmp_4) == (4'd4)) begin
        __tmp_3 = Condition__Overflow;
    end else if ((__tmp_4) == (4'd5)) begin
        __tmp_3 = Condition__NotCarry;
    end else if ((__tmp_4) == (4'd6)) begin
        __tmp_3 = Condition__NotZero;
    end else if ((__tmp_4) == (4'd7)) begin
        __tmp_3 = Condition__NotSigned;
    end else if ((__tmp_4) == (4'd8)) begin
        __tmp_3 = Condition__NotOverflow;
    end else if ((__tmp_4) == (4'd9)) begin
        __tmp_3 = Condition__UnsignedLessEqual;
    end else if ((__tmp_4) == (4'd10)) begin
        __tmp_3 = Condition__UnsignedGreater;
    end else if ((__tmp_4) == (4'd11)) begin
        __tmp_3 = Condition__SignedLess;
    end else if ((__tmp_4) == (4'd12)) begin
        __tmp_3 = Condition__SignedGreaterEqual;
    end else if ((__tmp_4) == (4'd13)) begin
        __tmp_3 = Condition__SignedLessEqual;
    end else if ((__tmp_4) == (4'd14)) begin
        __tmp_3 = Condition__SignedGreater;
    end else begin
        __tmp_3 = Condition__Always;
    end
    __tmp_8 = instruction_word;
    __tmp_7 = __tmp_8[4:3];
    if ((__tmp_7) == (2'd0)) begin
        __tmp_6 = MemoryMode__Bits32;
    end else if ((__tmp_7) == (2'd1)) begin
        __tmp_6 = MemoryMode__Bits8;
    end else if ((__tmp_7) == (2'd2)) begin
        __tmp_6 = MemoryMode__Bits16;
    end else begin
        __tmp_6 = MemoryMode__IO;
    end
    __tmp_11 = instruction_word;
    __tmp_10 = __tmp_11[2:0];
    __tmp_14 = instruction_word;
    __tmp_13 = __tmp_14[5'd6];
    if ((__tmp_13) == (1'd0)) begin
        __tmp_12 = OpCode__Nop;
    end else begin
        __tmp_12 = OpCode__System;
    end
    if ((alu_op) == (AluOp__Nop)) begin
        __tmp_15 = OpCode__Nop;
    end else begin
        __tmp_15 = OpCode__AluRegReg;
    end
    if ((alu_op) == (AluOp__Nop)) begin
        __tmp_16 = OpCode__Nop;
    end else begin
        __tmp_16 = OpCode__AluRegImm;
    end
    __tmp_19 = instruction_word;
    __tmp_18 = __tmp_19[5'd6];
    if ((__tmp_18) == (1'd0)) begin
        __tmp_17 = OpCode__Load;
    end else begin
        __tmp_17 = OpCode__Store;
    end
    __tmp_22 = instruction_word;
    __tmp_21 = __tmp_22[5'd6];
    if ((__tmp_21) == (1'd0)) begin
        __tmp_20 = OpCode__Jump;
    end else begin
        __tmp_20 = OpCode__UI;
    end
    if ((condition) == (Condition__Never)) begin
        __tmp_23 = OpCode__Nop;
    end else begin
        __tmp_23 = OpCode__MoveRegReg;
    end
    if ((condition) == (Condition__Never)) begin
        __tmp_24 = OpCode__Nop;
    end else begin
        __tmp_24 = OpCode__MoveRegImm;
    end
    if ((__tmp_10) == (3'd0)) begin
        __tmp_9 = __tmp_12;
    end else if ((__tmp_10) == (3'd1)) begin
        __tmp_9 = __tmp_15;
    end else if ((__tmp_10) == (3'd2)) begin
        __tmp_9 = __tmp_16;
    end else if ((__tmp_10) == (3'd3)) begin
        __tmp_9 = __tmp_17;
    end else if ((__tmp_10) == (3'd4)) begin
        __tmp_9 = __tmp_20;
    end else if ((__tmp_10) == (3'd5)) begin
        __tmp_9 = OpCode__Branch;
    end else if ((__tmp_10) == (3'd6)) begin
        __tmp_9 = __tmp_23;
    end else begin
        __tmp_9 = __tmp_24;
    end
    __tmp_25 = instruction_word;
    __tmp_26 = instruction_word;
    __tmp_27 = instruction_word;
    __tmp_29 = instruction_word;
    __tmp_28 = __tmp_29[5'd3];
    __tmp_30 = instruction_word;
    __tmp_32 = instruction_word;
    __tmp_31 = __tmp_32[5'd3];
end

always_comb begin
    alu_op = __tmp_0;
end

always_comb begin
    condition = __tmp_3;
end

always_comb begin
    mem_mode = __tmp_6;
end

always_comb begin
    op_code = __tmp_9;
end

always_comb begin
    reg_load = __tmp_25[11:7];
    reg_lhs = __tmp_26[16:12];
    reg_rhs = __tmp_27[21:17];
end

always_comb begin
    imm_decoder.instruction_word = instruction_word;
    imm_decoder.op_code = op_code;
    imm_out = imm_decoder.imm;
end

always_comb begin
    alu_op_out = AluOp__Nop;
    move_condition_out = Condition__Never;
    jump_condition_out = Condition__Never;
    load_flags_out = 1'd0;
    reg_lhs_select_out = 5'd0;
    reg_rhs_select_out = 5'd0;
    reg_load_select_out = 5'd0;
    lhs_bus_source_out = LhsBusSource__Register;
    rhs_bus_source_out = RhsBusSource__Register;
    data_bus_source_out = DataBusSource__Result;
    mem_enable_out = 1'd0;
    mem_mode_out = MemoryMode__Bits32;
    mem_sign_ext_out = 1'd0;
    mem_write_out = 1'd0;
    set_k_flag_out = 1'd0;
    clear_k_flag_out = 1'd0;
    case (op_code)
        OpCode__System: begin
            if ((__tmp_28) == (1'd0)) begin
                alu_op_out = AluOp__Cond;
                move_condition_out = Condition__Always;
                jump_condition_out = Condition__Always;
                reg_load_select_out = reg_load;
                lhs_bus_source_out = LhsBusSource__Syscall;
                rhs_bus_source_out = RhsBusSource__Pc;
                data_bus_source_out = DataBusSource__Result;
                set_k_flag_out = 1'd1;
            end else begin
                clear_k_flag_out = 1'd1;
            end
        end
        OpCode__AluRegReg: begin
            alu_op_out = alu_op;
            load_flags_out = 1'd1;
            reg_lhs_select_out = reg_lhs;
            reg_rhs_select_out = reg_rhs;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            rhs_bus_source_out = RhsBusSource__Register;
            data_bus_source_out = DataBusSource__Result;
        end
        OpCode__AluRegImm: begin
            alu_op_out = alu_op;
            load_flags_out = 1'd1;
            reg_lhs_select_out = reg_lhs;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            rhs_bus_source_out = RhsBusSource__Immediate;
            data_bus_source_out = DataBusSource__Result;
        end
        OpCode__Load: begin
            reg_lhs_select_out = reg_lhs;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            data_bus_source_out = DataBusSource__Memory;
            mem_enable_out = 1'd1;
            mem_mode_out = mem_mode;
            mem_sign_ext_out = __tmp_30[5'd5];
        end
        OpCode__Store: begin
            reg_lhs_select_out = reg_lhs;
            reg_rhs_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            rhs_bus_source_out = RhsBusSource__Register;
            mem_enable_out = 1'd1;
            mem_mode_out = mem_mode;
            mem_write_out = 1'd1;
        end
        OpCode__Branch: begin
            alu_op_out = AluOp__Cond;
            move_condition_out = Condition__Always;
            jump_condition_out = condition;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Pc;
            rhs_bus_source_out = RhsBusSource__Pc;
            data_bus_source_out = DataBusSource__Result;
        end
        OpCode__MoveRegReg: begin
            alu_op_out = AluOp__Cond;
            move_condition_out = condition;
            reg_lhs_select_out = reg_lhs;
            reg_rhs_select_out = reg_rhs;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            rhs_bus_source_out = RhsBusSource__Register;
            data_bus_source_out = DataBusSource__Result;
        end
        OpCode__MoveRegImm: begin
            alu_op_out = AluOp__Cond;
            move_condition_out = condition;
            reg_lhs_select_out = reg_lhs;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            rhs_bus_source_out = RhsBusSource__Immediate;
            data_bus_source_out = DataBusSource__Result;
        end
        OpCode__Jump: begin
            alu_op_out = AluOp__Cond;
            move_condition_out = Condition__Always;
            jump_condition_out = Condition__Always;
            reg_lhs_select_out = reg_lhs;
            reg_load_select_out = reg_load;
            lhs_bus_source_out = LhsBusSource__Register;
            rhs_bus_source_out = RhsBusSource__Pc;
            data_bus_source_out = DataBusSource__Result;
        end
        OpCode__UI: begin
            if ((__tmp_31) == (1'd0)) begin
                alu_op_out = AluOp__Cond;
                move_condition_out = Condition__Always;
                reg_load_select_out = reg_load;
                rhs_bus_source_out = RhsBusSource__Immediate;
                data_bus_source_out = DataBusSource__Result;
            end else begin
                alu_op_out = AluOp__Add;
                reg_load_select_out = reg_load;
                lhs_bus_source_out = LhsBusSource__Pc;
                rhs_bus_source_out = RhsBusSource__Immediate;
                data_bus_source_out = DataBusSource__Result;
            end
        end
        default: begin
        end
    endcase
end


endmodule

typedef struct packed {
    SyncState h_state;
    SyncState v_state;
    logic[9:0] h_counter;
    logic[9:0] v_counter;
    logic new_line;
    logic new_frame;
    logic reset;
    logic pclk;
} SyncGenerator__40_128_88_800_1_4_23_600__Interface;

module SyncGenerator__40_128_88_800_1_4_23_600 (
    output var SyncState h_state,
    output var SyncState v_state,
    output var logic[9:0] h_counter,
    output var logic[9:0] v_counter,
    output var logic new_line,
    output var logic new_frame,
    input var logic reset,
    input var logic pclk
);


always_ff @(posedge pclk) begin
    if (reset) begin
        h_state <= SyncState__Front;
        v_state <= SyncState__Front;
        h_counter <= 10'd0;
        v_counter <= 10'd0;
    end else begin
        case (h_state)
            SyncState__Front: begin
                if ((h_counter) == (10'd39)) begin
                    h_state <= SyncState__Sync;
                    h_counter <= 10'd0;
                end else begin
                    h_counter <= (h_counter) + (10'd1);
                end
            end
            SyncState__Sync: begin
                if ((h_counter) == (10'd127)) begin
                    h_state <= SyncState__Back;
                    h_counter <= 10'd0;
                end else begin
                    h_counter <= (h_counter) + (10'd1);
                end
            end
            SyncState__Back: begin
                if ((h_counter) == (10'd87)) begin
                    h_state <= SyncState__Active;
                    h_counter <= 10'd0;
                end else begin
                    h_counter <= (h_counter) + (10'd1);
                end
            end
            default: begin
                if ((h_counter) == (10'd799)) begin
                    h_state <= SyncState__Front;
                    h_counter <= 10'd0;
                end else begin
                    h_counter <= (h_counter) + (10'd1);
                end
            end
        endcase
        if (new_line) begin
            case (v_state)
                SyncState__Front: begin
                    if ((v_counter) == (10'd0)) begin
                        v_state <= SyncState__Sync;
                        v_counter <= 10'd0;
                    end else begin
                        v_counter <= (v_counter) + (10'd1);
                    end
                end
                SyncState__Sync: begin
                    if ((v_counter) == (10'd3)) begin
                        v_state <= SyncState__Back;
                        v_counter <= 10'd0;
                    end else begin
                        v_counter <= (v_counter) + (10'd1);
                    end
                end
                SyncState__Back: begin
                    if ((v_counter) == (10'd22)) begin
                        v_state <= SyncState__Active;
                        v_counter <= 10'd0;
                    end else begin
                        v_counter <= (v_counter) + (10'd1);
                    end
                end
                default: begin
                    if ((v_counter) == (10'd599)) begin
                        v_state <= SyncState__Front;
                        v_counter <= 10'd0;
                    end else begin
                        v_counter <= (v_counter) + (10'd1);
                    end
                end
            endcase
        end
    end
end

always_comb begin
    new_line = ((h_state) == (SyncState__Active)) & ((h_counter) == (10'd799));
    new_frame = ((v_state) == (SyncState__Active)) & ((v_counter) == (10'd599));
end


endmodule

typedef struct packed {
    logic[9:0] bitmap_index;
    logic[2:0] bitmap_row;
    logic[31:0] row_data_in;
    logic[5:0] palette_index;
    logic[3:0] color_index;
    logic[23:0] color_in;
    logic[5:0] tile_column;
    logic[5:0] tile_row;
    logic[9:0] bitmap_index_in;
    logic[5:0] palette_index_in;
    logic r_out;
    logic g_out;
    logic b_out;
    logic reset;
    logic sclk;
    logic pclk;
    logic[1:0] cpu_addr_in;
    logic[31:0] cpu_data_in;
    logic[31:0] cpu_data_out;
    logic cpu_write;
} Vdp__Interface;

module Vdp (
    output var logic[9:0] bitmap_index,
    output var logic[2:0] bitmap_row,
    input var logic[31:0] row_data_in,
    output var logic[5:0] palette_index,
    output var logic[3:0] color_index,
    input var logic[23:0] color_in,
    output var logic[5:0] tile_column,
    output var logic[5:0] tile_row,
    input var logic[9:0] bitmap_index_in,
    input var logic[5:0] palette_index_in,
    output var logic r_out,
    output var logic g_out,
    output var logic b_out,
    input var logic reset,
    input var logic sclk,
    input var logic pclk,
    input var logic[1:0] cpu_addr_in,
    input var logic[31:0] cpu_data_in,
    output var logic[31:0] cpu_data_out,
    input var logic cpu_write
);

var SyncGenerator__40_128_88_800_1_4_23_600__Interface sync /*verilator split_var*/;
SyncGenerator__40_128_88_800_1_4_23_600 sync__instance (
    .h_state(sync.h_state),
    .v_state(sync.v_state),
    .h_counter(sync.h_counter),
    .v_counter(sync.v_counter),
    .new_line(sync.new_line),
    .new_frame(sync.new_frame),
    .reset(sync.reset),
    .pclk(sync.pclk)
);
var logic[8:0] h_offset;
var logic[8:0] v_offset;
var logic[1:0] cpu_addr_reg;
var logic[8:0] h_pixel;
var logic[8:0] v_pixel;
var logic active_reg[3:0];
var logic hsync_reg[3:0];
var logic vsync_reg[3:0];
var logic active;
var logic hsync;
var logic vsync;
var logic[2:0] bitmap_col_latch_1;
var logic[2:0] bitmap_row_latch;
var logic[2:0] bitmap_col_latch_2;
var logic[5:0] palette_index_latch_1;
var logic[5:0] palette_index_latch_2;
var logic[3:0] bitmap_pixel_latch;
var DviController__Interface dvi /*verilator split_var*/;
DviController dvi__instance (
    .r_in(dvi.r_in),
    .g_in(dvi.g_in),
    .b_in(dvi.b_in),
    .mode(dvi.mode),
    .hsync(dvi.hsync),
    .vsync(dvi.vsync),
    .r_out(dvi.r_out),
    .g_out(dvi.g_out),
    .b_out(dvi.b_out),
    .reset(dvi.reset),
    .sclk(dvi.sclk),
    .pclk(dvi.pclk)
);

var logic[1:0] __tmp_0;
var logic[31:0] __tmp_1;
var logic[31:0] __tmp_2;
var logic __tmp_3[3:0];
var logic __tmp_4[3:0];
var logic __tmp_5[3:0];
var logic __tmp_6[3:0];
var logic __tmp_7[3:0];
var logic __tmp_8[3:0];
var logic __tmp_9[3:0];
var logic __tmp_10[3:0];
var logic __tmp_11[3:0];
var logic[8:0] __tmp_12;
var logic[8:0] __tmp_13;
var logic[3:0] __tmp_14;
var logic[2:0] __tmp_15;
var logic[31:0] __tmp_16;
var logic[31:0] __tmp_17;
var logic[31:0] __tmp_18;
var logic[31:0] __tmp_19;
var logic[31:0] __tmp_20;
var logic[31:0] __tmp_21;
var logic[31:0] __tmp_22;
var logic[31:0] __tmp_23;
var logic[31:0] __tmp_24;
var logic[1:0] __tmp_25;
var logic[9:0] __tmp_26;
var logic[9:0] __tmp_27;
var logic __tmp_28[3:0];
var logic __tmp_29[3:0];
var logic __tmp_30[3:0];
var logic[8:0] __tmp_31;
var logic[8:0] __tmp_32;
var logic[23:0] __tmp_33;
var logic[23:0] __tmp_34;
var logic[23:0] __tmp_35;
var TmdsMode __tmp_36;
always_comb begin
    __tmp_0 = cpu_addr_in;
    __tmp_1 = cpu_data_in;
    __tmp_2 = cpu_data_in;
    __tmp_3 = active_reg;
    __tmp_4 = active_reg;
    __tmp_5 = active_reg;
    __tmp_6 = hsync_reg;
    __tmp_7 = hsync_reg;
    __tmp_8 = hsync_reg;
    __tmp_9 = vsync_reg;
    __tmp_10 = vsync_reg;
    __tmp_11 = vsync_reg;
    __tmp_12 = h_pixel;
    __tmp_13 = v_pixel;
    __tmp_15 = bitmap_col_latch_2;
    __tmp_16 = row_data_in;
    __tmp_17 = row_data_in;
    __tmp_18 = row_data_in;
    __tmp_19 = row_data_in;
    __tmp_20 = row_data_in;
    __tmp_21 = row_data_in;
    __tmp_22 = row_data_in;
    __tmp_23 = row_data_in;
    if ((__tmp_15) == (3'd0)) begin
        __tmp_14 = __tmp_16[3:0];
    end else if ((__tmp_15) == (3'd1)) begin
        __tmp_14 = __tmp_17[7:4];
    end else if ((__tmp_15) == (3'd2)) begin
        __tmp_14 = __tmp_18[11:8];
    end else if ((__tmp_15) == (3'd3)) begin
        __tmp_14 = __tmp_19[15:12];
    end else if ((__tmp_15) == (3'd4)) begin
        __tmp_14 = __tmp_20[19:16];
    end else if ((__tmp_15) == (3'd5)) begin
        __tmp_14 = __tmp_21[23:20];
    end else if ((__tmp_15) == (3'd6)) begin
        __tmp_14 = __tmp_22[27:24];
    end else begin
        __tmp_14 = __tmp_23[31:28];
    end
    __tmp_25 = cpu_addr_reg;
    if ((__tmp_25) == (2'd0)) begin
        __tmp_24 = {23'd0, h_offset};
    end else if ((__tmp_25) == (2'd1)) begin
        __tmp_24 = {23'd0, v_offset};
    end else if ((__tmp_25) == (2'd2)) begin
        __tmp_24 = {31'd0, (sync.h_state) != (SyncState__Active)};
    end else begin
        __tmp_24 = {31'd0, (sync.v_state) != (SyncState__Active)};
    end
    __tmp_26 = sync.h_counter;
    __tmp_27 = sync.v_counter;
    __tmp_28 = active_reg;
    __tmp_29 = hsync_reg;
    __tmp_30 = vsync_reg;
    __tmp_31 = h_pixel;
    __tmp_32 = v_pixel;
    __tmp_33 = color_in;
    __tmp_34 = color_in;
    __tmp_35 = color_in;
    if (active) begin
        __tmp_36 = TmdsMode__Video;
    end else begin
        __tmp_36 = TmdsMode__Control;
    end
end

always_ff @(posedge pclk) begin
    if (reset) begin
        h_offset <= 9'd0;
        v_offset <= 9'd0;
    end else if (cpu_write) begin
        if ((__tmp_0) == (2'd0)) begin
            h_offset <= __tmp_1[8:0];
        end else if ((__tmp_0) == (2'd1)) begin
            v_offset <= __tmp_2[8:0];
        end else begin
        end
    end
end

always_ff @(posedge pclk) begin
    cpu_addr_reg <= cpu_addr_in;
end

always_ff @(posedge pclk) begin
    active_reg[2'd0] <= ((sync.h_state) == (SyncState__Active)) & ((sync.v_state) == (SyncState__Active));
    hsync_reg[2'd0] <= (sync.h_state) == (SyncState__Sync);
    vsync_reg[2'd0] <= (sync.v_state) == (SyncState__Sync);
    active_reg[2'd1] <= __tmp_3[2'd0];
    active_reg[2'd2] <= __tmp_4[2'd1];
    active_reg[2'd3] <= __tmp_5[2'd2];
    hsync_reg[2'd1] <= __tmp_6[2'd0];
    hsync_reg[2'd2] <= __tmp_7[2'd1];
    hsync_reg[2'd3] <= __tmp_8[2'd2];
    vsync_reg[2'd1] <= __tmp_9[2'd0];
    vsync_reg[2'd2] <= __tmp_10[2'd1];
    vsync_reg[2'd3] <= __tmp_11[2'd2];
end

always_ff @(posedge pclk) begin
    bitmap_col_latch_1 <= __tmp_12[2:0];
    bitmap_row_latch <= __tmp_13[2:0];
end

always_ff @(posedge pclk) begin
    bitmap_col_latch_2 <= bitmap_col_latch_1;
    palette_index_latch_1 <= palette_index_in;
end

always_ff @(posedge pclk) begin
    palette_index_latch_2 <= palette_index_latch_1;
    bitmap_pixel_latch <= __tmp_14;
end

always_comb begin
    sync.reset = reset;
    sync.pclk = pclk;
end

always_comb begin
    cpu_data_out = __tmp_24;
end

always_comb begin
    h_pixel = (__tmp_26[9:1]) + (h_offset);
    v_pixel = (__tmp_27[9:1]) + (v_offset);
end

always_comb begin
    active = __tmp_28[2'd3];
    hsync = __tmp_29[2'd3];
    vsync = __tmp_30[2'd3];
end

always_comb begin
    tile_column = __tmp_31[8:3];
    tile_row = __tmp_32[8:3];
end

always_comb begin
    bitmap_index = bitmap_index_in;
    bitmap_row = bitmap_row_latch;
end

always_comb begin
    palette_index = palette_index_latch_2;
    color_index = bitmap_pixel_latch;
end

always_comb begin
    dvi.r_in = __tmp_33[7:0];
    dvi.g_in = __tmp_34[15:8];
    dvi.b_in = __tmp_35[23:16];
    dvi.mode = __tmp_36;
    dvi.hsync = hsync;
    dvi.vsync = vsync;
    r_out = dvi.r_out;
    g_out = dvi.g_out;
    b_out = dvi.b_out;
    dvi.reset = reset;
    dvi.sclk = sclk;
    dvi.pclk = pclk;
end


endmodule

typedef struct packed {
    logic[31:0] lhs;
    logic[31:0] rhs;
    Flags flags_in;
    logic conditional;
    AluOp op;
    logic[31:0] result;
    Flags flags_out;
} Alu__32__Interface;

module Alu__32 (
    input var logic[31:0] lhs,
    input var logic[31:0] rhs,
    input var Flags flags_in,
    input var logic conditional,
    input var AluOp op,
    output var logic[31:0] result,
    output var Flags flags_out
);

var Adder__32__Interface adder /*verilator split_var*/;
Adder__32 adder__instance (
    .lhs(adder.lhs),
    .rhs(adder.rhs),
    .carry_in(adder.carry_in),
    .op(adder.op),
    .result(adder.result),
    .carry_out(adder.carry_out),
    .sign(adder.sign),
    .overflow(adder.overflow)
);
var logic[63:0] mult_result;
var Mult32__Interface mult /*verilator split_var*/;
Mult32 mult__instance (
    .lhs(mult.lhs),
    .rhs(mult.rhs),
    .result(mult.result),
    .op(mult.op)
);
var logic[4:0] shift_amount;
var logic zero;

var AdderOp __tmp_0;
var MultOp __tmp_1;
var logic[31:0] __tmp_2;
var logic[31:0] __tmp_3;
var logic[63:0] __tmp_4;
var logic[63:0] __tmp_5;
var logic[31:0] __tmp_6;
var Flags __tmp_7;
var Flags __tmp_8;
var Flags __tmp_9;
var Flags __tmp_10;
always_comb begin
    case (op)
        AluOp__Add: begin
            __tmp_0 = AdderOp__Add;
        end
        AluOp__AddC: begin
            __tmp_0 = AdderOp__AddC;
        end
        AluOp__Sub: begin
            __tmp_0 = AdderOp__Sub;
        end
        AluOp__SubB: begin
            __tmp_0 = AdderOp__SubB;
        end
        default: begin
            __tmp_0 = AdderOp__Add;
        end
    endcase
    case (op)
        AluOp__MulHuu: begin
            __tmp_1 = MultOp__MulUU;
        end
        AluOp__MulHss: begin
            __tmp_1 = MultOp__MulSS;
        end
        AluOp__MulHus: begin
            __tmp_1 = MultOp__MulUS;
        end
        default: begin
            __tmp_1 = MultOp__MulUU;
        end
    endcase
    __tmp_2 = rhs;
    __tmp_4 = mult_result;
    __tmp_5 = mult_result;
    if (conditional) begin
        __tmp_6 = rhs;
    end else begin
        __tmp_6 = lhs;
    end
    case (op)
        AluOp__Add, AluOp__AddC, AluOp__Sub, AluOp__SubB: begin
            __tmp_3 = adder.result;
        end
        AluOp__MulHuu, AluOp__MulHss, AluOp__MulHus: begin
            __tmp_3 = __tmp_4[63:32];
        end
        AluOp__And: begin
            __tmp_3 = (lhs) & (rhs);
        end
        AluOp__Or: begin
            __tmp_3 = (lhs) | (rhs);
        end
        AluOp__Xor: begin
            __tmp_3 = (lhs) ^ (rhs);
        end
        AluOp__Shl: begin
            __tmp_3 = (lhs) << ({27'd0, shift_amount});
        end
        AluOp__Lsr: begin
            __tmp_3 = (lhs) >> ({27'd0, shift_amount});
        end
        AluOp__Asr: begin
            __tmp_3 = $signed(lhs) >>> $signed({27'd0, shift_amount});
        end
        AluOp__Mul: begin
            __tmp_3 = __tmp_5[31:0];
        end
        AluOp__Cond: begin
            __tmp_3 = __tmp_6;
        end
        default: begin
            __tmp_3 = 32'd0;
        end
    endcase
    begin
        __tmp_8.carry = adder.carry_out;
        __tmp_8.zero = zero;
        __tmp_8.sign = adder.sign;
        __tmp_8.overflow = adder.overflow;
    end
    begin
        __tmp_9.carry = adder.carry_out;
        __tmp_9.zero = (zero) & (flags_in.zero);
        __tmp_9.sign = adder.sign;
        __tmp_9.overflow = adder.overflow;
    end
    begin
        __tmp_10.carry = flags_in.carry;
        __tmp_10.zero = zero;
        __tmp_10.sign = flags_in.sign;
        __tmp_10.overflow = flags_in.overflow;
    end
    case (op)
        AluOp__Add, AluOp__Sub: begin
            __tmp_7 = __tmp_8;
        end
        AluOp__AddC, AluOp__SubB: begin
            __tmp_7 = __tmp_9;
        end
        AluOp__And, AluOp__Or, AluOp__Xor, AluOp__Shl, AluOp__Lsr, AluOp__Asr, AluOp__Mul, AluOp__MulHuu, AluOp__MulHss, AluOp__MulHus: begin
            __tmp_7 = __tmp_10;
        end
        default: begin
            __tmp_7 = flags_in;
        end
    endcase
end

always_comb begin
    adder.lhs = lhs;
    adder.rhs = rhs;
    adder.carry_in = flags_in.carry;
    adder.op = __tmp_0;
end

always_comb begin
    mult.lhs = lhs;
    mult.rhs = rhs;
    mult.op = __tmp_1;
    mult_result = mult.result;
end

always_comb begin
    shift_amount = __tmp_2[4:0];
end

always_comb begin
    result = __tmp_3;
end

always_comb begin
    zero = (result) == (32'd0);
    flags_out = __tmp_7;
end


endmodule

typedef struct packed {
    logic[31:0] instruction_address_out;
    logic[31:0] instruction_word_in;
    logic fetch_request;
    logic[29:0] mem_address_out;
    logic[31:0] mem_data_out;
    logic[31:0] mem_data_in;
    logic mem_enable_out;
    logic[3:0] mem_byte_enable_out;
    logic mem_write_out;
    logic[11:0] io_address_out;
    logic[31:0] io_data_out;
    logic[31:0] io_data_in;
    logic io_enable_out;
    logic io_write_out;
    logic k_flag_out;
    logic stall_out;
    logic[29:0] syscall_addr_in;
    logic enable;
    logic reset;
    logic clk;
} Cpu__Interface;

module Cpu (
    output var logic[31:0] instruction_address_out,
    input var logic[31:0] instruction_word_in,
    output var logic fetch_request,
    output var logic[29:0] mem_address_out,
    output var logic[31:0] mem_data_out,
    input var logic[31:0] mem_data_in,
    output var logic mem_enable_out,
    output var logic[3:0] mem_byte_enable_out,
    output var logic mem_write_out,
    output var logic[11:0] io_address_out,
    output var logic[31:0] io_data_out,
    input var logic[31:0] io_data_in,
    output var logic io_enable_out,
    output var logic io_write_out,
    output var logic k_flag_out,
    output var logic stall_out,
    input var logic[29:0] syscall_addr_in,
    input var logic enable,
    input var logic reset,
    input var logic clk
);

var Alu__32__Interface alu /*verilator split_var*/;
Alu__32 alu__instance (
    .lhs(alu.lhs),
    .rhs(alu.rhs),
    .flags_in(alu.flags_in),
    .conditional(alu.conditional),
    .op(alu.op),
    .result(alu.result),
    .flags_out(alu.flags_out)
);
var ConditionUnit__Interface move_condition_unit /*verilator split_var*/;
ConditionUnit move_condition_unit__instance (
    .flags(move_condition_unit.flags),
    .condition(move_condition_unit.condition),
    .conditional(move_condition_unit.conditional)
);
var ConditionUnit__Interface jump_condition_unit /*verilator split_var*/;
ConditionUnit jump_condition_unit__instance (
    .flags(jump_condition_unit.flags),
    .condition(jump_condition_unit.condition),
    .conditional(jump_condition_unit.conditional)
);
var FlagRegister__Interface flag_register /*verilator split_var*/;
FlagRegister flag_register__instance (
    .flags_in(flag_register.flags_in),
    .load(flag_register.load),
    .flags_out(flag_register.flags_out),
    .enable(flag_register.enable),
    .clk(flag_register.clk)
);
var KernelModeRegister__Interface kernel_mode_register /*verilator split_var*/;
KernelModeRegister kernel_mode_register__instance (
    .set(kernel_mode_register.set),
    .clear(kernel_mode_register.clear),
    .k_flag_out(kernel_mode_register.k_flag_out),
    .enable(kernel_mode_register.enable),
    .reset(kernel_mode_register.reset),
    .clk(kernel_mode_register.clk)
);
var ProgramCounter__32__Interface program_counter /*verilator split_var*/;
ProgramCounter__32 program_counter__instance (
    .data_in(program_counter.data_in),
    .inc(program_counter.inc),
    .load(program_counter.load),
    .pc_next_out(program_counter.pc_next_out),
    .pc_value_out(program_counter.pc_value_out),
    .enable(program_counter.enable),
    .reset(program_counter.reset),
    .clk(program_counter.clk)
);
var RegisterFile__32_32__Interface register_file /*verilator split_var*/;
RegisterFile__32_32 register_file__instance (
    .lhs_select(register_file.lhs_select),
    .rhs_select(register_file.rhs_select),
    .load_select(register_file.load_select),
    .data_in(register_file.data_in),
    .lhs_out(register_file.lhs_out),
    .rhs_out(register_file.rhs_out),
    .enable(register_file.enable),
    .reset(register_file.reset),
    .clk(register_file.clk)
);
var InstructionDecoder__Interface instruction_decoder /*verilator split_var*/;
InstructionDecoder instruction_decoder__instance (
    .instruction_word(instruction_decoder.instruction_word),
    .alu_op_out(instruction_decoder.alu_op_out),
    .move_condition_out(instruction_decoder.move_condition_out),
    .jump_condition_out(instruction_decoder.jump_condition_out),
    .load_flags_out(instruction_decoder.load_flags_out),
    .reg_lhs_select_out(instruction_decoder.reg_lhs_select_out),
    .reg_rhs_select_out(instruction_decoder.reg_rhs_select_out),
    .reg_load_select_out(instruction_decoder.reg_load_select_out),
    .lhs_bus_source_out(instruction_decoder.lhs_bus_source_out),
    .rhs_bus_source_out(instruction_decoder.rhs_bus_source_out),
    .data_bus_source_out(instruction_decoder.data_bus_source_out),
    .mem_enable_out(instruction_decoder.mem_enable_out),
    .mem_mode_out(instruction_decoder.mem_mode_out),
    .mem_sign_ext_out(instruction_decoder.mem_sign_ext_out),
    .mem_write_out(instruction_decoder.mem_write_out),
    .set_k_flag_out(instruction_decoder.set_k_flag_out),
    .clear_k_flag_out(instruction_decoder.clear_k_flag_out),
    .imm_out(instruction_decoder.imm_out)
);
var DataSwizzle__Interface swizzle /*verilator split_var*/;
DataSwizzle swizzle__instance (
    .unpacked_data_in(swizzle.unpacked_data_in),
    .unpacked_data_out(swizzle.unpacked_data_out),
    .mode_in(swizzle.mode_in),
    .byte_address_in(swizzle.byte_address_in),
    .sign_extend_in(swizzle.sign_extend_in),
    .write_in(swizzle.write_in),
    .packed_data_in(swizzle.packed_data_in),
    .packed_data_out(swizzle.packed_data_out),
    .byte_enable_out(swizzle.byte_enable_out),
    .io_data_in(swizzle.io_data_in),
    .io_data_out(swizzle.io_data_out),
    .io_enable_out(swizzle.io_enable_out),
    .clk(swizzle.clk)
);
var logic stall_execute;
var logic stall_fetch;
var logic inc_pc;
var LhsBusSource lhs_bus_source;
var RhsBusSource rhs_bus_source;
var logic[31:0] ir;
var logic[31:0] lhs_mux;
var logic[31:0] rhs_mux;
var logic[31:0] lhs;
var logic[31:0] rhs;
var logic[31:0] offset;
var AluOp alu_op;
var Condition move_condition;
var Condition jump_condition;
var logic load_flags;
var logic mem_enable;
var MemoryMode mem_mode;
var logic mem_sign_ext;
var logic mem_write;
var logic[31:0] mem_address;
var logic[31:0] result;
var logic[31:0] result_data;
var logic[31:0] mem_data;
var logic[31:0] io_data;
var DataBusSource data_bus_source_2;
var DataBusSource data_bus_source_3;
var DataBusSource data_bus_source_4;
var logic[4:0] reg_load_select_2;
var logic[4:0] reg_load_select_3;
var logic[4:0] reg_load_select_4;
var logic[31:0] data_bus;

var logic[31:0] __tmp_0;
var DataBusSource __tmp_1;
var logic[4:0] __tmp_2;
var LhsBusSource __tmp_3;
var LhsBusSource __tmp_4;
var RhsBusSource __tmp_5;
var RhsBusSource __tmp_6;
var logic[31:0] __tmp_7;
var logic[31:0] __tmp_8;
var logic[31:0] __tmp_9;
var logic[31:0] __tmp_10;
var logic[31:0] __tmp_11;
var logic[31:0] __tmp_12;
always_comb begin
    if (reset) begin
        __tmp_0 = 32'd0;
    end else if (stall_execute) begin
        __tmp_0 = ir;
    end else if (stall_fetch) begin
        __tmp_0 = 32'd0;
    end else begin
        __tmp_0 = instruction_word_in;
    end
    if (stall_execute) begin
        __tmp_1 = DataBusSource__Result;
    end else begin
        __tmp_1 = instruction_decoder.data_bus_source_out;
    end
    if (stall_execute) begin
        __tmp_2 = 5'd0;
    end else begin
        __tmp_2 = instruction_decoder.reg_load_select_out;
    end
    if ((instruction_decoder.reg_lhs_select_out) == (5'd0)) begin
        __tmp_4 = instruction_decoder.lhs_bus_source_out;
    end else if ((instruction_decoder.reg_lhs_select_out) == (reg_load_select_2)) begin
        __tmp_4 = LhsBusSource__Forward3;
    end else if ((instruction_decoder.reg_lhs_select_out) == (reg_load_select_3)) begin
        __tmp_4 = LhsBusSource__Forward4;
    end else if ((instruction_decoder.reg_lhs_select_out) == (reg_load_select_4)) begin
        __tmp_4 = LhsBusSource__Forward5;
    end else begin
        __tmp_4 = instruction_decoder.lhs_bus_source_out;
    end
    if ((instruction_decoder.lhs_bus_source_out) == (LhsBusSource__Register)) begin
        __tmp_3 = __tmp_4;
    end else begin
        __tmp_3 = instruction_decoder.lhs_bus_source_out;
    end
    if ((instruction_decoder.reg_rhs_select_out) == (5'd0)) begin
        __tmp_6 = instruction_decoder.rhs_bus_source_out;
    end else if ((instruction_decoder.reg_rhs_select_out) == (reg_load_select_2)) begin
        __tmp_6 = RhsBusSource__Forward3;
    end else if ((instruction_decoder.reg_rhs_select_out) == (reg_load_select_3)) begin
        __tmp_6 = RhsBusSource__Forward4;
    end else if ((instruction_decoder.reg_rhs_select_out) == (reg_load_select_4)) begin
        __tmp_6 = RhsBusSource__Forward5;
    end else begin
        __tmp_6 = instruction_decoder.rhs_bus_source_out;
    end
    if ((instruction_decoder.rhs_bus_source_out) == (RhsBusSource__Register)) begin
        __tmp_5 = __tmp_6;
    end else begin
        __tmp_5 = instruction_decoder.rhs_bus_source_out;
    end
    case (lhs_bus_source)
        LhsBusSource__Register: begin
            __tmp_7 = register_file.lhs_out;
        end
        LhsBusSource__Pc: begin
            __tmp_7 = program_counter.pc_value_out;
        end
        LhsBusSource__Syscall: begin
            __tmp_7 = {syscall_addr_in, 2'd0};
        end
        LhsBusSource__Forward3: begin
            __tmp_7 = alu.result;
        end
        LhsBusSource__Forward4: begin
            __tmp_7 = result;
        end
        default: begin
            __tmp_7 = data_bus;
        end
    endcase
    case (rhs_bus_source)
        RhsBusSource__Register: begin
            __tmp_8 = register_file.rhs_out;
        end
        RhsBusSource__Pc: begin
            __tmp_8 = program_counter.pc_value_out;
        end
        RhsBusSource__Immediate: begin
            __tmp_8 = instruction_decoder.imm_out;
        end
        RhsBusSource__Forward3: begin
            __tmp_8 = alu.result;
        end
        RhsBusSource__Forward4: begin
            __tmp_8 = result;
        end
        default: begin
            __tmp_8 = data_bus;
        end
    endcase
    __tmp_9 = mem_address;
    __tmp_10 = mem_address;
    __tmp_11 = mem_address;
    case (data_bus_source_4)
        DataBusSource__Result: begin
            __tmp_12 = result_data;
        end
        default: begin
            __tmp_12 = swizzle.unpacked_data_out;
        end
    endcase
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        ir <= __tmp_0;
    end
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        lhs <= lhs_mux;
        rhs <= rhs_mux;
        offset <= instruction_decoder.imm_out;
    end
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        if (stall_execute) begin
            alu_op <= AluOp__Nop;
            move_condition <= Condition__Never;
            jump_condition <= Condition__Never;
            load_flags <= 1'd0;
        end else begin
            alu_op <= instruction_decoder.alu_op_out;
            move_condition <= instruction_decoder.move_condition_out;
            jump_condition <= instruction_decoder.jump_condition_out;
            load_flags <= instruction_decoder.load_flags_out;
        end
    end
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        if (stall_execute) begin
            mem_enable <= 1'd0;
            mem_mode <= MemoryMode__Bits32;
            mem_sign_ext <= 1'd0;
            mem_write <= 1'd0;
        end else begin
            mem_enable <= instruction_decoder.mem_enable_out;
            mem_mode <= instruction_decoder.mem_mode_out;
            mem_sign_ext <= instruction_decoder.mem_sign_ext_out;
            mem_write <= instruction_decoder.mem_write_out;
        end
    end
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        result <= alu.result;
    end
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        result_data <= result;
        mem_data <= mem_data_in;
        io_data <= io_data_in;
    end
end

always_ff @(posedge clk) begin
    if ((enable) | (reset)) begin
        data_bus_source_2 <= __tmp_1;
        data_bus_source_3 <= data_bus_source_2;
        data_bus_source_4 <= data_bus_source_3;
        reg_load_select_2 <= __tmp_2;
        reg_load_select_3 <= reg_load_select_2;
        reg_load_select_4 <= reg_load_select_3;
    end
end

always_comb begin
    flag_register.enable = enable;
    flag_register.clk = clk;
    kernel_mode_register.enable = enable;
    kernel_mode_register.reset = reset;
    kernel_mode_register.clk = clk;
    program_counter.enable = enable;
    program_counter.reset = reset;
    program_counter.clk = clk;
    register_file.enable = enable;
    register_file.reset = reset;
    register_file.clk = clk;
    swizzle.clk = clk;
end

always_comb begin
    stall_execute = (((((((instruction_decoder.lhs_bus_source_out) == (LhsBusSource__Register)) & ((instruction_decoder.reg_lhs_select_out) != (5'd0))) & ((instruction_decoder.reg_lhs_select_out) == (reg_load_select_2))) & ((data_bus_source_2) != (DataBusSource__Result))) | (((((instruction_decoder.rhs_bus_source_out) == (RhsBusSource__Register)) & ((instruction_decoder.reg_rhs_select_out) != (5'd0))) & ((instruction_decoder.reg_rhs_select_out) == (reg_load_select_2))) & ((data_bus_source_2) != (DataBusSource__Result)))) | (((((instruction_decoder.lhs_bus_source_out) == (LhsBusSource__Register)) & ((instruction_decoder.reg_lhs_select_out) != (5'd0))) & ((instruction_decoder.reg_lhs_select_out) == (reg_load_select_3))) & ((data_bus_source_3) != (DataBusSource__Result)))) | (((((instruction_decoder.rhs_bus_source_out) == (RhsBusSource__Register)) & ((instruction_decoder.reg_rhs_select_out) != (5'd0))) & ((instruction_decoder.reg_rhs_select_out) == (reg_load_select_3))) & ((data_bus_source_3) != (DataBusSource__Result)));
    stall_fetch = ((instruction_decoder.jump_condition_out) != (Condition__Never)) | (jump_condition_unit.conditional);
    inc_pc = (~(stall_execute)) & (~(stall_fetch));
end

always_comb begin
    stall_out = (stall_execute) | (stall_fetch);
end

always_comb begin
    lhs_bus_source = __tmp_3;
    rhs_bus_source = __tmp_5;
end

always_comb begin
    instruction_address_out = program_counter.pc_next_out;
    fetch_request = ((~(reset)) & (~(stall_execute))) & (~(stall_fetch));
    program_counter.inc = inc_pc;
end

always_comb begin
    instruction_decoder.instruction_word = ir;
    register_file.lhs_select = instruction_decoder.reg_lhs_select_out;
    register_file.rhs_select = instruction_decoder.reg_rhs_select_out;
    kernel_mode_register.set = instruction_decoder.set_k_flag_out;
    kernel_mode_register.clear = instruction_decoder.clear_k_flag_out;
    lhs_mux = __tmp_7;
    rhs_mux = __tmp_8;
end

always_comb begin
    alu.lhs = lhs;
    alu.rhs = rhs;
    alu.flags_in = flag_register.flags_out;
    alu.conditional = move_condition_unit.conditional;
    alu.op = alu_op;
    flag_register.flags_in = alu.flags_out;
    flag_register.load = load_flags;
    move_condition_unit.flags = flag_register.flags_out;
    move_condition_unit.condition = move_condition;
end

always_comb begin
    mem_address = (lhs) + (offset);
end

always_comb begin
    jump_condition_unit.flags = flag_register.flags_out;
    jump_condition_unit.condition = jump_condition;
    program_counter.data_in = mem_address;
    program_counter.load = jump_condition_unit.conditional;
end

always_comb begin
    swizzle.unpacked_data_in = rhs;
    swizzle.mode_in = mem_mode;
    swizzle.byte_address_in = __tmp_9[1:0];
    swizzle.sign_extend_in = mem_sign_ext;
    swizzle.write_in = mem_write;
    mem_address_out = __tmp_10[31:2];
    mem_data_out = swizzle.packed_data_out;
    mem_enable_out = (mem_enable) & (~(swizzle.io_enable_out));
    mem_byte_enable_out = swizzle.byte_enable_out;
    mem_write_out = mem_write;
    io_address_out = __tmp_11[11:0];
    io_data_out = swizzle.io_data_out;
    io_enable_out = (mem_enable) & (swizzle.io_enable_out);
    io_write_out = mem_write;
    k_flag_out = kernel_mode_register.k_flag_out;
end

always_comb begin
    swizzle.packed_data_in = mem_data;
    swizzle.io_data_in = io_data;
end

always_comb begin
    data_bus = __tmp_12;
end

always_comb begin
    register_file.data_in = data_bus;
    register_file.load_select = reg_load_select_4;
end


endmodule

typedef struct packed {
    logic[7:0] data_in;
    logic[7:0] data_out;
    logic read;
    logic write;
    logic empty;
    logic full;
    logic[6:0] count;
    logic reset;
    logic clk;
} Fifo__8_6__Interface;

module Fifo__8_6 (
    input var logic[7:0] data_in,
    output var logic[7:0] data_out,
    input var logic read,
    input var logic write,
    output var logic empty,
    output var logic full,
    output var logic[6:0] count,
    input var logic reset,
    input var logic clk
);

var logic[7:0] mem[63:0];
var logic[5:0] read_ptr;
var logic[5:0] write_ptr;
var logic do_read;
var logic do_write;
var logic[5:0] next_read_ptr;
var logic[5:0] next_write_ptr;
var logic[6:0] next_count;
var logic perform_write;

var logic[7:0] __tmp_0[63:0];
var logic[7:0] __tmp_1[63:0];
always_comb begin
    __tmp_0 = mem;
    __tmp_1 = mem;
end

always_ff @(posedge clk) begin
    read_ptr <= next_read_ptr;
    write_ptr <= next_write_ptr;
    count <= next_count;
    if (perform_write) begin
        mem[write_ptr] <= data_in;
    end
end

always_comb begin
    empty = (count) == (7'd0);
    full = (count) == (7'd64);
end

always_comb begin
    do_read = (read) & (~(empty));
    do_write = (write) & (~(full));
end

always_comb begin
    if (reset) begin
        next_read_ptr = 6'd0;
        next_write_ptr = 6'd0;
        next_count = 7'd0;
        data_out = 8'd0;
        perform_write = 1'd0;
    end else if ((do_read) & (write)) begin
        next_read_ptr = (read_ptr) + (6'd1);
        next_write_ptr = (write_ptr) + (6'd1);
        next_count = count;
        data_out = __tmp_0[read_ptr];
        perform_write = 1'd1;
    end else if (do_read) begin
        next_read_ptr = (read_ptr) + (6'd1);
        next_write_ptr = write_ptr;
        next_count = (count) - (7'd1);
        data_out = __tmp_1[read_ptr];
        perform_write = 1'd0;
    end else if (do_write) begin
        next_read_ptr = read_ptr;
        next_write_ptr = (write_ptr) + (6'd1);
        next_count = (count) + (7'd1);
        data_out = 8'd0;
        perform_write = 1'd1;
    end else begin
        next_read_ptr = read_ptr;
        next_write_ptr = write_ptr;
        next_count = count;
        data_out = 8'd0;
        perform_write = 1'd0;
    end
end


endmodule

typedef struct packed {
    logic[1:0] addr_in;
    logic[7:0] data_in;
    logic[7:0] data_out;
    logic write;
    logic rxd;
    logic txd;
    logic enable;
    logic reset;
    logic clk;
} SerialController__Interface;

module SerialController (
    input var logic[1:0] addr_in,
    input var logic[7:0] data_in,
    output var logic[7:0] data_out,
    input var logic write,
    input var logic rxd,
    output var logic txd,
    input var logic enable,
    input var logic reset,
    input var logic clk
);

var Uart__Interface uart /*verilator split_var*/;
Uart uart__instance (
    .data_in(uart.data_in),
    .transmit(uart.transmit),
    .fetch(uart.fetch),
    .data_out(uart.data_out),
    .received(uart.received),
    .rxd(uart.rxd),
    .txd(uart.txd),
    .reset(uart.reset),
    .clk(uart.clk)
);
var Fifo__8_6__Interface rx_fifo /*verilator split_var*/;
Fifo__8_6 rx_fifo__instance (
    .data_in(rx_fifo.data_in),
    .data_out(rx_fifo.data_out),
    .read(rx_fifo.read),
    .write(rx_fifo.write),
    .empty(rx_fifo.empty),
    .full(rx_fifo.full),
    .count(rx_fifo.count),
    .reset(rx_fifo.reset),
    .clk(rx_fifo.clk)
);
var Fifo__8_6__Interface tx_fifo /*verilator split_var*/;
Fifo__8_6 tx_fifo__instance (
    .data_in(tx_fifo.data_in),
    .data_out(tx_fifo.data_out),
    .read(tx_fifo.read),
    .write(tx_fifo.write),
    .empty(tx_fifo.empty),
    .full(tx_fifo.full),
    .count(tx_fifo.count),
    .reset(tx_fifo.reset),
    .clk(tx_fifo.clk)
);
var logic[1:0] addr_reg;
var logic[7:0] data_reg;
var logic write_reg;
var logic enable_reg;

var logic[7:0] __tmp_0;
var logic[1:0] __tmp_1;
always_comb begin
    __tmp_1 = addr_reg;
    if ((__tmp_1) == (2'd0)) begin
        __tmp_0 = rx_fifo.data_out;
    end else if ((__tmp_1) == (2'd1)) begin
        __tmp_0 = 8'd0;
    end else if ((__tmp_1) == (2'd2)) begin
        __tmp_0 = {1'd0, rx_fifo.count};
    end else begin
        __tmp_0 = {1'd0, tx_fifo.count};
    end
end

always_ff @(posedge clk) begin
    addr_reg <= addr_in;
    data_reg <= data_in;
    write_reg <= write;
    enable_reg <= enable;
end

always_comb begin
    uart.rxd = rxd;
    txd = uart.txd;
    uart.reset = reset;
    uart.clk = clk;
end

always_comb begin
    rx_fifo.data_in = uart.data_out;
    rx_fifo.write = uart.received;
    rx_fifo.reset = reset;
    rx_fifo.clk = clk;
end

always_comb begin
    uart.data_in = tx_fifo.data_out;
    uart.transmit = ~(tx_fifo.empty);
    tx_fifo.read = uart.fetch;
    tx_fifo.reset = reset;
    tx_fifo.clk = clk;
end

always_comb begin
    data_out = __tmp_0;
    tx_fifo.data_in = data_reg;
    rx_fifo.read = ((enable_reg) & (~(write_reg))) & ((addr_reg) == (2'd0));
    tx_fifo.write = ((enable_reg) & (write_reg)) & ((addr_reg) == (2'd1));
end


endmodule

module Art32 (
    input var InPort__1__Interface reset_n,
    input var InPort__1__Interface clk25,
    output var OutPort__18__Interface sram_address,
    output var OutPort__1__Interface sram_we_n,
    inout tri logic[15:0] sram_a_data__port,
    output var OutPort__1__Interface sram_a_ce_n,
    output var OutPort__1__Interface sram_a_oe_n,
    output var OutPort__1__Interface sram_a_lb_n,
    output var OutPort__1__Interface sram_a_ub_n,
    inout tri logic[15:0] sram_b_data__port,
    output var OutPort__1__Interface sram_b_ce_n,
    output var OutPort__1__Interface sram_b_oe_n,
    output var OutPort__1__Interface sram_b_lb_n,
    output var OutPort__1__Interface sram_b_ub_n,
    input var InPort__1__Interface rxd,
    output var OutPort__1__Interface txd,
    output var OutPort__1__Interface hdmi_clk,
    output var OutPort__1__Interface hdmi_d0,
    output var OutPort__1__Interface hdmi_d1,
    output var OutPort__1__Interface hdmi_d2,
    output var OutPort__1__Interface led_r,
    output var OutPort__1__Interface led_g,
    output var OutPort__1__Interface led_b
);

var logic clk200;
var logic clk80;
var logic clk40;
var logic reset;
var Pll__Interface pll /*verilator split_var*/;
Pll pll__instance (
    .clk25(pll.clk25),
    .clk200(pll.clk200),
    .clk80(pll.clk80),
    .clk40(pll.clk40),
    .locked(pll.locked)
);
var logic reset40;
var logic cpu_reset_1;
var logic cpu_reset_2;
var logic cpu_reset_3;
var logic cpu_reset_4;
var logic cpu_reset_5;
var logic cpu_reset_6;
var logic cpu_reset_7;
var logic cpu_reset;
var Cpu__Interface cpu /*verilator split_var*/;
Cpu cpu__instance (
    .instruction_address_out(cpu.instruction_address_out),
    .instruction_word_in(cpu.instruction_word_in),
    .fetch_request(cpu.fetch_request),
    .mem_address_out(cpu.mem_address_out),
    .mem_data_out(cpu.mem_data_out),
    .mem_data_in(cpu.mem_data_in),
    .mem_enable_out(cpu.mem_enable_out),
    .mem_byte_enable_out(cpu.mem_byte_enable_out),
    .mem_write_out(cpu.mem_write_out),
    .io_address_out(cpu.io_address_out),
    .io_data_out(cpu.io_data_out),
    .io_data_in(cpu.io_data_in),
    .io_enable_out(cpu.io_enable_out),
    .io_write_out(cpu.io_write_out),
    .k_flag_out(cpu.k_flag_out),
    .stall_out(cpu.stall_out),
    .syscall_addr_in(cpu.syscall_addr_in),
    .enable(cpu.enable),
    .reset(cpu.reset),
    .clk(cpu.clk)
);
var logic[63:0] cycle_count;
var logic[63:0] stall_count;
var logic[5:0] timer_div;
var logic[63:0] timer;
var KernelRam__Interface kram /*verilator split_var*/;
KernelRam kram__instance (
    .instr_addr_in(kram.instr_addr_in),
    .instr_out(kram.instr_out),
    .data_addr_in(kram.data_addr_in),
    .data_in(kram.data_in),
    .data_out(kram.data_out),
    .data_byte_enable(kram.data_byte_enable),
    .data_write(kram.data_write),
    .clk(kram.clk)
);
var InOutPort__16__Interface sram_a_data /*verilator split_var*/;
assign sram_a_data__port = sram_a_data.oe ? sram_a_data.d_out : 16'bz;
assign sram_a_data.d_in = sram_a_data__port;
var InOutPort__16__Interface sram_b_data /*verilator split_var*/;
assign sram_b_data__port = sram_b_data.oe ? sram_b_data.d_out : 16'bz;
assign sram_b_data.d_in = sram_b_data__port;
var SramInterface__Interface sram /*verilator split_var*/;
SramInterface sram__instance (
    .sram_address_out(sram.sram_address_out),
    .sram_we_n_out(sram.sram_we_n_out),
    .sram_we_out(sram.sram_we_out),
    .sram_a_data_in(sram.sram_a_data_in),
    .sram_a_data_out(sram.sram_a_data_out),
    .sram_a_ce_n_out(sram.sram_a_ce_n_out),
    .sram_a_oe_n_out(sram.sram_a_oe_n_out),
    .sram_a_lb_n_out(sram.sram_a_lb_n_out),
    .sram_a_ub_n_out(sram.sram_a_ub_n_out),
    .sram_b_data_in(sram.sram_b_data_in),
    .sram_b_data_out(sram.sram_b_data_out),
    .sram_b_ce_n_out(sram.sram_b_ce_n_out),
    .sram_b_oe_n_out(sram.sram_b_oe_n_out),
    .sram_b_lb_n_out(sram.sram_b_lb_n_out),
    .sram_b_ub_n_out(sram.sram_b_ub_n_out),
    .address_in(sram.address_in),
    .data_in(sram.data_in),
    .data_out(sram.data_out),
    .write_read_n_in(sram.write_read_n_in),
    .byte_enable_in(sram.byte_enable_in),
    .reset(sram.reset),
    .clk(sram.clk),
    .clk2(sram.clk2)
);
var VideoRam__Interface vram /*verilator split_var*/;
VideoRam vram__instance (
    .cpu_address(vram.cpu_address),
    .cpu_data_in(vram.cpu_data_in),
    .cpu_data_out(vram.cpu_data_out),
    .cpu_byte_enable(vram.cpu_byte_enable),
    .cpu_write(vram.cpu_write),
    .vdp_bitmap_index(vram.vdp_bitmap_index),
    .vdp_bitmap_row(vram.vdp_bitmap_row),
    .vdp_row_data_out(vram.vdp_row_data_out),
    .vdp_palette_index(vram.vdp_palette_index),
    .vdp_color_index(vram.vdp_color_index),
    .vdp_color_out(vram.vdp_color_out),
    .vdp_tile_column(vram.vdp_tile_column),
    .vdp_tile_row(vram.vdp_tile_row),
    .vdp_bitmap_index_out(vram.vdp_bitmap_index_out),
    .vdp_palette_index_out(vram.vdp_palette_index_out),
    .cpu_clk(vram.cpu_clk),
    .vdp_clk(vram.vdp_clk)
);
var Mmu__Interface mmu /*verilator split_var*/;
Mmu mmu__instance (
    .mem_page(mmu.mem_page),
    .mem_enable(mmu.mem_enable),
    .mem_write(mmu.mem_write),
    .k_flag(mmu.k_flag),
    .kram_write(mmu.kram_write),
    .sram_write(mmu.sram_write),
    .vram_write(mmu.vram_write),
    .kram_data_in(mmu.kram_data_in),
    .sram_data_in(mmu.sram_data_in),
    .vram_data_in(mmu.vram_data_in),
    .data_out(mmu.data_out),
    .reset(mmu.reset),
    .clk(mmu.clk)
);
var SerialController__Interface serial /*verilator split_var*/;
SerialController serial__instance (
    .addr_in(serial.addr_in),
    .data_in(serial.data_in),
    .data_out(serial.data_out),
    .write(serial.write),
    .rxd(serial.rxd),
    .txd(serial.txd),
    .enable(serial.enable),
    .reset(serial.reset),
    .clk(serial.clk)
);
var Vdp__Interface vdp /*verilator split_var*/;
Vdp vdp__instance (
    .bitmap_index(vdp.bitmap_index),
    .bitmap_row(vdp.bitmap_row),
    .row_data_in(vdp.row_data_in),
    .palette_index(vdp.palette_index),
    .color_index(vdp.color_index),
    .color_in(vdp.color_in),
    .tile_column(vdp.tile_column),
    .tile_row(vdp.tile_row),
    .bitmap_index_in(vdp.bitmap_index_in),
    .palette_index_in(vdp.palette_index_in),
    .r_out(vdp.r_out),
    .g_out(vdp.g_out),
    .b_out(vdp.b_out),
    .reset(vdp.reset),
    .sclk(vdp.sclk),
    .pclk(vdp.pclk),
    .cpu_addr_in(vdp.cpu_addr_in),
    .cpu_data_in(vdp.cpu_data_in),
    .cpu_data_out(vdp.cpu_data_out),
    .cpu_write(vdp.cpu_write)
);
var LedController__Interface led /*verilator split_var*/;
LedController led__instance (
    .data_in(led.data_in),
    .data_out(led.data_out),
    .write(led.write),
    .r_out(led.r_out),
    .g_out(led.g_out),
    .b_out(led.b_out),
    .reset(led.reset),
    .clk(led.clk)
);
var logic[29:0] syscall_addr;
var logic[11:0] io_addr_reg;
var logic k_flag_reg;

var logic[63:0] __tmp_0;
var logic[31:0] __tmp_1;
var logic[31:0] __tmp_2;
var logic[29:0] __tmp_3;
var logic[29:0] __tmp_4;
var logic[29:0] __tmp_5;
var logic[29:0] __tmp_6;
var logic[11:0] __tmp_7;
var logic[31:0] __tmp_8;
var logic[11:0] __tmp_9;
var logic[31:0] __tmp_10;
var logic[31:0] __tmp_11;
var logic[11:0] __tmp_12;
var logic[63:0] __tmp_13;
var logic[63:0] __tmp_14;
var logic[63:0] __tmp_15;
var logic[63:0] __tmp_16;
var logic[63:0] __tmp_17;
var logic[63:0] __tmp_18;
var logic[31:0] __tmp_19;
always_comb begin
    if (cpu_reset) begin
        __tmp_0 = 64'd0;
    end else begin
        __tmp_0 = (cycle_count) + (64'd1);
    end
    __tmp_1 = cpu.io_data_out;
    __tmp_2 = cpu.instruction_address_out;
    __tmp_3 = cpu.mem_address_out;
    __tmp_4 = cpu.mem_address_out;
    __tmp_5 = cpu.mem_address_out;
    __tmp_6 = cpu.mem_address_out;
    __tmp_7 = cpu.io_address_out;
    __tmp_8 = cpu.io_data_out;
    __tmp_9 = cpu.io_address_out;
    __tmp_10 = cpu.io_data_out;
    __tmp_12 = io_addr_reg;
    __tmp_13 = cycle_count;
    __tmp_14 = cycle_count;
    __tmp_15 = stall_count;
    __tmp_16 = stall_count;
    __tmp_17 = timer;
    __tmp_18 = timer;
    if (k_flag_reg) begin
        __tmp_19 = {syscall_addr, 2'd0};
    end else begin
        __tmp_19 = 32'd2863311530;
    end
    if (((__tmp_12) >= (12'd0)) & ((__tmp_12) <= (12'd3))) begin
        __tmp_11 = {24'd0, serial.data_out};
    end else if (((__tmp_12) >= (12'd4)) & ((__tmp_12) <= (12'd7))) begin
        __tmp_11 = vdp.cpu_data_out;
    end else if ((__tmp_12) == (12'd8)) begin
        __tmp_11 = __tmp_13[31:0];
    end else if ((__tmp_12) == (12'd9)) begin
        __tmp_11 = __tmp_14[63:32];
    end else if ((__tmp_12) == (12'd10)) begin
        __tmp_11 = __tmp_15[31:0];
    end else if ((__tmp_12) == (12'd11)) begin
        __tmp_11 = __tmp_16[63:32];
    end else if ((__tmp_12) == (12'd12)) begin
        __tmp_11 = __tmp_17[31:0];
    end else if ((__tmp_12) == (12'd13)) begin
        __tmp_11 = __tmp_18[63:32];
    end else if ((__tmp_12) == (12'd14)) begin
        __tmp_11 = 32'd2863311530;
    end else if ((__tmp_12) == (12'd15)) begin
        __tmp_11 = {8'd0, led.data_out};
    end else if ((__tmp_12) == (12'd4095)) begin
        __tmp_11 = __tmp_19;
    end else begin
        __tmp_11 = 32'd2863311530;
    end
end

always_ff @(posedge clk40) begin
    reset40 <= reset;
end

always_ff @(posedge clk40) begin
    cpu_reset_1 <= reset40;
    cpu_reset_2 <= cpu_reset_1;
    cpu_reset_3 <= cpu_reset_2;
    cpu_reset_4 <= cpu_reset_3;
    cpu_reset_5 <= cpu_reset_4;
    cpu_reset_6 <= cpu_reset_5;
    cpu_reset_7 <= cpu_reset_6;
end

always_ff @(posedge clk40) begin
    cycle_count <= __tmp_0;
end

always_ff @(posedge clk40) begin
    if (cpu_reset) begin
        stall_count <= 64'd0;
    end else if (cpu.stall_out) begin
        stall_count <= (stall_count) + (64'd1);
    end
end

always_ff @(posedge clk40) begin
    if (cpu_reset) begin
        timer_div <= 6'd0;
        timer <= 64'd0;
    end else if ((timer_div) == (6'd39)) begin
        timer_div <= 6'd0;
        timer <= (timer) + (64'd1);
    end else begin
        timer_div <= (timer_div) + (6'd1);
    end
end

always_ff @(posedge clk40) begin
    if ((((cpu.io_write_out) & (cpu.io_enable_out)) & (cpu.k_flag_out)) & ((cpu.io_address_out) == (12'd4095))) begin
        syscall_addr <= __tmp_1[31:2];
    end
end

always_ff @(posedge clk40) begin
    io_addr_reg <= cpu.io_address_out;
    k_flag_reg <= cpu.k_flag_out;
end

always_comb begin
    pll.clk25 = clk25.d_in;
    clk200 = pll.clk200;
    clk80 = pll.clk80;
    clk40 = pll.clk40;
    reset = (~(reset_n.d_in)) | (~(pll.locked));
end

always_comb begin
    cpu_reset = (((((((reset40) | (cpu_reset_1)) | (cpu_reset_2)) | (cpu_reset_3)) | (cpu_reset_4)) | (cpu_reset_5)) | (cpu_reset_6)) | (cpu_reset_7);
end

always_comb begin
    cpu.enable = 1'd1;
    cpu.reset = cpu_reset;
    cpu.clk = clk40;
end

always_comb begin
    kram.instr_addr_in = __tmp_2[14:2];
    cpu.instruction_word_in = kram.instr_out;
    kram.data_addr_in = __tmp_3[12:0];
    kram.data_in = cpu.mem_data_out;
    kram.data_byte_enable = cpu.mem_byte_enable_out;
    kram.clk = clk40;
end

always_comb begin
    sram_address.d_out = sram.sram_address_out;
    sram_we_n.d_out = sram.sram_we_n_out;
    sram_a_data.d_out = sram.sram_a_data_out;
    sram.sram_a_data_in = sram_a_data.d_in;
    sram_a_data.oe = sram.sram_we_out;
    sram_a_ce_n.d_out = sram.sram_a_ce_n_out;
    sram_a_oe_n.d_out = sram.sram_a_oe_n_out;
    sram_a_lb_n.d_out = sram.sram_a_lb_n_out;
    sram_a_ub_n.d_out = sram.sram_a_ub_n_out;
    sram_b_data.d_out = sram.sram_b_data_out;
    sram.sram_b_data_in = sram_b_data.d_in;
    sram_b_data.oe = sram.sram_we_out;
    sram_b_ce_n.d_out = sram.sram_b_ce_n_out;
    sram_b_oe_n.d_out = sram.sram_b_oe_n_out;
    sram_b_lb_n.d_out = sram.sram_b_lb_n_out;
    sram_b_ub_n.d_out = sram.sram_b_ub_n_out;
    sram.address_in = __tmp_4[17:0];
    sram.data_in = cpu.mem_data_out;
    sram.byte_enable_in = cpu.mem_byte_enable_out;
    sram.reset = reset40;
    sram.clk = clk40;
    sram.clk2 = clk80;
end

always_comb begin
    vram.cpu_address = __tmp_5[21:0];
    vram.cpu_data_in = cpu.mem_data_out;
    vram.cpu_byte_enable = cpu.mem_byte_enable_out;
    vram.cpu_clk = clk40;
    vram.vdp_clk = clk40;
end

always_comb begin
    mmu.mem_page = __tmp_6[29:22];
    mmu.mem_enable = cpu.mem_enable_out;
    mmu.mem_write = cpu.mem_write_out;
    mmu.k_flag = cpu.k_flag_out;
    kram.data_write = mmu.kram_write;
    sram.write_read_n_in = mmu.sram_write;
    vram.cpu_write = mmu.vram_write;
    mmu.kram_data_in = kram.data_out;
    mmu.sram_data_in = sram.data_out;
    mmu.vram_data_in = vram.cpu_data_out;
    cpu.mem_data_in = mmu.data_out;
    mmu.reset = reset40;
    mmu.clk = clk40;
end

always_comb begin
    serial.addr_in = __tmp_7[1:0];
    serial.data_in = __tmp_8[7:0];
    serial.write = cpu.io_write_out;
    serial.rxd = rxd.d_in;
    txd.d_out = serial.txd;
    serial.enable = (cpu.io_enable_out) & ((cpu.io_address_out) <= (12'd3));
    serial.reset = reset40;
    serial.clk = clk40;
end

always_comb begin
    vram.vdp_bitmap_index = vdp.bitmap_index;
    vram.vdp_bitmap_row = vdp.bitmap_row;
    vdp.row_data_in = vram.vdp_row_data_out;
    vram.vdp_palette_index = vdp.palette_index;
    vram.vdp_color_index = vdp.color_index;
    vdp.color_in = vram.vdp_color_out;
    vram.vdp_tile_column = vdp.tile_column;
    vram.vdp_tile_row = vdp.tile_row;
    vdp.bitmap_index_in = vram.vdp_bitmap_index_out;
    vdp.palette_index_in = vram.vdp_palette_index_out;
    hdmi_clk.d_out = clk40;
    hdmi_d0.d_out = vdp.b_out;
    hdmi_d1.d_out = vdp.g_out;
    hdmi_d2.d_out = vdp.r_out;
    vdp.reset = reset40;
    vdp.sclk = clk200;
    vdp.pclk = clk40;
    vdp.cpu_addr_in = __tmp_9[1:0];
    vdp.cpu_data_in = cpu.io_data_out;
    vdp.cpu_write = (((cpu.io_write_out) & (cpu.io_enable_out)) & ((cpu.io_address_out) >= (12'd4))) & ((cpu.io_address_out) <= (12'd7));
end

always_comb begin
    led.data_in = __tmp_10[23:0];
    led.write = ((cpu.io_write_out) & (cpu.io_enable_out)) & ((cpu.io_address_out) == (12'd15));
    led_r.d_out = led.r_out;
    led_g.d_out = led.g_out;
    led_b.d_out = led.b_out;
    led.reset = reset40;
    led.clk = clk40;
end

always_comb begin
    cpu.syscall_addr_in = syscall_addr;
end

always_comb begin
    cpu.io_data_in = __tmp_11;
end


endmodule

