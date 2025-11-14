#!/bin/sh

docker run --rm -v $(pwd):/io -w /io --entrypoint /bin/sh ghcr.io/pyo3/maturin:latest -c "
  for PYDIR in cp312-cp312; do
    maturin build --release --manylinux 2014 --interpreter /opt/python/\$PYDIR/bin/python -o dist/
  done
"
