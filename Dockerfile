FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    build-essential cmake git libssl-dev wget pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root
RUN git clone --branch main https://github.com/open-quantum-safe/liboqs.git
WORKDIR /root/liboqs
RUN mkdir build && cd build && \
    cmake -GNinja -DOQS_USE_OPENSSL=ON .. && \
    ninja && ninja install

# Platzhalter für deinen Provider-Build
# (In der Realität würde hier dein Repo geklont werden)
# COPY . /root/my_provider

ENV LD_LIBRARY_PATH="/usr/local/lib"

# Der Beweis-Befehl
CMD ["openssl", "s_client", "-connect", "localhost:4433", "-groups", "MLKEM1024-P384-HYBRID"]

