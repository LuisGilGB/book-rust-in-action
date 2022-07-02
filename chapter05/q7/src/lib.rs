#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
  fn from(n: f64) -> Self {
    if n >= 1.0 {
      Q7(127)
    } else if n <= -1.0 {
      Q7(-128)
    } else {
      Q7((n * 128.0) as i8)
    }
  }
}

impl From<Q7> for f64 {
  fn from(n: Q7) -> f64 {
    (n.0 as f64) * 2_f64.powf(-7.0)
  }
}

impl From<f32> for Q7 {
  fn from(n: f32) -> Self {
    Q7::from(n as f64)
  }
}

impl From<Q7> for f32 {
  fn from(n: Q7) -> f32 {
    f64::from(n) as f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn out_of_bounds() {
    assert_eq!(Q7::from(10.), Q7::from(1.));
    assert_eq!(Q7::from(-10.), Q7::from(-1.));
  }

  #[test]
  fn f32_to_q7() {
    let f1: f32 = 0.7;
    let q1 = Q7::from(f1);

    let f2: f32 = -0.4;
    let q2 = Q7::from(f2);

    let f3: f32 = 123.0;
    let q3 = Q7::from(f3);

    assert_eq!(q1, Q7(89));
    assert_eq!(q2, Q7(-51));
    assert_eq!(q3, Q7(127));
  }

  #[test]
  fn q7_to_f32() {
    let q1 = Q7::from(0.7);
    let f1 = f32::from(q1);
    assert!((0.7 - f1).abs() < 1.0/128.0);
    assert_eq!(0.6953125, f1);

    let q2 = Q7::from(f1);
    let f2 = f32::from(q2);
    assert_eq!(f1, f2);

    let q3 = Q7::from(-0.4);
    let f3 = f32::from(q3);
    assert!((-0.4 - f3).abs() < 1.0/128.0);
  }
}
