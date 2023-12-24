use super::MAX_SIM_STEPS;
use crate::module;
use gsim::LogicState;
use proptest::prelude::ProptestConfig;
use proptest::sample::size_range;
use std::num::NonZeroU8;
use test_strategy::*;

module! {
    FETCH_UNIT: FetchUnit = "fetch_unit" {
        out instruction_address,
        out request_instruction_data,
        in instruction_data,

        out instruction,
        out instruction_16bit,
        out instruction_valid,
        out program_counter,

        in advance,
        in jump_target,
        in do_jump,

        in enable,
        in reset,
        in clk,
    }
}

struct GoldenFetchUnit {
    instruction_buffer: [u16; 3],
    valid_half_words: u8,
    program_counter: u32,
}

impl GoldenFetchUnit {
    #[inline]
    fn new() -> Self {
        Self {
            instruction_buffer: [0; 3],
            valid_half_words: 0,
            program_counter: 0,
        }
    }

    fn clock(
        &mut self,
        read_instruction_word: impl FnOnce(u32) -> u32,
        advance: bool,
        jump_target: u32,
        do_jump: bool,
        enable: bool,
        reset: bool,
    ) {
        assert_eq!(jump_target % 2, 0, "unaligned jump target");

        if reset {
            self.instruction_buffer = [0; 3];
            self.valid_half_words = 0;
            self.program_counter = 0;
        } else {
            let next_instruction_buffer;
            let next_valid_half_words;
            let next_program_counter;

            if do_jump {
                let instruction_word = read_instruction_word(jump_target & !0x3);
                next_program_counter = jump_target;

                if (jump_target & 0x2) > 0 {
                    next_valid_half_words = 1;
                    next_instruction_buffer = [(instruction_word >> 16) as u16, 0, 0];
                } else {
                    next_valid_half_words = 2;
                    next_instruction_buffer =
                        [instruction_word as u16, (instruction_word >> 16) as u16, 0];
                }
            } else if advance {
                assert!(self.instruction_valid(), "illegal advance signal");

                if self.instruction_16bit() {
                    next_program_counter = self.program_counter + 2;

                    match self.valid_half_words {
                        1 => {
                            let instruction_word = read_instruction_word(self.program_counter + 2);
                            next_valid_half_words = 2;
                            next_instruction_buffer =
                                [instruction_word as u16, (instruction_word >> 16) as u16, 0];
                        }
                        2 => {
                            let instruction_word = read_instruction_word(self.program_counter + 4);
                            next_valid_half_words = 3;
                            next_instruction_buffer = [
                                self.instruction_buffer[1],
                                instruction_word as u16,
                                (instruction_word >> 16) as u16,
                            ];
                        }
                        3 => {
                            next_valid_half_words = 2;
                            next_instruction_buffer =
                                [self.instruction_buffer[1], self.instruction_buffer[2], 0];
                        }
                        _ => unreachable!("illegal number of valid half words"),
                    }
                } else {
                    next_program_counter = self.program_counter + 4;

                    match self.valid_half_words {
                        2 => {
                            let instruction_word = read_instruction_word(self.program_counter + 4);
                            next_valid_half_words = 2;
                            next_instruction_buffer =
                                [instruction_word as u16, (instruction_word >> 16) as u16, 0];
                        }
                        3 => {
                            let instruction_word = read_instruction_word(self.program_counter + 6);
                            next_valid_half_words = 3;
                            next_instruction_buffer = [
                                self.instruction_buffer[2],
                                instruction_word as u16,
                                (instruction_word >> 16) as u16,
                            ];
                        }
                        _ => unreachable!("illegal number of valid half words"),
                    }
                }
            } else {
                next_program_counter = self.program_counter;

                match self.valid_half_words {
                    0 => {
                        let instruction_word = read_instruction_word(self.program_counter);
                        next_valid_half_words = 2;
                        next_instruction_buffer =
                            [instruction_word as u16, (instruction_word >> 16) as u16, 0];
                    }
                    1 => {
                        let instruction_word = read_instruction_word(self.program_counter + 2);
                        next_valid_half_words = 3;
                        next_instruction_buffer = [
                            self.instruction_buffer[0],
                            instruction_word as u16,
                            (instruction_word >> 16) as u16,
                        ];
                    }
                    2 => {
                        next_valid_half_words = 2;
                        next_instruction_buffer = self.instruction_buffer;
                    }
                    3 => {
                        next_valid_half_words = 3;
                        next_instruction_buffer = self.instruction_buffer;
                    }
                    _ => unreachable!("illegal number of valid half words"),
                }
            }

            if enable {
                self.instruction_buffer = next_instruction_buffer;
                self.valid_half_words = next_valid_half_words;
                self.program_counter = next_program_counter;
            }
        }
    }

    #[inline]
    fn instruction(&self) -> u32 {
        (self.instruction_buffer[0] as u32) | ((self.instruction_buffer[1] as u32) << 16)
    }

    #[inline]
    fn instruction_16bit(&self) -> bool {
        ((self.instruction_buffer[0] & 0b_1_0000_111) != 0b_1_0000_011)
            && ((self.instruction_buffer[0] & 0b111111) != 0b111111)
    }

    #[inline]
    fn instruction_valid(&self) -> bool {
        (self.valid_half_words >= 2) || ((self.valid_half_words >= 1) && self.instruction_16bit())
    }

    #[inline]
    fn program_counter(&self) -> u32 {
        self.program_counter
    }
}

fn assert_no_access(_: u32) -> u32 {
    panic!("accessed memory unexpectedly");
}

const WIDTH_30: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(30) };
const WIDTH_31: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(31) };
const WIDTH_32: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(32) };

#[proptest(ProptestConfig { cases : 500, ..ProptestConfig::default() })]
fn linear_advance(#[any(size_range(500..=500).lift())] mem: Vec<u32>) {
    FETCH_UNIT.with(|fetch_unit| {
        let mut golden = GoldenFetchUnit::new();

        golden.clock(assert_no_access, false, 0, false, true, true);

        assert!(!golden.instruction_valid());
        assert_eq!(golden.program_counter(), 0);

        let mut sim = fetch_unit.sim.borrow_mut();
        sim.reset();

        sim.set_wire_drive(fetch_unit.instruction_data, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.advance, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.jump_target, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.do_jump, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.enable, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(fetch_unit.reset, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_0)
            .unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_1)
            .unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.reset, &LogicState::LOGIC_0)
            .unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        let actual_valid = sim.get_wire_state(fetch_unit.instruction_valid).unwrap();
        let actual_pc = sim.get_wire_state(fetch_unit.program_counter).unwrap();
        assert!(actual_valid.eq(&LogicState::LOGIC_0, NonZeroU8::MIN));
        assert!(actual_pc.eq(&LogicState::LOGIC_0, WIDTH_31));

        let mut current_pc = 0;

        for _ in 0..500 {
            let mut memory_accessed = false;
            let read_instruction_word = |addr: u32| -> u32 {
                assert_eq!(addr % 4, 0, "unaligned memory access");
                memory_accessed = true;
                mem[((addr >> 2) as usize) % mem.len()]
            };

            let expected_pc = if golden.instruction_valid() {
                if golden.instruction_16bit() {
                    current_pc + 2
                } else {
                    current_pc + 4
                }
            } else {
                current_pc
            };

            let advance = golden.instruction_valid();

            golden.clock(read_instruction_word, advance, 0, false, true, false);

            assert_eq!(golden.program_counter(), expected_pc);
            current_pc = expected_pc;

            let expected = golden.instruction();
            let expected_valid = golden.instruction_valid();
            let expected_16bit = golden.instruction_16bit();

            sim.set_wire_drive(fetch_unit.advance, &LogicState::from_bool(advance))
                .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            let request_instruction_data = sim
                .get_wire_state(fetch_unit.request_instruction_data)
                .unwrap()
                .to_bool()
                .unwrap();
            assert_eq!(request_instruction_data, memory_accessed);

            let instruction_address = sim
                .get_wire_state(fetch_unit.instruction_address)
                .unwrap()
                .to_int(WIDTH_30)
                .unwrap();
            sim.set_wire_drive(
                fetch_unit.instruction_data,
                &LogicState::from_int(mem[(instruction_address as usize) % mem.len()]),
            )
            .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_1)
                .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();
            sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_0)
                .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            let actual_valid = sim
                .get_wire_state(fetch_unit.instruction_valid)
                .unwrap()
                .to_bool()
                .unwrap();
            let actual_pc = sim
                .get_wire_state(fetch_unit.program_counter)
                .unwrap()
                .to_int(WIDTH_31)
                .unwrap()
                << 1;
            assert_eq!(actual_valid, expected_valid);
            assert_eq!(actual_pc, expected_pc);
            if expected_valid {
                let actual = sim
                    .get_wire_state(fetch_unit.instruction)
                    .unwrap()
                    .to_int(WIDTH_32)
                    .unwrap();
                let actual_16bit = sim
                    .get_wire_state(fetch_unit.instruction_16bit)
                    .unwrap()
                    .to_bool()
                    .unwrap();
                assert_eq!(actual, expected);
                assert_eq!(actual_16bit, expected_16bit);
            }
        }
    })
}

#[proptest(ProptestConfig { cases : 500, ..ProptestConfig::default() })]
fn jump_once(#[any(size_range(50..=50).lift())] mem: Vec<u32>, #[strategy(..50u32)] jump_at: u32) {
    FETCH_UNIT.with(|fetch_unit| {
        let mut golden = GoldenFetchUnit::new();

        golden.clock(assert_no_access, false, 0, false, true, true);

        assert!(!golden.instruction_valid());
        assert_eq!(golden.program_counter(), 0);

        let mut sim = fetch_unit.sim.borrow_mut();
        sim.reset();

        sim.set_wire_drive(fetch_unit.instruction_data, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.advance, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.jump_target, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.do_jump, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.enable, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(fetch_unit.reset, &LogicState::LOGIC_1)
            .unwrap();
        sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_0)
            .unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_1)
            .unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();
        sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_0)
            .unwrap();
        sim.set_wire_drive(fetch_unit.reset, &LogicState::LOGIC_0)
            .unwrap();
        sim.run_sim(MAX_SIM_STEPS).unwrap();

        let actual_valid = sim.get_wire_state(fetch_unit.instruction_valid).unwrap();
        let actual_pc = sim.get_wire_state(fetch_unit.program_counter).unwrap();
        assert!(actual_valid.eq(&LogicState::LOGIC_0, NonZeroU8::MIN));
        assert!(actual_pc.eq(&LogicState::LOGIC_0, WIDTH_31));

        let mut current_pc = 0;

        for i in 0..50 {
            let mut memory_accessed = false;
            let read_instruction_word = |addr: u32| -> u32 {
                assert_eq!(addr % 4, 0, "unaligned memory access");
                memory_accessed = true;
                mem[((addr >> 2) as usize) % mem.len()]
            };

            let do_jump = i == jump_at;

            let expected_pc = if do_jump {
                0
            } else if golden.instruction_valid() {
                if golden.instruction_16bit() {
                    current_pc + 2
                } else {
                    current_pc + 4
                }
            } else {
                current_pc
            };

            let advance = golden.instruction_valid();

            golden.clock(read_instruction_word, advance, 0, do_jump, true, false);

            assert_eq!(golden.program_counter(), expected_pc);
            current_pc = expected_pc;

            let expected = golden.instruction();
            let expected_valid = golden.instruction_valid();
            let expected_16bit = golden.instruction_16bit();

            sim.set_wire_drive(fetch_unit.advance, &LogicState::from_bool(advance))
                .unwrap();
            sim.set_wire_drive(fetch_unit.do_jump, &LogicState::from_bool(do_jump))
                .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            let request_instruction_data = sim
                .get_wire_state(fetch_unit.request_instruction_data)
                .unwrap()
                .to_bool()
                .unwrap();
            assert_eq!(request_instruction_data, memory_accessed);

            let instruction_address = sim
                .get_wire_state(fetch_unit.instruction_address)
                .unwrap()
                .to_int(WIDTH_30)
                .unwrap();
            sim.set_wire_drive(
                fetch_unit.instruction_data,
                &LogicState::from_int(mem[(instruction_address as usize) % mem.len()]),
            )
            .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_1)
                .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();
            sim.set_wire_drive(fetch_unit.clk, &LogicState::LOGIC_0)
                .unwrap();
            sim.run_sim(MAX_SIM_STEPS).unwrap();

            let actual_valid = sim
                .get_wire_state(fetch_unit.instruction_valid)
                .unwrap()
                .to_bool()
                .unwrap();
            let actual_pc = sim
                .get_wire_state(fetch_unit.program_counter)
                .unwrap()
                .to_int(WIDTH_31)
                .unwrap()
                << 1;
            assert_eq!(actual_valid, expected_valid);
            assert_eq!(actual_pc, expected_pc);
            if expected_valid {
                let actual = sim
                    .get_wire_state(fetch_unit.instruction)
                    .unwrap()
                    .to_int(WIDTH_32)
                    .unwrap();
                let actual_16bit = sim
                    .get_wire_state(fetch_unit.instruction_16bit)
                    .unwrap()
                    .to_bool()
                    .unwrap();
                assert_eq!(actual, expected);
                assert_eq!(actual_16bit, expected_16bit);
            }
        }
    })
}
