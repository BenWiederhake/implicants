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

/* Compile with:

gcc -o print print.c -I../include/ -L../target/debug/ \
    -limplicants -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil

Or more pedantically:

gcc -o print print.c -I../include/ -L../target/debug/ \
    -limplicants -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil \
    -Wall -Wextra -pedantic -std=c99

This is also valid C++ code!  The header file properly includes C++
headers when it can.  Copy this file to 'print.cpp', and compile as:

g++ -o print print.cpp -I../include/ -L../target/debug/ \
    -limplicants -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil

Or more pedantically:

g++ -o print print.cpp -I../include/ -L../target/debug/ \
    -limplicants -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil \
    -Wall -Wextra -pedantic -std=c++98

*/

#include <assert.h>
#include <implicants.h>
#include <stdio.h>

static int my_fn(void* __base, uint32_t v) {
    (void)sizeof(__base);  /* Ignore argument */
    return 1 & (v >> (1 + (v & 1)));
}

static void print_it(void* __base, uint32_t mask_gap, uint32_t value, int is_prime) {
    (void)sizeof(__base);  /* Ignore argument */
    printf("%08x/%08x is a%s implicant.\n", mask_gap, value,
        is_prime ? " prime" : "n");
}

int main(int argc, char** argv) {
    (void)sizeof(argc);  /* Ignore argument */
    (void)sizeof(argv);  /* Ignore argument */

    printf("Hello world!\n");
    implicants_generate(my_fn, NULL, print_it, NULL, 3);
    printf("That's all.\n");

    return 0;
}
