   <?php

   /** @assert-if-true int $a */
   function is_int(mixed $a): bool
   {
       return is_int($a);
   }

   /** @assert-if-true string $a */
   function is_string(mixed $a): bool
   {
       return is_string($a);
   }

   function use_int(int $a): void
   {
       echo "I got integer: $a";
   }

   function use_string(string $a): void
   {
       echo "I got string: $a";
   }

   /**
    * @template K of array-key
    * @template V
    *
    * @param K $key
    * @param V $value
    *
    * @return array<K, V>
    */
   function create(string|int $key, mixed $value): array
   {
       if (is_string($key)) {
           use_string($key);
       } else {
           use_int($key);
       }

       if (is_int($key)) {
           use_int($key);
       } else {
           use_string($key);
       }

       return [$key => $value];
   }
