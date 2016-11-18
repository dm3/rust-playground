#![feature(test)]
#![feature(rand)]

extern crate test;
extern crate rand;
extern crate playground;

use std::collections::HashMap;
use rand::{XorShiftRng, SeedableRng, Rand};
use playground::maps;
use self::test::Bencher;

#[bench]
fn u32_insert_bench(b: &mut Bencher) {
    let mut rand = XorShiftRng::from_seed([0, 1, 2, 3]);
    let sz = 500000;
    let mut m = maps::u32_map::ConstSizeVecMap::new(sz);
    b.iter(|| {
        let idx = usize::rand(&mut rand) & 0x6FFF;
        let x = m.get(idx);
        let y = maps::rand_value(&mut rand);
        m.insert(idx, maps::combine(x, y));
    });
}

fn bench_hash_map<F, T>(b: &mut Bencher, f: F)
    where F: Fn(usize) -> HashMap<usize, maps::Value, T>,
          T: std::hash::BuildHasher {
    let mut rand = XorShiftRng::from_seed([0, 1, 2, 3]);
    let sz = 500000;
    let mut m = f(sz);
    b.iter(|| {
        let idx = usize::rand(&mut rand) & 0x6FFF;
        let x = m.remove(&idx).unwrap_or_else(|| maps::Value::default());
        let y = maps::rand_value(&mut rand);
        m.insert(idx, maps::combine(x, y));
    });
}

#[bench]
fn simple_hasher_insert_bench(b: &mut Bencher) {
    bench_hash_map(b, maps::simple::new);
}

#[bench]
fn fnv_hasher_insert_bench(b: &mut Bencher) {
    bench_hash_map(b, maps::fnv::new);
}

#[bench]
fn default_hasher_insert_bench(b: &mut Bencher) {
    bench_hash_map(b, maps::default::new);
}
