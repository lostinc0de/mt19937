macro_rules! mersenne_twister {
    ($Name: ident, $Type: ty,
     $W: tt, $N: tt, $M: tt,
     $R: tt, $A: tt,
     $U: tt, $D: tt,
     $S: tt, $B: tt,
     $T: tt, $C: tt,
     $L: tt, $F: tt) => {
        pub struct $Name {
            state: [$Type; $N],
            index: usize,
        }
        
        impl $Name {
            const MASK_LOW: $Type = ((1u64 << $R) - 1) as $Type;
            const MASK_HIGH: $Type = !Self::MASK_LOW as $Type;

            pub fn new(seed: $Type) -> Self {
                let mut ret = Self { 
                    state: [seed; $N],
                    index: $N,
                };
                ret.seed(seed);
                ret
            }
            pub fn seed(&mut self, seed: $Type) {
                self.index = $N;
                self.state[0] = seed;
                for i in 1..$N {
                    let prev = self.state[i - 1] as u128;
                    self.state[i] = ($F * (prev ^ (prev >> ($W - 2))) + (i as u128)) as $Type;
                }
            }
            // Helper function for the twist method
            fn compute_state(curr: $Type, next: $Type, next_m: $Type) -> $Type {
                let x = (curr & Self::MASK_HIGH) + (next & Self::MASK_LOW);
                let xa = if x % 2 != 0 {
                            (x >> 1) ^ $A
                         } else {
                             x >> 1
                         };
                let ret = next_m ^ xa;
                ret
            }
            fn twist(&mut self) {
                // Loops are unrolled to avoid modulo operation
                for i in 0..($N - $M) {
                    self.state[i] = Self::compute_state(self.state[i], self.state[i + 1],
                                                        self.state[i + $M]);
                }
                for i in ($N - $M)..($N - 1) {
                    self.state[i] = Self::compute_state(self.state[i], self.state[i + 1],
                                                        self.state[i + $M - $N]);
                }
                self.state[$N - 1] = Self::compute_state(self.state[$N - 1], self.state[0],
                                                         self.state[$M - 1]);
                self.index = 0;
            }
            pub fn get_number(&mut self) -> $Type {
                if self.index >= $N {
                    self.twist();
                }
                let mut y = self.state[self.index];
                self.index = self.index + 1;
                y = y ^ ((y >> $U) & $D);
                y = y ^ ((y << $S) & $B);
                y = y ^ ((y << $T) & $C);
                y = y ^ (y >> $L);
                y
            }
        }
    }
}

mersenne_twister!(MT19937, u32,
                  32, 624, 397, 31,
                  0x9908B0DF,
                  11, 0xFFFFFFFF,
                  7, 0x9D2C5680,
                  15, 0xEFC60000,
                  18, 1812433253);
mersenne_twister!(MT19937_64, u64,
                  64, 312, 156, 31,
                  0xB5026F5AA96619E9,
                  29, 0x5555555555555555,
                  17, 0x71D67FFFEDA60000,
                  37, 0xFFF7EEE000000000,
                  43, 6364136223846793005);
