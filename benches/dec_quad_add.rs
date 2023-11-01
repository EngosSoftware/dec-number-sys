#![feature(test)]

extern crate test;

use dec_number_sys::*;
use test::Bencher;

#[bench]
fn bench_dec_quad_add_0001(b: &mut Bencher) {
  let dc = &mut dec_context_128();
  let dql = &dec_quad_from_string("0.1", dc);
  let dqr = &dec_quad_from_string("0.2", dc);
  b.iter(|| {
    let _ = dec_quad_add(dql, dqr, dc);
  });
}
