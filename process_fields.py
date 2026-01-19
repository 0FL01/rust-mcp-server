#!/usr/bin/env python3
import sys
import re

def process_file(filepath):
    """Process a single file to remove doc comments and add schemars attributes."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Pattern to match doc comments followed by field with #[serde(...)]
    pattern = r'(\n\s*///[^\n]*\n)+\s*(#\[serde\([^)]*\)\])'
    
    def replace_with_schemars(match):
        serde_attr = match.group(2)
        return f'\n    #[schemars(description = "")]\n    {serde_attr}'
    
    new_content = re.sub(pattern, replace_with_schemars, content)
    
    with open(filepath, 'w') as f:
        f.write(new_content)
    
    original_doc_count = len(re.findall(r'\n\s*///[^\n]*\n', content))
    new_schemars_count = len(re.findall(r'#\[schemars\(description = ""\)\]', new_content))
    return original_doc_count, new_schemars_count

if __name__ == '__main__':
    filepath = sys.argv[1]
    original, new = process_file(filepath)
    print(f"Processed {filepath}: {original} doc comments -> {new} schemars attributes")
