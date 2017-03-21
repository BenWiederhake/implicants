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

extern crate implicants;

fn main() {
    let my_fn = |x: u32| (x % 3) == 0;
    let mut buffer: Vec<(u32, u32, bool)> = Vec::new();

    {
        // Need to return borrow of 'buffer' before we iterate.
        let mut store_it = |mask_gap: u32, value: u32, is_prime: bool| {
            buffer.push((mask_gap, value, is_prime));
        };
        println!("Hello world!");
        implicants::generate(&my_fn, &mut store_it, 3);
    }

    for (mask_gap, value, is_prime) in buffer {
        println!("{:032b}/{:032b} is a{} implicant.",
                 mask_gap,
                 value,
                 if is_prime { " prime" } else { "n" });
    }
    println!("That's it.");
}
