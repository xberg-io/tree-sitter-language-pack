package fifo_pkg;
  parameter int WIDTH = 8;
  typedef logic [WIDTH-1:0] word_t;

  function automatic int next_ptr(int ptr, int depth);
    return (ptr + 1) % depth;
  endfunction
endpackage

interface fifo_if (input logic clk);
  logic push;
  logic pop;
endinterface

class Transaction;
  rand bit [7:0] data;

  function void display();
    $display("data=%0d", data);
  endfunction

  task automatic send();
    #1;
  endtask
endclass

module fifo #(parameter int DEPTH = 4) (
  input  logic clk,
  input  logic rst_n,
  fifo_if bus
);
  import fifo_pkg::*;

  word_t mem [DEPTH];
  int wr_ptr;

  always_ff @(posedge clk) begin
    if (!rst_n) wr_ptr <= 0;
    else if (bus.push) wr_ptr <= next_ptr(wr_ptr, DEPTH);
  end
endmodule

module top;
  logic clk;
  fifo_if bus (.clk(clk));
  fifo #(.DEPTH(8)) u_fifo (.clk(clk), .rst_n(1'b1), .bus(bus));
endmodule
