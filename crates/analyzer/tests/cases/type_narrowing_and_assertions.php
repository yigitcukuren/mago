 <?php

 /**
  * @assert-if-true array<array-key, mixed> $value
  *
  * @return ($value is array ? true : false)
  */
 function is_array(mixed $value): bool
 {
     return is_array($value);
 }

 /**
  * @template K as array-key
  * @template V
  *
  * @param array<K, V> $array
  *
  * @return list<V>
  */
 function array_values(array $array): array
 {
     return array_values($array);
 }

 /**
  * @template K as array-key
  * @template V
  * @template S
  * @template U
  *
  * @param (callable(V, S): U)|null $callback
  * @param array<K, V> $array
  * @param array<array-key, S> ...$arrays
  *
  * @return array<K, U>
  */
 function array_map(null|callable $callback, array $array, array ...$arrays): array
 {
     return array_map($callback, $array, ...$arrays);
 }

 /**
  * @template Tk
  * @template Tv
  * @template T
  *
  * @param iterable<Tk, Tv> $iterable Iterable to be mapped over
  * @param (Closure(Tv): T) $function
  *
  * @return ($iterable is non-empty-array ? non-empty-list<T> : list<T>)
  */
 function map(iterable $iterable, Closure $function): array
 {
     if (is_array($iterable)) {
         return array_values(array_map($function, $iterable));
     }

     $result = [];
     foreach ($iterable as $value) {
         $result[] = $function($value);
     }

     return $result;
 }
