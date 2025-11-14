# Benchmarks

Benchmarking code see [benches/benchmark.py](../benches/benchmark.py)

The benchmarks are compared with the well known `PyYAML` module using the optimized `CLoader` and `CDumper`.

## Single file loading and saving

| Method | File        | PyYAML [s] | yaml_rs [s] | Factor |
|--------|-------------|------------|-------------|--------|
|  load  | garden.yaml | 0.0006     | 0.0001      |  6.01  |
|  save  | garden.yaml | 0.0006     | 0.0001      |  5.38  |
|  load  | deep.yaml   | 0.0012     | 0.0002      |  7.30  |
|  save  | deep.yaml   | 0.0013     | 0.0002      |  8.33  |
|  load  | deeper.yaml | 0.0050     | 0.0006      |  7.65  |
|  save  | deeper.yaml | 0.0058     | 0.0005      | 10.61  |
|  load  | 1mb.yaml    | 0.2938     | 0.0522      |  5.62  |
|  save  | 1mb.yaml    | 0.3143     | 0.0259      |  8.75  |
|  load  | 5mb.yaml    | 1.5216     | 0.2439      |  6.24  |
|  save  | 5mb.yaml    | 1.5863     | 0.1777      |  8.93  |
|  load  | 15mb.yaml   | 4.4366     | 0.7501      |  5.91  |
|  save  | 15mb.yaml   | 4.6395     | 0.4922      |  9.43  |

## Multithreaded loading and saving

A threapool with 8 threads is used to load and save the 10 15mb.yaml files in parallel.

| Method | PyYAML [s] | yaml_rs [s] | Factor |
|--------|------------|-------------|--------|
|  load  | 33 each    | 1.5 each    | 22     |
|  save  | 41 each    | 0.6 each    | 68.3   |
|  total | 173.1455   | 5.7127      | 30.31  |

yaml_rs releases the GIL. PyYAML seems not to release the GIL. See results in the table above.
This is another advantage of yaml_rs in multithreaded applications.
