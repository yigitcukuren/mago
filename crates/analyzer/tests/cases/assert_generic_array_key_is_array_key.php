  <?php

  /**
   * @assert truthy $assertion
   */
  function assert(mixed $assertion, string|null $description = null): bool
  {
      return assert($assertion, $description);
  }

  /** @assert-if-true array-key $k */
  function is_array_key(mixed $k): bool
  {
      return is_array_key($k);
  }

  /**
   * @template K of array-key
   *
   * @param K $k
   *
   * @return K
   */
  function x(mixed $k): mixed
  {
      assert(is_array_key($k), 'its array key');

      return $k;
  }
