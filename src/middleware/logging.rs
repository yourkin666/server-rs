//! 请求日志中间件
//!
//! 高性能的请求日志记录，包含请求追踪

use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{info_span, Instrument};
use uuid::Uuid;

/// 请求日志中间件
///
/// 记录每个请求的基本信息和响应时间
pub async fn request_logging(request: Request, next: Next) -> Response {
    let start_time = Instant::now();
    let request_id = Uuid::new_v4();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();

    // 获取客户端 IP (简化版本)
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    // 创建带有请求ID的span，用于分布式追踪
    let span = info_span!(
        "request",
        method = %method,
        uri = %uri,
        version = ?version,
        request_id = %request_id,
        client_ip = %client_ip,
    );

    // 在span中执行请求处理
    async move {
        tracing::info!("请求开始");

        let response = next.run(request).await;

        let elapsed = start_time.elapsed();
        let status = response.status();

        tracing::info!(
            status = %status,
            elapsed_ms = elapsed.as_millis(),
            "请求完成"
        );

        response
    }
    .instrument(span)
    .await
}

/// 创建带有请求ID的响应头中间件
pub async fn add_request_id(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();

    // 将请求ID添加到请求扩展中，以便在处理器中使用
    request.extensions_mut().insert(request_id.clone());

    let mut response = next.run(request).await;

    // 将请求ID添加到响应头中
    response.headers_mut().insert(
        "x-request-id",
        request_id
            .parse()
            .expect("Request ID (UUID) should be a valid header value"),
    );

    response
}
