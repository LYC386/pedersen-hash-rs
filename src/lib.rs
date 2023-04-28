use babyjubjub_rs::Point;
use bitvec::{macros::internal::funty::Fundamental, prelude::*};
use std::{collections::HashMap, hash::Hash};
const GENPOINT_PREFIX: &str = "PedersenGenerator";
const WINDOW_SIZE: u128 = 4;
const N_WINDOW_PER_SEGMENT: u128 = 50;

pub fn pedersen_hash<T: AsRef<[u8]>>(msg: T) {
    let bits_per_segment = WINDOW_SIZE * N_WINDOW_PER_SEGMENT;
    let mut bits: BitVec<u8, Lsb0> = BitVec::from_slice(msg.as_ref());
    let n_segments = ((bits.len().as_u128() - 1) / bits_per_segment) + 1;

    for s in 0..n_segments {
        continue;
    }
}

fn get_base_point(point_idx: usize, bases: Vec<Point>) -> Point {
    if point_idx < bases.len() {
        return bases[point_idx].clone();
    }
    let try_idx = 0u128;
    // const S = GENPOINT_PREFIX + "_" + padLeftZeros(pointIdx, 32) + "_" + padLeftZeros(tryIdx, 32);
    //     const h = createBlakeHash("blake256").update(S).digest();
    //     h[31] = h[31] & 0xBF;  // Set 255th bit to 0 (256th is the signal and 254th is the last possible bit to 1)
    //     p = babyJub.unpackPoint(h);
    //     tryIdx++;

    // let (game_code, len) = loop {
    //     match loby_matcher(&mut stream) {
    //         Ok(game_code) => break game_code,
    //         // Err(e) if e.kind() == ErrorKind::Other && e.to_string() == "wrong game code" => {
    //         // or just (for ease of use)
    //         Err(e) if e.to_string() == "wrong game code" => {
    //             stream.write("Wrong code\n".as_bytes())?;
    //         }
    //         Err(e) => return Err(e),
    //     };
    // };

    loop {
        let s = format!(
            "{}_{}_{}",
            GENPOINT_PREFIX,
            pad_left_zeros(&point_idx.to_string(), 32),
            pad_left_zeros(&try_idx.to_string(), 32)
        );
    }
}

fn pad_left_zeros(idx_str: &str, n: u64) -> String {
    let mut result = idx_str.to_string();
    while result.len() < n.as_usize() {
        result = "0".to_string() + &result;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_zeros() {
        let idx = "10654981";
        let res = pad_left_zeros(idx, 32);
        assert_eq!(&res, "00000000000000000000000010654981");
    }
}
