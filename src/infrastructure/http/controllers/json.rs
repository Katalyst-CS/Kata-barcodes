use crate::domain::inbound::generation::GenerationRequestDto;
use crate::domain::outbound::error::ErrorResponseDto;
use crate::shared::crypto::check_sign;
use log::info;
use salvo::prelude::Json;
use salvo::{handler, http::StatusCode, Request, Response};

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
    let data = match body.data {
        Some(d) => d,
        None => {
            let response = raise_error_valid("data".to_string());
            res.render(Json(response));
            return;
        }
    };
    let sign = match body.sign {
        Some(s) => s,
        None => {
            let response = raise_error_valid("sign".to_string());
            res.render(Json(response));
            return;
        }
    };
    let image: String = match body.image {
        Some(i) => i,
        None => {
            let response = raise_error_valid("image".to_string());
            res.render(Json(response));
            return;
        }
    };
    let sign_str = format!("{}.{}", data, image);
    info!("String chain created {}", sign_str);
    let check = check_sign(sign, sign_str);
    info!("Signature: {}", check.unwrap());

    res.render(Json(vec!["hola", "world"]));
}

fn raise_error_valid(field: String) -> ErrorResponseDto {
    let message = format!("Invalid value for {}", field);
    ErrorResponseDto::new(message.as_str(), 400u32)
}
