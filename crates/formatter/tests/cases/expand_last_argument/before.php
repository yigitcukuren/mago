<?php

$value = strtr($this->fileExcerpt($trace['file'], $trace['line'], 5), [
    '#DD0000' => 'var(--highlight-string)',
    '#007700' => 'var(--highlight-keyword)',
    '#0000BB' => 'var(--highlight-default)',
    '#FF8000' => 'var(--highlight-comment)',
]);