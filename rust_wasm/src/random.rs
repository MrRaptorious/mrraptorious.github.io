use std::mem::size_of;

use web_sys::Crypto;

const RNG_BUFFER_SIZE: usize = 128;

pub struct Rng {
    crypto: Crypto,
    rng_buffer: [u8; RNG_BUFFER_SIZE],
    current: usize,
}

pub fn create_rng() -> Rng {
    let window = web_sys::window().expect("no global `window` exists");
    let crypto_mod = window.crypto().expect("should have crypto");
    let rand_array = [0u8; 128];

    let mut rng = Rng {
        crypto: crypto_mod,
        rng_buffer: rand_array,
        current: 0,
    };

    // first init
    rng.refill_buffer();

    rng
}

impl Rng {
    fn refill_buffer(&mut self) {
        self.crypto
            .get_random_values_with_u8_array(&mut self.rng_buffer)
            .unwrap();
        self.current = 0;
    }

    pub fn get_bytes<const SIZE: usize>(&mut self) -> [u8; SIZE] {
        assert!(SIZE < RNG_BUFFER_SIZE);

        if (self.current + SIZE) > RNG_BUFFER_SIZE {
            self.refill_buffer();
        }

        let mut result = [0u8; SIZE];
        result.copy_from_slice(&self.rng_buffer[self.current..self.current + SIZE]);

        self.current += SIZE;

        result
    }

    pub fn get_i32(&mut self) -> i32 {
        i32::from_le_bytes(self.get_bytes::<{ size_of::<i32>() }>())
    }

    pub fn get_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.get_bytes::<{ size_of::<u32>() }>())
    }

    /// returns an float between 0 and 1
    pub fn get(&mut self) -> f32 {
        self.get_u32() as f32 / (u32::MAX as f32 + 1.0)
    }

    pub fn get_bool(&mut self) -> bool {
        u8::from_le_bytes(self.get_bytes::<{ size_of::<u8>() }>()) & 0x1 == 1
    }

    pub fn get_i32_between(&mut self, start: usize, end: usize) -> i32 {
        assert!(start <= end);
        let range_size = (end - start) as f32 + 1.0; // Wichtig: +1.0
        let random_scaled = self.get() * range_size;

        ((start as f32) + random_scaled).floor() as i32
    }

    pub fn get_between(&mut self, start: f32, end: f32) -> f32 {
        assert!(start <= end);
        let range_size = end - start;
        start + self.get() * range_size
    }
}
