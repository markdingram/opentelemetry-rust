use crate::metrics::{
    AsyncRunner, BatchObserver, Descriptor, InstrumentKind, Meter, NumberKind, Result,
};

use super::{async_instrument::BatchObserverResult, SumObserver};

//use super::AsyncRunner;

/// Configuration for building a Batch Observer.
#[derive(Debug)]
pub struct BatchObserverBuilder<'a> {
    meter: &'a Meter,
    // runner: AsyncRunner,
    descriptors: Vec<Descriptor>,
}

impl<'a> BatchObserverBuilder<'a> {
    pub(crate) fn new(meter: &'a Meter) -> Self {
        BatchObserverBuilder {
            meter,
            // runner: AsyncRunner::Batch(Box::new(callback)),
            descriptors: Vec::default(),
        }
    }

    /// adds a sum observer with given name to the batch
    pub fn u64_sum_observer<T>(&mut self, name: T) -> Result<SumObserver<u64>>
    where
        T: Into<String>,
    {
        let descriptor = Descriptor::new(
            name.into(),
            self.meter.instrumentation_library().name,
            self.meter.instrumentation_library().version,
            InstrumentKind::Counter,
            NumberKind::U64,
        );
        self.descriptors.push(descriptor.clone());

        let instrument = self
            .meter
            .new_async_instrument(descriptor, AsyncRunner::None)?;

        Ok(SumObserver {
            instrument,
            _marker: std::marker::PhantomData,
        })
    }

    /// Creates a new batch observer.
    pub fn try_init<F>(self, callback: F) -> Result<BatchObserver>
    where
        F: Fn(BatchObserverResult) + Send + Sync + 'static,
    {
        let runner = AsyncRunner::Batch(Box::new(callback));
        let instrument = self
            .meter
            .new_async_instrument_batch(self.descriptors, runner)?;
        Ok(BatchObserver::new(instrument))
    }
}
