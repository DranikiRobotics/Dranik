#[derive(Debug)]
pub(crate) struct Telemetry;

impl ::dranik_api::Telemetry for Telemetry {
    fn send(&self, _message: dranik_api::TelemetryMessage) {
        todo!()
    }
}
