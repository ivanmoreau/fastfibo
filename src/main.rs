#![feature(start)]
#![no_std]

use core::{intrinsics::transmute, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpz_struct {
    pub _mp_alloc: i32,
    pub _mp_size: i32,
    pub _mp_d: *mut u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpz_t ([__mpz_struct; 1usize]);
pub type mpz_ptr = *mut __mpz_struct;
pub type mpz_srcptr = *const __mpz_struct;

extern "C" {
    pub fn __gmpz_init(arg1: &mut mpz_t);
    pub fn __gmpz_set_ui(arg1: &mut mpz_t, arg2: u64);
    pub fn __gmpz_add(arg1: &mut mpz_t, arg2: &mpz_t, arg3: &mpz_t);
    pub fn __gmp_printf(arg1: *const i8, ...) -> i32;
}

fn gmpz_init() -> mpz_t {
    let mut n: mpz_t = unsafe { transmute(0u128) };
    unsafe { __gmpz_init(&mut n) }
    n
}

fn gmpz_set_u64(mut big: mpz_t, n: u64) -> mpz_t {
    unsafe { __gmpz_set_ui(&mut big, n) }
    big
}

fn gmpz_add(fst: &mpz_t, snd: &mpz_t, mut buffer: mpz_t) -> mpz_t {
    unsafe { __gmpz_add(&mut buffer, fst, snd) }
    buffer
}

fn gmpz_steal(val: &mpz_t) -> mpz_t {
    mpz_t([__mpz_struct {
        _mp_alloc: val.0[0]._mp_alloc,
        _mp_size: val.0[0]._mp_size,
        _mp_d: val.0[0]._mp_d
    };1])
}

fn gmp_printf_val(big: &mpz_t) {
    let csttr: [u8; 5] = [37, 90, 100, 10, 0];
    unsafe { __gmp_printf(transmute(&csttr), big) };
}

fn fib_iter_good(n: u64) -> mpz_t {
    //let mut n = n;
    let mut i = gmpz_init();
    i = gmpz_set_u64(i, 1);
    let mut j = gmpz_init();
    j = gmpz_set_u64(j, 0);
    let mut h = gmpz_init();
    for _ in 0..n {
        //j = &i + &j;
        let oldj = mpz_t([__mpz_struct {
                _mp_alloc: i.0[0]._mp_alloc,
                _mp_size: i.0[0]._mp_size,
                _mp_d: i.0[0]._mp_d
            }; 1]);
        h = gmpz_add(&j, &i, h);
        i = j;
        j = h;
        h = oldj
    } j
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let val = fib_iter_good(10_000_000);
    gmp_printf_val(&val);
    0
}
