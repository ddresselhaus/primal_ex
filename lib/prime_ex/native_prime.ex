defmodule PrimeEx.NativePrime do
  use Rustler, otp_app: :prime_ex, crate: :primeex_nativeprime

  def err(), do: throw :nif_not_loaded

  def primes(_a), do: err()
  def n_primes(_a), do: err()

end
