module TmdsEncoder (
    input wire       mode,
    input wire [1:0] control_data,
    input wire [7:0] video_data,
    output reg [9:0] encoded,

    input wire reset,
    input wire pclk
);

    /* verilator lint_off WIDTH */

    localparam
        MODE_CONTROL = 1'b0,
        MODE_VIDEO   = 1'b1;

    initial encoded = 10'h0;

    /* verilator lint_off UNOPTFLAT */
    
    wire [8:0] xored;
    assign xored[0] = video_data[0];
    assign xored[1] = video_data[1] ^ xored[0];
    assign xored[2] = video_data[2] ^ xored[1];
    assign xored[3] = video_data[3] ^ xored[2];
    assign xored[4] = video_data[4] ^ xored[3];
    assign xored[5] = video_data[5] ^ xored[4];
    assign xored[6] = video_data[6] ^ xored[5];
    assign xored[7] = video_data[7] ^ xored[6];
    assign xored[8] = 1'b1;

    wire [8:0] xnored;
    assign xnored[0] = video_data[0];
    assign xnored[1] = ~(video_data[1] ^ xnored[0]);
    assign xnored[2] = ~(video_data[2] ^ xnored[1]);
    assign xnored[3] = ~(video_data[3] ^ xnored[2]);
    assign xnored[4] = ~(video_data[4] ^ xnored[3]);
    assign xnored[5] = ~(video_data[5] ^ xnored[4]);
    assign xnored[6] = ~(video_data[6] ^ xnored[5]);
    assign xnored[7] = ~(video_data[7] ^ xnored[6]);
    assign xnored[8] = 1'b0;

    /* verilator lint_on UNOPTFLAT */

    wire [3:0] ones = video_data[0] + video_data[1] + video_data[2] + video_data[3] + video_data[4] + video_data[5] + video_data[6] + video_data[7];

    reg [8:0] data_word;
    reg [8:0] data_word_inv;
    always @(*) begin
        if ((ones > 4) || ((ones == 4) && (video_data[0] == 1'b0))) begin
            data_word     =  xnored;
            data_word_inv = ~xnored;
        end else begin
            data_word     =  xored;
            data_word_inv = ~xored;
        end
    end

    wire [3:0] data_word_disparity
        = 4'b1100
        + data_word[0]
        + data_word[1]
        + data_word[2]
        + data_word[3]
        + data_word[4]
        + data_word[5]
        + data_word[6]
        + data_word[7];

    reg [3:0] dc_bias;
    initial dc_bias = 4'h0;
    always @(posedge pclk) begin
        if (reset) begin
            encoded <= 10'h0;
            dc_bias <= 4'h0;
        end else begin
            case (mode)
                MODE_CONTROL: begin
                    case (control_data)
                        2'b00: encoded <= 10'b1101010100;
                        2'b01: encoded <= 10'b0010101011;
                        2'b10: encoded <= 10'b0101010100;
                        2'b11: encoded <= 10'b1010101011;
                    endcase

                    dc_bias <= 4'h0;
                end

                MODE_VIDEO: begin
                    if ((dc_bias == 4'h0) || (data_word_disparity == 4'h0)) begin
                        if (data_word[8]) begin
                            encoded <= { 2'b01, data_word[7:0] };
                            dc_bias <= dc_bias + data_word_disparity;
                        end else begin
                            encoded <= { 2'b10, data_word_inv[7:0] };
                            dc_bias <= dc_bias - data_word_disparity;
                        end
                    end else if (dc_bias[3] == data_word_disparity[3]) begin
                        encoded <= { 1'b1, data_word[8], data_word_inv[7:0] };
                        dc_bias <= dc_bias + data_word[8] - data_word_disparity;
                    end else begin
                        encoded <= { 1'b0, data_word };
                        dc_bias <= dc_bias - data_word_inv[8] + data_word_disparity;
                    end
                end
            endcase
        end
    end

    /* verilator lint_on WIDTH */

endmodule
