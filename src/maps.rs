//mod stdcc_ccmap {
//    #[link(name = "c++", kind = "static")]
//    #[link(name = "stdcc_ccmap", kind = "static")]
//    extern {
//        fn um_create();
//        fn ccht_create();
//        fn um_destroy();
//        fn ccht_destroy();
//        fn um_increment_count(i: i32);
//        fn ccht_increment_count(i: i32);
//        fn um_get_count(i: u32) -> i32;
//        fn ccht_get_count(i: u32) -> i32;
//    }
//}

use rand::{Rng, Rand};

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Slice(i32, i32, i32, i32, u32);

// 8 + 8 + 5*32*4 = 656 bytes
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Value {
    pt: u8, vt: u8,
    sec1: Slice,
    sec5: Slice,
    sec30: Slice,
    min1: Slice,
}

pub fn rand_slice<R: Rng>(rng: &mut R) -> Slice {
    Slice (i32::rand(rng), i32::rand(rng), i32::rand(rng), i32::rand(rng), u32::rand(rng))
}

pub fn rand_value<R: Rng>(rng: &mut R) -> Value {
    Value {
        pt: u8::rand(rng), vt: u8::rand(rng),
        sec1: rand_slice(rng),
        sec5: rand_slice(rng),
        sec30: rand_slice(rng),
        min1: rand_slice(rng),
    }
}

pub fn combine(v1: Value, v2: Value) -> Value {
    v2
}

pub mod u32_map {
    use std::mem;

    #[derive(Clone)]
    pub struct ConstSizeVecMap {
        v: Vec<super::Value>,
    }

    impl ConstSizeVecMap {
        pub fn new(sz: usize) -> ConstSizeVecMap {
            ConstSizeVecMap { v: vec![super::Value::default(); sz], }
        }

        pub fn insert(&mut self, key: usize, value: super::Value) -> super::Value {
            let len = self.v.len();
            if len <= key {
                panic!("Map is too small!");
            }
            let old_value = mem::replace(&mut self.v[key], value);
            old_value
        }

        #[inline]
        pub fn get(&mut self, key: usize) -> super::Value {
            self.v[key]
        }
    }
}

//mod small_int {
//    use std::collections::SmallIntMap;
//}

pub mod fnv {
    use std::hash::{Hasher, BuildHasher, BuildHasherDefault};
    use std::collections::HashMap;

    pub type FnvBuildHasher = BuildHasherDefault<FnvHasher>;

    pub fn new(capacity: usize) -> HashMap<usize, super::Value, FnvBuildHasher> {
        HashMap::<usize, super::Value, _>::with_capacity_and_hasher(
            capacity, FnvBuildHasher::default())
    }

    pub struct FnvHasher(u64);

    impl Default for FnvHasher {

        #[inline]
        fn default() -> FnvHasher {
            FnvHasher(0xcbf29ce484222325)
        }
    }

    impl Hasher for FnvHasher {

        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            let FnvHasher(mut hash) = *self;

            for byte in bytes.iter() {
                hash = hash ^ (*byte as u64);
                hash = hash.wrapping_mul(0x100000001b3);
            }

            *self = FnvHasher(hash);
        }
    }
}

// https://gist.github.com/arthurprs/88eef0b57b9f8341c54e2d82ec775698
// great if your keys are integers
pub mod simple {
    use std::collections::HashMap;
    use std::hash::{Hasher, BuildHasher, BuildHasherDefault};

    pub type SimpleBuildHasher = BuildHasherDefault<SimpleHasher>;

    pub fn new(capacity: usize) -> HashMap<usize, super::Value, SimpleBuildHasher> {
        HashMap::<usize, super::Value, _>::with_capacity_and_hasher(
            capacity, SimpleBuildHasher::default())
    }

    pub struct SimpleHasher(u64);

    #[inline]
    fn load_u64_le(buf: &[u8], len: usize) -> u64 {
        use std::ptr;
        debug_assert!(len <= buf.len());
        let mut data = 0u64;
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), &mut data as *mut _ as *mut u8, len);
        }
        data.to_le()
    }

    impl Default for SimpleHasher {
        #[inline]
        fn default() -> SimpleHasher {
            SimpleHasher(0)
        }
    }

    impl Hasher for SimpleHasher {

        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            *self = SimpleHasher(load_u64_le(bytes, bytes.len()));
        }
    }
}

pub mod default {
    use std::collections::HashMap;

    pub fn new(capacity: usize) -> HashMap<usize, super::Value> {
        HashMap::<usize, super::Value>::with_capacity(capacity)
    }
}
