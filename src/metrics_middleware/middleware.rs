use async_trait::async_trait;
use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, MiddleWareResult,
    WebContentType,
};

use crate::metrics::{MyMetrics, MyMetricsFactory};

const METRICS_END_POINT: &str = "/metrics";

pub struct MetricsMiddleware {}

impl MetricsMiddleware {
    fn new() -> MetricsMiddleware {
        Self {}
    }
}

#[async_trait]
impl HttpServerMiddleware for MetricsMiddleware {
    async fn handle_request(&self, ctx: HttpContext) -> Result<MiddleWareResult, HttpFailResult> {
        let metrics = MyMetrics {};
        let histogram =
            metrics.create_duration_timer(ctx.get_method().as_ref(), ctx.req.uri().path());
        let path = ctx.get_path_lower_case();

        if !path.starts_with(METRICS_END_POINT) {
            histogram.observe_duration(); //todo Подумать куда вынести конец подсчета и как это сделать
            return Ok(MiddleWareResult::Next(ctx));
        }

        if path == METRICS_END_POINT {
            let buffer = metrics.get_my_metrics();
            return Ok(MiddleWareResult::Ok(HttpOkResult::Content {
                content_type: Some(WebContentType::Text),
                content: buffer,
            }));
        }

        return Ok(MiddleWareResult::Next(ctx));
    }
}
