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

extern crate subint;
mod bits;
mod masked_count;

#[cfg(feature = "c-abi")]
pub mod c;

use std::collections::HashMap;
use bits::Bitset;

type ChunkMap = HashMap<u32, Bitset>;

struct Context<'a, 'b> {
    sampling_fn: &'a Fn(u32) -> bool,
    report_fn: &'b mut FnMut(u32, u32, bool),
    arity: u32,
}

impl<'x, 'y> Context<'x, 'y> {
    fn new_chunk(&self) -> Bitset {
        Bitset::of(self.arity)
    }

    fn insert_chunk<'a>(&self, into: &'a mut ChunkMap, at: u32) -> &'a mut Bitset {
        into.entry(at).or_insert_with(|| self.new_chunk())
    }
}

fn build_rank_0(ctx: &Context, into: &mut ChunkMap) {
    assert!(ctx.arity < 32,
            "Can only handle at most 31 bits, but tried {} bits",
            ctx.arity);
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

#[cfg(test)]
fn test_sample_mod3(v: u32) -> bool {
    (v % 3) == 0
}
#[cfg(test)]
fn test_sample_mux(v: u32) -> bool {
    1 == 1 & (v >> (1 + (v & 1)))
}
#[cfg(test)]
fn test_sample_fail(_: u32) -> bool {
    panic!("But there is nothing to sample?!");
}

#[cfg(test)]
fn test_report_fail(_: u32, _: u32, _: bool) {
    panic!("But there is nothing to report?!");
}

#[test]
fn test_build_0() {
    // Prepare
    let ctx = Context {
        sampling_fn: &test_sample_mod3,
        report_fn: &mut test_report_fail,
        arity: 3,
    };
    let mut chunks = ChunkMap::new();

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
    let ctx = Context {
        sampling_fn: &|_| true,
        report_fn: &mut test_report_fail,
        arity: 3,
    };
    let mut chunks = ChunkMap::new();

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
    let ctx = Context {
        sampling_fn: &|_| false,
        report_fn: &mut test_report_fail,
        arity: 3,
    };
    let mut chunks = ChunkMap::new();

    // Call under test
    build_rank_0(&ctx, &mut chunks);

    // Check
    assert_eq!(0, chunks.len());
}

fn build_rank_n(ctx: &Context, rank: u32, into: &mut ChunkMap, from: &ChunkMap) {
    assert!(into.is_empty());

    /* Quick path in case there's nothing to do *at all*. */
    if from.is_empty() {
        return;
    }

    let arity_mask = subint::of(ctx.arity);
    // For each destination chunk:
    for mask_m in arity_mask.permute(rank) {
        // Pick a subchunk from which we're going to read
        let overmask_m = mask_m & (mask_m - 1);
        let subchunk: Option<&Bitset> = from.get(&overmask_m);
        if subchunk.is_none() {
            /* This chunk would be blank anyway. */
            continue;
        }
        let subchunk = subchunk.unwrap();

        let is_any;
        // Need to end lifetime of 'chunk' before we remove it from the container,
        // so wrap it into a separate scope.
        {
            let chunk: &mut Bitset = ctx.insert_chunk(into, mask_m);
            let collapsed_dim = mask_m & !overmask_m;
            assert_eq!(1, collapsed_dim.count_ones(), "{}", collapsed_dim);
            // For each face:
            for i in masked_count::up(arity_mask.invert(mask_m)) {
                // If both "sides" of the current "face" are implicants,
                // then the current "face" is an implicant, too.
                if subchunk.is(i) && subchunk.is(i | collapsed_dim) {
                    chunk.set(i);
                }
            }
            is_any = chunk.is_any();
        }

        if !is_any {
            // None were set, so prune it for the next layer.
            into.remove(&mask_m);
        }
    }
}

#[test]
fn test_build_n() {
    // Prepare
    let ctx = Context {
        sampling_fn: &test_sample_mux,
        report_fn: &mut test_report_fail,
        arity: 3,
    };
    let mut chunks_from = ChunkMap::new();
    build_rank_0(&ctx, &mut chunks_from);
    assert_eq!(1, chunks_from.len());
    let chunks_from = chunks_from;
    let mut chunks_into = ChunkMap::new();

    // Call under test
    build_rank_n(&ctx, 1, &mut chunks_into, &chunks_from);

    // Check
    assert_eq!(3, chunks_into.len());
    let c: &Bitset = &chunks_into[&0b001]; // XXM
    assert_eq!(false, c.is(0b000));
    assert_eq!(false, c.is(0b010));
    assert_eq!(false, c.is(0b100));
    assert_eq!(true, c.is(0b110));
    let c: &Bitset = &chunks_into[&0b010]; // XMX
    assert_eq!(false, c.is(0b000));
    assert_eq!(false, c.is(0b001));
    assert_eq!(false, c.is(0b100));
    assert_eq!(true, c.is(0b101));
    let c: &Bitset = &chunks_into[&0b100]; // MXX
    assert_eq!(false, c.is(0b000));
    assert_eq!(false, c.is(0b001));
    assert_eq!(true, c.is(0b010));
    assert_eq!(false, c.is(0b011));
}

#[test]
fn test_build_n_empty() {
    // Prepare
    let ctx = Context {
        sampling_fn: &test_sample_fail,
        report_fn: &mut test_report_fail,
        arity: 3,
    };
    let mut chunks_from = ChunkMap::new();
    ctx.insert_chunk(&mut chunks_from, 0).set(0);
    assert_eq!(1, chunks_from.len());
    let chunks_from = chunks_from;
    let mut chunks_into = ChunkMap::new();

    // Call under test
    build_rank_n(&ctx, 1, &mut chunks_into, &chunks_from);

    // Check
    assert_eq!(0, chunks_into.len());
}

#[test]
fn test_build_n_empty_imm() {
    // Prepare
    let ctx = Context {
        sampling_fn: &test_sample_fail,
        report_fn: &mut test_report_fail,
        arity: 3,
    };
    let chunks_from = ChunkMap::new();
    let mut chunks_into = ChunkMap::new();

    // Call under test
    build_rank_n(&ctx, 1, &mut chunks_into, &chunks_from);

    // Check
    assert_eq!(0, chunks_into.len());
}

fn report_0n(ctx: &mut Context, chunks: &ChunkMap) {
    let arity_mask = subint::of(ctx.arity);
    // For each chunk:
    for (&mask_m, chunk) in chunks {
        // For each face:
        for face in masked_count::up(arity_mask.invert(mask_m)) {
            if !chunk.is(face) {
                /* If it's not an implicant, then it's not an implicant.
                 * Furthermore, it's definitely not a prime implicant. */
                continue;
            }
            let mut has_peer = false;
            // For each potential peer:
            // TODO: Compute the relevant 'peer_dir's more cleverly.
            for peer_dir in arity_mask.permute(1) {
                // If that peer exists and is on:
                if (mask_m & peer_dir) == 0 && chunk.is(face ^ peer_dir) {
                    // … then we found a more general implicant.
                    has_peer = true;
                    break;
                }
            }
            // The above loop exhaustively checks for *all* potentially more
            // general implicants.  So if we still haven't found a peer,
            // then this is actually a prime implicant!
            let is_prime = !has_peer;
            (ctx.report_fn)(mask_m, face, is_prime);
        }
    }
}

#[test]
fn test_report() {
    // Prepare
    let mut report_target: Vec<(u32, u32, bool)> = vec![];
    {
        let mut report = |mask_m: u32, mask_nonm: u32, prime: bool| {
            report_target.push((mask_m, mask_nonm, prime));
        };
        let mut ctx = Context {
            sampling_fn: &test_sample_fail,
            report_fn: &mut report,
            arity: 3,
        };
        let mut chunks_from = ChunkMap::new();
        {
            let chunk = ctx.insert_chunk(&mut chunks_from, 0);
            chunk.set(0b000);
            chunk.set(0b110);
            chunk.set(0b111);
        }
        assert_eq!(1, chunks_from.len());

        // Call under test
        report_0n(&mut ctx, &chunks_from);
    }

    // Check
    assert_eq!(3, report_target.len());
    println!("{:?}", report_target);
    let mut seen = vec![false; 3];
    for entry in report_target {
        match entry {
            (0, 0b000, true) => {
                seen[0] = true;
            }
            (0, 0b110, false) => {
                seen[1] = true;
            }
            (0, 0b111, false) => {
                seen[2] = true;
            }
            _ => {
                panic!("unexpected entry: {:?}", entry);
            }
        }
    }
    assert_eq!(vec![true, true, true], seen);
}

pub fn generate(sampling_fn: &Fn(u32) -> bool,
                report_fn: &mut FnMut(u32, u32, bool),
                arity: u32) {
    let mut ctx = Context {
        sampling_fn: sampling_fn,
        report_fn: report_fn,
        arity: arity,
    };
    let mut map0 = ChunkMap::new();
    let mut map1 = ChunkMap::new();
    build_rank_0(&ctx, &mut map0);
    report_0n(&mut ctx, &map0);

    for rank in 1..ctx.arity {
        let (from, into) = if rank % 2 == 0 {
            (&mut map1, &mut map0)
        } else {
            (&mut map0, &mut map1)
        };
        build_rank_n(&ctx, rank, into, from);
        from.clear();
        report_0n(&mut ctx, into);
    }
}
