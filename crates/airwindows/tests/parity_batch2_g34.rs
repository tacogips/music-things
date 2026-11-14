mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_POINT: ParityRecord = ParityRecord {
    name: "Point",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn point() {
    let mut reset =
        music_things_airwindows::create_with_seed("Point", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..3 {
        reset.set_param(i, (i + 1) as f32 / 5.0);
    }
    reset.reset();
    for i in 0..3 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 5.0);
    }
    let text = include_str!("fixtures/batch2/g34/Point.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/Point.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Point", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "Point max-abs {maximum}");
    check_plugin(&RECORD_POINT, text, bytes);
}

const RECORD_POCKEY: ParityRecord = ParityRecord {
    name: "Pockey",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pockey() {
    let mut reset =
        music_things_airwindows::create_with_seed("Pockey", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..3 {
        reset.set_param(i, (i + 1) as f32 / 5.0);
    }
    reset.reset();
    for i in 0..3 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 5.0);
    }
    let text = include_str!("fixtures/batch2/g34/Pockey.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/Pockey.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Pockey", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "Pockey max-abs {maximum}");
    check_plugin(&RECORD_POCKEY, text, bytes);
}

const RECORD_POCKEY2: ParityRecord = ParityRecord {
    name: "Pockey2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pockey2() {
    let mut reset =
        music_things_airwindows::create_with_seed("Pockey2", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..3 {
        reset.set_param(i, (i + 1) as f32 / 5.0);
    }
    reset.reset();
    for i in 0..3 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 5.0);
    }
    let text = include_str!("fixtures/batch2/g34/Pockey2.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/Pockey2.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Pockey2", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "Pockey2 max-abs {maximum}");
    check_plugin(&RECORD_POCKEY2, text, bytes);
}

const RECORD_PODCAST: ParityRecord = ParityRecord {
    name: "Podcast",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn podcast() {
    let mut reset =
        music_things_airwindows::create_with_seed("Podcast", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..2 {
        reset.set_param(i, (i + 1) as f32 / 4.0);
    }
    reset.reset();
    for i in 0..2 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 4.0);
    }
    let text = include_str!("fixtures/batch2/g34/Podcast.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/Podcast.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Podcast", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "Podcast max-abs {maximum}");
    check_plugin(&RECORD_PODCAST, text, bytes);
}

const RECORD_PODCASTDELUXE: ParityRecord = ParityRecord {
    name: "PodcastDeluxe",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn podcastdeluxe() {
    let mut reset =
        music_things_airwindows::create_with_seed("PodcastDeluxe", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..1 {
        reset.set_param(i, (i + 1) as f32 / 3.0);
    }
    reset.reset();
    for i in 0..1 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 3.0);
    }
    let text = include_str!("fixtures/batch2/g34/PodcastDeluxe.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/PodcastDeluxe.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("PodcastDeluxe", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "PodcastDeluxe max-abs {maximum}");
    check_plugin(&RECORD_PODCASTDELUXE, text, bytes);
}

const RECORD_POP2: ParityRecord = ParityRecord {
    name: "Pop2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pop2() {
    let mut reset =
        music_things_airwindows::create_with_seed("Pop2", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..5 {
        reset.set_param(i, (i + 1) as f32 / 7.0);
    }
    reset.reset();
    for i in 0..5 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 7.0);
    }
    let text = include_str!("fixtures/batch2/g34/Pop2.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/Pop2.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Pop2", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "Pop2 max-abs {maximum}");
    check_plugin(&RECORD_POP2, text, bytes);
}

const RECORD_POP: ParityRecord = ParityRecord {
    name: "Pop",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pop() {
    let mut reset = music_things_airwindows::create_with_seed("Pop", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..3 {
        reset.set_param(i, (i + 1) as f32 / 5.0);
    }
    reset.reset();
    for i in 0..3 {
        assert_eq!(reset.get_param(i), (i + 1) as f32 / 5.0);
    }
    let text = include_str!("fixtures/batch2/g34/Pop.txt");
    let bytes = include_bytes!("fixtures/batch2/g34/Pop.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Pop", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15].into_iter().enumerate() {
                effect.set_param(i, value);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert!(maximum <= 1.0e-6, "Pop max-abs {maximum}");
    check_plugin(&RECORD_POP, text, bytes);
}
