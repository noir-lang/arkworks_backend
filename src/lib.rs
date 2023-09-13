#![forbid(unsafe_code)]
#![warn(unused_crate_dependencies, unused_extern_crates)]
#![warn(unreachable_pub)]
#![warn(clippy::semicolon_if_nothing_returned)]

use acvm::acir::circuit::{Circuit, Opcode};

pub mod bridge;
mod concrete_cfg;
mod serializer;

pub use concrete_cfg::{from_fe, Curve, CurveAcir, Fr};

pub fn compute_num_constraints(acir: &Circuit) -> u32 {
    let mut num_opcodes = acir.opcodes.len();

    for opcode in acir.opcodes.iter() {
        match opcode {
            Opcode::Arithmetic(arith) => {
                // Each multiplication term adds an extra constraint
                // plus one for the linear combination gate.
                num_opcodes += arith.num_mul_terms() + 1;
            }
            Opcode::Directive(_) => (),
            _ => unreachable!(
                "currently we do not support non-arithmetic opcodes {:?}",
                opcode
            ),
        }
    }

    num_opcodes as u32
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;
    use acvm::acir::circuit::{Opcode, PublicInputs};
    use acvm::acir::native_types::{Expression, Witness};
    use acvm::FieldElement;

    #[test]
    fn simple_equal() {
        let a = Witness(1);
        let b = Witness(2);

        // assert a == b
        let arith = Expression {
            mul_terms: vec![],
            linear_combinations: vec![(FieldElement::one(), a), (-FieldElement::one(), b)],
            q_c: FieldElement::zero(),
        };
        let opcode = Opcode::Arithmetic(arith);
        let _circ = Circuit {
            current_witness_index: 2,
            opcodes: vec![opcode],
            public_parameters: PublicInputs(BTreeSet::from([Witness(1)])),
            return_values: PublicInputs(BTreeSet::new()),
            private_parameters: BTreeSet::new(),
            assert_messages: Vec::new(),
        };
        let a_val = FieldElement::from(6_i128);
        let b_val = FieldElement::from(6_i128);
        let _values = vec![&a_val, &b_val];

        todo!("re-add some meaningful test here");
    }
}
