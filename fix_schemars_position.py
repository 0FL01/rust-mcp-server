#!/usr/bin/env python3
import sys
import re

def process_file(filepath):
    """Fix schemars attributes that appear after field declarations."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Pattern: field declaration followed by schemars (wrong order)
    pattern = r'(pub\s+\w+[^:]*:[^,\n]+,)\n\s+(#\[schemars\(description = ""\)\])\n'
    
    def fix_order(match):
        field = match.group(1)
        schemars = match.group(2)
        # Return schemars before field
        return f'{schemars}\n    {field}\n'
    
    original_count = len(re.findall(pattern, content))
    new_content = re.sub(pattern, fix_order, content)
    
    with open(filepath, 'w') as f:
        f.write(new_content)
    
    return original_count

if __name__ == '__main__':
    filepath = sys.argv[1]
    fixed = process_file(filepath)
    print(f"Fixed {filepath}: {fixed} schemars positions")
