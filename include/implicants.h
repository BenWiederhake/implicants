/* implicants – Enumerate (prime) implicants of an arbitrary function
 * Copyright (C) 2017  Ben Wiederhake
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef IMPLICANTS_C_HEADER
#define IMPLICANTS_C_HEADER

#if defined(__cplusplus) && __cplusplus > 199711L
#include <cstdint>
extern "C" {
#else
#include <stdint.h>
#endif

typedef int (*sample_fn_t)(void* base, uint32_t v);
typedef void (*report_fn_t)(void* base, uint32_t m, uint32_t nonm, int is_prime);

void implicants_generate(
    sample_fn_t sample, void* sample_base,
    report_fn_t report, void* report_base,
    uint32_t arity);

#if defined(__cplusplus) && __cplusplus > 199711L
}
#endif

#endif /* IMPLICANTS_C_HEADER */
