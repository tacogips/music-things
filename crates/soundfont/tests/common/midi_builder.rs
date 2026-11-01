#![allow(dead_code)]

fn vlq(mut value: u32, out: &mut Vec<u8>) {
    let mut bytes = [0_u8; 4];
    let mut index = 3;
    bytes[index] = (value & 0x7f) as u8;
    while value > 0x7f {
        value >>= 7;
        index -= 1;
        bytes[index] = ((value & 0x7f) as u8) | 0x80;
    }
    out.extend_from_slice(&bytes[index..]);
}

fn event(track: &mut Vec<u8>, delta: u32, bytes: &[u8]) {
    vlq(delta, track);
    track.extend_from_slice(bytes);
}

fn file(format: u16, tracks: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = b"MThd\0\0\0\x06".to_vec();
    bytes.extend_from_slice(&format.to_be_bytes());
    bytes.extend_from_slice(&(tracks.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&480_u16.to_be_bytes());
    for track in tracks {
        bytes.extend_from_slice(b"MTrk");
        bytes.extend_from_slice(&(track.len() as u32).to_be_bytes());
        bytes.extend_from_slice(track);
    }
    bytes
}

pub fn scenario_midi() -> Vec<u8> {
    let mut tempo = Vec::new();
    event(&mut tempo, 0, &[0xff, 0x51, 3, 0x07, 0xa1, 0x20]);
    event(&mut tempo, 1920, &[0xff, 0x51, 3, 0x06, 0x1a, 0x80]);
    event(&mut tempo, 0, &[0xff, 0x2f, 0]);

    let mut music = Vec::new();
    event(&mut music, 0, &[0xb0, 0, 2]);
    event(&mut music, 0, &[0xc0, 10]);
    event(&mut music, 0, &[0xb0, 7, 100]);
    event(&mut music, 0, &[10, 64]); // running controller status
    event(&mut music, 0, &[0x90, 60, 100]);
    event(&mut music, 120, &[64, 90]); // running note status
    event(&mut music, 0, &[0x91, 67, 80]);
    event(&mut music, 120, &[0x99, 36, 110]);
    event(&mut music, 0, &[0xa0, 60, 50]);
    event(&mut music, 0, &[0xd0, 40]);
    event(&mut music, 0, &[0xb0, 64, 127]);
    event(&mut music, 0, &[0xe0, 127, 127]);
    event(&mut music, 240, &[0x90, 60, 0]);
    event(&mut music, 0, &[0x81, 67, 32]);
    event(&mut music, 0, &[0x89, 36, 0]);
    event(&mut music, 0, &[0xe0, 0, 64]);
    event(&mut music, 480, &[0x80, 64, 64]);
    event(&mut music, 0, &[0xb0, 64, 0]);
    event(&mut music, 0, &[0xe0, 0, 0]);
    event(&mut music, 960, &[0xff, 0x2f, 0]);
    file(1, &[tempo, music])
}

pub fn format0_midi() -> Vec<u8> {
    let mut track = Vec::new();
    event(&mut track, 0, &[0xff, 0x51, 3, 0x07, 0xa1, 0x20]);
    event(&mut track, 0, &[0xc0, 5]);
    event(&mut track, 0, &[0x90, 48, 100]);
    event(&mut track, 480, &[0xff, 0x51, 3, 0x06, 0x1a, 0x80]);
    event(&mut track, 480, &[0x80, 48, 0]);
    event(&mut track, 0, &[0xff, 0x2f, 0]);
    file(0, &[track])
}

pub fn truncated_midi() -> Vec<u8> {
    let mut bytes = scenario_midi();
    let first_track_length = u32::from_be_bytes(bytes[18..22].try_into().unwrap()) as usize;
    bytes.truncate(22 + first_track_length / 2);
    bytes
}

pub fn truncated_late_midi() -> Vec<u8> {
    let mut bytes = scenario_midi();
    bytes.truncate(bytes.len() - 8);
    bytes
}

pub fn bad_meta_midi() -> Vec<u8> {
    let mut tempo = Vec::new();
    event(&mut tempo, 0, &[0xff, 0x51, 3, 0x07, 0xa1, 0x20]);
    event(&mut tempo, 120, &[0xff, 0x51, 2, 0x06, 0x1a]);

    let mut music = Vec::new();
    event(&mut music, 0, &[0x90, 60, 100]);
    event(&mut music, 240, &[0x80, 60, 0]);
    event(&mut music, 0, &[0x90, 64, 90]);
    event(&mut music, 240, &[0x80, 64, 0]);
    event(&mut music, 0, &[0xff, 0x2f, 0]);
    file(1, &[tempo, music])
}
