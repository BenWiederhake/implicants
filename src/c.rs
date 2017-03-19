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

use generate;

type SampleFnC = extern "C" fn(*const u8, u32) -> bool;
type ReportFnC = extern "C" fn(*const u8, u32, u32, bool);

#[no_mangle]
pub extern "C" fn implicants_generate(
        sample: SampleFnC, sample_base: *const u8,
        report: ReportFnC, report_base: *const u8,
        arity: u32) {
    let sample_wrapped = &|v| { sample(sample_base, v) };
    let mut report_wrapped = &mut |m, nonm, prime| {
        report(report_base, m, nonm, prime);
    };

    generate(sample_wrapped, report_wrapped, arity);
}
