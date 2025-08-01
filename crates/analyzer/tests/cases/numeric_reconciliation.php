  <?php

  interface Stringable
  {
      public function __toString(): string;
  }

  /**
   * @assert-if-true numeric $value
   */
  function is_numeric(mixed $value): bool
  {
      return is_numeric($value);
  }

  /**
   * @assert-if-true string $value
   */
  function is_string(mixed $value): bool
  {
      return is_string($value);
  }

  /**
   * @return null|numeric-string
   */
  function to_numeric_string(mixed $value): null|string
  {
      if (is_string($value) && is_numeric($value)) {
          return $value;
      }

      if (is_numeric($value)) {
          return (string) $value;
      }

      if ($value instanceof Stringable) {
          $str = (string) $value;
          if (is_numeric($str)) {
              return $str;
          }
      }

      return null;
  }
