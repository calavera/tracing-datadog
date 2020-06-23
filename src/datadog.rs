use crate::data::Visitor;
use datadog_apm::{Client, Config, Span as DdSpan, Trace};
use tracing::span::Id;
use tracing_distributed::{Event, Span, Telemetry};
/// Telemetry capability that publishes events and spans to datadog.com.
#[derive(Debug)]
pub struct DatadogTelemetry {
    client: Client,
}

impl DatadogTelemetry {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            client: Client::new(config),
        }
    }

    fn client(&self) -> Client {
        self.client.clone()
    }
}

impl Telemetry for DatadogTelemetry {
    type Visitor = Visitor;
    type TraceId = Id;
    type SpanId = Id;

    fn mk_visitor(&self) -> Self::Visitor {
        Default::default()
    }

    fn report_span(&self, span: Span<Self::Visitor, Self::SpanId, Self::TraceId>) {
        let trace = span_to_trace(span);
        self.client().send_trace(trace);
    }

    fn report_event(&self, _event: Event<Self::Visitor, Self::SpanId, Self::TraceId>) {
        todo!("datadog-apm-rust doesn't support events")
    }
}

fn span_to_trace(span: Span<Visitor, Id, Id>) -> Trace {
    Trace {
        id: span.trace_id.into_u64(),
        priority: 1,
        spans: vec![span_to_datadog_data(&span)],
    }
}

fn span_to_datadog_data(span: &Span<Visitor, Id, Id>) -> DdSpan {
    let parent_id = span.parent_id.clone().map(|p| p.into_u64());
    let duration = span
        .completed_at
        .duration_since(span.initialized_at)
        .unwrap_or_default();

    let mut tags = span.values.0.clone();
    tags.insert("span.level".to_string(), span.meta.level().to_string());
    tags.insert("span.target".to_string(), span.meta.target().to_string());

    let name = span.meta.name().to_string();

    DdSpan {
        id: span.id.into_u64(),
        resource: name.clone(),
        r#type: "custom".to_string(),
        start: span.initialized_at,
        error: None,
        http: None,
        sql: None,
        name,
        parent_id,
        duration,
        tags,
    }
}
