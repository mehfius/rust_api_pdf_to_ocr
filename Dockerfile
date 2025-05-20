FROM rust:1.82 AS builder

RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-por \
    libtesseract-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/rust_api_pdf_to_ocr

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-por \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/rust_api_pdf_to_ocr/target/release/rust_api_pdf_to_ocr /usr/local/bin/

EXPOSE 5001

CMD ["rust_api_pdf_to_ocr"]