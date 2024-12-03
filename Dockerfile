
FROM rust:1.82.0 AS compiler
WORKDIR /usr/src/app
COPY ./Cargo.toml ./
RUN cargo build --release
RUN rm -rf src/ target/release/deps
COPY ./src .

# Compila la aplicación en modo release
RUN cargo build --release

# -----------------------------------------
# --- Etapa 2

FROM alpine:3.20
WORKDIR /usr/src/app
COPY --from=compiler /usr/src/app/target/release/kata-barcode .
EXPOSE 8080
ENV TZ=Europe/Madrid
RUN apk update & apk upgrade -y
RUN apk add --no-cache tzdata  # Instala tzdata en Alpine
CMD ["./kata-barcode",  "serve"]