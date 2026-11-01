pub(crate) fn float2string(v: f32) -> String {
    let x = f64::from(v);
    if x.is_nan() {
        return "nan".into();
    }
    if x.abs() >= 1e8 {
        return "Huge!".into();
    }
    let a = x.abs();
    let ip = a.floor() as u64;
    let mut s = if x < 0.0 {
        format!("-{ip}")
    } else {
        ip.to_string()
    };
    let digits = ip.to_string().len();
    if digits < 8 {
        s.push('.');
        let mut f = a - a.floor();
        for _ in digits..8 {
            f *= 10.0;
            let d = f.floor();
            s.push(char::from(b'0' + d as u8));
            f -= d;
        }
    }
    s
}
pub(crate) fn db2string(v: f32) -> String {
    if v <= 0.0 {
        "-oo".into()
    } else {
        float2string((20.0 * f64::from(v).log10()) as f32)
    }
}
pub(crate) fn int2string(v: i32) -> String {
    if !(-9_999_999..100_000_000).contains(&v) {
        "Huge!".into()
    } else {
        v.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vectors() {
        for line in include_str!("../tests/fixtures/shim_format_vectors.txt").lines() {
            let mut parts = line.splitn(3, ' ');
            let kind = parts.next().unwrap();
            let value = parts.next().unwrap();
            let expected = parts.next().unwrap();
            let result = match kind {
                "f2s" => float2string(f32::from_bits(u32::from_str_radix(value, 16).unwrap())),
                "db" => db2string(f32::from_bits(u32::from_str_radix(value, 16).unwrap())),
                "i2s" => int2string(value.parse().unwrap()),
                _ => panic!("unknown format {kind}"),
            };
            assert_eq!(result, expected, "{line}");
        }
    }
}
