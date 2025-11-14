# yaml_rs

The Python module is called `yaml_rs` and installable via `pip`. It is a faster alternative to the `pyyaml` module with higher speed. Parsing is up to **57 times faster**  and saving sometimes faster, sometimes slower (see [benchmarks](https://github.com/brmmm3/yaml-rs/doc/benchmarks.md)).  
It releases the GIL.

## Installation

For building this wheel from source you need the tool `maturin`.

Install `maturin`:

```sh
cargo install maturin
```

IMPORTANT: In order to build this project at least Rust version 1.61 is needed!

**Build wheel:**

Build wheel (on Linux):

```sh
maturin build --release --strip
```

Build wheel on Windows:

```sh
maturin build --release --strip --no-sdist
```

``maturin`` will build the wheels for all Python versions installed on your system.

Alternatively you can use the build script `build_wheels.py`. The precondition to run this script is to have `pyenv` installed.
The script can build the wheel for specific Python versions or for all Python versions installed by `pyenv`.
In addition it runs ``pytest`` after successfull creation of each wheel.

```sh
python build_wheels.py
```

By default the script will build the wheel for the current Python interpreter.
If you want to build the wheel for specific Python version(s) by providing the argument `--versions`.

```sh
python build_wheels.py --versions 3.11.8,3.12.2
```

To build the wheel for all installed Python versions:

```sh
python build_wheels.py --versions *
```

Instruction how to install ``pyenv`` can be found [here](https://github.com/pyenv/pyenv).

## Examples

Load YAML file:

```python
import yaml_rs

data = yaml_rs.load("filename.yaml")
```

Save YAML file:

```python
import yaml_rs

yaml_rs.save("filename.yaml", data)
```

Load YAML data from string:

```python
import yaml_rs

data = yaml_rs.from_string(yaml_data)
```

Load YAML data from byte string:

```python
import yaml_rs

data = yaml_rs.from_string(yaml_byte_data)
```
