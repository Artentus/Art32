module BaudTickGen #(
    parameter BAUD_RATE = 115200,
    parameter CLK_RATE = 100_000_000,
    parameter OVERSAMPLING = 1
) (
    output wire tick,
    
    input wire enable,
    input wire clk
);

    function integer log2(input integer v);
    begin
        log2=0;
        while ((v >> log2) != 0)
            log2 = log2 + 1;
    end
    endfunction

    localparam acc_width = log2(CLK_RATE / BAUD_RATE) + 8;
    reg [acc_width:0] acc = 0;

    localparam shiftlimiter = log2((BAUD_RATE * OVERSAMPLING) >> (31 - acc_width)); // This makes sure inc calculation doesn't overflow (verilog uses 32bit variables internally)
    localparam inc = ((BAUD_RATE * OVERSAMPLING << (acc_width - shiftlimiter)) + (CLK_RATE >> (shiftlimiter + 1))) / (CLK_RATE >> shiftlimiter); // Calculate accumulate increment

    always @(posedge clk) begin
        if (enable)
            acc <= acc[acc_width-1:0] + inc[acc_width:0];
        else
            acc <= inc[acc_width:0];
    end
    
    assign tick = acc[acc_width];

endmodule

module UartRx #(
    parameter BAUD_RATE = 115200,
    parameter CLK_RATE = 100_000_000,
    parameter OVERSAMPLING = 16
) (
    output reg [7:0] data_out,
    output wire      received,

    input wire rxd,

    input wire reset,
    input wire clk
);

    function integer log2(input integer v);
    begin
        log2=0;
        while ((v >> log2) != 0)
            log2 = log2 + 1;
    end
    endfunction

    localparam
        STATE_IDLE      = 4'b0000,
        STATE_BIT_START = 4'b0001,
        STATE_BIT0      = 4'b1000,
        STATE_BIT1      = 4'b1001,
        STATE_BIT2      = 4'b1010,
        STATE_BIT3      = 4'b1011,
        STATE_BIT4      = 4'b1100,
        STATE_BIT5      = 4'b1101,
        STATE_BIT6      = 4'b1110,
        STATE_BIT7      = 4'b1111,
        STATE_BIT_STOP  = 4'b0010;

    localparam l2o = log2(OVERSAMPLING);

    reg [3:0] state = STATE_IDLE;
    reg [1:0] rxd_sync = 2'b11;
    reg [1:0] filter = 2'b11;
    reg rxd_bit = 1'b1;
    wire tick;
    reg [l2o-2:0] os_count = 0;
    wire sample;
    reg [1:0] ready = 0;

    assign sample = tick && ({1'b0, os_count} == ((OVERSAMPLING / 2) - 1));
    assign received = ready[0] && !ready[1];
    
    BaudTickGen #(BAUD_RATE, CLK_RATE, OVERSAMPLING) gen (
        .clk(clk),
        .enable(1'b1),
        .tick(tick)
    );

    always @(posedge clk) begin
        if (reset) begin
            state <= STATE_IDLE;
            rxd_sync <= 2'b11;
            filter <= 2'b11;
            rxd_bit <= 1'b1;
            os_count <= 0;
            ready <= 0;
        end else begin
            if (tick) begin
                rxd_sync <= {rxd_sync[0], rxd};
                os_count <= (state == STATE_IDLE) ? 0 : os_count + 1;

                if( rxd_sync[1] && !(filter[0] && filter[1])) filter <= filter + 2'd1;
                if(!rxd_sync[1] &&  (filter[0] || filter[1])) filter <= filter - 2'd1;

                if ( filter[0] &&  filter[1]) rxd_bit <= 1'b1;
                else
                if (!filter[0] && !filter[1]) rxd_bit <= 1'b0;
            end

            case (state)
                STATE_IDLE:      if(!rxd_bit) state <= STATE_BIT_START;
                STATE_BIT_START: if(sample) state <= STATE_BIT0;
                STATE_BIT0:      if(sample) state <= STATE_BIT1;
                STATE_BIT1:      if(sample) state <= STATE_BIT2;
                STATE_BIT2:      if(sample) state <= STATE_BIT3;
                STATE_BIT3:      if(sample) state <= STATE_BIT4;
                STATE_BIT4:      if(sample) state <= STATE_BIT5;
                STATE_BIT5:      if(sample) state <= STATE_BIT6;
                STATE_BIT6:      if(sample) state <= STATE_BIT7;
                STATE_BIT7:      if(sample) state <= STATE_BIT_STOP;
                STATE_BIT_STOP:  if(sample) state <= STATE_IDLE;
                default:         state <= STATE_IDLE;
            endcase

            if (sample && state[3])
                data_out <= {rxd_bit, data_out[7:1]};

            ready[1] <= ready[0];
            ready[0] <= (sample && (state == STATE_BIT_STOP) && rxd_bit);
        end
    end

endmodule

module UartTx #(
    parameter BAUD_RATE = 115200,
    parameter CLK_RATE = 100_000_000
) (
    input  wire [7:0] data_in,
    input  wire       transmit,
    output wire       fetch,

    output wire txd,

    input wire reset,
    input wire clk
);

    localparam
        STATE_IDLE      = 4'b0000, // tx = high
        STATE_BIT_START = 4'b0100, // tx = low
        STATE_BIT0      = 4'b1000, // tx = data bit 0
        STATE_BIT1      = 4'b1001, // tx = data bit 1
        STATE_BIT2      = 4'b1010, // tx = data bit 2
        STATE_BIT3      = 4'b1011, // tx = data bit 3
        STATE_BIT4      = 4'b1100, // tx = data bit 4
        STATE_BIT5      = 4'b1101, // tx = data bit 5
        STATE_BIT6      = 4'b1110, // tx = data bit 6
        STATE_BIT7      = 4'b1111, // tx = data bit 7
        STATE_BIT_STOP1 = 4'b0010, // tx = high
        STATE_BIT_STOP2 = 4'b0011; // tx = high

    reg [3:0] state = STATE_IDLE;
    reg [7:0] shift = 0;
    wire busy;
    wire tick;

    assign busy = state != STATE_IDLE;
    assign fetch = (state == STATE_IDLE) && transmit;

    BaudTickGen #(BAUD_RATE, CLK_RATE) gen (
        .clk(clk),
        .enable(busy),
        .tick(tick)
    );

    always @(posedge clk) begin
        if (reset) begin
            state <= STATE_IDLE;
            shift <= 0;
        end else begin
            case (state)
                STATE_IDLE: if (transmit) begin
                    shift <= data_in;
                    state <= STATE_BIT_START;
                end
                STATE_BIT_START: if(tick) state <= STATE_BIT0;
                STATE_BIT0:      if(tick) state <= STATE_BIT1;
                STATE_BIT1:      if(tick) state <= STATE_BIT2;
                STATE_BIT2:      if(tick) state <= STATE_BIT3;
                STATE_BIT3:      if(tick) state <= STATE_BIT4;
                STATE_BIT4:      if(tick) state <= STATE_BIT5;
                STATE_BIT5:      if(tick) state <= STATE_BIT6;
                STATE_BIT6:      if(tick) state <= STATE_BIT7;
                STATE_BIT7:      if(tick) state <= STATE_BIT_STOP1;
                STATE_BIT_STOP1: if(tick) state <= STATE_BIT_STOP2;
                STATE_BIT_STOP2: if(tick) state <= STATE_IDLE;
                default:         state <= STATE_IDLE;
            endcase

            if (tick && state[3])
                shift <= (shift >> 1);
        end
    end

    //           high if state IDLE, STOP1, STOP2
    //           |                           high if transmitting bits and bit is 1
    //           |                           |
    //           V                           V
    assign txd = (!state[3] && !state[2]) || (state[3] && shift[0]);

endmodule

module Uart (
    input  wire [7:0] data_in,
    input  wire       transmit,
    output wire       fetch,

    output wire [7:0] data_out,
    output wire       received,

    input  wire rxd,
    output wire txd,

    input wire reset,
    input wire clk
);

    localparam BAUD_RATE = 115200; // 115.2 kBaud
    localparam CLK_RATE = 40_000_000; // 40 MHz
    localparam OVERSAMPLING = 16;

    UartRx #(BAUD_RATE, CLK_RATE, OVERSAMPLING) rx (
        .rxd(rxd),
        .data_out(data_out),
        .received(received),
        .reset(reset),
        .clk(clk)
    );

    UartTx #(BAUD_RATE, CLK_RATE) tx (
        .txd(txd),
        .data_in(data_in),
        .transmit(transmit),
        .fetch(fetch),
        .reset(reset),
        .clk(clk)
    );

endmodule
