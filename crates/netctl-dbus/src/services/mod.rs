pub mod hostnamed;
pub mod networkd;
pub mod resolved;

pub use hostnamed::{HostnamedOps, HostnamedService};
pub use networkd::{NetworkdOps, NetworkdService};
pub use resolved::{ResolvedOps, ResolvedService};
