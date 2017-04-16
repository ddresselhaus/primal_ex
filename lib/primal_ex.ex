defmodule PrimalEx do

  alias PrimalEx.NativePrime

  def primes(x) do
    NativePrime.primes(x)
  end

  def primes(x,y) when x > y do
    {:error, "The first arugment cannot be greater than the second"}
  end

  def primes(x,y)  do
    NativePrime.primes(x,y)
  end

  def n_primes(x) do
    NativePrime.n_primes(x)
  end

  def n_primes(x,y) when x > y do
    {:error, "The first arugment cannot be greater than the second"}
  end

  def n_primes(x,y)  do
    NativePrime.n_primes(x,y)
  end

end
