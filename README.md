# yaml_rs

The Python module is called `yaml_rs` and is installable via `pip`. It is a faster alternative to the `PyYAML` module. Parsing is up to **7.6 times faster** and saving up to **10.6 times** (see [benchmarks](https://github.com/brmmm3/yaml-rs/doc/benchmarks.md)).  
As `yaml_rs` releases the GIL it's performance is even better compared to `PyYAML`.

`yaml_rs` is just a thin Python layer around the `saphyr` Rust crate, which is a fast ad fully YAML 1.2 compliant parser and generator.

## Installation

For building this wheel from source you need the tool `maturin`.

Install `maturin`:

```sh
cargo install maturin
```

On Linux further docker is needed for building manylinux wheels.

**Build wheel:**

First create the docker image:

```sh
create_docker_image.sh
```

Then build the wheels with docker (on Linux):

```sh
build_wheels_with_docker.sh
```

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
