FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    tesseract-ocr \
    tesseract-ocr-por \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY target/release/rust_api_pdf_to_ocr /usr/local/bin/

EXPOSE 5001

CMD ["rust_api_pdf_to_ocr"]