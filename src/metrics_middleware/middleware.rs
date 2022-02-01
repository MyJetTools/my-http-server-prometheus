use crate::metrics::{MyMetrics, MyMetricsFactory};
use async_trait::async_trait;
use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
    WebContentType,
};

const METRICS_END_POINT: &str = "/metrics";

pub struct MetricsMiddleware {}

impl MetricsMiddleware {
    pub fn new() -> MetricsMiddleware {
        Self {}
    }
}

#[async_trait]
impl HttpServerMiddleware for MetricsMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let metrics = MyMetrics {};
        let path = ctx.request.get_path_lower_case();
        if path == METRICS_END_POINT {
            let buffer = metrics.get_my_metrics();
            return Ok(HttpOkResult::Content {
                content_type: Some(WebContentType::Text),
                content: buffer,
            });
        }
        let histogram = metrics
            .create_duration_timer(ctx.request.get_method().as_ref(), ctx.request.get_path());

        if !path.starts_with(METRICS_END_POINT) {
            let result = get_next.next(ctx).await;
            histogram.observe_duration();
            return result;
        }

        let result = get_next.next(ctx).await;
        histogram.observe_duration();
        result
    }
}
