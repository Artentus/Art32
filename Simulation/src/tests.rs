use gsim::*;

//mod fetch_unit;
//mod program_counter;
mod alu;
mod fpu;

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

fn assert_wire_state<VCD: std::io::Write>(
    sim: &Simulator<VCD>,
    line: usize,
    wire: WireId,
    expected: &LogicState,
    statement: &str,
) {
    let actual = sim.get_wire_state(wire).unwrap();
    let wire_width = sim.get_wire_width(wire).unwrap();

    assert!(
        actual.eq(expected, wire_width),
        "[LINE {line}]  ({statement})  expected: {}  actual: {}",
        expected.display_string(wire_width),
        actual.display_string(wire_width),
    );
}

macro_rules! run_test {
    (@CHAIN $sim:ident; $line:expr; $time:expr;) => {};
    (@CHAIN $sim:ident; $line:expr; $time:expr; $wire:ident <= $state:tt; $($t:tt)*) => {
        let _line = $line;
        $sim.set_wire_drive($wire, &$crate::tests::logic_state!($state)).unwrap();
        $sim.run_sim($crate::tests::MAX_SIM_STEPS).unwrap();

        $crate::tests::run_test!(@CHAIN $sim; _line + 1; $time; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; $time:expr; posedge $wire:ident; $($t:tt)*) => {
        let _line = $line;
        $sim.set_wire_drive($wire, &$crate::tests::logic_state!(true)).unwrap();
        $sim.run_sim($crate::tests::MAX_SIM_STEPS).unwrap();

        $crate::tests::run_test!(@CHAIN $sim; _line + 1; $time; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; $time:expr; negedge $wire:ident; $($t:tt)*) => {
        let _line = $line;
        $sim.set_wire_drive($wire, &$crate::tests::logic_state!(false)).unwrap();
        $sim.run_sim($crate::tests::MAX_SIM_STEPS).unwrap();

        $crate::tests::run_test!(@CHAIN $sim; _line + 1; $time; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; $time:expr; assert $wire:ident == $state:tt; $($t:tt)*) => {
        let _line = $line;
        let expected = $crate::tests::logic_state!($state);
        let statement = stringify!(assert $wire == $state);
        $crate::tests::assert_wire_state(&$sim, _line, $wire, &expected, statement);

        $crate::tests::run_test!(@CHAIN $sim; _line + 1; $time; $($t)*);
    };
    (@CHAIN $sim:ident; $line:expr; $time:expr; trace; $($t:tt)*) => {
        let _line = $line;
        $sim.trace($time).unwrap();

        $crate::tests::run_test!(@CHAIN $sim; _line + 1; $time + 1; $($t)*);
    };
    ($sim:ident[$time:expr] => { $($t:tt)* }) => {
        $sim.run_sim($crate::tests::MAX_SIM_STEPS);
        $crate::tests::run_test!(@CHAIN $sim; 1; $time; $($t)*);
    };
    ($sim:ident => { $($t:tt)* }) => {
        $sim.run_sim($crate::tests::MAX_SIM_STEPS).unwrap();
        $crate::tests::run_test!(@CHAIN $sim; 1; 0; $($t)*);
    };
}

use run_test;
