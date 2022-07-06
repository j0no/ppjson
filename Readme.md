# ppjson - please print json

small cli tool to print json file objects

## Usage

```bash
$ ppjson --help
USAGE:
    ppjson [OPTIONS] [ARGS]

ARGS:
    <INPUT_JSON_FILE>    a json file
    <KEY>                key

OPTIONS:
    -h, --help       Print help information
    -t               format as table
    -V, --version    Print version information


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

## Build and Install

```bash
make build
```

```bash
make install # sudo if linux
```

