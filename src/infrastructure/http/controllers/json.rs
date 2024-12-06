use salvo::{Request, Response};

pub fn json_handler(req: &mut Request, res: &mut Response)
{
  let body = req.body();
}