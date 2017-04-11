#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate primal;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
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
    "Elixir.PrimeEx.NativePrime",
    [
        ("primes", 1, primes),
        ("n_primes", 1, n_primes)
    ],
    None
}

fn primes<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num: usize = try!(args[0].decode());
    let thing  = primal::Primes::all().take_while(|p| *p < num);
    let sieve = Vec::from_iter(thing);
    Ok((atoms::ok(), sieve).encode(env))
}

fn n_primes<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let num: usize = try!(args[0].decode());
    let thing  = primal::Primes::all().take(num);
    let sieve = Vec::from_iter(thing);
    Ok((atoms::ok(), sieve).encode(env))
}
