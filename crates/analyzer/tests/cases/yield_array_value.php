 <?php

 /**
  * @param array<string, string> $array
  * @return iterable<string>
  */
 function generator(array $array): iterable
 {
     yield $array['key'] ?? 'default';
 }
