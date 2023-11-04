use gsim::*;

mod adder;
mod program_counter;

const MAX_SIM_STEPS: u64 = 1_000_000;

macro_rules! logic_state {
    (true) => {
        gsim::LogicState::from_bool(true)
    };
    (false) => {
        gsim::LogicState::from_bool(false)
    };
    ($state:ident) => {
        gsim::LogicState::$state
    };
    ($value:expr) => {
        gsim::LogicState::from_int($value)
    };
}

use logic_state;

fn assert_wire_state(
    sim: &Simulator,
    line: usize,
    wire: WireId,
    expected: &LogicState,
    statement: &str,
) {
    let actual = sim.get_wire_state(wire);
    let wire_width = sim.get_wire_width(wire);

    assert!(
        actual.eq(expected, wire_width),
        "[LINE {line}]  ({statement})  expected: {}  actual: {}",
        expected.display_string(wire_width),
        actual.display_string(wire_width),
    );
}

macro_rules! run_test {
    (@CHAIN $sim:ident; $line:expr;) => {};
    (@CHAIN $sim:ident; $line:expr; $wire:ident <= $state:tt; $($t:tt)*) => {
        let line = $line;
        $sim.set_wire_drive($wire, &$crate::tests::logic_state!($state));
        $sim.run_sim($crate::tests::MAX_SIM_STEPS);

        $crate::tests::run_test!(@CHAIN $sim; line + 1; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; posedge $wire:ident; $($t:tt)*) => {
        let line = $line;
        $sim.set_wire_drive($wire, &$crate::tests::logic_state!(true));
        $sim.run_sim($crate::tests::MAX_SIM_STEPS);

        $crate::tests::run_test!(@CHAIN $sim; line + 1; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; negedge $wire:ident; $($t:tt)*) => {
        let line = $line;
        $sim.set_wire_drive($wire, &$crate::tests::logic_state!(false));
        $sim.run_sim($crate::tests::MAX_SIM_STEPS);

        $crate::tests::run_test!(@CHAIN $sim; line + 1; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; assert $wire:ident == $state:tt; $($t:tt)*) => {
        let line = $line;
        let expected = $crate::tests::logic_state!($state);
        let statement = stringify!(assert $wire == $state);
        $crate::tests::assert_wire_state(&$sim, line, $wire, &expected, statement);

        $crate::tests::run_test!(@CHAIN $sim; line + 1; $($t)*);
    };
    ($sim:ident => { $($t:tt)* }) => {
        $sim.run_sim($crate::tests::MAX_SIM_STEPS);
        $crate::tests::run_test!(@CHAIN $sim; 1; $($t)*);
    };
}

use run_test;
