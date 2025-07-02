//! 性能指标中间件
//!
//! 收集HTTP请求的性能指标

use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

/// 性能指标中间件
///
/// 记录请求数量、响应时间等关键指标
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let start_time = Instant::now();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    // 执行请求
    let response = next.run(request).await;

    let elapsed = start_time.elapsed();
    let status_code = response.status().as_u16();

    // 记录指标 (这里使用简单的日志，后续可以集成 Prometheus)
    tracing::debug!(
        method = %method,
        path = %path,
        status_code = status_code,
        elapsed_ms = elapsed.as_millis(),
        "HTTP请求指标"
    );

    response
}
