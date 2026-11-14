use crate::dither::fpd_from_seed;

const SIZES: [usize; 26] = [
    15150, 14618, 14358, 13818, 13562, 13046, 11966, 11130, 10598, 9810, 9522, 8982, 8786, 8462,
    8310, 7982, 7322, 6818, 6506, 6002, 5838, 5502, 5010, 4850, 4296, 4180,
];
pub(super) const DELAYS: [usize; 26] = [
    7573, 7307, 7177, 6907, 6779, 6521, 5981, 5563, 5297, 4903, 4759, 4489, 4391, 4229, 4153, 3989,
    3659, 3407, 3251, 2999, 2917, 2749, 2503, 2423, 2146, 2088,
];

pub(super) struct Mv {
    pub(super) params: [f32; 5],
    pub(super) seed: u32,
    pub(super) left: [Box<[f64]>; 26],
    pub(super) right: [Box<[f64]>; 26],
    pub(super) avg_l: [f64; 26],
    pub(super) avg_r: [f64; 26],
    pub(super) positions: [usize; 26],
    pub(super) feedback_l: f64,
    pub(super) feedback_r: f64,
    pub(super) fpd_l: u32,
    pub(super) fpd_r: u32,
}
impl Mv {
    pub(super) fn new(seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 1.0, 1.0],
            seed,
            left: std::array::from_fn(|i| vec![0.0; SIZES[i]].into_boxed_slice()),
            right: std::array::from_fn(|i| vec![0.0; SIZES[i]].into_boxed_slice()),
            avg_l: [0.0; 26],
            avg_r: [0.0; 26],
            positions: [1; 26],
            feedback_l: 0.0,
            feedback_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
    pub(super) fn clear(&mut self) {
        for line in self.left.iter_mut().chain(self.right.iter_mut()) {
            line.fill(0.0);
        }
        self.avg_l.fill(0.0);
        self.avg_r.fill(0.0);
        self.positions.fill(1);
        self.feedback_l = 0.0;
        self.feedback_r = 0.0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
}
