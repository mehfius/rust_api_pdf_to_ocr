# PDF to OCR Service

A high-performance OCR (Optical Character Recognition) service built with Rust that converts PDF documents to text. This service is particularly optimized for Portuguese language text recognition.

## Features

- Fast and efficient PDF processing using MuPDF
- OCR processing using Tesseract
- Optimized for Portuguese language text recognition
- Clean text output with control character removal
- Processing time measurement
- RESTful API endpoint
- Docker support for easy deployment

## Prerequisites

- Rust (latest stable version)
- Tesseract OCR engine
- Portuguese language data for Tesseract
- MuPDF library

## Installation

### Option 1: Local Installation

1. Install Tesseract OCR and MuPDF:
```bash
# Ubuntu/Debian
sudo apt-get install tesseract-ocr
sudo apt-get install tesseract-ocr-por
sudo apt-get install libmupdf-dev

# macOS
brew install tesseract
brew install tesseract-lang
brew install mupdf
```

2. Clone the repository:
```bash
git clone https://github.com/mehfius/rust_api_pdf_to_ocr.git
cd rust_api_pdf_to_ocr
```

3. Build the project:
```bash
cargo build --release
```

### Option 2: Docker Installation

1. Build the Docker image:
```bash
docker build -t rust-api-pdf-to-ocr .
```

2. Run the container:
```bash
docker run -p 5001:5001 rust-api-pdf-to-ocr
```

## Usage

### Local Usage

1. Start the server:
```bash
cargo run --release
```

The server will start at `http://127.0.0.1:5001`

### Docker Usage

The service will be available at `http://127.0.0.1:5001` after running the container.

### API Usage

Send a POST request to the `/pdf_to_ocr` endpoint with a JSON payload containing the PDF URL:
```json
{
    "url": "https://example.com/path/to/document.pdf"
}
```

Example using curl:
```bash
curl -X POST http://127.0.0.1:5001/pdf_to_ocr \
     -H "Content-Type: application/json" \
     -d '{"url": "https://example.com/path/to/document.pdf"}'
```

## Response Format

Successful response:
```json
{
    "results": [
        {
            "page": 1,
            "ocr_result": {
                "text": "extracted text from page 1"
            }
        },
        {
            "page": 2,
            "ocr_result": {
                "text": "extracted text from page 2"
            }
        }
    ],
    "page_count": 2
}
```

Error response:
```json
{
    "error": "error message"
}
```

## Dependencies

- actix-web = "4.9"
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"
- image = "0.25.6"
- base64 = "0.21"
- reqwest = { version = "0.11", features = ["json"] }
- mupdf = "0.5.0"

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 