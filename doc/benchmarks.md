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
|  load  | deeper.yaml | 0.0051     | 0.0006      |  8.14  |
|  save  | deeper.yaml | 0.0059     | 0.0004      | 13.43  |
|  load  | 1mb.yaml    | 0.3081     | 0.0514      |  5.99  |
|  save  | 1mb.yaml    | 0.3160     | 0.0327      |  9.67  |
|  load  | 5mb.yaml    | 1.5586     | 0.2325      |  6.70  |
|  save  | 5mb.yaml    | 1.6097     | 0.1565      | 10.29  |
|  load  | 15mb.yaml   | 4.4366     | 0.7501      |  5.91  |
|  save  | 15mb.yaml   | 4.6395     | 0.4922      |  9.43  |

## Multithreaded loading and saving

A threapool with 8 threads is used to load and save the 10 15mb.yaml files in parallel.

| Method | PyYAML [s] | yaml_rs [s] | Factor |
|--------|------------|-------------|--------|
|  load  | 33 each    | 1.0 each    | 33     |
|  save  | 41 each    | 0.5 each    | 82     |
|  total | 190.5891   | 4.4974      | 42.38  |

yaml_rs releases the GIL. PyYAML seems not to release the GIL. See results in the table above.
This is another advantage of yaml_rs in multithreaded applications.
