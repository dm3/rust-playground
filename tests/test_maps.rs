#![feature(test)]
#![feature(rand)]

extern crate test;
extern crate rand;
extern crate playground;

use rand::{XorShiftRng, SeedableRng, Rand};
use playground::maps;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault};

//#[test]
//fn um_insert_works() {
//    maps::stdcc_ccmap::um_create();
//    maps::stdcc_ccmap::um_increment_count(1);
//
//    assert_eq!(1, maps::stdcc_ccmap::um_get_count(1);
//}
//
//#[test]
//fn cc_insert_works() {
//    maps::stdcc_ccmap::ccht_create();
//    maps::stdcc_ccmap::ccht_increment_count(1);
//
//    assert_eq!(1, maps::stdcc_ccmap::ccht_get_count(1);
//}

#[test]
fn u32_insert() {
    let mut m = maps::u32_map::ConstSizeVecMap::new(10000);
    let mut rand = XorShiftRng::from_seed([0, 1, 2, 3]);
    let v1 = maps::rand_value(&mut rand);
    assert_eq!(maps::Value::default(), m.insert(0, v1));
    let v2 = maps::rand_value(&mut rand);
    assert_eq!(v1, m.insert(0, v2));
    assert_eq!(v2, m.get(0));
}

#[test]
fn simple_hasher_insert() {
    let mut m = maps::simple::new(10000);
    let mut rand = XorShiftRng::from_seed([0, 1, 2, 3]);
    let v1 = maps::rand_value(&mut rand);
    assert_eq!(None, m.insert(0, v1));
    let v2 = maps::rand_value(&mut rand);
    assert_eq!(Some(v1), m.insert(0, v2));
    assert_eq!(&v2, m.get(&0).unwrap());
}
