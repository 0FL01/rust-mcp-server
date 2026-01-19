#!/usr/bin/env python3
import sys
import re

def process_file(filepath):
    """Fix all doc comments and schemars positioning."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Pattern: doc comment + field + schemars (wrong order) + next_field
    # We need to: remove doc, move schemars before field
    pattern = r'(\n\s{4}///[^\n]*\n)+\s{4}([^#\n]*?,)\n\s{4}(#\[schemars\(description = ""\)\])\n'
    
    def fix_order(match):
        schemars = match.group(3)
        field = match.group(2)
        # Return schemars + field (without doc comment)
        return f'\n    {schemars}\n    {field}\n'
    
    new_content = re.sub(pattern, fix_order, content)
    
    with open(filepath, 'w') as f:
        f.write(new_content)
    
    # Check if anything changed
    return content != new_content

if __name__ == '__main__':
    filepath = sys.argv[1]
    changed = process_file(filepath)
    print(f"Processed {filepath}: changed={changed}")
