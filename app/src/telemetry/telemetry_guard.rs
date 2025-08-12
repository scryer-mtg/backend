use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;

pub struct TelemetryGuard {
    pub(super) logger_provider: SdkLoggerProvider,
    pub(super) tracer_provider: SdkTracerProvider,
    pub(super) meter_provider: SdkMeterProvider,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Err(e) = self.meter_provider.shutdown() {
            eprintln!("{e:?}")
        }
        if let Err(e) = self.tracer_provider.shutdown() {
            eprintln!("{e:?}")
        }
        if let Err(e) = self.logger_provider.shutdown() {
            eprintln!("{e:?}")
        }
    }
}
