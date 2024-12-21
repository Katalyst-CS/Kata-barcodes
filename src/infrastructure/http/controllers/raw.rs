extern crate base64;
use crate::{
    core::generators::{
        barcodes::proxy::{barcode_list, BarcodeGeneratorProxy},
        images::proxy::ImageGeneratorProxy,
    },
    domain::outbound::error::ErrorResponseDto,
};
use salvo::{handler, http::StatusCode, writing::Json, Request, Response};

#[handler]
pub async fn index(req: &mut Request, res: &mut Response) {
    let default_height: u32 = match std::env::var("APP_HEIGHT") {
        Ok(val) => val.parse::<u32>().unwrap_or(60), // Intentamos convertir a u32, si falla, usamos 60
        Err(_) => 60, // Si no encontramos la variable de entorno, usamos 60
    };
    let barcode: Option<String> = req.param::<String>("code");
    let img_type: Option<String> = req.param::<String>("ext");
    let barcodes_type = barcode_list();
    let content = req.query::<String>("data");
    let height: u32 = req.query::<u32>("height").unwrap_or_else(|| default_height);
    println!("Barcode type: {}", barcode.clone().unwrap());
    println!("Image type: {}", img_type.clone().unwrap());
    println!("Data: {}", content.clone().unwrap());
    let data: Option<String> = match content {
        None => {
            res.status_code(StatusCode::BAD_REQUEST)
                .render(Json(ErrorResponseDto::new(
                    "Not found content for barcode",
                    0x1002,
                )));
            return;
        }
        Some(d) => Some(d),
    };
    if !barcodes_type.contains(&barcode.clone().unwrap()) {
        let response = ErrorResponseDto::new(
            format!("Unknow barcode type {}", barcode.unwrap()).as_str(),
            0x1001,
        );
        res.status_code(StatusCode::NOT_IMPLEMENTED)
            .render(Json(response));
        return;
    }

    let generator = match barcode.clone() {
        None => {
            res.status_code(StatusCode::BAD_REQUEST)
                .render(Json(ErrorResponseDto::new(
                    "Not found barcode type",
                    0x1003,
                )));
            return;
        }
        Some(b) => BarcodeGeneratorProxy::new(&b),
    };
    match img_type.clone() {
        None => {
            res.status_code(StatusCode::BAD_REQUEST)
                .render(Json(ErrorResponseDto::new("Not found image type", 0x1003)));
            return;
        }
        Some(_) => 0,
    };
    let itypes = ImageGeneratorProxy::list();
    if !itypes.contains(&img_type.clone().unwrap().as_str()) {
        let response: ErrorResponseDto = ErrorResponseDto::new(
            format!("Unknow image type {}", img_type.clone().unwrap()).as_str(),
            0x1004,
        );
        res.status_code(StatusCode::NOT_IMPLEMENTED)
            .render(Json(response));
        return;
    }

    let out_img = generator.generate(
        data.unwrap().as_str(),
        height,
        &img_type.clone().unwrap().as_str(),
    );
    let out: Vec<u8> = match out_img {
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST)
                .render(Json(ErrorResponseDto::new(&e, 0x1005)));
            return;
        }
        Ok(bytes) => bytes,
    };
    res.headers_mut().insert(
        "Content-Type",
        format!("image/{}", img_type.unwrap()).parse().unwrap(),
    );
    res.body(out);
}
