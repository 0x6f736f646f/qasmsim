#![cfg(test)]

extern crate qasmsim;

use std::f64::consts::FRAC_1_SQRT_2;

use qasmsim::statevector::{assert_approx_eq, Complex, StateVector};

#[test]
fn endianess() {
    let source = "
  OPENQASM 2.0;
  qreg q[1];
  qreg r[1];
  U (pi/2, 0, pi) r[0];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
        ]),
    )
}

#[test]
fn call_custom_gate_on_qubit() {
    let source = "
  OPENQASM 2.0;
  gate h q {
    U(pi/2, 0, pi) q;
  }
  qreg q[1];
  h q[0];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn call_custom_gate_on_register() {
    let source = "
  OPENQASM 2.0;
  gate h q {
    U(pi/2, 0, pi) q;
  }
  qreg q[2];
  h q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
        ]),
    )
}

#[test]
fn call_custom_gate_inside_custom_gate() {
    let source = "
  OPENQASM 2.0;
  gate u2(phi, lambda) q {
    U(pi/2, phi, lambda) q;
  }
  gate h q {
    u2(0, pi) q;
  }
  qreg q[2];
  h q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
        ]),
    )
}

#[test]
fn test_one_register_bell_circuit() {
    let source = "
  OPENQASM 2.0;
  qreg q[2];
  U (pi/2, 0, pi) q[0];
  CX q[0], q[1];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn test_two_registers_bell_circuit() {
    let source = "
  OPENQASM 2.0;
  qreg q[1];
  qreg r[1];
  U (pi/2, 0, pi) q[0];
  CX q[0], r[0];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn test_no_indices_bell_circuit() {
    let source = "
  OPENQASM 2.0;
  qreg q[1];
  qreg r[1];
  U (pi/2, 0, pi) q;
  CX q, r;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn test_no_indices_superposition() {
    let source = "
  OPENQASM 2.0;
  qreg q[4];
  U (pi/2, 0, pi) q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![Complex::from(0.25); 16]),
    )
}

#[test]
fn test_quantum_experience_header_is_included() {
    let source = "
  OPENQASM 2.0;
  include \"qelib1.inc\";
  qreg q[4];
  h q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![Complex::from(0.25); 16]),
    )
}

#[test]
fn test_measurements() {
    let subtests = vec![
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     measure q -> c;
     ",
            0b00_u64,
        ),
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     x q[0];
     measure q -> c;
     ",
            0b01_u64,
        ),
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     x q[1];
     measure q -> c;
     ",
            0b10_u64,
        ),
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     x q;
     measure q -> c;
     ",
            0b11_u64,
        ),
    ];
    for (index, (source, expected_result)) in subtests.iter().enumerate() {
        let result = &qasmsim::run(source, None).unwrap();
        println!("Using source sample #{}", index);
        assert_eq!(*result.memory().get("c").unwrap(), *expected_result);
    }
}

#[test]
fn test_all_classical_memory_is_displayed() {
    let source = "
  OPENQASM 2.0;
  include \"qelib1.inc\";
  qreg q[2];
  creg c[2];
  creg d[2];
  creg e[2];
  x q;
  measure q -> c;
  ";
    let result = &qasmsim::run(source, None).unwrap();
    assert_eq!(result.memory().len(), 3);
    assert_eq!(*result.memory().get("c").unwrap(), 0b11);
    assert_eq!(*result.memory().get("d").unwrap(), 0b0);
    assert_eq!(*result.memory().get("e").unwrap(), 0b0);
}

#[test]
fn test_conditional() {
    let source = "
  OPENQASM 2.0;
  include \"qelib1.inc\";
  qreg q[2];
  creg c[2];
  creg d[2];
  x q[1];
  measure q[1] -> c[1];
  if (c==2) x q;
  measure q -> d;
  ";
    let result = &qasmsim::run(source, None).unwrap();
    assert_eq!(result.memory().len(), 2);
    assert_eq!(*result.memory().get("c").unwrap(), 0b10);
    assert_eq!(*result.memory().get("d").unwrap(), 0b01);
}
