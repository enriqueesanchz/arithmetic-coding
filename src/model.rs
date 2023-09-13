use fenwick::array::{update, prefix_sum};

pub enum EOF {
    /// Choose a symbol as the EOF
    Specify(u32),
    /// index 0
    Start,
    /// index `counts.len()` - 1
    End,
    /// adds an element to `counts` and sets EOF to `counts.len() - 1`
    EndAddOne,
}

pub struct Model {
    count: Vec<u32>,
    total: u64,
    fenwick_tree: Vec<u32>,
    eof: u32
}

impl Model {
    pub fn new(num_symbols: u32, eof: EOF) -> Self {

        let mut count = vec![1u32; num_symbols as usize];
        let eof: u32 = match eof {
            EOF::Specify(index) => {
                assert!(index < count.len() as u32);
                index
            }
            EOF::Start => 0,
            EOF::End => count.len() as u32 - 1,
            EOF::EndAddOne => {
                count.push(1);
                count.len() as u32 - 1
            }
        };
        
        let total = count.len() as u64;
        let mut fenwick_tree = vec![0u32; count.len()];
        
        for (i, count) in count.iter().enumerate() {
            update(&mut fenwick_tree, i, *count);
        }

        Self {
            count,
            total,
            fenwick_tree,
            eof
        }
    }

    pub fn update_symbol(&mut self, symbol: u32) {
        self.total += 1;
        self.count[symbol as usize] += 1;
        update(&mut self.fenwick_tree, symbol as usize, 1);
    }

    pub fn high(&self, symbol: u32) -> f64 {
        let high = prefix_sum(&self.fenwick_tree, symbol as usize);
        high as f64 / self.total as f64
    }

    pub fn low(&self, symbol: u32) -> f64 {
        let low = prefix_sum(&self.fenwick_tree, symbol as usize) - self.count[symbol as usize];
        low as f64 / self.total as f64
    }

    pub fn prob(&self, symbol: u32) -> (f64, f64) {
        let high = prefix_sum(&self.fenwick_tree, symbol as usize);
        let low = high - self.count[symbol as usize];

        (low as f64 / self.total as f64, high as f64 / self.total as f64)
    }

    pub fn eof(&self) -> u32 {
        self.eof
    }
}


#[cfg(test)]
mod tests {
    use super::{EOF, Model};

    #[test]
    fn constructor_eof_u32() {
        let model = Model::new(4, EOF::Specify(2));

        assert_eq!(2, model.eof());
        assert_eq!(model.prob(0), (0.0, 0.25));
        assert_eq!(model.prob(1), (0.25, 0.5));
        assert_eq!(model.prob(2), (0.5, 0.75));
        assert_eq!(model.prob(3), (0.75, 1.0));
    }

    #[test]
    fn constructor_eof_start() {
        let model = Model::new(4, EOF::Start);

        assert_eq!(0, model.eof());
        assert_eq!(model.prob(0), (0.0, 0.25));
        assert_eq!(model.prob(1), (0.25, 0.5));
        assert_eq!(model.prob(2), (0.5, 0.75));
        assert_eq!(model.prob(3), (0.75, 1.0));
    }

    #[test]
    fn constructor_eof_end() {
        let model = Model::new(4, EOF::End);

        assert_eq!(3, model.eof());
        assert_eq!(model.prob(0), (0.0, 0.25));
        assert_eq!(model.prob(1), (0.25, 0.5));
        assert_eq!(model.prob(2), (0.5, 0.75));
        assert_eq!(model.prob(3), (0.75, 1.0));
    }

    #[test]
    fn constructor_eof_endaddone() {
        let model = Model::new(3, EOF::EndAddOne);

        assert_eq!(3, model.eof());
        assert_eq!(model.prob(0), (0.0, 0.25));
        assert_eq!(model.prob(1), (0.25, 0.5));
        assert_eq!(model.prob(2), (0.5, 0.75));
        assert_eq!(model.prob(3), (0.75, 1.0));
    }

    #[test]
    fn probability_min() {
        let model = Model::new(2315, EOF::EndAddOne);
        assert_eq!(model.prob(0), (model.low(0), model.high(0)));
    }

    #[test]
    fn probability_high() {
        let count = 1_000;

        let model = Model::new(count + 1, EOF::EndAddOne);

        assert_eq!(
            model.prob(count),
            (model.low(count), model.high(count))
        );
    }

    #[test]
    fn update_symbols() {
        let mut model = Model::new(4, EOF::End);

        model.update_symbol(2);
        model.update_symbol(2);
        model.update_symbol(2);
        model.update_symbol(3);
        model.update_symbol(1);
        model.update_symbol(3);

        assert_eq!(model.prob(0), (0.0, 0.1));
        assert_eq!(model.prob(1), (0.1, 0.3));
        assert_eq!(model.prob(2), (0.3, 0.7));
        assert_eq!(model.prob(3), (0.7, 1.0));
    }
}
