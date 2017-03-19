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

//! Thin layer of arbitrary bitset implementation.
//! I want to be able to replace it easily.

extern crate fixedbitset;

pub struct Bitset {
    backing: fixedbitset::FixedBitSet,
}

impl Bitset {
    pub fn of(nbits: u32) -> Self {
        assert!(nbits < 32, "Can only handle at most 31 bits, but tried {} bits", nbits);
        // I could probably extend that to include 32, but then this would overflow on x86:
        let len = (1usize << nbits) as usize;
        Bitset {
            backing: fixedbitset::FixedBitSet::with_capacity(len),
        }
    }

    pub fn set(&mut self, mask: u32) {
        self.backing.set(mask as usize, true);
    }

    pub fn is(&self, mask: u32) -> bool {
        assert!((mask as usize) < self.backing.len(),
            "Accessed {}, but len is only {}", mask, self.backing.len());
        self.backing.contains(mask as usize)
    }

    pub fn is_any(&self) -> bool {
        self.backing.count_ones(..) > 0
    }
}
