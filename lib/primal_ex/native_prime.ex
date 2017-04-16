defmodule PrimalEx.NativePrime do
  use Rustler, otp_app: :primal_ex, crate: :primalex_nativeprime

  def err(), do: throw :nif_not_loaded

  def primes(_a), do: err()
  def primes(_a,_b), do: err()
  def n_primes(_a), do: err()

end
