FROM ghcr.io/pyo3/maturin:latest

# 1. Install the tools we need
RUN yum update -y && \
    yum install -y curl gcc gcc-c++ make perl python3 python3-pip && \
    yum clean all

# 2. Install Rust (stable) via rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 3. Install toolchain
RUN rustup target add x86_64-unknown-linux-musl

# 3. Install maturin (with patchelf for auditwheel)
RUN pip3 install --no-cache-dir --upgrade pip setuptools setuptools_rust wheel tomli
RUN pip3 install --no-cache-dir "maturin[patchelf]"

# 4. Copy your project into the image
WORKDIR /src
COPY . .

CMD ["maturin", "build", "--release", "--target", "x86_64-unknown-linux-musl", "--out", "dist"]
