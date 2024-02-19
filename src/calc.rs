use rand::{distributions::Uniform, Rng};

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
            f: (0..max_height.pow(max_width as u32))
                .map(|idx| (idx == 0).then_some(true))
                .collect(),
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
                    rng.sample(&range)
                }
            })
            .collect()
    }
}
