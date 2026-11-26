context counter_ctx is
  library ieee;
  use ieee.std_logic_1164.all;
end context counter_ctx;

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

package counter_pkg is
  constant WIDTH : natural := 8;
  type state_t is (IDLE, RUNNING);
  subtype count_t is unsigned(WIDTH - 1 downto 0);
  function increment(value : count_t) return count_t;
  procedure reset(signal value : out count_t);
  component counter is
    port (clk : in std_logic; count : out count_t);
  end component;
end package counter_pkg;

package body counter_pkg is
  function increment(value : count_t) return count_t is
  begin
    return value + 1;
  end function increment;

  procedure reset(signal value : out count_t) is
  begin
    value <= (others => '0');
  end procedure reset;
end package body counter_pkg;

library ieee;
use ieee.std_logic_1164.all;
use work.counter_pkg.all;

entity counter is
  port (
    clk   : in  std_logic;
    count : out count_t
  );
end entity counter;

architecture rtl of counter is
  signal value : count_t := (others => '0');
begin
  tick : process (clk)
  begin
    if rising_edge(clk) then
      value <= increment(value => value);
    end if;
  end process tick;
  count <= value;
end architecture rtl;

library ieee;
use ieee.std_logic_1164.all;
use work.counter_pkg.all;

entity top is
end entity top;

architecture structural of top is
  signal clk   : std_logic;
  signal count : count_t;
begin
  u_entity : entity work.counter(rtl)
    port map (clk => clk, count => count);
  u_component : counter
    port map (clk => clk, count => count);
end architecture structural;

package wide_counter_pkg is new work.generic_counter_pkg
  generic map (WIDTH => 16);

configuration top_cfg of top is
  for structural
  end for;
end configuration top_cfg;
