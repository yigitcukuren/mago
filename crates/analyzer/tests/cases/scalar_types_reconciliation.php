  <?php

  /**
   * @assert-if-true bool $value
   *
   * @return ($value is bool ? true : false)
   *
   * @pure
   */
  function is_bool(mixed $value): bool
  {
      return is_bool($value);
  }

  /**
   * @assert-if-true float $value
   *
   * @return ($value is float ? true : false)
   *
   * @pure
   */
  function is_float(mixed $value): bool
  {
      return is_float($value);
  }

  /**
   * @assert-if-true int $value
   *
   * @return ($value is int ? true : false)
   *
   * @pure
   */
  function is_int(mixed $value): bool
  {
      return is_int($value);
  }

  /**
   * @assert-if-true string $value
   *
   * @return ($value is string ? true : false)
   *
   * @pure
   */
  function is_string(mixed $value): bool
  {
      return is_string($value);
  }

  function format_string_or_int(string|int $element): string
  {
      if (is_int($element)) {
          return (string) $element;
      } else {
          return '\'' . $element . '\'';
      }
  }

  function format_string_or_bool(string|bool $element): string
  {
      if (is_bool($element)) {
          return $element ? 'true' : 'false';
      } else {
          return '\'' . $element . '\'';
      }
  }

  function format_int_or_float(int|float $element): string
  {
      if (is_int($element)) {
          return (string) $element;
      } else {
          return (string) $element;
      }
  }

  function format_string_or_int_or_bool(string|int|bool $element): string
  {
      if (is_string($element)) {
          return '\'' . $element . '\'';
      } elseif (is_int($element)) {
          return (string) $element;
      } else {
          return $element ? 'true' : 'false';
      }
  }

  function format_string_or_float_or_bool(string|float|bool $element): string
  {
      if (is_string($element)) {
          return '\'' . $element . '\'';
      } elseif (is_float($element)) {
          return (string) $element;
      } else {
          return $element ? 'true' : 'false';
      }
  }

  function format_any(string|int|float|bool $element): string
  {
      if (is_string($element)) {
          return '\'' . $element . '\'';
      } elseif (is_int($element)) {
          return (string) $element;
      } elseif (is_float($element)) {
          return (string) $element;
      } else {
          return $element ? 'true' : 'false';
      }
  }
