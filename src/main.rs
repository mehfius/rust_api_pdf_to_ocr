use actix_web::{post, web, App, HttpResponse, HttpServer};
use base64::engine::general_purpose;
use base64::Engine;
use mupdf::{Document, Matrix, Colorspace};
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use std::io::Cursor;
use image::{DynamicImage, ImageFormat};
use std::env;

#[derive(Deserialize)]
struct PdfInput {
    url: String,
}

#[post("/pdf_to_ocr")]
async fn convert_pdf(input: Option<web::Json<PdfInput>>) -> HttpResponse {
    // Initialize HTTP client
    let client = Client::new();
    
    // Verifica se o JSON de entrada é válido
    let input = match input {
        Some(i) => i,
        None => {
            return HttpResponse::BadRequest().json(json!({
                "error": "JSON inválido ou ausente"
            }))
        }
    };

    // Baixa o PDF da URL fornecida (async)
    let response = match reqwest::get(&input.url).await {
        Ok(res) => res,
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({
                "error": format!("Falha ao baixar PDF: {}", e)
            }))
        }
    };

    let pdf_data = match response.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({
                "error": format!("Erro ao ler resposta: {}", e)
            }))
        }
    };

    // Carrega o PDF usando mupdf
    let doc = match Document::from_bytes(&pdf_data, "") {
        Ok(doc) => doc,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Não foi possível carregar o PDF: {}", e)
            }))
        }
    };

    let page_count = doc.pages().unwrap().count();
    let mut ocr_results = Vec::new();

    // Itera sobre todas as páginas do PDF
    for page_index in 0..page_count {
        let page = match doc.load_page(page_index as i32) {
            Ok(page) => page,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Erro ao carregar página {}: {}", page_index + 1, e)
                }))
            }
        };

        // Configura a matriz de transformação para renderização (escala 2x)
        let matrix = Matrix::new(2.0, 0.0, 0.0, 2.0, 0.0, 0.0);
        let colorspace = Colorspace::device_rgb();

        // Converte a página para pixmap
        let pixmap = match page.to_pixmap(&matrix, &colorspace, false, false) {
            Ok(pixmap) => pixmap,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Erro ao gerar pixmap da página {}: {}", page_index + 1, e)
                }))
            }
        };

        let width = pixmap.width() as u32;
        let height = pixmap.height() as u32;
        let samples = pixmap.samples().to_vec();

        // Cria imagem RGB a partir do pixmap
        let img = match image::RgbImage::from_raw(width, height, samples) {
            Some(img) => DynamicImage::ImageRgb8(img),
            None => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Falha ao criar imagem da página {}", page_index + 1)
                }))
            }
        };

        // Converte imagem para PNG na memória
        let mut buffer = Cursor::new(Vec::new());
        if let Err(e) = img.write_to(&mut buffer, ImageFormat::Png) {
            return HttpResponse::InternalServerError().json(json!({
                "error": format!("Erro ao salvar imagem da página {}: {}", page_index + 1, e)
            }));
        }

        // Codifica a imagem em Base64
        let base64 = general_purpose::STANDARD.encode(buffer.into_inner());

        let base_url = env::var("URL").unwrap_or_else(|_| "http://0.0.0.0:5000/ocr".to_string());

        let ocr_response = match client
            .post(base_url)
            .json(&json!({ "base64": base64 }))
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Erro ao enviar para OCR (página {}): {}", page_index + 1, e)
                }))
            }
        };

        // Coleta o resultado do OCR
        let ocr_result = match ocr_response.json::<serde_json::Value>().await {
            Ok(result) => result,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "error": format!("Erro ao processar resposta OCR (página {}): {}", page_index + 1, e)
                }))
            }
        };

        ocr_results.push(json!({
            "page": page_index + 1,
            "ocr_result": ocr_result
        }));
    }

    // Retorna os resultados do OCR e o número de páginas
    HttpResponse::Ok().json(json!({
        "results": ocr_results,
        "page_count": page_count
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor rodando em http://0.0.0.0:5001");
    HttpServer::new(|| App::new().service(convert_pdf))
        .bind("0.0.0.0:5001")?
        .run()
        .await
}