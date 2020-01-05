pub mod ast;

#[cfg(test)]
mod tests {
  use grammar::ast::*;
  use open_qasm2;

  #[test]
  fn test_parse_open_qasm() {
    let source = "
  OPENQASM 2.0;
  qreg q[2];
  creg c[2];
  ";
    let parser = open_qasm2::OpenQasmProgramParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Box::new(OpenQasmProgram{
      version: "2.0".to_string(),
      program: vec![
        Statement::QRegDecl("q".to_string(), 2),
        Statement::CRegDecl("c".to_string(), 2)
      ]
    }));
  }

  #[test]
  fn test_parse_id_gate_macro() {
    let source = "
  gate id q {}
  ";
    let parser = open_qasm2::StatementParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Statement::GateDecl(
      "id".to_string(), vec![], vec!["q".to_string()], vec![]
    ));
  }

  #[test]
  fn test_parse_id_gate_macro_with_parenthesis() {
    let source = "
  gate id () q {}
  ";
    let parser = open_qasm2::StatementParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Statement::GateDecl(
      "id".to_string(), vec![], vec!["q".to_string()], vec![]
    ));
  }

  #[test]
  fn test_parse_cx_gate_macro() {
    let source = "
  gate cx c, t {
    CX c, t;
  }
  ";
    let parser = open_qasm2::StatementParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Statement::GateDecl(
      "cx".to_string(), vec![], vec!["c".to_string(), "t".to_string()], vec![
        GateOperation::Unitary(UnitaryOperation::CX(
          Argument::Id("c".to_string()),
          Argument::Id("t".to_string())
        ))
      ]
    ));
  }

  #[test]
  fn test_parse_u_gate_macro() {
    let source = "
  gate u (theta, phi, lambda) q {
    U (theta, phi, lambda) q;
  }
  ";
    let parser = open_qasm2::StatementParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Statement::GateDecl(
      "u".to_string(),
      vec!["theta".to_string(), "phi".to_string(), "lambda".to_string()],
      vec!["q".to_string()],
      vec![
        GateOperation::Unitary(UnitaryOperation::U(
          Expression::Id("theta".to_string()),
          Expression::Id("phi".to_string()),
          Expression::Id("lambda".to_string()),
          Argument::Id("q".to_string())
        ))
      ]
    ));
  }

  #[test]
  fn test_parse_gate_macro_with_gate_expansion() {
    let source = "
  gate rz (phi) a {
    u1 (phi) a;
  }
  ";
    let parser = open_qasm2::StatementParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Statement::GateDecl(
      "rz".to_string(),
      vec!["phi".to_string()],
      vec!["a".to_string()],
      vec![
        GateOperation::Unitary(UnitaryOperation::GateExpansion(
          "u1".to_string(),
          vec![Expression::Id("phi".to_string())],
          vec![Argument::Id("a".to_string())]
        ))
      ]
    ));
  }

  #[test]
  fn test_parse_expressions_in_arguments() {
    let source = "
    U(pi/2, 0, pi) q;
    ";
    let parser = open_qasm2::StatementParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Statement::QuantumOperation(
      QuantumOperation::Unitary(
        UnitaryOperation::U(
          Expression::Op(
            Opcode::Div,
            Box::new(Expression::Pi),
            Box::new(Expression::Real(2.0))
          ),
          Expression::Real(0.0),
          Expression::Pi,
          Argument::Id(String::from("q"))
        )
      )
    ));
  }

  #[test]
  fn test_operator_precedence() {
    let source = "
    -pi + (1 - 2) * 3 / 4
    ";
    let parser = open_qasm2::ExprParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, Expression::Op(
      Opcode::Add,
      Box::new(Expression::Minus(Box::new(Expression::Pi))),
      Box::new(Expression::Op(
        Opcode::Div,
        Box::new(Expression::Op(
          Opcode::Mul,
          Box::new(Expression::Op(
            Opcode::Sub,
            Box::new(Expression::Real(1.0)),
            Box::new(Expression::Real(2.0))
          )),
          Box::new(Expression::Real(3.0))
        )),
        Box::new(Expression::Real(4.0))
      ))
    ));
  }

  #[test]
  fn test_parse_program_without_version_string() {
    let source = "
  qreg q[1];
  creg c[1];
  h q;
  ";
    let parser = open_qasm2::ProgramParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, vec![
      Statement::QRegDecl("q".to_string(), 1),
      Statement::CRegDecl("c".to_string(), 1),
      Statement::QuantumOperation(
        QuantumOperation::Unitary(
          UnitaryOperation::GateExpansion(
            "h".to_string(), vec![], vec![Argument::Id("q".to_string())])
        )
      )
    ]);
  }

  #[test]
  fn test_program_with_measure_and_reset() {
    let source = "
  qreg q[1];
  creg c[1];
  h q;
  measure q -> c;
  reset q;
  ";
    let parser = open_qasm2::ProgramParser::new();
    let tree = parser.parse(source).unwrap();
    assert_eq!(tree, vec![
      Statement::QRegDecl("q".to_string(), 1),
      Statement::CRegDecl("c".to_string(), 1),
      Statement::QuantumOperation(
        QuantumOperation::Unitary(
          UnitaryOperation::GateExpansion(
            "h".to_string(), vec![], vec![Argument::Id("q".to_string())])
        )
      ),
      Statement::QuantumOperation(
        QuantumOperation::Measure(
          Argument::Id("q".to_string()),
          Argument::Id("c".to_string())
        )
      ),
      Statement::QuantumOperation(
        QuantumOperation::Reset(Argument::Id("q".to_string()))
      )
    ]);
  }
}