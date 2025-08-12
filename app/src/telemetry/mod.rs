use crate::built_info;
use crate::telemetry::telemetry_guard::TelemetryGuard;
use foundation::NumberDurationExt;
use opentelemetry::trace::TracerProvider;
use opentelemetry::{KeyValue, global};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{Compression, LogExporter, MetricExporter, SpanExporter, WithTonicConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::{
    MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Temporality,
};
use opentelemetry_sdk::trace::{RandomIdGenerator, SdkTracerProvider};
use opentelemetry_semantic_conventions::attribute;
use tracing::Level;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

mod telemetry_guard;

fn resource() -> Resource {
    Resource::builder()
        .with_service_name(built_info::PKG_NAME)
        .with_attribute(KeyValue::new(
            attribute::SERVICE_VERSION,
            built_info::PKG_VERSION,
        ))
        .build()
}

fn init_logger_provider() -> SdkLoggerProvider {
    SdkLoggerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(
            LogExporter::builder()
                .with_tonic()
                .with_compression(Compression::Zstd)
                .build()
                .expect("Building a log exporter"),
        )
        .build()
}

fn init_tracer_provider() -> SdkTracerProvider {
    let tracer_provider = SdkTracerProvider::builder()
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(
            SpanExporter::builder()
                .with_tonic()
                .with_compression(Compression::Zstd)
                .build()
                .expect("Building a span exporter"),
        )
        .build();

    global::set_tracer_provider(tracer_provider.clone());

    tracer_provider
}

fn init_meter_provider() -> SdkMeterProvider {
    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(
            PeriodicReader::builder(
                MetricExporter::builder()
                    .with_tonic()
                    .with_compression(Compression::Zstd)
                    .with_temporality(Temporality::Cumulative)
                    .build()
                    .expect("Building a metric exporter"),
            )
            .with_interval(10.seconds())
            .build(),
        )
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

fn env_filter() -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(
            if built_info::DEBUG {
                Level::DEBUG
            } else {
                Level::INFO
            }
            .into(),
        )
        .from_env_lossy()
        // Prevent telemetry from generating telemetry
        // https://github.com/open-telemetry/opentelemetry-rust/issues/2877
        .add_directive(
            "hyper=off"
                .parse()
                .expect("hard-coded default directive should be valid"),
        )
        .add_directive(
            "tonic=off"
                .parse()
                .expect("hard-coded default directive should be valid"),
        )
        .add_directive(
            "h2=off"
                .parse()
                .expect("hard-coded default directive should be valid"),
        )
        .add_directive(
            "reqwest=off"
                .parse()
                .expect("hard-coded default directive should be valid"),
        )
}

pub fn init_telemetry() -> TelemetryGuard {
    let logger_provider = init_logger_provider();
    let tracer_provider = init_tracer_provider();
    let meter_provider = init_meter_provider();

    tracing_subscriber::registry()
        // Write logs to stdout
        .with(tracing_subscriber::fmt::layer().with_filter(env_filter()))
        // Export metrics to otel
        .with(MetricsLayer::new(meter_provider.clone()).with_filter(env_filter()))
        // Export spans with log events to otel
        .with(
            OpenTelemetryLayer::new(tracer_provider.tracer("tracing-otel-subscriber"))
                .with_filter(env_filter()),
        )
        // Export logs to otel
        .with(OpenTelemetryTracingBridge::new(&logger_provider).with_filter(env_filter()))
        .init();

    TelemetryGuard {
        logger_provider,
        tracer_provider,
        meter_provider,
    }
}
