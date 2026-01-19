#!/usr/bin/env python3
import sys
import re

def process_file(filepath):
    """Remove all doc comments before field declarations in struct definitions."""
    with open(filepath, 'r') as f:
        lines = f.readlines()
    
    result = []
    i = 0
    removed_count = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if this is a doc comment line (starts with /// with 4 spaces or at struct level)
        if re.match(r'^(\s{4}///|^///)', line):
            # Check if the next non-empty, non-doc-comment line is a field declaration
            j = i + 1
            while j < len(lines) and (lines[j].strip() == '' or re.match(r'^\s{4}///', lines[j])):
                j += 1
            
            # Check if we found a field declaration (contains pub or serde)
            if j < len(lines):
                next_line = lines[j]
                # Check if it's a field declaration (has pub or serde attribute)
                if 'pub ' in next_line or '#[serde' in next_line or '#[schemars' in next_line:
                    # This is a field-level doc comment - skip all doc lines
                    removed_count += (j - i)
                    i = j
                    continue
            
        result.append(line)
        i += 1
    
    with open(filepath, 'w') as f:
        f.writelines(result)
    
    return removed_count

if __name__ == '__main__':
    filepath = sys.argv[1]
    removed = process_file(filepath)
    print(f"Cleaned {filepath}: removed {removed} additional doc comment lines")
