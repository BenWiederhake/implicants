// implicants – Enumerate (prime) implicants of an arbitrary function
// Copyright (C) 2017  Ben Wiederhake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(closure_to_fn_coercion)]

extern crate subint;
mod bits;

use std::collections::HashMap;
use subint::raw::mk_ones;
use bits::Bitset;

type ChunkMap = HashMap<u32, Bitset>;
type SampleFn = fn(u32) -> bool;
type ReportFn = fn(u32, u32, bool);

struct Context {
    sampling_fn: SampleFn,
    report_fn: ReportFn,
    arity: u32,
}

fn build_rank_0(ctx: &Context, into: &mut ChunkMap) {
    assert_eq!(into.len(), 0);
    let chunk: &mut Bitset = into.entry(0).or_insert(Bitset::of(ctx.arity));

    for i in 0..mk_ones(ctx.arity) {
        if (ctx.sampling_fn)(i) {
            chunk.set(i);
        }
    }
}

#[test]
fn test_build_0() {
    // Prepare
    let sample_fn: SampleFn = |a| { (a % 3) == 0 };
    let false_fn: ReportFn = |a, b, c| { panic!("But there is nothing to report?!"); };
    let ctx = Context{
        sampling_fn: sample_fn,
        report_fn: false_fn,
        arity: 3,
    };
    let mut chunks = HashMap::new();

    // Call under test
    build_rank_0(&ctx, &mut chunks);

    // Check
    assert_eq!(1, chunks.len());
    let c: &Bitset = &chunks[&0];
    assert_eq!(true, c.is(0));
    assert_eq!(false, c.is(1));
    assert_eq!(false, c.is(2));
    assert_eq!(true, c.is(3));
    assert_eq!(false, c.is(4));
    assert_eq!(false, c.is(5));
    assert_eq!(true, c.is(6));
    assert_eq!(false, c.is(7));
}
