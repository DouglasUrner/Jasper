//! System Console

//-----------------------------------------------------------------------------
// Public Definitions
//-----------------------------------------------------------------------------

/// Console Interface.
pub mod interface {
    /// Console write
    ///
    /// 'core::fmt::Write' will do for now. Re-export it here because
    /// implementing  'consale::Write' is a clearer statement of intention.
    pub use core::fmt::Write;
}
