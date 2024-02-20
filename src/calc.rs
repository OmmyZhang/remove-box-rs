use gloo_console::log;
use rand::{distributions::Uniform, Rng};

#[derive(Debug, PartialEq)]
pub enum PlayMode {
    Check,
    Operate,
    Random,
}

#[derive(Debug, PartialEq)]
pub struct Calc {
    pub max_width: usize,
    pub max_height: usize,
    f: Vec<Option<bool>>,
}

impl Calc {
    pub fn new(max_width: usize, max_height: usize) -> Self {
        Self {
            max_width,
            max_height,
            f: vec![None; max_height.pow(max_width as u32)],
        }
    }

    pub fn gen_map(&self, width: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(2, self.max_height);
        let blank = self.max_width - width;
        (0..self.max_width)
            .map(|idx| {
                if idx < blank / 2 || idx >= blank / 2 + width {
                    0
                } else {
                    rng.sample(range)
                }
            })
            .collect()
    }

    pub fn encode_map(&self, mp: &[usize]) -> usize {
        mp.iter()
            .cloned()
            .rev()
            .reduce(|acc, e| acc * self.max_height + e)
            .unwrap()
    }

    pub fn get_how_to_play(
        &mut self,
        status: usize,
        play_mode: PlayMode,
    ) -> Option<(usize, usize, usize, usize)> {
        for len in (1..=self.max_width).rev() {
            // mh^0 + mh^1 + ... + mh^(len-1)
            let diff = (self.max_height.pow(len as u32) - 1) / (self.max_height - 1);
            for start_i in (0..=(self.max_width - len)).rev() {
                let max_h = (start_i..(start_i + len))
                    .map(|i| status / self.max_height.pow(i as u32) % self.max_height)
                    .min()
                    .unwrap();
                // log!("max_h:", start_i, len, max_h);
                if max_h == 0 {
                    continue;
                }
                if play_mode == PlayMode::Random {
                    let mut rng = rand::thread_rng();
                    let h = rng.gen_range(1..=max_h);
                    let j = rng.gen_range(0..=(max_h - h));
                    return Some((start_i, start_i + len - 1, j, j + h - 1));
                }
                for h in (1..=max_h).rev() {
                    let new_status = status - diff * h * self.max_height.pow(start_i as u32);
                    // log!(start_i, len, h, new_status);
                    if !self.check_can_win(new_status) {
                        if play_mode == PlayMode::Check {
                            self.f[status] = Some(true);
                            return Some((0, 0, 0, 0));
                        } else {
                            let mut rng = rand::thread_rng();
                            let base_h = rng.gen_range(0..=(max_h - h));
                            return Some((start_i, start_i + len - 1, base_h, base_h + h - 1));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn check_can_win(&mut self, status: usize) -> bool {
        if let Some(r) = self.f[status] {
            return r;
        }
        // log!("!!status:", status);

        if self.get_how_to_play(status, PlayMode::Check).is_some() {
            self.f[status] = Some(true);
            true
        } else {
            self.f[status] = Some(false);
            false
        }
    }

    // return (i1, i2, j1, j2)
    pub fn play(&mut self, map: &[usize], difficulty: f64) -> (usize, usize, usize, usize) {
        let status = self.encode_map(map);

        if rand::random::<f64>() > difficulty || !self.check_can_win(status) {
            self.get_how_to_play(status, PlayMode::Random).unwrap()
        } else {
            self.get_how_to_play(status, PlayMode::Operate).unwrap()
        }
    }
}
