//! v1.4 focused math additions.
//!
//! This module is intentionally additive. v1.3 source remains untouched while
//! the v1.4 engine shim dispatches only the operations introduced here.

use serde_json::{json, Value};

const MAX_DIM: usize = 64;
const PIVOT_EPS_FACTOR: f64 = 64.0;
const MAX_SIGFIGS: u32 = 15;

pub fn execute(op: &str, args: &[Value]) -> Option<Result<Value, &'static str>> {
    match op {
        "alg.linear_root" => Some(linear_root(args)),
        "linalg.solve" => Some(linear_solve(args)),
        "alg.proportion" => Some(proportion(args)),
        "num.round_sigfig" => Some(round_sigfig(args)),
        _ => None,
    }
}

fn need(args: &[Value], n: usize) -> Result<(), &'static str> {
    if args.len() == n { Ok(()) } else { Err("ARG") }
}

fn number(v: &Value) -> Result<f64, &'static str> {
    let x = v.as_f64().ok_or("TYPE")?;
    if x.is_finite() { Ok(x) } else { Err("NONFINITE") }
}

fn linear_root(args: &[Value]) -> Result<Value, &'static str> {
    need(args, 2)?;
    let a = number(&args[0])?;
    let b = number(&args[1])?;
    if a == 0.0 { return Err("DOMAIN"); }
    let x = -b / a;
    if x.is_finite() { Ok(json!(x)) } else { Err("NONFINITE") }
}

fn proportion(args: &[Value]) -> Result<Value, &'static str> {
    need(args, 3)?;
    let a = number(&args[0])?;
    let b = number(&args[1])?;
    let c = number(&args[2])?;
    if a == 0.0 { return Err("DOMAIN"); }
    let x = (b / a) * c;
    if x.is_finite() { Ok(json!(x)) } else { Err("NONFINITE") }
}

fn sigfig_count(v: &Value) -> Result<u32, &'static str> {
    let x = number(v)?;
    if x.fract() != 0.0 { return Err("TYPE"); }
    if !(1.0..=MAX_SIGFIGS as f64).contains(&x) { return Err("DOMAIN"); }
    Ok(x as u32)
}

fn round_sigfig(args: &[Value]) -> Result<Value, &'static str> {
    need(args, 2)?;
    let value = number(&args[0])?;
    let sigfigs = sigfig_count(&args[1])?;
    if value == 0.0 { return Ok(json!(0.0)); }

    // Rust's decimal scientific formatter gives a bounded, deterministic
    // decimal rounding step and remains well-behaved for very large, very
    // small, and subnormal finite f64 values. Parsing the rounded decimal back
    // to f64 keeps the public result numeric rather than textual.
    let precision = (sigfigs - 1) as usize;
    let rounded_text = format!("{:.*e}", precision, value);
    let rounded = rounded_text.parse::<f64>().map_err(|_| "NONFINITE")?;
    if rounded.is_finite() { Ok(json!(rounded)) } else { Err("NONFINITE") }
}

fn matrix(v: &Value) -> Result<Vec<Vec<f64>>, &'static str> {
    let rows = v.as_array().ok_or("TYPE")?;
    if rows.is_empty() || rows.len() > MAX_DIM { return Err("SHAPE"); }
    let n = rows.len();
    let mut out = Vec::with_capacity(n);
    for row in rows {
        let values = row.as_array().ok_or("TYPE")?;
        if values.len() != n { return Err("SHAPE"); }
        out.push(values.iter().map(number).collect::<Result<Vec<_>, _>>()?);
    }
    Ok(out)
}

fn vector(v: &Value, n: usize) -> Result<Vec<f64>, &'static str> {
    let values = v.as_array().ok_or("TYPE")?;
    if values.len() != n { return Err("SHAPE"); }
    values.iter().map(number).collect()
}

fn linear_solve(args: &[Value]) -> Result<Value, &'static str> {
    need(args, 2)?;
    let mut a = matrix(&args[0])?;
    let n = a.len();
    let mut b = vector(&args[1], n)?;

    let norm_inf = a
        .iter()
        .map(|row| row.iter().map(|x| x.abs()).sum::<f64>())
        .fold(0.0_f64, f64::max);
    let pivot_tol = f64::EPSILON * PIVOT_EPS_FACTOR * n as f64 * norm_inf.max(1.0);

    for k in 0..n {
        let mut pivot = k;
        let mut pivot_abs = a[k][k].abs();
        for (i, row) in a.iter().enumerate().skip(k + 1) {
            let candidate = row[k].abs();
            if candidate > pivot_abs {
                pivot = i;
                pivot_abs = candidate;
            }
        }
        if pivot_abs <= pivot_tol { return Err("DOMAIN"); }
        if pivot != k {
            a.swap(k, pivot);
            b.swap(k, pivot);
        }

        let pivot_value = a[k][k];
        for i in (k + 1)..n {
            let factor = a[i][k] / pivot_value;
            a[i][k] = 0.0;
            for j in (k + 1)..n {
                a[i][j] -= factor * a[k][j];
            }
            b[i] -= factor * b[k];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut rhs = b[i];
        for (j, xj) in x.iter().enumerate().skip(i + 1) {
            rhs -= a[i][j] * xj;
        }
        let pivot = a[i][i];
        if pivot.abs() <= pivot_tol { return Err("DOMAIN"); }
        x[i] = rhs / pivot;
        if !x[i].is_finite() { return Err("NONFINITE"); }
    }

    Ok(json!(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_root_solves_ax_plus_b() {
        assert_eq!(linear_root(&[json!(2.0), json!(-8.0)]).unwrap(), json!(4.0));
        assert_eq!(linear_root(&[json!(-3.0), json!(9.0)]).unwrap(), json!(3.0));
        assert_eq!(linear_root(&[json!(0.0), json!(1.0)]), Err("DOMAIN"));
    }

    #[test]
    fn proportion_solves_rule_of_three() {
        assert_eq!(proportion(&[json!(2.0), json!(5.0), json!(8.0)]).unwrap(), json!(20.0));
        assert_eq!(proportion(&[json!(4.0), json!(10.0), json!(3.0)]).unwrap(), json!(7.5));
        assert_eq!(proportion(&[json!(0.0), json!(5.0), json!(8.0)]), Err("DOMAIN"));
    }

    #[test]
    fn significant_figure_rounding_handles_scale_and_zero() {
        assert_eq!(round_sigfig(&[json!(1234.567), json!(3)]).unwrap(), json!(1230.0));
        assert_eq!(round_sigfig(&[json!(0.012345), json!(3)]).unwrap(), json!(0.0123));
        assert_eq!(round_sigfig(&[json!(-9876.5), json!(2)]).unwrap(), json!(-9900.0));
        assert_eq!(round_sigfig(&[json!(0.0), json!(4)]).unwrap(), json!(0.0));
        assert_eq!(round_sigfig(&[json!(12.3), json!(0)]), Err("DOMAIN"));
        assert_eq!(round_sigfig(&[json!(12.3), json!(16)]), Err("DOMAIN"));
        assert_eq!(round_sigfig(&[json!(12.3), json!(2.5)]), Err("TYPE"));
    }

    #[test]
    fn linear_solve_handles_two_by_two() {
        let out = linear_solve(&[
            json!([[2.0, 1.0], [1.0, -1.0]]),
            json!([5.0, 1.0]),
        ]).unwrap();
        let x = out.as_array().unwrap();
        assert!((x[0].as_f64().unwrap() - 2.0).abs() < 1e-12);
        assert!((x[1].as_f64().unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn linear_solve_pivots_and_rejects_singular_systems() {
        let pivoted = linear_solve(&[
            json!([[0.0, 1.0], [2.0, 3.0]]),
            json!([1.0, 5.0]),
        ]).unwrap();
        assert_eq!(pivoted, json!([1.0, 1.0]));

        assert_eq!(
            linear_solve(&[
                json!([[1.0, 2.0], [2.0, 4.0]]),
                json!([3.0, 6.0]),
            ]),
            Err("DOMAIN")
        );
    }
}
