# Fast fib on Rust (kinda)

Sooo, I'm just trying to calculate fib(10_000_000) as fast as posible. BUT THERE IS SOMETHING REALLY REALLY WEIRD. TL;DR: it's still not as fast as posible.

Let's take a look into the binary:

```asm
100002630: 4c 89 ff                    	movq	%r15, %rdi
100002633: 4c 89 f6                    	movq	%r14, %rsi
100002636: 4c 89 fa                    	movq	%r15, %rdx
100002639: e8 72 09 00 00              	callq	2418 <___gmpz_add>
10000263e: 4c 89 f7                    	movq	%r14, %rdi
100002641: 4c 89 fe                    	movq	%r15, %rsi
100002644: 4c 89 f2                    	movq	%r14, %rdx
100002647: e8 a4 10 00 00              	callq	4260 <___gmpz_sub>
10000264c: 48 ff c3                    	incq	%rbx
10000264f: 75 df                       	jne	-33 <_main+0x70>
```

this is the code that take the most time (x1_000_000). It's pretty simple, pretty straight forward. So this should be fast, amirite?

Now the weird part of all of this: Agda is faster. 2m:45s faster.

This is the code in Agda: https://github.com/ggzor/agda-fib-tail-rec/blob/main/Fib.agda

That binary is also like 180000 asm lines bigger than the Rust one. SO, What The Function is going here? The short answer: I don't know, mate.

I can't optimize the Rust code more than the current state. Right now it's totally unsafe, and it compiles to 52 lines of assembly code (--release, no_std).

But what it's really strange is that both GHC and Rust versions are using GMP as the BigInt lib. LIKE I'M NOT EVEN USING BINDINGS AT ALL.

So yeah, it's weird. Thus in only can be in the GMP side; but still, https://gitlab.haskell.org/ghc/ghc/-/wikis/replacing-gmp-notes#reasons-for-replacing-gmp-as-the-bignum-library states the following:

> GHC does not have a custom-modified version of GMP (in fact, GHC uses the system build of GMP if that is available).  The memory configuration of GMP uses GMP's Custom Allocation routines.

So my only guess is that "GMP's Custom Allocation routines" are the trick that makes the Agda/MAlozo/Haskell/GHC/C-- version sooooo fast.

But it's 3:47 in the morning to keep up investigating this and I'm tired. So it'll be in another time.

Rust: 07m:55s aprox. https://github.com/ivanmoreau/fastfibo

Agda: 05m:14s aprox. https://github.com/ggzor/agda-fib-tail-rec/blob/main/Fib.agda

(Both are macho x86_64 binaries)

### Conclusions as of right now

the standard memory allocation of GMP is slow? 🤷🏼‍♀️
