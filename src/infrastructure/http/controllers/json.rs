use salvo::{Request, Response};
use crate::domain::inbound::generation::GenerationRequestDto;

pub async fn json_handler(req: &mut Request, res: &mut Response)
{
  let body = req.parse_json::<GenerationRequestDto>().await;
}
