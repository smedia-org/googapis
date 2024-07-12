/// An event signifying a Compute Engine resource is impacted by the disaster
/// recovery.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisasterRecoveryEvent {
    /// The severity level.
    #[prost(enumeration = "disaster_recovery_event::Severity", optional, tag = "1")]
    pub severity: ::core::option::Option<i32>,
    /// Details about the impact on the Compute Engine resource, e.g. "the resource
    /// is deleted during the disaster recovery".
    #[prost(string, optional, tag = "2")]
    pub details: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `DisasterRecoveryEvent`.
pub mod disaster_recovery_event {
    /// The severity of the disaster recovery event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Unspecified.
        Unspecified = 0,
        /// The Compute Engine resource is broken. A person must take an action.
        ActionRequired = 1,
        /// The Compute Engine resource is functioning. A change was applied to the
        /// resource during disaster recovery. Please take action to review
        /// the change to avoid unexpected problems.
        ActionSuggested = 2,
        /// Normal maintenance opeartions during disaster recovery, such as start up,
        /// shut down.
        Notice = 3,
    }
}
