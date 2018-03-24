#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
extern crate primal;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use rustler::schedule::NifScheduleFlags;
use std::iter::FromIterator;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.PrimalEx.NativePrime",
    [
        ("primes", 1, primes, NifScheduleFlags::DirtyCpu),
        ("primes", 2, primes_x_y, NifScheduleFlags::DirtyCpu),
        ("n_primes", 1, n_primes, NifScheduleFlags::DirtyCpu),
        ("n_primes", 2, n_primes_x_y, NifScheduleFlags::DirtyCpu),
        ("nth_prime", 1, nth_prime, NifScheduleFlags::DirtyCpu),
        ("count_primes", 1, count_primes, NifScheduleFlags::DirtyCpu),
        ("is_prime", 1, is_prime, NifScheduleFlags::DirtyCpu),
    ],
    None
}

fn primes<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num: usize = try!(args[0].decode());
    let primes_iter  = primal::Primes::all().take_while(|p| *p < num);
    let primes_vec = Vec::from_iter(primes_iter);
    Ok((atoms::ok(), primes_vec).encode(env))
}

fn primes_x_y<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let x: usize = try!(args[0].decode());
    let y: usize = try!(args[1].decode());
    let primes_iter  = primal::Primes::all().take_while(|p| *p < y).filter(|p| *p > x );
    let primes_vec = Vec::from_iter(primes_iter);
    Ok((atoms::ok(), primes_vec).encode(env))
}

fn n_primes<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num: usize = try!(args[0].decode());
    let primes_iter  = primal::Primes::all().take(num);
    let primes_vec = Vec::from_iter(primes_iter);
    Ok((atoms::ok(), primes_vec).encode(env))
}

fn n_primes_x_y<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let x: usize = try!(args[0].decode());
    let y: usize = try!(args[1].decode());
    let limit = bare_nth_prime(x+y);

    let sieve = primal::Sieve::new(limit);
    let primes_iter = sieve.primes_from(y).take(x);
    let primes_vec = Vec::from_iter(primes_iter);
    Ok((atoms::ok(), primes_vec).encode(env))
}

fn bare_nth_prime(x: usize) -> usize {
    let (_, hi) = primal::estimate_nth_prime(x as u64);
    let sieve = primal::Sieve::new(hi as usize);
    sieve.nth_prime(x)
}

fn nth_prime<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let x: usize = try!(args[0].decode());
    let results = bare_nth_prime(x);
    Ok((atoms::ok(), results).encode(env))
}

fn count_primes<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let x: usize = try!(args[0].decode());
    let count = primal::StreamingSieve::prime_pi(x);
    Ok((atoms::ok(), count).encode(env))
}

fn is_prime<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let x: usize = try!(args[0].decode());
    let result = primal::is_prime(x as u64);
    Ok((atoms::ok(), result).encode(env))
}
