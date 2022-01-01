# Dir Renamer

It's a CLI tool to rename every file in a directory following `patern number . extension`. It preserves the actual order of the files and with the option `--ignore-extensions` or `-i` file extensions will be indexed separately, it's useful to rename videos and it's subtitle files in order or whatever.

## Parameters

- `--dir` or `-d`: path of the directory to order.
- `--name` or `-n`: pattern to rename following `name number.extension`, for example `-n file` outputs `file1.txt`.
- `--ignore-extensions` or `-i`: by default it indexes file extensions separetely, so enable this to prevent that behaviour.

> `-` and `--` can be writen indistinctly. For example: `-dir`, `--d`, `--dir`, `-d` are read as the same.
