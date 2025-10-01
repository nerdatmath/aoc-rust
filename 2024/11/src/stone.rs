#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stone(pub u64);

impl Stone {
    pub fn blink(&self) -> Vec<Self> {
        match self.0 {
            0 => vec![Stone(1)],
            n @ (10..100) => vec![Stone(n / 10), Stone(n % 10)],
            n @ (1000..10000) => vec![Stone(n / 100), Stone(n % 100)],
            n @ (100000..1000000) => vec![Stone(n / 1000), Stone(n % 1000)],
            n @ (10000000..100000000) => vec![Stone(n / 10000), Stone(n % 10000)],
            n @ (1000000000..10000000000) => vec![Stone(n / 100000), Stone(n % 100000)],
            n @ (100000000000..1000000000000) => vec![Stone(n / 1000000), Stone(n % 1000000)],
            n @ (10000000000000..100000000000000) => vec![Stone(n / 10000000), Stone(n % 10000000)],
            n @ (1000000000000000..10000000000000000) => {
                vec![Stone(n / 100000000), Stone(n % 100000000)]
            }
            n @ (100000000000000000..1000000000000000000) => {
                vec![Stone(n / 1000000000), Stone(n % 1000000000)]
            }
            n @ (10000000000000000000..) => vec![Stone(n / 10000000000), Stone(n % 10000000000)],
            n => vec![Stone(n * 2024)],
        }
    }
}
