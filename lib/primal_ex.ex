defmodule PrimalEx do

  alias PrimalEx.NativePrime

  def primes(x) do
    NativePrime.primes(x)
  end

  def n_primes(x) do
    NativePrime.n_primes(x)
  end

end
