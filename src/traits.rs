use core::hash::Hasher;
use functions::{mix_with_p0, read64, wyhash_core, wyhash_finish, wyrng};
use rand_core::{impls, Error, RngCore, SeedableRng};

/// WyHash hasher
#[derive(Default)]
pub struct WyHash {
    h: u64,
    size: u64,
}

impl WyHash {
    /// Create hasher with a seed
    pub fn with_seed(seed: u64) -> Self {
        WyHash { h: seed, size: 0 }
    }
}

impl Hasher for WyHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            self.h = mix_with_p0(self.h);
        } else {
            for bytes in bytes.chunks(u64::max_value() as usize) {
                self.h = wyhash_core(bytes, self.h);
                self.size += bytes.len() as u64
            }
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        wyhash_finish(self.size, self.h)
    }
}

/// Random number generator seed type
#[derive(Default)]
pub struct WyRngSeed(pub [u8; 8]);

impl AsMut<[u8]> for WyRngSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// WyRng random number generator
#[derive(Default)]
pub struct WyRng(u64);

impl RngCore for WyRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = wyrng(self.0);
        self.0
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for WyRng {
    type Seed = WyRngSeed;

    fn from_seed(seed: Self::Seed) -> Self {
        WyRng(read64(&seed.0))
    }

    fn seed_from_u64(state: u64) -> Self {
        WyRng(state)
    }
}
