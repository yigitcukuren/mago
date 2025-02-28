<?php

return $propertyMetadata->withSchema(
    ($this->addNullabilityToTypeDefinition)(
        [
            "type" => "string",
            "format" => "decimal",
        ],
        $type
    )
);
