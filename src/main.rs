#![feature(start)]
#![no_std]

use core::intrinsics::transmute;

use libc::{FILE, c_char, c_int, c_ulong, size_t};



use tailcall::tailcall;

pub type mp_limb_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpz_struct {
    pub _mp_alloc: c_int,
    pub _mp_size: c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1usize];
pub type mpz_ptr = *mut __mpz_struct;
pub type mpz_srcptr = *const __mpz_struct;

extern "C" {
    pub fn __gmpz_init(arg1: mpz_ptr);
    pub fn __gmpz_set_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_set_str(
        arg1: mpz_ptr,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;
    // size_t mpz_out_str (FILE *, int, mpz_srcptr);
    pub fn __gmpz_out_str(arg1: *const FILE, arg2: c_int, arg3: mpz_ptr) -> size_t;
    pub fn __gmpz_add_ui(arg1: mpz_ptr, arg2: mpz_ptr, arg3: c_ulong);
    pub fn __gmpz_add(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_ptr);
    pub fn __gmpz_sub(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_ptr);
    pub fn __gmp_printf(arg1: *const c_char, ...) -> c_int;
}

fn fib_iter(n: u64) -> mpz_t {
    unsafe {
        let mut n = n;
        let i: mpz_t = transmute(0u128);
        let j: mpz_t = transmute(0u128);
        let i_pointer: mpz_ptr = transmute(&i);
        let j_pointer: mpz_ptr = transmute(&j);
        __gmpz_init(i_pointer);
        __gmpz_set_ui(i_pointer, 1);
        __gmpz_init(j_pointer);
        __gmpz_set_ui(j_pointer, 0);
        while n > 0u64 {
            __gmpz_add(j_pointer, i_pointer, j_pointer);
            __gmpz_sub(i_pointer, j_pointer, i_pointer);
            n = n - 1;
        } j
    }
}

#[tailcall]
fn fib_tail(n: u64, i: mpz_t, j: mpz_t) -> mpz_t {
    unsafe {
        match n {
            0 => j,
            n => {
                __gmpz_add(transmute(&i), transmute(&j), transmute(&i));
                fib_tail(n-1 , j, i)
            }
        }
    }
}

fn fib_iter2(n: u64) -> mpz_t {
    unsafe {
        let mut n = n;
        let i: mpz_t = transmute(0u128);
        let j: mpz_t = transmute(0u128);
        let h: mpz_t = transmute(0u128);
        let mut i_pointer: mpz_ptr = transmute(&i);
        let mut j_pointer: mpz_ptr = transmute(&j);
        let mut h_pointer: mpz_ptr = transmute(&h);
        __gmpz_init(i_pointer);
        __gmpz_set_ui(i_pointer, 1);
        __gmpz_init(j_pointer);
        __gmpz_set_ui(j_pointer, 0);
        //let csttr: [u8; 5] = [37, 90, 100, 10, 0];
        //let mut tmp_i: mpz_t = transmute(0u128);
        //let tmp_pointer_i: mpz_ptr = transmute(&tmp_i);
        while n > 0u64 {
            //__gmp_printf(transmute(&csttr), j_pointer);

            //let tmp_pointer_h: mpz_ptr = transmute(&[__mpz_struct {
            //    _mp_alloc: (*h_pointer)._mp_alloc,
            //    _mp_size: (*h_pointer)._mp_size,
            //    _mp_d: (*h_pointer)._mp_d
            //}; 1]);
            let tmp_pointer_i: mpz_ptr = transmute(&[__mpz_struct {
                _mp_alloc: (*i_pointer)._mp_alloc,
                _mp_size: (*i_pointer)._mp_size,
                _mp_d: (*i_pointer)._mp_d
            }; 1]);
            //let tmp_pointer_j: mpz_ptr = transmute(&[__mpz_struct {
            //    _mp_alloc: (*j_pointer)._mp_alloc,
            //    _mp_size: (*j_pointer)._mp_size,
            //    _mp_d: (*j_pointer)._mp_d
            //}; 1]);

            __gmpz_add(h_pointer, i_pointer, j_pointer);
            //println!("i: {}, j: {}, h:{}", i_pointer, j_pointer, h_pointer);
            
            i_pointer = transmute(&[__mpz_struct {
                    _mp_alloc: (*j_pointer)._mp_alloc,
                    _mp_size: (*j_pointer)._mp_size,
                    _mp_d: (*j_pointer)._mp_d
                }; 1]);
            j_pointer = transmute(&[__mpz_struct {
                    _mp_alloc: (*h_pointer)._mp_alloc,
                    _mp_size: (*h_pointer)._mp_size,
                    _mp_d: (*h_pointer)._mp_d
                }; 1]);
            h_pointer = transmute(&[__mpz_struct {
                    _mp_alloc: (*tmp_pointer_i)._mp_alloc,
                    _mp_size: (*tmp_pointer_i)._mp_size,
                    _mp_d: (*tmp_pointer_i)._mp_d
                }; 1]);
            //__gmp_printf(transmute(&csttr), i_pointer);
            //__gmp_printf(transmute(&csttr), j_pointer);
            n = n - 1;
        } [*j_pointer; 1]
    }
}

#[start]
fn hola_tail2(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        //let b: u128 = 3;
        //let n: mpz_t = transmute(b);
        //let n_pointer: mpz_ptr = transmute(&n);
        //__gmpz_init(n_pointer);
        //__gmpz_set_ui(n_pointer, 43);
        //println!("as");
        let n = fib_iter2(8);
        let n_pointer: mpz_ptr = transmute(&n);
        let csttr: [u8; 5] = [37, 90, 100, 10, 0];
        __gmp_printf(transmute(&csttr), n_pointer);
        //__gmp_printf((&CString::new("%Zd\n").expect("")).as_ptr(), n_pointer);
        //println!("Hello, world!");
    }
    0
}


fn hola_tail(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        let i: mpz_t = transmute(0u128);
        let j: mpz_t = transmute(0u128);
        let n = fib_tail(10, i, j);
        let n_pointer: mpz_ptr = transmute(&n);
        let csttr: [u8; 5] = [37, 90, 100, 10, 0];
        __gmp_printf(transmute(&csttr), n_pointer);
    } 0
}


fn hola(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        //let b: u128 = 3;
        //let n: mpz_t = transmute(b);
        //let n_pointer: mpz_ptr = transmute(&n);
        //__gmpz_init(n_pointer);
        //__gmpz_set_ui(n_pointer, 43);
        //println!("as");
        let n = fib_iter(10_000_000);
        let n_pointer: mpz_ptr = transmute(&n);
        let csttr: [u8; 5] = [37, 90, 100, 10, 0];
        __gmp_printf(transmute(&csttr), n_pointer);
        //__gmp_printf((&CString::new("%Zd\n").expect("")).as_ptr(), n_pointer);
        //println!("Hello, world!");
    }
    0
}

