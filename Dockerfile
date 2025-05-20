FROM debian:bullseye-slim

RUN echo "Verificando pacotes instalados:" \
    && dpkg -l | grep -E 'ca-certificates|libssl1.1|tesseract-ocr' || echo "Algum pacote não encontrado" \
    && ldd --version | head -n 1

COPY target/release/rust_api_pdf_to_ocr /usr/local/bin/

EXPOSE 5001

CMD ["rust_api_pdf_to_ocr"]