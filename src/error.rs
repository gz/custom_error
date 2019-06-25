use core::any::TypeId;
use core::fmt::{Debug, Display};

/// Copied from libstd for now
/// This will be in libcore again: (https://doc.rust-lang.org/nightly/src/std/error.rs.html#5)
pub trait Error: Debug + Display {
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
}
