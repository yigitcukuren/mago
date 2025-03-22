<?php

final class Issue164
{
    private function test(): Response
    {
        $this->assertStringEqualsStringIgnoringLineEndings(<<<HTML
        ||||||||||||||||||||||||||||||||
                                        
        ||||||||||||||||||||||||        
                                
        ||||||||||||||||        
                        
        ||||||||        
                
        ||||    
            
        ||  
        HTML, $html);
        
        $this->assertStringEqualsStringIgnoringLineEndings("
        ||||||||||||||||||||||||||||||||
                                        
        ||||||||||||||||||||||||        
                                
        ||||||||||||||||        
                        
        ||||||||        
                
        ||||    
            
        ||  
        ", $html);

        $this->assertStringEqualsStringIgnoringLineEndings('
        ||||||||||||||||||||||||||||||||
                                        
        ||||||||||||||||||||||||        
                                
        ||||||||||||||||        
                        
        ||||||||        
                
        ||||    
            
        ||  
        ', $html);


        $this->assertStringEqualsStringIgnoringLineEndings("
        ||||||||||||||||||||||||||||||||
                                        
        ||||||||||||||||||||||||        
                                
        ||||||||||||||||        
                        
        ||||||||        
                
        ||||    
            
        || $tag
        ", $html);
    }
}