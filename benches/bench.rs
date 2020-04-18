#![feature(test)]
extern crate set_bits;
extern crate test;

use test::Bencher;

#[bench]
fn bench_set_all_bits_of_large_region(b: &mut Bencher) -> () {
    const BYTES_OF_POOL: usize = 2 << 20;
    struct LargeRegion {
        _pool: [u8; BYTES_OF_POOL],
    };

    let pool: Box<LargeRegion> = Box::new(LargeRegion {
        _pool: [0; BYTES_OF_POOL],
    });
    let ptr = Box::into_raw(pool);

    b.iter(|| {
        set_bits::set(ptr as usize, 0, BYTES_OF_POOL * 8);
    });

    unsafe {
        let _release = Box::from_raw(ptr);
    }
}
