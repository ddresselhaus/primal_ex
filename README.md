# PrimalEx

NIF Elixir bindings for the [Primal](https://github.com/huonw/primal) Rust library.

It utilizes the [Rustler](https://github.com/hansihe/rustler) library to handle the NIF interop, which may or may not be production-ready. Be aware of the caveats outlined in [this blog post.](http://hansihe.com/2017/02/05/rustler-safe-erlang-elixir-nifs-in-rust.html)

## Installation
The package can be installed
by adding `prime_ex` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [{:primal_ex, "~> 0.1.4"}]
end
```

You may have to install Rust as well. Official installation instructions are [here](https://www.rust-lang.org/en-US/install.html)

## Usage examples

```elixir

# Get a list of primes less than 40
PrimalEx.primes(40)
>>> {:ok, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]}

# Get a list of the primes between 100 and 120
PrimalEx.primes(100, 120)
>>> {:ok, [101, 103, 107, 109, 113]}

# Get a list of the first 10 primes
PrimalEx.n_primes(10)
>>> {:ok, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]}

# Get a list of the first 10 primes >= 1000
PrimalEx.n_primes(10, 1000)
>>> {:ok, [1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061]}

# Get the 10th prime
PrimalEx.nth_prime(10)
>>> {:ok, 29}

# Count the primes below 10**9
PrimalEx.count_primes(10**9)
>>> {:ok, 50847534}


```

