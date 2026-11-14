use std::collections::HashMap;
pub struct FixtureParam {
    pub name: String,
    pub default_bits: u32,
    pub label_r0: String,
    pub display_r0: String,
    pub label_r1: String,
    pub display_r1: String,
}
pub struct Fixture {
    pub name: String,
    pub upstream: String,
    pub description: String,
    pub stimulus_fnv_44100: u64,
    pub stimulus_fnv_96000: u64,
    pub params: Vec<FixtureParam>,
    pub renders: [Vec<f32>; 3],
}
impl Fixture {
    pub fn parse(meta: &str, data: &[u8]) -> Self {
        let values: HashMap<&str, &str> = meta
            .lines()
            .map(|line| line.split_once('=').expect("fixture key=value"))
            .collect();
        let get = |k: &str| {
            *values
                .get(k)
                .unwrap_or_else(|| panic!("missing fixture key {k}"))
        };
        assert_eq!(get("format"), "aw-fixture-v1");
        let count: usize = get("params").parse().unwrap();
        let params = (0..count)
            .map(|i| {
                let val = |key: &str| get(&format!("param.{i}.{key}")).to_owned();
                FixtureParam {
                    name: val("name"),
                    default_bits: u32::from_str_radix(&val("default"), 16).unwrap(),
                    label_r0: val("label_r0"),
                    display_r0: val("display_r0"),
                    label_r1: val("label_r1"),
                    display_r1: val("display_r1"),
                }
            })
            .collect();
        assert_eq!(data.len(), 12096);
        let samples: Vec<f32> = data
            .chunks_exact(4)
            .map(|x| f32::from_le_bytes(x.try_into().unwrap()))
            .collect();
        let renders = std::array::from_fn(|i| samples[i * 1008..(i + 1) * 1008].to_vec());
        let mut description = String::new();
        let mut chars = get("description").chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => description.push('\n'),
                    Some('\\') => description.push('\\'),
                    other => panic!("bad description escape {other:?}"),
                }
            } else {
                description.push(c);
            }
        }
        Self {
            name: get("name").into(),
            upstream: get("upstream").into(),
            description,
            stimulus_fnv_44100: u64::from_str_radix(get("stimulus_fnv_44100"), 16).unwrap(),
            stimulus_fnv_96000: u64::from_str_radix(get("stimulus_fnv_96000"), 16).unwrap(),
            params,
            renders,
        }
    }
}
