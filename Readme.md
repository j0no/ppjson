# ppjson - please print json

small cli tool to print json file objects

## Usage

```bash
$ ppjson --help
USAGE:
    ppjson [OPTIONS] [--] [ARGS]

ARGS:
    <INPUT_JSON_FILE>    a json file
    <KEY>                key

OPTIONS:
    -d <DELIMETER>        set key delimeter
    -h, --help                 Print help information
    -k                         print keys
    -t                         format as table
    -V, --version              Print version information
```

### Example

```bash
$ ppjson package.json scripts
{
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  }
}
```

## Stack

- Rust
- [tabled](https://github.com/zhiburt/tabled) for table formatting 

## Build and Install

```bash
make build
```

```bash
make install # sudo if linux
```

You can add the following to .zshrc or .bashrc to print scripts be deafult for command `pp`

```bash
alias pp="ppjson package.json scripts"
```

