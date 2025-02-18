use chacha20::ChaCha20;
use cipher::{KeyIvInit, StreamCipher};
use rand::RngCore;
use std::sync::{Arc, Mutex};

pub struct Secgen256 {
    key: [u8; 32],
    nonce: [u8; 12],
    buffer: Vec<u8>,
    prev_key: [u8; 32],
}

impl Secgen256 {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut key);
        rand::thread_rng().fill_bytes(&mut nonce);

        Secgen256 {
            key,
            nonce,
            buffer: vec![],
            prev_key: [0u8; 32],
        }
    }

    fn genbytes(&mut self) -> Vec<u8> {
        let mut cipher = ChaCha20::new(&self.key.into(), &self.nonce.into());
        let mut plaintext = [0u8; 64];
        cipher.apply_keystream(&mut plaintext);

        self.nonceinc();

        let mut output = plaintext.to_vec();
        if !self.buffer.is_empty() {
            let hidden_data = xorbytes(&self.key, &self.buffer[..64]);
            output.extend_from_slice(&hidden_data);
        }

        self.buffer = plaintext.to_vec();
        output
    }

    fn nonceinc(&mut self) {
        let mut value = u32::from_le_bytes(self.nonce[8..12].try_into().unwrap());
        value += 1;
        self.nonce[8..12].copy_from_slice(&value.to_le_bytes());
    }

    fn mutator(&mut self) {
        let mut new_key = [0u8; 32];
        for i in 0..32 {
            new_key[i] = self.key[i] ^ self.prev_key[i];
        }
        self.prev_key = self.key;
        self.key = new_key;
    }

    pub fn genstream(&mut self, size: usize) -> Vec<u8> {
        let mut result = vec![];
        for _ in 0..size / 64 {
            result.extend_from_slice(&self.genbytes()[..64]);
        }
        result.truncate(size);
        result
    }
}

fn xorbytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>()
}

fn main() {
    let mut rng = Secgen256::new();

    let random_data = rng.genstream(1024);
    println!("new {} bytes of stuff generated", random_data.len());
}