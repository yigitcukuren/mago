 <?php

 /**
  * @assert-if-true array<array-key, mixed> $value
  *
  * @return ($value is array ? true : false)
  *
  * @pure
  */
 function is_array(mixed $value): bool
 {
     return is_array($value);
 }

 /**
  * @assert-if-true list<mixed> $array
  *
  * @return ($array is list ? true : false)
  *
  * @pure
  */
 function array_is_list(array $array): bool
 {
     return array_is_list($array);
 }

 /**
  * @return null|list<mixed>
  */
 function to_list(mixed $value): null|array
 {
     if (!is_array($value) || !array_is_list($value)) {
         return null;
     } else {
         return $value;
     }
 }
