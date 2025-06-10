# Human-Readable Error Improvements

## Summary

Enhanced the error handling in the `rep` grep implementation to provide more user-friendly and helpful error messages.

## Changes Made

### 1. Enhanced Error Messages (src/error.rs)
- Added emoji indicators (❌ for errors, 💡 for tips)
- Included helpful tips for each error type
- Provided specific guidance on how to fix common issues
- Added a new `Help` variant for displaying help without error formatting

### 2. Improved Argument Parsing (src/args.rs)
- Enhanced usage instructions with detailed options and examples
- Added specific error messages for different missing argument scenarios
- Implemented help flag support (--help, -h)
- Clearer formatting with sections for options and examples

### 3. Better File Operation Errors (src/file_ops.rs)
- Distinguished between different types of IO errors (not found, permission denied, etc.)
- Added detection for specific filenames vs glob patterns
- More descriptive error messages for glob pattern failures

### 4. Enhanced Main Error Display (src/main.rs)
- Added blank line before errors for better visibility
- Errors are displayed on stderr as expected

## Examples of Improved Error Messages

### No Arguments
```
❌ Invalid arguments: Missing search pattern and file(s)

Usage: rep [OPTIONS] <PATTERN> <FILE(S)...>

Options:
  -n    Show line numbers
  -i    Case-insensitive search
  -c    Count matches only
  -v    Invert match (show non-matching lines)
  -w    Match whole words only
  -V    Verbose mode

Examples:
  rep "hello" file.txt
  rep -n "error" *.log
  rep -i "TODO" src/*.rs

💡 Tip: Use 'rep --help' to see usage instructions
```

### File Not Found
```
❌ Cannot find file: 'nonexistent.txt'

💡 Tips:
  • Check if the file path is correct
  • Make sure you have read permissions
  • Use quotes for paths with spaces
```

### No Files Matched Pattern
```
❌ No files found matching your pattern

💡 Tips:
  • Check if the files exist in the current directory
  • Try using a simpler pattern (e.g., *.txt instead of complex globs)
  • Use 'ls' or 'dir' to see available files
  • Ensure you're in the correct directory
```

### Invalid Glob Pattern
```
❌ Invalid file pattern: '[unclosed' is not a valid file pattern: Pattern syntax error near position 0: invalid range pattern

💡 Tips:
  • Use * to match multiple characters (e.g., *.txt)
  • Use ? to match a single character
  • Use [abc] to match any of a, b, or c
  • Escape special characters with \
```

## Benefits

1. **Better User Experience**: Users get immediate guidance on how to fix issues
2. **Reduced Frustration**: Clear error messages prevent users from guessing what went wrong
3. **Educational**: Tips help users learn proper usage patterns
4. **Visual Clarity**: Emojis and formatting make errors stand out and easier to read
5. **Help Integration**: --help flag provides clean usage information without error formatting