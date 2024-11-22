use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::i2c::{ErrorType, I2c};

/// `critical-section`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with a `critical-section` [`Mutex`]. A critical section is taken for
/// the entire duration of a transaction. This allows sharing a single bus across multiple threads (interrupt priority levels).
/// The downside is critical sections typically require globally disabling interrupts, so `CriticalSectionDevice` will likely
/// negatively impact real-time properties, such as interrupt latency. If you can, prefer using
/// [`RefCellDevice`](super::RefCellDevice) instead, which does not require taking critical sections.
pub struct CriticalSectionDevice<'a, T> {
    bus: &'a Mutex<RefCell<T>>,
}

impl<'a, T> CriticalSectionDevice<'a, T> {
    /// Create a new `CriticalSectionDevice`.
    #[inline]
    pub fn new(bus: &'a Mutex<RefCell<T>>) -> Self {
        Self { bus }
    }
}

impl<T> ErrorType for CriticalSectionDevice<'_, T>
where
    T: I2c,
{
    type Error = T::Error;
}

impl<T> I2c for CriticalSectionDevice<'_, T>
where
    T: I2c,
{
    #[inline]
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);
            bus.read(address, read)
        })
    }

    #[inline]
    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);
            bus.write(address, write)
        })
    }

    #[inline]
    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);
            bus.write_read(address, write, read)
        })
    }

    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            let bus = &mut *self.bus.borrow_ref_mut(cs);
            bus.transaction(address, operations)
        })
    }
}
