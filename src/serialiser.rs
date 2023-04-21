use std::collections::BTreeMap;

use crate::concrete_cfg::{from_fe, CurveAcir, CurveAcirArithGate, Fr};
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

        let values: Vec<Fr> = witness_map.into_values().map(from_fe).collect();

        // let num_variables = (circ.current_witness_index + 1) as usize;
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
