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

/// Basic iterator type.  Thanks to
/// https://www.quora.com/What-are-some-of-the-amazing-math-tricks-that-you-have-come-across-as-a-coder/answer/Glenn-Rhoads
/// for this idea.
pub struct UpIter {
    submask: u32,
    mask: u32,
}

pub fn up(mask: u32) -> UpIter {
    UpIter {
        submask: 0,
        mask: mask,
    }
}

impl Iterator for UpIter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.submask > self.mask {
            // Illegal state.  Used in order to indicate end.
            return None;
        }

        let ret = self.submask;
        if self.submask == self.mask {
            // Cause an illegal state.
            self.submask = 5;
            self.mask = 0;
        } else {
            self.submask = (self.submask.wrapping_sub(self.mask)) & self.mask;
        }

        Some(ret)
    }
}

#[test]
fn test_count_simple() {
    let mut i = up(0xCAFEBABE);
    assert_eq!(0, i.submask);
    assert_eq!(Some(0), i.next());
    assert_eq!(0b10, i.submask);
    assert_eq!(Some(0b10), i.next());
    assert_eq!(0b100, i.submask);
    assert_eq!(Some(0b100), i.next());
    assert_eq!(0b110, i.submask);
}

#[test]
fn test_count_corner() {
    assert_eq!(vec![0], up(0).collect::<Vec<_>>());
    assert_eq!(vec![0, 1], up(1).collect::<Vec<_>>());
    assert_eq!(vec![0, 0x80], up(0x80).collect::<Vec<_>>());
    assert_eq!(vec![0, 1, 8, 9], up(9).collect::<Vec<_>>());
    assert_eq!(vec![0x0000_0000, 0x0000_0001, 0x8000_0000, 0x8000_0001],
               up(0x8000_0001).collect::<Vec<_>>());
    assert_eq!(vec![0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70],
               up(0x70).collect::<Vec<_>>());
}
