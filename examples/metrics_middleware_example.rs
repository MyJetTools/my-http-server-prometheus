use std::{net::SocketAddr, sync::Arc, time::Duration};

use app::AppContext;
use my_http_server::MyHttpServer;
use my_http_server_prometheus::metrics_middleware::middleware::MetricsMiddleware;

mod app;

#[tokio::main]
async fn main() {
    let app = AppContext::new();
    let app = Arc::new(app);

    let mut http_server: MyHttpServer = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 5000)));

    http_server.add_middleware(Arc::new(MetricsMiddleware {}));

    http_server.start(app);

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
