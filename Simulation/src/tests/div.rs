use super::MAX_SIM_STEPS;
use crate::module;
use gsim::{LogicBitState, LogicState};
use proptest::prelude::ProptestConfig;
use std::num::NonZeroU8;
use test_strategy::*;

module! {
    DIV: Div = "div" {
        in numerator,
        in denominator,

        out quotient,
        out remainder,

        in start,
        out ready,

        in enable,
        in reset,
        in clk,
    }
}

#[proptest(ProptestConfig { cases : 5000, ..ProptestConfig::default() })]
fn div(numerator: u32, #[strategy(1u32..)] denominator: u32) {
    let expected_quotient = numerator / denominator;
    let expected_remainder = numerator % denominator;
    let (actual_quotient_str, actual_remainder_str) = DIV.with(|div| {
        let numerator = LogicState::from_int(numerator);
        let denominator = LogicState::from_int(denominator);

        let mut sim = div.sim.borrow_mut();
        sim.reset();

        sim.set_wire_drive(div.numerator, &numerator).unwrap();
        sim.set_wire_drive(div.denominator, &denominator).unwrap();
        sim.set_wire_drive(div.enable, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(div.reset, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(div.start, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(div.clk, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(div.reset, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(div.clk, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(div.clk, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(div.reset, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(div.start, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(div.clk, &LogicState::LOGIC_1).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(div.clk, &LogicState::LOGIC_0).unwrap();
        sim.set_wire_drive(div.start, &LogicState::LOGIC_0).unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        let mut cycle_count = 1;
        loop {
            let ready = sim.get_wire_state(div.ready).unwrap().get_bit_state(0);
            match ready {
                LogicBitState::HighZ | LogicBitState::Undefined => {
                    panic!("invalid ready output: {ready}")
                }
                LogicBitState::Logic0 => (),
                LogicBitState::Logic1 => break,
            }

            if cycle_count > 33 {
                panic!("failed to produce result in time")
            }

            sim.set_wire_drive(div.clk, &LogicState::LOGIC_1).unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();
            sim.set_wire_drive(div.clk, &LogicState::LOGIC_0).unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            cycle_count += 1;
        }

        let quotient = sim.get_wire_state(div.quotient).unwrap();
        let quotient = quotient.display_string(NonZeroU8::new(32).unwrap());
        let remainder = sim.get_wire_state(div.remainder).unwrap();
        let remainder = remainder.display_string(NonZeroU8::new(32).unwrap());
        (quotient, remainder)
    });

    let actual_quotient = u32::from_str_radix(&actual_quotient_str, 2);
    let actual_remainder = u32::from_str_radix(&actual_remainder_str, 2);

    if let (Ok(actual_quotient), Ok(actual_remainder)) = (actual_quotient, actual_remainder) {
        if (actual_quotient != expected_quotient) || (actual_remainder != expected_remainder) {
            panic!(
                "\n  numerator: {}\ndenominator: {}\n   expected: {}, {}\n     actual: {}, {}",
                numerator,
                denominator,
                expected_quotient,
                expected_remainder,
                actual_quotient,
                actual_remainder,
            );
        }
    } else {
        panic!(
            "\n  numerator: {}\ndenominator: {}\n   expected: {}, {}\n     actual: {}, {}",
            numerator,
            denominator,
            expected_quotient,
            expected_remainder,
            actual_quotient_str,
            actual_remainder_str,
        );
    }
}
