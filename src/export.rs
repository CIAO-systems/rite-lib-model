//! RITE exporter trait
use super::{BoxedError, Initializable, record::Record};

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Start = 0,
    End = 1,
}

impl Default for Signal {
    fn default() -> Self {
        Signal::Start
    }
}
/// The interface for RITE exporter components
pub trait Exporter: Initializable {
    /// Takes a [Record] and writes it
    ///
    fn write(&mut self, record: &Record) -> Result<(), BoxedError>;

    /// Event signaling function
    ///
    /// Exporters can utilize this, to collect records and process them at the
    /// end
    fn event(&mut self, #[allow(unused_variables)] signal: Signal) -> Result<(), BoxedError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Initializable,
        export::{Exporter, Signal},
    };

    #[test]
    fn test_signal_default() {
        let signal = Signal::default();
        assert_eq!(signal, Signal::Start);
    }

    #[test]
    fn test_exporter_defaults() {
        struct TestExporter;
        impl Initializable for TestExporter {
            fn init(
                &mut self,
                _config: Option<crate::xml::config::Configuration>,
            ) -> Result<(), crate::BoxedError> {
                Ok(())
            }
        }
        impl Exporter for TestExporter {
            fn write(&mut self, _record: &crate::record::Record) -> Result<(), crate::BoxedError> {
                Ok(())
            }
        }

        let mut exporter = TestExporter;
        let result = exporter.event(Signal::Start);
        assert!(result.is_ok());
    }
}
