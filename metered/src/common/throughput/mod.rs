//! A module providing the `Throughput` metric.

use crate::clear::Clear;
use crate::metric::Metric;
use crate::time_source::{Instant, StdInstant};
use aspect::{Advice, Enter, OnResult};
use serde::Serialize;

mod atomic_tps;
mod tx_per_sec;

pub use atomic_tps::AtomicTxPerSec;
pub use tx_per_sec::TxPerSec;

/// A metric providing a transaction per second count backed by an histogram.
///
/// Because it retrieves the current time before calling the expression, stores it to appropriatly build time windows of 1 second and registers results to an histogram, this is a rather heavy-weight metric better applied at entry-points.
///
/// By default, `Throughput` uses an atomic transaction count backend and a synchronized time source, which work better in multithread scenarios. Non-threaded applications can gain performance by using unsynchronized structures instead.
#[derive(Clone, Debug, Serialize)]
pub struct Throughput<T: Instant = StdInstant, P: RecordThroughput = AtomicTxPerSec<T>>(
    P,
    std::marker::PhantomData<T>,
);

pub trait RecordThroughput: Default {
    fn on_result(&self);
}

impl<P: RecordThroughput, T: Instant> Default for Throughput<T, P> {
    fn default() -> Self {
        Throughput(P::default(), std::marker::PhantomData)
    }
}

impl<P: RecordThroughput + Serialize + Clear, T: Instant, R> Metric<R> for Throughput<T, P> {}

impl<P: RecordThroughput, T: Instant> Enter for Throughput<T, P> {
    type E = ();

    fn enter(&self) {}
}

impl<P: RecordThroughput + Clear, T: Instant> Clear for Throughput<T, P> {
    fn clear(&self) {
        self.0.clear();
    }
}

impl<P: RecordThroughput + Serialize, T: Instant, R> OnResult<R> for Throughput<T, P> {
    fn on_result(&self, _enter: (), _: &R) -> Advice {
        self.0.on_result();
        Advice::Return
    }
}
