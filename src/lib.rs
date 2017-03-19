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
mod masked_count;

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

impl Context {
    fn new_chunk(&self) -> Bitset {
        Bitset::of(self.arity)
    }

    fn insert_chunk<'a>(&self, into: &'a mut ChunkMap, at: u32) -> &'a mut Bitset {
        into.entry(at).or_insert(self.new_chunk())
    }
}

fn build_rank_0(ctx: &Context, into: &mut ChunkMap) {
    assert!(ctx.arity < 32, "Can only handle at most 31 bits, but tried {} bits", ctx.arity);
    assert_eq!(into.len(), 0);

    let is_any;

    // Need to end lifetime of 'chunk' before we remove it from the container,
    // so wrap it into a separate scope.
    {
    let chunk: &mut Bitset = ctx.insert_chunk(into, 0);
    // I could probably extend that to include 32, but then this would overflow on x86:
    for i in 0..(1 << ctx.arity) {
        if (ctx.sampling_fn)(i) {
            chunk.set(i);
        }
    }
    is_any = chunk.is_any();
    }

    if !is_any {
        into.remove(&0);
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

#[test]
fn test_build_0_full() {
    // Prepare
    let sample_fn: SampleFn = |a| { true };
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
    assert_eq!(true, c.is(1));
    assert_eq!(true, c.is(2));
    assert_eq!(true, c.is(3));
    assert_eq!(true, c.is(4));
    assert_eq!(true, c.is(5));
    assert_eq!(true, c.is(6));
    assert_eq!(true, c.is(7));
}

#[test]
fn test_build_0_empty() {
    // Prepare
    let sample_fn: SampleFn = |a| { false };
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
    assert_eq!(0, chunks.len());
}
