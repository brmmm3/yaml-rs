#!/bin/sh

docker run --rm -v $(pwd):/io -w /io --entrypoint /bin/sh ghcr.io/pyo3/maturin:latest -c "
  for PYDIR in cp310-cp310 cp311-cp311 cp312-cp312 cp313-cp313 cp314-cp314; do
    maturin build --release --manylinux 2014 --interpreter /opt/python/\$PYDIR/bin/python -o dist/
  done
"
