#!/usr/bin/env python3
import sys
import re

def process_file(filepath):
    """Process a file to remove doc comments and add schemars attributes."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Count original doc comments (excluding struct-level)
    original_doc_count = len(re.findall(r'\n\s{4}///[^\n]*\n', content))
    
    # Pattern 1: doc comments followed by #[serde(...)]
    pattern1 = r'(\n\s{4}///[^\n]*\n)+\s{4}(#\[serde\([^)]*\)\])'
    def replace1(match):
        serde_attr = match.group(2)
        return f'\n    #[schemars(description = "")]\n    {serde_attr}'
    content = re.sub(pattern1, replace1, content)
    
    # Pattern 2: doc comments followed by pub field (no serde attribute)
    pattern2 = r'(\n\s{4}///[^\n]*\n)+\s{4}(pub\s+\w+[^:]*:[^,;]+)'
    def replace2(match):
        field_decl = match.group(2)
        return f'\n    #[schemars(description = "")]\n    {field_decl}'
    content = re.sub(pattern2, replace2, content)
    
    # Pattern 3: doc comments followed by #[serde(flatten)]
    pattern3 = r'(\n\s{4}///[^\n]*\n)+\s{4}(#\[serde\(flatten\)\])'
    def replace3(match):
        serde_attr = match.group(2)
        return f'\n    #[schemars(description = "")]\n    {serde_attr}'
    content = re.sub(pattern3, replace3, content)
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    new_schemars_count = len(re.findall(r'#\[schemars\(description = ""\)\]', content))
    return original_doc_count, new_schemars_count

if __name__ == '__main__':
    filepath = sys.argv[1]
    original, new = process_file(filepath)
    print(f"Processed {filepath}: {original} doc comments -> {new} schemars attributes")
