use complex::Complex;
use std::f64;

#[derive(Clone, Debug, PartialEq)]
pub struct StateVector {
  bases: Vec<Complex>
}

impl StateVector {

  pub fn new(bitsize: usize) -> Self {
    let mut bases = vec![Complex(0.0, 0.0); 2_usize.pow(bitsize as u32)];
    bases[0].0 = 1.0;
    StateVector { bases }
  }

  pub fn from_bases(bases: Vec<Complex>) -> Self {
    StateVector { bases }
  }

  pub fn len(&self) -> usize {
    self.bases.len()
  }

  pub fn bit_width(&self) -> usize {
    (self.len() as f64).log2() as usize
  }

  /// Apply a controlled not
  pub fn cnot(mut self, c: usize, t: usize) -> Self {
    let bit_width = self.bit_width();
    let exchangable_rows = find_exchangeable_rows(bit_width, c, t);
    for (index_a, index_b) in exchangable_rows {
      self.bases.swap(index_a, index_b);
    }
    self
  }

  /// Apply a 3 degree rotation to the target bit.
  pub fn u(mut self, theta: f64, phi: f64, lambda: f64, target: usize)
  -> Self {
    let bit_width = self.bit_width();
    let target_rows = find_target_rows(bit_width, target);
    let u_matrix = build_u(theta, phi, lambda);
    for (index_0, index_1) in target_rows {
      let selected = (self.bases[index_0], self.bases[index_1]);
      self.bases[index_0] = u_matrix.0 * selected.0 + u_matrix.1 * selected.1;
      self.bases[index_1] = u_matrix.2 * selected.0 + u_matrix.3 * selected.1;
    }
    self
  }
}


fn find_exchangeable_rows(bit_width: usize, c: usize, t: usize)
-> Vec<(usize, usize)>
{
  let max = 2_usize.pow(bit_width as u32 - 2);
  let mut out = Vec::with_capacity(max);
  for n in 0..max {
    let mut mask = 1;
    let mut histogram_index_10 = 0;
    let mut histogram_index_11 = 0;
    for i in 0..bit_width {
      if i == t {
        histogram_index_11 += 2_usize.pow(t as u32);
      }
      else if i == c {
        histogram_index_10 += 2_usize.pow(c as u32);
        histogram_index_11 += 2_usize.pow(c as u32);
      }
      else {
        let bit = ((n & mask) != 0) as usize;
        histogram_index_10 += bit * 2_usize.pow(i as u32);
        histogram_index_11 += bit * 2_usize.pow(i as u32);
        mask <<= 1;
      };
    }
    out.push((histogram_index_10, histogram_index_11))
  }
  out
}

fn find_target_rows(bit_width: usize, t: usize) -> Vec<(usize, usize)> {
  let max = 2_usize.pow(bit_width as u32 - 1);
  let mut out = Vec::with_capacity(max);
  for n in 0..max {
    let mut mask = 1;
    let mut histogram_index_0 = 0;
    let mut histogram_index_1 = 0;
    for i in 0..bit_width {
      if i == t {
        histogram_index_1 += 2_usize.pow(t as u32);
      }
      else {
        let bit = ((n & mask) != 0) as usize;
        histogram_index_0 += bit * 2_usize.pow(i as u32);
        histogram_index_1 += bit * 2_usize.pow(i as u32);
        mask <<= 1;
      };
    }
    out.push((histogram_index_0, histogram_index_1))
  }
  out
}

fn build_u(theta: f64, phi: f64, lambda: f64) -> (Complex, Complex, Complex, Complex) {
  (
    Complex((theta/2.0).cos(), 0.0),
    -e_power_to(lambda) * (theta/2.0).sin(),
    e_power_to(phi) * (theta/2.0).sin(),
    e_power_to(phi+lambda) * (theta/2.0).cos()
  )
}

fn e_power_to(x: f64) -> Complex {
  Complex(x.cos(), x.sin())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f64::consts::PI;

  #[test]
  fn test_cnot_c0t1() {
    let p = Default::default();
    let a = Complex(1.0, 0.0);
    let b = Complex(0.0, 1.0);
    let v = StateVector::from_bases(vec!(p, a, p, b))
      .cnot(0, 1);
    assert_eq!(v, StateVector::from_bases(vec!(p, b, p, a)));
  }

  #[test]
  fn test_cnot_c1t0_of_2_bits() {
    let p = Default::default();
    let a = Complex(1.0, 0.0);
    let b = Complex(0.0, 1.0);
    let v = StateVector::from_bases(vec!(p, p, a, b))
      .cnot(1, 0);
    assert_eq!(v, StateVector::from_bases(vec!(p, p, b, a)));
  }

  #[test]
  fn test_cnot_c2t0_of_3_bits() {
    let p = Default::default();
    let a = Complex(1.0, 0.0);
    let b = Complex(0.0, 1.0);
    let v = StateVector::from_bases(vec!(p, p, p, p, a, b, a, b))
      .cnot(2, 0);
    assert_eq!(v, StateVector::from_bases(vec!(p, p, p, p, b, a, b, a)));
  }

  #[test]
  fn test_cnot_c0t2_of_3_bits() {
    let p = Default::default();
    let a = Complex(1.0, 0.0);
    let b = Complex(0.0, 1.0);
    let v = StateVector::from_bases(vec!(p, a, p, a, p, b, p, b))
      .cnot(0, 2);
    assert_eq!(v, StateVector::from_bases(vec!(p, b, p, b, p, a, p, a)));
  }

  #[test]
  fn test_cnot_is_reversible() {
    let p = Default::default();
    let a = Complex(1.0, 0.0);
    let b = Complex(0.0, 1.0);
    let v = StateVector::from_bases(vec!(p, a, p, b))
      .cnot(0, 1)
      .cnot(0, 1);
    assert_eq!(v, StateVector::from_bases(vec!(p, a, p, b)));
  }

  #[test]
  fn test_e_power_to() {
    assert_eq!(e_power_to(0.0), Complex(1.0, 0.0));
    assert_eq!(e_power_to(PI/2.0), Complex(0.0, 1.0));
    assert_eq!(e_power_to(PI/4.0), Complex((PI/4.0).cos(), (PI/4.0).sin()));
  }

  #[test]
  fn test_euler_identity() {
    assert_eq!(e_power_to(PI) + 1, Default::default())
  }
}