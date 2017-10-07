pub struct Fib16 {
    seed: u16,
    lfsr: u16,
    bit: u16,
}

impl Default for Fib16 {
    fn default() -> Fib16 {
        Fib16 {
            seed: 0xACE1u16,
            lfsr: 0xACE1u16,
            bit: 0,
        }
    }
}

impl Fib16 {
    fn from_seed(seed: u16) -> Fib16 {
        Fib16 {
            seed: seed,
            lfsr: seed,
            bit: 0,
        }
    }
}

impl Iterator for Fib16 {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        self.bit = ((self.lfsr >> 0) ^ (self.lfsr >> 2) ^ (self.lfsr >> 3) ^
                        (self.lfsr >> 5)) & 1;
        self.lfsr = (self.lfsr >> 1) | (self.bit << 15);

        if self.lfsr != self.seed {
            Some(self.lfsr)
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let f = Fib16::default();
    for i in f {
        println!("{}", i);
    }
}
