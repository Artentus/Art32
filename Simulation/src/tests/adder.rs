use super::run_test;
use crate::import;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Op {
    Add = 0,
    AddC = 1,
    Sub = 2,
    SubC = 3,
}

impl Op {
    const ALL: &'static [Self] = &[Self::Add, Self::AddC, Self::Sub, Self::SubC];
}

const fn carrying_add(lhs: u8, rhs: u8, carry: bool) -> (u8, bool) {
    let (s1, c1) = lhs.overflowing_add(rhs);
    let (s2, c2) = s1.overflowing_add(carry as u8);
    (s2, c1 | c2)
}

#[test]
fn adder() {
    let (mut sim, ports) = import!("adder");

    let lhs = ports.inputs["lhs"];
    let rhs = ports.inputs["rhs"];
    let carry_in = ports.inputs["carry_in"];
    let op = ports.inputs["op"];

    let result = ports.outputs["result"];
    let carry_out = ports.outputs["carry_out"];
    let sign = ports.outputs["sign"];
    let overflow = ports.outputs["overflow"];

    for &use_op in Op::ALL {
        for use_lhs in 0..=u8::MAX {
            for use_rhs in 0..=u8::MAX {
                for use_carry_in in [false, true] {
                    let golden_rhs = match use_op {
                        Op::Add | Op::AddC => use_rhs,
                        Op::Sub | Op::SubC => !use_rhs,
                    };

                    let use_lhs_sign = (use_lhs as i8) < 0;
                    let use_rhs_sign = (golden_rhs as i8) < 0;

                    let golden_carry = match use_op {
                        Op::Add => false,
                        Op::AddC => use_carry_in,
                        Op::Sub => true,
                        Op::SubC => use_carry_in,
                    };

                    let (expect_result, expect_carry_out) =
                        carrying_add(use_lhs, golden_rhs, golden_carry);
                    let expect_sign = (expect_result as i8) < 0;
                    let expect_overflow =
                        (use_lhs_sign == use_rhs_sign) & (use_lhs_sign != expect_sign);

                    run_test!(sim => {
                        lhs <= {use_lhs as u32};
                        rhs <= {use_rhs as u32};
                        carry_in <= {use_carry_in as u32};
                        op <= {use_op as u32};

                        assert result == {expect_result as u32};
                        assert carry_out == {expect_carry_out as u32};
                        assert sign == {expect_sign as u32};
                        assert overflow == {expect_overflow as u32};
                    });
                }
            }
        }
    }
}
