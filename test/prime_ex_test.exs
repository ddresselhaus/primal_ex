defmodule PrimalExTest do
  use ExUnit.Case

  test "primes/1 returns prime numbers less than x" do
    expected_result =[
      2,
      3,
      5,
      7,
      11,
      13,
      17,
      19,
      23,
      29,
      31,
      37
    ]
    {:ok, result} = PrimalEx.primes(40)
    assert result == expected_result
  end

  test "n_primes/1 returns the first x prime numbers" do
    expected_result =[
      2,
      3,
      5,
      7,
      11,
      13,
      17,
      19,
      23,
      29,
      31,
      37
    ]
    {:ok, result} = PrimalEx.n_primes(12)
    assert result == expected_result
  end

end
