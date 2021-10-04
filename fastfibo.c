#include "gmps/gmp-6.2.1/gmp.h"

void
fib_iter(unsigned long int n, mpz_t x){
    mpz_t i, j;
    mpz_init(i); mpz_init(j);
    mpz_set_ui(i, 1);
    mpz_set_ui(j, 0);
    while (n > 0){
        mpz_add(j, i, j);
        mpz_sub(i, j, i);
        n = n - 1;
    }
    *x = *j; 
}

void
main(){
    mpz_t n;
    fib_iter(10000000, n);
    gmp_printf("%Zd\n", n);
}