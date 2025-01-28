//! Log implementation using the UART.

use alloc::format;
use once_cell::sync::Lazy;
use tracing::{
	field::Visit,
	span::{Attributes, Id, Record},
	Collect, Event, Metadata,
};
use tracing_core::span::Current;
use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;
static mut SERIAL_PORT: Lazy<SerialPort> = Lazy::new(|| unsafe {
	let mut serial_port = SerialPort::new(SERIAL_IO_PORT);
	serial_port.init();
	serial_port
});

pub struct EmbeddedCollector {}

impl EmbeddedCollector {
	pub fn new() -> Self {
		Self {}
	}
}

impl Collect for EmbeddedCollector {
	fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
		true
	}

	fn new_span(&self, _span: &Attributes<'_>) -> Id {
		tracing::span::Id::from_u64(0xAAAA)
	}

	fn record(&self, _span: &Id, _values: &Record<'_>) {}

	fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

	fn event(&self, event: &Event<'_>) {
		let mut visitor = EmbeddedVisitor::new();
		event.record(&mut visitor);
	}

	fn enter(&self, _span: &Id) {}

	fn exit(&self, _span: &Id) {}

	fn current_span(&self) -> Current {
		Current::none()
	}
}

struct EmbeddedVisitor {}

impl EmbeddedVisitor {
	fn new() -> Self {
		Self {}
	}
}

impl Visit for EmbeddedVisitor {
	fn record_debug(
		&mut self,
		field: &tracing_core::Field,
		value: &dyn core::fmt::Debug,
	) {
		unsafe {
			if field.name() == "message" {
				for c in format!("{:?}\n", value).chars() {
					SERIAL_PORT.send(c as u8);
				}
			} else {
				for c in
					format!("unknown: {}: {:?}\n", field.name(), value).chars()
				{
					SERIAL_PORT.send(c as u8);
				}
			}
		}
	}
}
