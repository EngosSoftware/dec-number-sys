#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_quad_init_0001(b: &mut Bencher) {
  b.iter(|| {
    let _ = DecQuad::default();
  });
}

#[bench]
fn bench_dec_quad_init_0002(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_ZERO;
  });
}

#[bench]
fn bench_dec_quad_init_0003(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_ONE;
  });
}

#[bench]
fn bench_dec_quad_init_0004(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_TWO;
  });
}

#[bench]
fn bench_dec_quad_init_0005(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_TEN;
  });
}

#[bench]
fn bench_dec_quad_init_0006(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_HUNDRED;
  });
}

#[bench]
fn bench_dec_quad_init_0007(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_THOUSAND;
  });
}

#[bench]
fn bench_dec_quad_init_0008(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_MILLION;
  });
}

#[bench]
fn bench_dec_quad_init_0009(b: &mut Bencher) {
  b.iter(|| {
    let _ = DEC_QUAD_BILLION;
  });
}
