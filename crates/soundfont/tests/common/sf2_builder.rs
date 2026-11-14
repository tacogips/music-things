#![allow(dead_code)]

fn chunk(id: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(id);
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    if data.len() % 2 != 0 {
        out.push(0);
    }
    out
}
fn list(kind: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut data = kind.to_vec();
    for c in chunks {
        data.extend_from_slice(c);
    }
    chunk(b"LIST", &data)
}
fn name(out: &mut Vec<u8>, s: &str) {
    let mut n = [0; 20];
    n[..s.len()].copy_from_slice(s.as_bytes());
    out.extend_from_slice(&n);
}
fn u16le(out: &mut Vec<u8>, n: u16) {
    out.extend_from_slice(&n.to_le_bytes());
}
fn u32le(out: &mut Vec<u8>, n: u32) {
    out.extend_from_slice(&n.to_le_bytes());
}
fn gen(out: &mut Vec<u8>, op: u16, amount: i16) {
    u16le(out, op);
    u16le(out, amount as u16);
}
fn range(lo: u8, hi: u8) -> i16 {
    i16::from_le_bytes([lo, hi])
}
fn bag(out: &mut Vec<u8>, gen_index: u16) {
    u16le(out, gen_index);
    u16le(out, 0);
}
fn phdr(out: &mut Vec<u8>, label: &str, preset: u16, bank: u16, bag_index: u16) {
    name(out, label);
    u16le(out, preset);
    u16le(out, bank);
    u16le(out, bag_index);
    for _ in 0..3 {
        u32le(out, 0);
    }
}
fn inst(out: &mut Vec<u8>, label: &str, bag_index: u16) {
    name(out, label);
    u16le(out, bag_index);
}
fn shdr(out: &mut Vec<u8>, label: &str, start: u32, end: u32, root: u8) {
    name(out, label);
    for n in [start, end, start + 8, end - 8, 44100] {
        u32le(out, n);
    }
    out.extend_from_slice(&[root, 0]);
    u16le(out, 0);
    u16le(out, 1);
}

pub fn synthetic_sf2() -> Vec<u8> {
    let mut smpl = Vec::new();
    for i in 0..256 {
        let value = if i < 5 {
            [-32768, -1, 0, 1, 32767][i]
        } else {
            ((i as f64 * std::f64::consts::TAU / 256.0).sin() * 20000.0) as i16
        };
        smpl.extend_from_slice(&value.to_le_bytes());
    }
    for i in 0..128 {
        smpl.extend_from_slice(&((i * 400 - 25400) as i16).to_le_bytes());
    }
    let sm24: Vec<u8> = (0..384).map(|i| (i * 37 + 1) as u8).collect();
    let sdta = list(b"sdta", &[chunk(b"smpl", &smpl), chunk(b"sm24", &sm24)]);
    let mut ph = Vec::new();
    phdr(&mut ph, "SynthSine", 0, 0, 0);
    phdr(&mut ph, "SynthSaw", 1, 0, 1);
    phdr(&mut ph, "Drums", 0, 128, 2);
    phdr(&mut ph, "EOP", 0, 0, 3);
    let mut pb = Vec::new();
    for n in 0..=3 {
        bag(&mut pb, n);
    }
    let mut pg = Vec::new();
    for n in 0..3 {
        gen(&mut pg, 41, n);
    }
    gen(&mut pg, 0, 0);
    let mut ins = Vec::new();
    inst(&mut ins, "Sine", 0);
    inst(&mut ins, "Saw", 3);
    inst(&mut ins, "Drums", 4);
    inst(&mut ins, "EOI", 6);
    let mut ib = Vec::new();
    let mut ig = Vec::new();
    let zones: &[&[(u16, i16)]] = &[
        &[
            (17, -100),
            (33, -6000),
            (34, -4000),
            (35, -3000),
            (36, -2500),
            (37, 300),
            (38, -2000),
            (39, 50),
            (40, 30),
            (7, 200),
            (11, -150),
            (5, 80),
            (10, 120),
            (13, 40),
            (21, -3000),
            (22, 100),
            (6, 90),
            (23, -3500),
            (24, 200),
            (8, 9000),
            (9, 120),
            (26, -3000),
        ],
        &[
            (43, range(0, 63)),
            (44, range(10, 110)),
            (51, 1),
            (52, -7),
            (54, 1),
            (53, 0),
        ],
        &[(43, range(64, 127)), (54, 3), (53, 0)],
        &[(48, 120), (54, 0), (53, 1)],
        &[(43, range(35, 45)), (57, 1), (54, 0), (53, 1)],
        &[(43, range(46, 55)), (57, 1), (54, 0), (53, 1)],
    ];
    for zone in zones {
        bag(&mut ib, (ig.len() / 4) as u16);
        for &(op, amount) in *zone {
            gen(&mut ig, op, amount);
        }
    }
    bag(&mut ib, (ig.len() / 4) as u16);
    gen(&mut ig, 0, 0);
    let mut sh = Vec::new();
    shdr(&mut sh, "Sine", 0, 256, 69);
    shdr(&mut sh, "Saw", 256, 384, 60);
    name(&mut sh, "EOS");
    for _ in 0..5 {
        u32le(&mut sh, 384);
    }
    sh.extend_from_slice(&[0, 0]);
    u16le(&mut sh, 0);
    u16le(&mut sh, 1);
    let pdta = list(
        b"pdta",
        &[
            chunk(b"phdr", &ph),
            chunk(b"pbag", &pb),
            chunk(b"pmod", &[0; 10]),
            chunk(b"pgen", &pg),
            chunk(b"inst", &ins),
            chunk(b"ibag", &ib),
            chunk(b"imod", &[0; 10]),
            chunk(b"igen", &ig),
            chunk(b"shdr", &sh),
        ],
    );
    let mut data = b"sfbk".to_vec();
    data.extend_from_slice(&sdta);
    data.extend_from_slice(&pdta);
    chunk(b"RIFF", &data)
}
pub fn broken_sf2_truncated() -> Vec<u8> {
    let mut bytes = synthetic_sf2();
    bytes.truncate(bytes.len() - 150);
    bytes
}
