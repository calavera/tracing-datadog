#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]

//! This crate provides:
//! - A tracing layer, `TelemetryLayer`, that can be used to publish trace data to datadog.com
//! - Utilities for implementing distributed tracing against the datadog.com backend
//!
//! As a tracing layer, `TelemetryLayer` can be composed with other layers to provide stdout logging, filtering, etc.

mod data;
mod datadog;
pub use datadog::DatadogTelemetry;

use tracing::span::Id;
#[doc(no_inline)]
pub use tracing_distributed::{TelemetryLayer, TraceCtxError};

/// Register the current span as the local root of a distributed trace.
pub fn register_dist_tracing_root(
    trace_id: Id,
    remote_parent_span: Option<Id>,
) -> Result<(), TraceCtxError> {
    tracing_distributed::register_dist_tracing_root(trace_id, remote_parent_span)
}

/// Retrieve the distributed trace context associated with the current span.
///
/// Returns the `Id`, if any, that the current span is associated with along with
/// the `Id` belonging to the current span.
pub fn current_dist_trace_ctx() -> Result<(Id, Id), TraceCtxError> {
    tracing_distributed::current_dist_trace_ctx()
}

/// Construct a TelemetryLayer that does not publish telemetry to any backend.
pub fn new_blackhole_telemetry_layer(
) -> TelemetryLayer<tracing_distributed::BlackholeTelemetry<Id, Id>, Id, Id> {
    TelemetryLayer::new(
        "datadog_blackhole_tracing_layer",
        tracing_distributed::BlackholeTelemetry::default(),
        move |tracing_id| tracing_id,
    )
}

/// Construct a TelemetryLayer that publishes telemetry to datadog.com using the provided datadog config.
pub fn new_datadog_telemetry_layer(
    service_name: &'static str,
    config: datadog_apm::Config,
) -> TelemetryLayer<DatadogTelemetry, Id, Id> {
    TelemetryLayer::new(
        service_name,
        DatadogTelemetry::new(config),
        move |tracing_id| tracing_id,
    )
}
