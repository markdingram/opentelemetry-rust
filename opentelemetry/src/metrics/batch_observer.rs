use crate::metrics::{
    async_instrument::BatchObserverResult, AsyncRunner, BatchObserver, Descriptor, InstrumentKind,
    Meter, NumberKind, Result,
};

//use super::AsyncRunner;

/// Configuration for building a Batch Observer.
#[derive(Debug)]
pub struct BatchObserverBuilder<'a> {
    meter: &'a Meter,
    runner: AsyncRunner,
    descriptors: Vec<Descriptor>,
}

impl<'a> BatchObserverBuilder<'a> {
    pub(crate) fn new<F>(meter: &'a Meter, callback: F) -> Self
    where
        F: Fn(BatchObserverResult) + Send + Sync + 'static,
    {
        BatchObserverBuilder {
            meter,
            runner: AsyncRunner::Batch(Box::new(callback)),
            descriptors: Vec::default(),
        }
    }

    /// adds a sum observer with given name to the batch
    pub fn u64_sum_observer<T>(&mut self, name: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.descriptors.push(Descriptor::new(
            name.into(),
            self.meter.instrumentation_library().name,
            self.meter.instrumentation_library().version,
            InstrumentKind::Counter,
            NumberKind::U64,
        ));
        Ok(())
    }

    /// Creates a new batch observer.
    pub fn try_init(self) -> Result<BatchObserver> {
        let instrument = self
            .meter
            .new_async_instrument_batch(self.descriptors, self.runner)?;
        Ok(BatchObserver::new(instrument))
    }
}
