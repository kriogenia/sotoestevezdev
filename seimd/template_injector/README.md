# SEimd into HTML

CLI tool to read SEimd ready HTML file and output the generated file with the injected markdown.

**Usage**: `template_injector [OPTIONS] --injectables <SEIMD_PATH> <INPUT_FILE>`

## Arguments:
<INPUT_FILE>  File to inject

## Options
```
-i, --injectables <SEIMD_PATH>  Path where the SEimd files are stored
--parent                        Wraps in parent if present in the file metadata
--legend                        Injects the legend if present in the metadata
-h, --help                      Print help
-V, --version                   Print version
```