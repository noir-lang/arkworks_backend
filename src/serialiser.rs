use std::{collections::BTreeMap, convert::TryInto};

use crate::concrete_cfg::{from_fe, CurveAcir, CurveAcirArithGate};
use acvm::{
    acir::{
        circuit::Circuit,
        native_types::{Expression, Witness},
    },
    FieldElement,
};

/// Converts an ACIR into an ACIR struct that the arkworks backend can consume.
pub fn serialize(acir: &Circuit, witness_map: BTreeMap<Witness, FieldElement>) -> CurveAcir {
    (acir, witness_map).into()
}

impl From<(&Circuit, BTreeMap<Witness, FieldElement>)> for CurveAcir {
    fn from(circ_val: (&Circuit, BTreeMap<Witness, FieldElement>)) -> CurveAcir {
        // Currently non-arithmetic gates are not supported
        // so we extract all of the arithmetic gates only
        let (circuit, witness_map) = circ_val;

        let public_inputs = circuit.public_inputs();
        let arith_gates: Vec<_> = circuit
            .opcodes
            .iter()
            .filter(|opcode| opcode.is_arithmetic())
            .map(|opcode| CurveAcirArithGate::from(opcode.clone().arithmetic().unwrap()))
            .collect();

        let num_variables: usize = circuit.num_vars().try_into().unwrap();

        let values: BTreeMap<Witness, _> = (0..num_variables)
            .map(|witness_index| {
                // Get the value if it exists. If i does not, then we fill it with the zero value
                let witness = Witness(witness_index as u32);
                let value = witness_map
                    .get(&witness)
                    .map_or(FieldElement::zero(), |field| *field);

                (witness, from_fe(value))
            })
            .collect();

        CurveAcir {
            gates: arith_gates,
            values,
            // num_variables,
            public_inputs,
        }
    }
}

impl From<Expression> for CurveAcirArithGate {
    fn from(arith_gate: Expression) -> CurveAcirArithGate {
        let converted_mul_terms: Vec<_> = arith_gate
            .mul_terms
            .into_iter()
            .map(|(coeff, l_var, r_var)| (from_fe(coeff), l_var, r_var))
            .collect();

        let converted_linear_combinations: Vec<_> = arith_gate
            .linear_combinations
            .into_iter()
            .map(|(coeff, var)| (from_fe(coeff), var))
            .collect();

        CurveAcirArithGate {
            mul_terms: converted_mul_terms,
            add_terms: converted_linear_combinations,
            constant_term: from_fe(arith_gate.q_c),
        }
    }
}
