
FROM ubuntu:25.04 AS compiler
RUN apt update; apt upgrade -y
RUN apt install -y curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    gcc \
    g++ \
    make
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /usr/src/app
COPY ./Cargo.toml ./
COPY src/ ./src

# Compila la aplicación en modo release
RUN cargo build --release

# -----------------------------------------
# --- Etapa 2

FROM ubuntu:25.04
WORKDIR /usr/src/app
COPY --from=compiler /usr/src/app/target/release/kata-barcode .
EXPOSE 8080
ENV TZ=Europe/Madrid
RUN apt update & apt upgrade -y
RUN ln -fs /usr/share/zoneinfo/$TZ /etc/localtime 
CMD ["./kata-barcode",  "serve"]

