use crate::core::generators::barcodes::proxy::BarcodeGeneratorProxy;
use crate::domain::inbound::generation::GenerationRequestDto;
use crate::domain::outbound::error::ErrorResponseDto;
use crate::domain::outbound::generation::GenerationResponseDto;
use crate::shared::checkers::{check_barcode, check_img};
use crate::shared::crypto::{check_sign, encode_64, generate_sign};
use log::{info, warn};
use salvo::prelude::Json;
use salvo::{handler, http::StatusCode, Request, Response};

// TODO: Check signs
#[handler]
pub async fn json_handler(req: &mut Request, res: &mut Response) {
    let body_result: Result<GenerationRequestDto, salvo::http::ParseError> =
        req.parse_json::<GenerationRequestDto>().await;
    let barcode_option: Option<String> = req.param::<String>("code");
    let body: GenerationRequestDto = match body_result {
        Err(_) => {
            let response = raise_error_valid("body".to_string());
            res.status_code(StatusCode::BAD_REQUEST)
                .render(Json(response));
            return;
        }
        Ok(b) => b,
    };
    // fields check
    info!("Checking the body data");
    let data = match body.data {
        Some(d) => {
            info!("Body.data: OK");
            d
        }
        None => {
            warn!("No data in body");
            let response = raise_error_valid("data".to_string());
            res.render(Json(response));
            return;
        }
    };
    let sign = match body.sign {
        Some(s) => {
            info!("Body.sign: OK");
            s
        }
        None => {
            warn!("No sign in body");
            let response = raise_error_valid("sign".to_string());
            res.render(Json(response));
            return;
        }
    };
    let image: String = match body.image {
        Some(i) => {
            info!("Body.image: OK");
            i
        }
        None => {
            warn!("No image in body");
            let response = raise_error_valid("image".to_string());
            res.render(Json(response));
            return;
        }
    };
    let height: u32 = match body.height {
        Some(i) => {
            info!("Body.height: OK");
            i
        },
        None => {
            warn!("No height in body");
            let response = raise_error_valid("height".to_string());
            res.status_code(StatusCode::BAD_REQUEST)
                .render(Json(response));
            return;
        }
    };
    let sign_str = format!("{}.{}", data, image);
    info!("String chain created {}", sign_str);
    let check = check_sign(sign, sign_str);
    info!("Signature: {}", check.unwrap());
    if !check_barcode(barcode_option.clone().unwrap()) {
        let response = raise_error_valid("code".to_string());
        res.status_code(StatusCode::BAD_REQUEST)
            .render(Json(response));
        return;
    }
    if !check_img(image.clone()) {
        let response = raise_error_valid("image".to_string());
        res.status_code(StatusCode::BAD_REQUEST)
            .render(Json(response));
        return;
    }
    let generator = BarcodeGeneratorProxy::new(&barcode_option.clone().unwrap());
    let out_img_result = generator.generate(data.as_str(), height, image.as_str());
    let out_img = match out_img_result {
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
                .render(Json(ErrorResponseDto::new(&e, 0x1005)));
            return;
        },
        Ok(bytes) => bytes
    };
    let out_img_str = encode_64(out_img);
    let out_sign = generate_sign(out_img_str.clone()).unwrap();
    let response = GenerationResponseDto::new(&out_img_str, out_sign.as_str());
    res.render(Json(response));
}

fn raise_error_valid(field: String) -> ErrorResponseDto {
    let message = format!("Invalid value for {}", field);
    ErrorResponseDto::new(message.as_str(), 400u32)
}
