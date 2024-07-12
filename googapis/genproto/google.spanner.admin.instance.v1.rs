/// Encapsulates progress related information for a Cloud Spanner long
/// running instance operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationProgress {
    /// Percent completion of the operation.
    /// Values are between 0 and 100 inclusive.
    #[prost(int32, tag = "1")]
    pub progress_percent: i32,
    /// Time the request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation failed or was completed
    /// successfully.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Indicates the expected fulfillment period of an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FulfillmentPeriod {
    /// Not specified.
    Unspecified = 0,
    /// Normal fulfillment period. The operation is expected to complete within
    /// minutes.
    Normal = 1,
    /// Extended fulfillment period. It can take up to an hour for the operation
    /// to complete.
    Extended = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaInfo {
    /// The location of the serving resources, e.g. "us-central1".
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// The type of replica.
    #[prost(enumeration = "replica_info::ReplicaType", tag = "2")]
    pub r#type: i32,
    /// If true, this location is designated as the default leader location where
    /// leader replicas are placed. See the [region types
    /// documentation](<https://cloud.google.com/spanner/docs/instances#region_types>)
    /// for more details.
    #[prost(bool, tag = "3")]
    pub default_leader_location: bool,
}
/// Nested message and enum types in `ReplicaInfo`.
pub mod replica_info {
    /// Indicates the type of replica.  See the [replica types
    /// documentation](<https://cloud.google.com/spanner/docs/replication#replica_types>)
    /// for more details.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReplicaType {
        /// Not specified.
        TypeUnspecified = 0,
        /// Read-write replicas support both reads and writes. These replicas:
        ///
        /// * Maintain a full copy of your data.
        /// * Serve reads.
        /// * Can vote whether to commit a write.
        /// * Participate in leadership election.
        /// * Are eligible to become a leader.
        ReadWrite = 1,
        /// Read-only replicas only support reads (not writes). Read-only replicas:
        ///
        /// * Maintain a full copy of your data.
        /// * Serve reads.
        /// * Do not participate in voting to commit writes.
        /// * Are not eligible to become a leader.
        ReadOnly = 2,
        /// Witness replicas don't support reads but do participate in voting to
        /// commit writes. Witness replicas:
        ///
        /// * Do not maintain a full copy of data.
        /// * Do not serve reads.
        /// * Vote whether to commit writes.
        /// * Participate in leader election but are not eligible to become leader.
        Witness = 3,
    }
}
/// A possible configuration for a Cloud Spanner instance. Configurations
/// define the geographic placement of nodes and their replication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceConfig {
    /// A unique identifier for the instance configuration.  Values
    /// are of the form
    /// `projects/<project>/instanceConfigs/\[a-z][-a-z0-9\]*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of this instance configuration as it appears in UIs.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Whether this instance config is a Google or User Managed
    /// Configuration.
    #[prost(enumeration = "instance_config::Type", tag = "5")]
    pub config_type: i32,
    /// The geographic placement of nodes in this instance configuration and their
    /// replication properties.
    #[prost(message, repeated, tag = "3")]
    pub replicas: ::prost::alloc::vec::Vec<ReplicaInfo>,
    /// Output only. The available optional replicas to choose from for user
    /// managed configurations. Populated for Google managed configurations.
    #[prost(message, repeated, tag = "6")]
    pub optional_replicas: ::prost::alloc::vec::Vec<ReplicaInfo>,
    /// Base configuration name, e.g. projects/<project_name>/instanceConfigs/nam3,
    /// based on which this configuration is created. Only set for user managed
    /// configurations. `base_config` must refer to a configuration of type
    /// GOOGLE_MANAGED in the same project as this configuration.
    #[prost(string, tag = "7")]
    pub base_config: ::prost::alloc::string::String,
    /// Cloud Labels are a flexible and lightweight mechanism for organizing cloud
    /// resources into groups that reflect a customer's organizational needs and
    /// deployment strategies. Cloud Labels can be used to filter collections of
    /// resources. They can be used to control how resource metrics are aggregated.
    /// And they can be used as arguments to policy management rules (e.g. route,
    /// firewall, load balancing, etc.).
    ///
    ///  * Label keys must be between 1 and 63 characters long and must conform to
    ///    the following regular expression: `\[a-z][a-z0-9_-\]{0,62}`.
    ///  * Label values must be between 0 and 63 characters long and must conform
    ///    to the regular expression `\[a-z0-9_-\]{0,63}`.
    ///  * No more than 64 labels can be associated with a given resource.
    ///
    /// See <https://goo.gl/xmQnxf> for more information on and examples of labels.
    ///
    /// If you plan to use labels in your own code, please note that additional
    /// characters may be allowed in the future. Therefore, you are advised to use
    /// an internal label representation, such as JSON, which doesn't rely upon
    /// specific characters being disallowed.  For example, representing labels
    /// as the string:  name + "_" + value  would prove problematic if we were to
    /// allow "_" in a future release.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// etag is used for optimistic concurrency control as a way
    /// to help prevent simultaneous updates of a instance config from overwriting
    /// each other. It is strongly suggested that systems make use of the etag in
    /// the read-modify-write cycle to perform instance config updates in order to
    /// avoid race conditions: An etag is returned in the response which contains
    /// instance configs, and systems are expected to put that etag in the request
    /// to update instance config to ensure that their change will be applied to
    /// the same version of the instance config.
    /// If no etag is provided in the call to update instance config, then the
    /// existing instance config is overwritten blindly.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Allowed values of the "default_leader" schema option for databases in
    /// instances that use this instance configuration.
    #[prost(string, repeated, tag = "4")]
    pub leader_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. If true, the instance config is being created or updated. If
    /// false, there are no ongoing operations for the instance config.
    #[prost(bool, tag = "10")]
    pub reconciling: bool,
    /// Output only. The current instance config state.
    #[prost(enumeration = "instance_config::State", tag = "11")]
    pub state: i32,
}
/// Nested message and enum types in `InstanceConfig`.
pub mod instance_config {
    /// The type of this configuration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified.
        Unspecified = 0,
        /// Google managed configuration.
        GoogleManaged = 1,
        /// User managed configuration.
        UserManaged = 2,
    }
    /// Indicates the current state of the instance config.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The instance config is still being created.
        Creating = 1,
        /// The instance config is fully created and ready to be used to create
        /// instances.
        Ready = 2,
    }
}
/// Autoscaling config for an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingConfig {
    /// Required. Autoscaling limits for an instance.
    #[prost(message, optional, tag = "1")]
    pub autoscaling_limits: ::core::option::Option<autoscaling_config::AutoscalingLimits>,
    /// Required. The autoscaling targets for an instance.
    #[prost(message, optional, tag = "2")]
    pub autoscaling_targets: ::core::option::Option<autoscaling_config::AutoscalingTargets>,
}
/// Nested message and enum types in `AutoscalingConfig`.
pub mod autoscaling_config {
    /// The autoscaling limits for the instance. Users can define the minimum and
    /// maximum compute capacity allocated to the instance, and the autoscaler will
    /// only scale within that range. Users can either use nodes or processing
    /// units to specify the limits, but should use the same unit to set both the
    /// min_limit and max_limit.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutoscalingLimits {
        /// The minimum compute capacity for the instance.
        #[prost(oneof = "autoscaling_limits::MinLimit", tags = "1, 2")]
        pub min_limit: ::core::option::Option<autoscaling_limits::MinLimit>,
        /// The maximum compute capacity for the instance. The maximum compute
        /// capacity should be less than or equal to 10X the minimum compute
        /// capacity.
        #[prost(oneof = "autoscaling_limits::MaxLimit", tags = "3, 4")]
        pub max_limit: ::core::option::Option<autoscaling_limits::MaxLimit>,
    }
    /// Nested message and enum types in `AutoscalingLimits`.
    pub mod autoscaling_limits {
        /// The minimum compute capacity for the instance.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MinLimit {
            /// Minimum number of nodes allocated to the instance. If set, this number
            /// should be greater than or equal to 1.
            #[prost(int32, tag = "1")]
            MinNodes(i32),
            /// Minimum number of processing units allocated to the instance. If set,
            /// this number should be multiples of 1000.
            #[prost(int32, tag = "2")]
            MinProcessingUnits(i32),
        }
        /// The maximum compute capacity for the instance. The maximum compute
        /// capacity should be less than or equal to 10X the minimum compute
        /// capacity.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MaxLimit {
            /// Maximum number of nodes allocated to the instance. If set, this number
            /// should be greater than or equal to min_nodes.
            #[prost(int32, tag = "3")]
            MaxNodes(i32),
            /// Maximum number of processing units allocated to the instance. If set,
            /// this number should be multiples of 1000 and be greater than or equal to
            /// min_processing_units.
            #[prost(int32, tag = "4")]
            MaxProcessingUnits(i32),
        }
    }
    /// The autoscaling targets for an instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutoscalingTargets {
        /// Required. The target high priority cpu utilization percentage that the
        /// autoscaler should be trying to achieve for the instance. This number is
        /// on a scale from 0 (no utilization) to 100 (full utilization). The valid
        /// range is [10, 90] inclusive.
        #[prost(int32, tag = "1")]
        pub high_priority_cpu_utilization_percent: i32,
        /// Required. The target storage utilization percentage that the autoscaler
        /// should be trying to achieve for the instance. This number is on a scale
        /// from 0 (no utilization) to 100 (full utilization). The valid range is
        /// [10, 100] inclusive.
        #[prost(int32, tag = "2")]
        pub storage_utilization_percent: i32,
    }
}
/// An isolated set of Cloud Spanner resources on which databases can be hosted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Required. A unique identifier for the instance, which cannot be changed
    /// after the instance is created. Values are of the form
    /// `projects/<project>/instances/\[a-z][-a-z0-9]*[a-z0-9\]`. The final
    /// segment of the name must be between 2 and 64 characters in length.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the instance's configuration. Values are of the form
    /// `projects/<project>/instanceConfigs/<configuration>`. See
    /// also \[InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig\] and
    /// \[ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs\].
    #[prost(string, tag = "2")]
    pub config: ::prost::alloc::string::String,
    /// Required. The descriptive name for this instance as it appears in UIs.
    /// Must be unique per project and between 4 and 30 characters in length.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// The number of nodes allocated to this instance. At most one of either
    /// node_count or processing_units should be present in the message.
    ///
    /// Users can set the node_count field to specify the target number of nodes
    /// allocated to the instance.
    ///
    /// This may be zero in API responses for instances that are not yet in state
    /// `READY`.
    ///
    /// See [the
    /// documentation](<https://cloud.google.com/spanner/docs/compute-capacity>)
    /// for more information about nodes and processing units.
    #[prost(int32, tag = "5")]
    pub node_count: i32,
    /// The number of processing units allocated to this instance. At most one of
    /// processing_units or node_count should be present in the message.
    ///
    /// Users can set the processing_units field to specify the target number of
    /// processing units allocated to the instance.
    ///
    /// This may be zero in API responses for instances that are not yet in state
    /// `READY`.
    ///
    /// See [the
    /// documentation](<https://cloud.google.com/spanner/docs/compute-capacity>)
    /// for more information about nodes and processing units.
    #[prost(int32, tag = "9")]
    pub processing_units: i32,
    /// Optional. The autoscaling configuration. Autoscaling is enabled if this
    /// field is set. When autoscaling is enabled, node_count and processing_units
    /// are treated as OUTPUT_ONLY fields and reflect the current compute capacity
    /// allocated to the instance.
    #[prost(message, optional, tag = "17")]
    pub autoscaling_config: ::core::option::Option<AutoscalingConfig>,
    /// Output only. The current instance state. For
    /// \[CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance\],
    /// the state must be either omitted or set to `CREATING`. For
    /// \[UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance\],
    /// the state must be either omitted or set to `READY`.
    #[prost(enumeration = "instance::State", tag = "6")]
    pub state: i32,
    /// Cloud Labels are a flexible and lightweight mechanism for organizing cloud
    /// resources into groups that reflect a customer's organizational needs and
    /// deployment strategies. Cloud Labels can be used to filter collections of
    /// resources. They can be used to control how resource metrics are aggregated.
    /// And they can be used as arguments to policy management rules (e.g. route,
    /// firewall, load balancing, etc.).
    ///
    ///  * Label keys must be between 1 and 63 characters long and must conform to
    ///    the following regular expression: `\[a-z][a-z0-9_-\]{0,62}`.
    ///  * Label values must be between 0 and 63 characters long and must conform
    ///    to the regular expression `\[a-z0-9_-\]{0,63}`.
    ///  * No more than 64 labels can be associated with a given resource.
    ///
    /// See <https://goo.gl/xmQnxf> for more information on and examples of labels.
    ///
    /// If you plan to use labels in your own code, please note that additional
    /// characters may be allowed in the future. And so you are advised to use an
    /// internal label representation, such as JSON, which doesn't rely upon
    /// specific characters being disallowed.  For example, representing labels
    /// as the string:  name + "_" + value  would prove problematic if we were to
    /// allow "_" in a future release.
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Deprecated. This field is not populated.
    #[prost(string, repeated, tag = "8")]
    pub endpoint_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The time at which the instance was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the instance was most recently updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Indicates the current state of the instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The instance is still being created. Resources may not be
        /// available yet, and operations such as database creation may not
        /// work.
        Creating = 1,
        /// The instance is fully created and ready to do work such as
        /// creating databases.
        Ready = 2,
    }
}
/// The request for
/// \[ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceConfigsRequest {
    /// Required. The name of the project for which a list of supported instance
    /// configurations is requested. Values are of the form
    /// `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of instance configurations to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// \[next_page_token][google.spanner.admin.instance.v1.ListInstanceConfigsResponse.next_page_token\]
    /// from a previous
    /// \[ListInstanceConfigsResponse][google.spanner.admin.instance.v1.ListInstanceConfigsResponse\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceConfigsResponse {
    /// The list of requested instance configurations.
    #[prost(message, repeated, tag = "1")]
    pub instance_configs: ::prost::alloc::vec::Vec<InstanceConfig>,
    /// `next_page_token` can be sent in a subsequent
    /// \[ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs\]
    /// call to fetch more of the matching instance configurations.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[GetInstanceConfigRequest][google.spanner.admin.instance.v1.InstanceAdmin.GetInstanceConfig\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceConfigRequest {
    /// Required. The name of the requested instance configuration. Values are of
    /// the form `projects/<project>/instanceConfigs/<config>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[CreateInstanceConfigRequest][InstanceAdmin.CreateInstanceConfigRequest\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceConfigRequest {
    /// Required. The name of the project in which to create the instance config.
    /// Values are of the form `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance config to create.  Valid identifiers are
    /// of the form `custom-\[-a-z0-9]*[a-z0-9\]` and must be between 2 and 64
    /// characters in length. The `custom-` prefix is required to avoid name
    /// conflicts with Google managed configurations.
    #[prost(string, tag = "2")]
    pub instance_config_id: ::prost::alloc::string::String,
    /// Required. The InstanceConfig proto of the configuration to create.
    /// instance_config.name must be
    /// `<parent>/instanceConfigs/<instance_config_id>`.
    /// instance_config.base_config must be a Google managed configuration name,
    /// e.g. <parent>/instanceConfigs/us-east1, <parent>/instanceConfigs/nam3.
    #[prost(message, optional, tag = "3")]
    pub instance_config: ::core::option::Option<InstanceConfig>,
    /// An option to validate, but not actually execute, a request,
    /// and provide the same response.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request for
/// \[UpdateInstanceConfigRequest][InstanceAdmin.UpdateInstanceConfigRequest\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceConfigRequest {
    /// Required. The user instance config to update, which must always include the
    /// instance config name. Otherwise, only fields mentioned in
    /// \[update_mask][google.spanner.admin.instance.v1.UpdateInstanceConfigRequest.update_mask\]
    /// need be included. To prevent conflicts of concurrent updates,
    /// \[etag][google.spanner.admin.instance.v1.InstanceConfig.reconciling\] can
    /// be used.
    #[prost(message, optional, tag = "1")]
    pub instance_config: ::core::option::Option<InstanceConfig>,
    /// Required. A mask specifying which fields in
    /// \[InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig\] should be
    /// updated. The field mask must always be specified; this prevents any future
    /// fields in \[InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig\]
    /// from being erased accidentally by clients that do not know about them. Only
    /// display_name and labels can be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// An option to validate, but not actually execute, a request,
    /// and provide the same response.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The request for
/// \[DeleteInstanceConfigRequest][InstanceAdmin.DeleteInstanceConfigRequest\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceConfigRequest {
    /// Required. The name of the instance configuration to be deleted.
    /// Values are of the form
    /// `projects/<project>/instanceConfigs/<instance_config>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Used for optimistic concurrency control as a way to help prevent
    /// simultaneous deletes of an instance config from overwriting each
    /// other. If not empty, the API
    /// only deletes the instance config when the etag provided matches the current
    /// status of the requested instance config. Otherwise, deletes the instance
    /// config without checking the current status of the requested instance
    /// config.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// An option to validate, but not actually execute, a request,
    /// and provide the same response.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The request for
/// \[ListInstanceConfigOperations][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceConfigOperationsRequest {
    /// Required. The project of the instance config operations.
    /// Values are of the form `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression that filters the list of returned operations.
    ///
    /// A filter expression consists of a field name, a
    /// comparison operator, and a value for filtering.
    /// The value must be a string, a number, or a boolean. The comparison operator
    /// must be one of: `<`, `>`, `<=`, `>=`, `!=`, `=`, or `:`.
    /// Colon `:` is the contains operator. Filter rules are not case sensitive.
    ///
    /// The following fields in the \[Operation][google.longrunning.Operation\]
    /// are eligible for filtering:
    ///
    ///   * `name` - The name of the long-running operation
    ///   * `done` - False if the operation is in progress, else true.
    ///   * `metadata.@type` - the type of metadata. For example, the type string
    ///      for
    ///      \[CreateInstanceConfigMetadata][google.spanner.admin.instance.v1.CreateInstanceConfigMetadata\]
    ///      is
    ///      `type.googleapis.com/google.spanner.admin.instance.v1.CreateInstanceConfigMetadata`.
    ///   * `metadata.<field_name>` - any field in metadata.value.
    ///      `metadata.@type` must be specified first, if filtering on metadata
    ///      fields.
    ///   * `error` - Error associated with the long-running operation.
    ///   * `response.@type` - the type of response.
    ///   * `response.<field_name>` - any field in response.value.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. By default, expressions are combined with AND logic. However,
    /// you can specify AND, OR, and NOT logic explicitly.
    ///
    /// Here are a few examples:
    ///
    ///   * `done:true` - The operation is complete.
    ///   * `(metadata.@type=` \
    ///     `type.googleapis.com/google.spanner.admin.instance.v1.CreateInstanceConfigMetadata)
    ///     AND` \
    ///     `(metadata.instance_config.name:custom-config) AND` \
    ///     `(metadata.progress.start_time < \"2021-03-28T14:50:00Z\") AND` \
    ///     `(error:*)` - Return operations where:
    ///     * The operation's metadata type is
    ///     \[CreateInstanceConfigMetadata][google.spanner.admin.instance.v1.CreateInstanceConfigMetadata\].
    ///     * The instance config name contains "custom-config".
    ///     * The operation started before 2021-03-28T14:50:00Z.
    ///     * The operation resulted in an error.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Number of operations to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// \[next_page_token][google.spanner.admin.instance.v1.ListInstanceConfigOperationsResponse.next_page_token\]
    /// from a previous
    /// \[ListInstanceConfigOperationsResponse][google.spanner.admin.instance.v1.ListInstanceConfigOperationsResponse\]
    /// to the same `parent` and with the same `filter`.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ListInstanceConfigOperations][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceConfigOperationsResponse {
    /// The list of matching instance config [long-running
    /// operations]\[google.longrunning.Operation\]. Each operation's name will be
    /// prefixed by the instance config's name. The operation's
    /// \[metadata][google.longrunning.Operation.metadata\] field type
    /// `metadata.type_url` describes the type of the metadata.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<super::super::super::super::longrunning::Operation>,
    /// `next_page_token` can be sent in a subsequent
    /// \[ListInstanceConfigOperations][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigOperations\]
    /// call to fetch more of the matching metadata.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[GetInstance][google.spanner.admin.instance.v1.InstanceAdmin.GetInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The name of the requested instance. Values are of the form
    /// `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If field_mask is present, specifies the subset of
    /// \[Instance][google.spanner.admin.instance.v1.Instance\] fields that should be
    /// returned. If absent, all
    /// \[Instance][google.spanner.admin.instance.v1.Instance\] fields are returned.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// \[CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The name of the project in which to create the instance. Values
    /// are of the form `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance to create.  Valid identifiers are of the
    /// form `\[a-z][-a-z0-9]*[a-z0-9\]` and must be between 2 and 64 characters in
    /// length.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The instance to create.  The name may be omitted, but if
    /// specified must be `<parent>/instances/<instance_id>`.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// The request for
/// \[ListInstances][google.spanner.admin.instance.v1.InstanceAdmin.ListInstances\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The name of the project for which a list of instances is
    /// requested. Values are of the form `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of instances to be returned in the response. If 0 or less, defaults
    /// to the server's maximum allowed page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// \[next_page_token][google.spanner.admin.instance.v1.ListInstancesResponse.next_page_token\]
    /// from a previous
    /// \[ListInstancesResponse][google.spanner.admin.instance.v1.ListInstancesResponse\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. Filter rules are
    /// case insensitive. The fields eligible for filtering are:
    ///
    ///   * `name`
    ///   * `display_name`
    ///   * `labels.key` where key is the name of a label
    ///
    /// Some examples of using filters are:
    ///
    ///   * `name:*` --> The instance has a name.
    ///   * `name:Howl` --> The instance's name contains the string "howl".
    ///   * `name:HOWL` --> Equivalent to above.
    ///   * `NAME:howl` --> Equivalent to above.
    ///   * `labels.env:*` --> The instance has the label "env".
    ///   * `labels.env:dev` --> The instance has the label "env" and the value of
    ///                        the label contains the string "dev".
    ///   * `name:howl labels.env:dev` --> The instance's name contains "howl" and
    ///                                  it has the label "env" with its value
    ///                                  containing "dev".
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Deadline used while retrieving metadata for instances.
    /// Instances whose metadata cannot be retrieved within this deadline will be
    /// added to
    /// \[unreachable][google.spanner.admin.instance.v1.ListInstancesResponse.unreachable\]
    /// in
    /// \[ListInstancesResponse][google.spanner.admin.instance.v1.ListInstancesResponse\].
    #[prost(message, optional, tag = "5")]
    pub instance_deadline: ::core::option::Option<::prost_types::Timestamp>,
}
/// The response for
/// \[ListInstances][google.spanner.admin.instance.v1.InstanceAdmin.ListInstances\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of requested instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// `next_page_token` can be sent in a subsequent
    /// \[ListInstances][google.spanner.admin.instance.v1.InstanceAdmin.ListInstances\]
    /// call to fetch more of the matching instances.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The list of unreachable instances.
    /// It includes the names of instances whose metadata could not be retrieved
    /// within
    /// \[instance_deadline][google.spanner.admin.instance.v1.ListInstancesRequest.instance_deadline\].
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for
/// \[UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. The instance to update, which must always include the instance
    /// name.  Otherwise, only fields mentioned in
    /// \[field_mask][google.spanner.admin.instance.v1.UpdateInstanceRequest.field_mask\]
    /// need be included.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// Required. A mask specifying which fields in
    /// \[Instance][google.spanner.admin.instance.v1.Instance\] should be updated.
    /// The field mask must always be specified; this prevents any future fields in
    /// \[Instance][google.spanner.admin.instance.v1.Instance\] from being erased
    /// accidentally by clients that do not know about them.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// \[DeleteInstance][google.spanner.admin.instance.v1.InstanceAdmin.DeleteInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The name of the instance to be deleted. Values are of the form
    /// `projects/<project>/instances/<instance>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata type for the operation returned by
/// \[CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceMetadata {
    /// The instance being created.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// The time at which the
    /// \[CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance\]
    /// request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The expected fulfillment period of this create operation.
    #[prost(enumeration = "FulfillmentPeriod", tag = "5")]
    pub expected_fulfillment_period: i32,
}
/// Metadata type for the operation returned by
/// \[UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceMetadata {
    /// The desired end state of the update.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// The time at which
    /// \[UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance\]
    /// request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The expected fulfillment period of this update operation.
    #[prost(enumeration = "FulfillmentPeriod", tag = "5")]
    pub expected_fulfillment_period: i32,
}
/// Metadata type for the operation returned by
/// \[CreateInstanceConfig][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstanceConfig\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceConfigMetadata {
    /// The target instance config end state.
    #[prost(message, optional, tag = "1")]
    pub instance_config: ::core::option::Option<InstanceConfig>,
    /// The progress of the
    /// \[CreateInstanceConfig][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstanceConfig\]
    /// operation.
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<OperationProgress>,
    /// The time at which this operation was cancelled.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata type for the operation returned by
/// \[UpdateInstanceConfig][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstanceConfig\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceConfigMetadata {
    /// The desired instance config after updating.
    #[prost(message, optional, tag = "1")]
    pub instance_config: ::core::option::Option<InstanceConfig>,
    /// The progress of the
    /// \[UpdateInstanceConfig][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstanceConfig\]
    /// operation.
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<OperationProgress>,
    /// The time at which this operation was cancelled.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// An isolated set of Cloud Spanner resources that databases can define
/// placements on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancePartition {
    /// Required. A unique identifier for the instance partition. Values are of the
    /// form
    /// `projects/<project>/instances/<instance>/instancePartitions/\[a-z][-a-z0-9]*[a-z0-9\]`.
    /// The final segment of the name must be between 2 and 64 characters in
    /// length. An instance partition's name cannot be changed after the instance
    /// partition is created.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the instance partition's configuration. Values are of
    /// the form `projects/<project>/instanceConfigs/<configuration>`. See also
    /// \[InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig\] and
    /// \[ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs\].
    #[prost(string, tag = "2")]
    pub config: ::prost::alloc::string::String,
    /// Required. The descriptive name for this instance partition as it appears in
    /// UIs. Must be unique per project and between 4 and 30 characters in length.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The current instance partition state.
    #[prost(enumeration = "instance_partition::State", tag = "7")]
    pub state: i32,
    /// Output only. The time at which the instance partition was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the instance partition was most recently
    /// updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The names of the databases that reference this
    /// instance partition. Referencing databases should share the parent instance.
    /// The existence of any referencing database prevents the instance partition
    /// from being deleted.
    #[prost(string, repeated, tag = "10")]
    pub referencing_databases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The names of the backups that reference this instance
    /// partition. Referencing backups should share the parent instance. The
    /// existence of any referencing backup prevents the instance partition from
    /// being deleted.
    #[prost(string, repeated, tag = "11")]
    pub referencing_backups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Used for optimistic concurrency control as a way
    /// to help prevent simultaneous updates of a instance partition from
    /// overwriting each other. It is strongly suggested that systems make use of
    /// the etag in the read-modify-write cycle to perform instance partition
    /// updates in order to avoid race conditions: An etag is returned in the
    /// response which contains instance partitions, and systems are expected to
    /// put that etag in the request to update instance partitions to ensure that
    /// their change will be applied to the same version of the instance partition.
    /// If no etag is provided in the call to update instance partition, then the
    /// existing instance partition is overwritten blindly.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
    /// Compute capacity defines amount of server and storage resources that are
    /// available to the databases in an instance partition. At most one of either
    /// node_count or processing_units should be present in the message. See [the
    /// documentation](<https://cloud.google.com/spanner/docs/compute-capacity>)
    /// for more information about nodes and processing units.
    #[prost(oneof = "instance_partition::ComputeCapacity", tags = "5, 6")]
    pub compute_capacity: ::core::option::Option<instance_partition::ComputeCapacity>,
}
/// Nested message and enum types in `InstancePartition`.
pub mod instance_partition {
    /// Indicates the current state of the instance partition.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The instance partition is still being created. Resources may not be
        /// available yet, and operations such as creating placements using this
        /// instance partition may not work.
        Creating = 1,
        /// The instance partition is fully created and ready to do work such as
        /// creating placements and using in databases.
        Ready = 2,
    }
    /// Compute capacity defines amount of server and storage resources that are
    /// available to the databases in an instance partition. At most one of either
    /// node_count or processing_units should be present in the message. See [the
    /// documentation](<https://cloud.google.com/spanner/docs/compute-capacity>)
    /// for more information about nodes and processing units.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ComputeCapacity {
        /// The number of nodes allocated to this instance partition.
        ///
        /// Users can set the node_count field to specify the target number of nodes
        /// allocated to the instance partition.
        ///
        /// This may be zero in API responses for instance partitions that are not
        /// yet in state `READY`.
        #[prost(int32, tag = "5")]
        NodeCount(i32),
        /// The number of processing units allocated to this instance partition.
        ///
        /// Users can set the processing_units field to specify the target number of
        /// processing units allocated to the instance partition.
        ///
        /// This may be zero in API responses for instance partitions that are not
        /// yet in state `READY`.
        #[prost(int32, tag = "6")]
        ProcessingUnits(i32),
    }
}
/// Metadata type for the operation returned by
/// \[CreateInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstancePartition\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstancePartitionMetadata {
    /// The instance partition being created.
    #[prost(message, optional, tag = "1")]
    pub instance_partition: ::core::option::Option<InstancePartition>,
    /// The time at which the
    /// \[CreateInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstancePartition\]
    /// request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request for
/// \[CreateInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstancePartition\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstancePartitionRequest {
    /// Required. The name of the instance in which to create the instance
    /// partition. Values are of the form
    /// `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance partition to create. Valid identifiers are
    /// of the form `\[a-z][-a-z0-9]*[a-z0-9\]` and must be between 2 and 64
    /// characters in length.
    #[prost(string, tag = "2")]
    pub instance_partition_id: ::prost::alloc::string::String,
    /// Required. The instance partition to create. The instance_partition.name may
    /// be omitted, but if specified must be
    /// `<parent>/instancePartitions/<instance_partition_id>`.
    #[prost(message, optional, tag = "3")]
    pub instance_partition: ::core::option::Option<InstancePartition>,
}
/// The request for
/// \[DeleteInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.DeleteInstancePartition\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstancePartitionRequest {
    /// Required. The name of the instance partition to be deleted.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/instancePartitions/{instance_partition}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If not empty, the API only deletes the instance partition when
    /// the etag provided matches the current status of the requested instance
    /// partition. Otherwise, deletes the instance partition without checking the
    /// current status of the requested instance partition.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// The request for
/// \[GetInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.GetInstancePartition\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstancePartitionRequest {
    /// Required. The name of the requested instance partition. Values are of
    /// the form
    /// `projects/{project}/instances/{instance}/instancePartitions/{instance_partition}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[UpdateInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstancePartition\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstancePartitionRequest {
    /// Required. The instance partition to update, which must always include the
    /// instance partition name. Otherwise, only fields mentioned in
    /// \[field_mask][google.spanner.admin.instance.v1.UpdateInstancePartitionRequest.field_mask\]
    /// need be included.
    #[prost(message, optional, tag = "1")]
    pub instance_partition: ::core::option::Option<InstancePartition>,
    /// Required. A mask specifying which fields in
    /// \[InstancePartition][google.spanner.admin.instance.v1.InstancePartition\]
    /// should be updated. The field mask must always be specified; this prevents
    /// any future fields in
    /// \[InstancePartition][google.spanner.admin.instance.v1.InstancePartition\]
    /// from being erased accidentally by clients that do not know about them.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Metadata type for the operation returned by
/// \[UpdateInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstancePartition\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstancePartitionMetadata {
    /// The desired end state of the update.
    #[prost(message, optional, tag = "1")]
    pub instance_partition: ::core::option::Option<InstancePartition>,
    /// The time at which
    /// \[UpdateInstancePartition][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstancePartition\]
    /// request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request for
/// \[ListInstancePartitions][google.spanner.admin.instance.v1.InstanceAdmin.ListInstancePartitions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancePartitionsRequest {
    /// Required. The instance whose instance partitions should be listed. Values
    /// are of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Number of instance partitions to be returned in the response. If 0 or less,
    /// defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// \[next_page_token][google.spanner.admin.instance.v1.ListInstancePartitionsResponse.next_page_token\]
    /// from a previous
    /// \[ListInstancePartitionsResponse][google.spanner.admin.instance.v1.ListInstancePartitionsResponse\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Deadline used while retrieving metadata for instance partitions.
    /// Instance partitions whose metadata cannot be retrieved within this deadline
    /// will be added to
    /// \[unreachable][google.spanner.admin.instance.v1.ListInstancePartitionsResponse.unreachable\]
    /// in
    /// \[ListInstancePartitionsResponse][google.spanner.admin.instance.v1.ListInstancePartitionsResponse\].
    #[prost(message, optional, tag = "4")]
    pub instance_partition_deadline: ::core::option::Option<::prost_types::Timestamp>,
}
/// The response for
/// \[ListInstancePartitions][google.spanner.admin.instance.v1.InstanceAdmin.ListInstancePartitions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancePartitionsResponse {
    /// The list of requested instancePartitions.
    #[prost(message, repeated, tag = "1")]
    pub instance_partitions: ::prost::alloc::vec::Vec<InstancePartition>,
    /// `next_page_token` can be sent in a subsequent
    /// \[ListInstancePartitions][google.spanner.admin.instance.v1.InstanceAdmin.ListInstancePartitions\]
    /// call to fetch more of the matching instance partitions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The list of unreachable instance partitions.
    /// It includes the names of instance partitions whose metadata could
    /// not be retrieved within
    /// \[instance_partition_deadline][google.spanner.admin.instance.v1.ListInstancePartitionsRequest.instance_partition_deadline\].
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for
/// \[ListInstancePartitionOperations][google.spanner.admin.instance.v1.InstanceAdmin.ListInstancePartitionOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancePartitionOperationsRequest {
    /// Required. The parent instance of the instance partition operations.
    /// Values are of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. An expression that filters the list of returned operations.
    ///
    /// A filter expression consists of a field name, a
    /// comparison operator, and a value for filtering.
    /// The value must be a string, a number, or a boolean. The comparison operator
    /// must be one of: `<`, `>`, `<=`, `>=`, `!=`, `=`, or `:`.
    /// Colon `:` is the contains operator. Filter rules are not case sensitive.
    ///
    /// The following fields in the \[Operation][google.longrunning.Operation\]
    /// are eligible for filtering:
    ///
    ///   * `name` - The name of the long-running operation
    ///   * `done` - False if the operation is in progress, else true.
    ///   * `metadata.@type` - the type of metadata. For example, the type string
    ///      for
    ///      \[CreateInstancePartitionMetadata][google.spanner.admin.instance.v1.CreateInstancePartitionMetadata\]
    ///      is
    ///      `type.googleapis.com/google.spanner.admin.instance.v1.CreateInstancePartitionMetadata`.
    ///   * `metadata.<field_name>` - any field in metadata.value.
    ///      `metadata.@type` must be specified first, if filtering on metadata
    ///      fields.
    ///   * `error` - Error associated with the long-running operation.
    ///   * `response.@type` - the type of response.
    ///   * `response.<field_name>` - any field in response.value.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. By default, expressions are combined with AND logic. However,
    /// you can specify AND, OR, and NOT logic explicitly.
    ///
    /// Here are a few examples:
    ///
    ///   * `done:true` - The operation is complete.
    ///   * `(metadata.@type=` \
    ///     `type.googleapis.com/google.spanner.admin.instance.v1.CreateInstancePartitionMetadata)
    ///     AND` \
    ///     `(metadata.instance_partition.name:custom-instance-partition) AND` \
    ///     `(metadata.start_time < \"2021-03-28T14:50:00Z\") AND` \
    ///     `(error:*)` - Return operations where:
    ///     * The operation's metadata type is
    ///     \[CreateInstancePartitionMetadata][google.spanner.admin.instance.v1.CreateInstancePartitionMetadata\].
    ///     * The instance partition name contains "custom-instance-partition".
    ///     * The operation started before 2021-03-28T14:50:00Z.
    ///     * The operation resulted in an error.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Number of operations to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. If non-empty, `page_token` should contain a
    /// \[next_page_token][google.spanner.admin.instance.v1.ListInstancePartitionOperationsResponse.next_page_token\]
    /// from a previous
    /// \[ListInstancePartitionOperationsResponse][google.spanner.admin.instance.v1.ListInstancePartitionOperationsResponse\]
    /// to the same `parent` and with the same `filter`.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Deadline used while retrieving metadata for instance partition
    /// operations. Instance partitions whose operation metadata cannot be
    /// retrieved within this deadline will be added to
    /// \[unreachable][ListInstancePartitionOperationsResponse.unreachable\] in
    /// \[ListInstancePartitionOperationsResponse][google.spanner.admin.instance.v1.ListInstancePartitionOperationsResponse\].
    #[prost(message, optional, tag = "5")]
    pub instance_partition_deadline: ::core::option::Option<::prost_types::Timestamp>,
}
/// The response for
/// \[ListInstancePartitionOperations][google.spanner.admin.instance.v1.InstanceAdmin.ListInstancePartitionOperations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancePartitionOperationsResponse {
    /// The list of matching instance partition [long-running
    /// operations]\[google.longrunning.Operation\]. Each operation's name will be
    /// prefixed by the instance partition's name. The operation's
    /// \[metadata][google.longrunning.Operation.metadata\] field type
    /// `metadata.type_url` describes the type of the metadata.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<super::super::super::super::longrunning::Operation>,
    /// `next_page_token` can be sent in a subsequent
    /// \[ListInstancePartitionOperations][google.spanner.admin.instance.v1.InstanceAdmin.ListInstancePartitionOperations\]
    /// call to fetch more of the matching metadata.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The list of unreachable instance partitions.
    /// It includes the names of instance partitions whose operation metadata could
    /// not be retrieved within
    /// \[instance_partition_deadline][google.spanner.admin.instance.v1.ListInstancePartitionOperationsRequest.instance_partition_deadline\].
    #[prost(string, repeated, tag = "3")]
    pub unreachable_instance_partitions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod instance_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Cloud Spanner Instance Admin API"]
    #[doc = ""]
    #[doc = " The Cloud Spanner Instance Admin API can be used to create, delete,"]
    #[doc = " modify and list instances. Instances are dedicated Cloud Spanner serving"]
    #[doc = " and storage resources to be used by Cloud Spanner databases."]
    #[doc = ""]
    #[doc = " Each instance has a \"configuration\", which dictates where the"]
    #[doc = " serving resources for the Cloud Spanner instance are located (e.g.,"]
    #[doc = " US-central, Europe). Configurations are created by Google based on"]
    #[doc = " resource availability."]
    #[doc = ""]
    #[doc = " Cloud Spanner billing is based on the instances that exist and their"]
    #[doc = " sizes. After an instance exists, there are no additional"]
    #[doc = " per-database or per-operation charges for use of the instance"]
    #[doc = " (though there may be additional network bandwidth charges)."]
    #[doc = " Instances offer isolation: problems with databases in one instance"]
    #[doc = " will not affect other instances. However, within an instance"]
    #[doc = " databases can affect each other. For example, if one database in an"]
    #[doc = " instance receives a lot of requests and consumes most of the"]
    #[doc = " instance resources, fewer resources are available for other"]
    #[doc = " databases in that instance, and their performance may suffer."]
    #[derive(Debug, Clone)]
    pub struct InstanceAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InstanceAdminClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InstanceAdminClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            InstanceAdminClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists the supported instance configurations for a given project."]
        pub async fn list_instance_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceConfigsRequest>,
        ) -> Result<tonic::Response<super::ListInstanceConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstanceConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a particular instance configuration."]
        pub async fn get_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceConfigRequest>,
        ) -> Result<tonic::Response<super::InstanceConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an instance config and begins preparing it to be used. The"]
        #[doc = " returned [long-running operation][google.longrunning.Operation]"]
        #[doc = " can be used to track the progress of preparing the new"]
        #[doc = " instance config. The instance config name is assigned by the caller. If the"]
        #[doc = " named instance config already exists, `CreateInstanceConfig` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        #[doc = ""]
        #[doc = " Immediately after the request returns:"]
        #[doc = ""]
        #[doc = "   * The instance config is readable via the API, with all requested"]
        #[doc = "     attributes. The instance config's"]
        #[doc = "     [reconciling][google.spanner.admin.instance.v1.InstanceConfig.reconciling]"]
        #[doc = "     field is set to true. Its state is `CREATING`."]
        #[doc = ""]
        #[doc = " While the operation is pending:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation renders the instance config immediately"]
        #[doc = "     unreadable via the API."]
        #[doc = "   * Except for deleting the creating resource, all other attempts to modify"]
        #[doc = "     the instance config are rejected."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Instances can be created using the instance configuration."]
        #[doc = "   * The instance config's"]
        #[doc = "   [reconciling][google.spanner.admin.instance.v1.InstanceConfig.reconciling]"]
        #[doc = "   field becomes false. Its state becomes `READY`."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format"]
        #[doc = " `<instance_config_name>/operations/<operation_id>` and can be used to track"]
        #[doc = " creation of the instance config. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateInstanceConfigMetadata][google.spanner.admin.instance.v1.CreateInstanceConfigMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig], if"]
        #[doc = " successful."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instanceConfigs.create` permission on"]
        #[doc = " the resource"]
        #[doc = " [parent][google.spanner.admin.instance.v1.CreateInstanceConfigRequest.parent]."]
        pub async fn create_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/CreateInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an instance config. The returned"]
        #[doc = " [long-running operation][google.longrunning.Operation] can be used to track"]
        #[doc = " the progress of updating the instance. If the named instance config does"]
        #[doc = " not exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " Only user managed configurations can be updated."]
        #[doc = ""]
        #[doc = " Immediately after the request returns:"]
        #[doc = ""]
        #[doc = "   * The instance config's"]
        #[doc = "     [reconciling][google.spanner.admin.instance.v1.InstanceConfig.reconciling]"]
        #[doc = "     field is set to true."]
        #[doc = ""]
        #[doc = " While the operation is pending:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation sets its metadata's"]
        #[doc = "     [cancel_time][google.spanner.admin.instance.v1.UpdateInstanceConfigMetadata.cancel_time]."]
        #[doc = "     The operation is guaranteed to succeed at undoing all changes, after"]
        #[doc = "     which point it terminates with a `CANCELLED` status."]
        #[doc = "   * All other attempts to modify the instance config are rejected."]
        #[doc = "   * Reading the instance config via the API continues to give the"]
        #[doc = "     pre-request values."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Creating instances using the instance configuration uses the new"]
        #[doc = "     values."]
        #[doc = "   * The instance config's new values are readable via the API."]
        #[doc = "   * The instance config's"]
        #[doc = "   [reconciling][google.spanner.admin.instance.v1.InstanceConfig.reconciling]"]
        #[doc = "   field becomes false."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format"]
        #[doc = " `<instance_config_name>/operations/<operation_id>` and can be used to track"]
        #[doc = " the instance config modification.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateInstanceConfigMetadata][google.spanner.admin.instance.v1.UpdateInstanceConfigMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig], if"]
        #[doc = " successful."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instanceConfigs.update` permission on"]
        #[doc = " the resource [name][google.spanner.admin.instance.v1.InstanceConfig.name]."]
        pub async fn update_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/UpdateInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the instance config. Deletion is only allowed when no"]
        #[doc = " instances are using the configuration. If any instances are using"]
        #[doc = " the config, returns `FAILED_PRECONDITION`."]
        #[doc = ""]
        #[doc = " Only user managed configurations can be deleted."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instanceConfigs.delete` permission on"]
        #[doc = " the resource [name][google.spanner.admin.instance.v1.InstanceConfig.name]."]
        pub async fn delete_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/DeleteInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the user-managed instance config [long-running"]
        #[doc = " operations][google.longrunning.Operation] in the given project. An instance"]
        #[doc = " config operation has a name of the form"]
        #[doc = " `projects/<project>/instanceConfigs/<instance_config>/operations/<operation>`."]
        #[doc = " The long-running operation"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type"]
        #[doc = " `metadata.type_url` describes the type of the metadata. Operations returned"]
        #[doc = " include those that have completed/failed/canceled within the last 7 days,"]
        #[doc = " and pending operations. Operations returned are ordered by"]
        #[doc = " `operation.metadata.value.start_time` in descending order starting"]
        #[doc = " from the most recently started operation."]
        pub async fn list_instance_config_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceConfigOperationsRequest>,
        ) -> Result<tonic::Response<super::ListInstanceConfigOperationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstanceConfigOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all instances in the given project."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all instance partitions for the given instance."]
        pub async fn list_instance_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancePartitionsRequest>,
        ) -> Result<tonic::Response<super::ListInstancePartitionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstancePartitions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a particular instance."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an instance and begins preparing it to begin serving. The"]
        #[doc = " returned [long-running operation][google.longrunning.Operation]"]
        #[doc = " can be used to track the progress of preparing the new"]
        #[doc = " instance. The instance name is assigned by the caller. If the"]
        #[doc = " named instance already exists, `CreateInstance` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * The instance is readable via the API, with all requested attributes"]
        #[doc = "     but no allocated resources. Its state is `CREATING`."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation renders the instance immediately unreadable"]
        #[doc = "     via the API."]
        #[doc = "   * The instance can be deleted."]
        #[doc = "   * All other attempts to modify the instance are rejected."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing for all successfully-allocated resources begins (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * Databases can be created in the instance."]
        #[doc = "   * The instance's allocated resource levels are readable via the API."]
        #[doc = "   * The instance's state becomes `READY`."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<instance_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track creation of the instance.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateInstanceMetadata][google.spanner.admin.instance.v1.CreateInstanceMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Instance][google.spanner.admin.instance.v1.Instance], if successful."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an instance, and begins allocating or releasing resources"]
        #[doc = " as requested. The returned [long-running"]
        #[doc = " operation][google.longrunning.Operation] can be used to track the"]
        #[doc = " progress of updating the instance. If the named instance does not"]
        #[doc = " exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * For resource types for which a decrease in the instance's allocation"]
        #[doc = "     has been requested, billing is based on the newly-requested level."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation sets its metadata's"]
        #[doc = "     [cancel_time][google.spanner.admin.instance.v1.UpdateInstanceMetadata.cancel_time],"]
        #[doc = "     and begins restoring resources to their pre-request values. The"]
        #[doc = "     operation is guaranteed to succeed at undoing all resource changes,"]
        #[doc = "     after which point it terminates with a `CANCELLED` status."]
        #[doc = "   * All other attempts to modify the instance are rejected."]
        #[doc = "   * Reading the instance via the API continues to give the pre-request"]
        #[doc = "     resource levels."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing begins for all successfully-allocated resources (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * All newly-reserved resources are available for serving the instance's"]
        #[doc = "     tables."]
        #[doc = "   * The instance's new resource levels are readable via the API."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<instance_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track the instance modification.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateInstanceMetadata][google.spanner.admin.instance.v1.UpdateInstanceMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Instance][google.spanner.admin.instance.v1.Instance], if successful."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.update` permission on"]
        #[doc = " the resource [name][google.spanner.admin.instance.v1.Instance.name]."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an instance."]
        #[doc = ""]
        #[doc = " Immediately upon completion of the request:"]
        #[doc = ""]
        #[doc = "   * Billing ceases for all of the instance's reserved resources."]
        #[doc = ""]
        #[doc = " Soon afterward:"]
        #[doc = ""]
        #[doc = "   * The instance and *all of its databases* immediately and"]
        #[doc = "     irrevocably disappear from the API. All data in the databases"]
        #[doc = "     is permanently deleted."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on an instance resource. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.setIamPolicy` on"]
        #[doc = " [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for an instance resource. Returns an empty"]
        #[doc = " policy if an instance exists but does not have a policy set."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.getIamPolicy` on"]
        #[doc = " [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified instance resource."]
        #[doc = ""]
        #[doc = " Attempting this RPC on a non-existent Cloud Spanner instance resource will"]
        #[doc = " result in a NOT_FOUND error if the user has `spanner.instances.list`"]
        #[doc = " permission on the containing Google Cloud Project. Otherwise returns an"]
        #[doc = " empty set of permissions."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a particular instance partition."]
        pub async fn get_instance_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstancePartitionRequest>,
        ) -> Result<tonic::Response<super::InstancePartition>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstancePartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an instance partition and begins preparing it to be used. The"]
        #[doc = " returned [long-running operation][google.longrunning.Operation]"]
        #[doc = " can be used to track the progress of preparing the new instance partition."]
        #[doc = " The instance partition name is assigned by the caller. If the named"]
        #[doc = " instance partition already exists, `CreateInstancePartition` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * The instance partition is readable via the API, with all requested"]
        #[doc = "     attributes but no allocated resources. Its state is `CREATING`."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation renders the instance partition immediately"]
        #[doc = "     unreadable via the API."]
        #[doc = "   * The instance partition can be deleted."]
        #[doc = "   * All other attempts to modify the instance partition are rejected."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing for all successfully-allocated resources begins (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * Databases can start using this instance partition."]
        #[doc = "   * The instance partition's allocated resource levels are readable via the"]
        #[doc = "     API."]
        #[doc = "   * The instance partition's state becomes `READY`."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format"]
        #[doc = " `<instance_partition_name>/operations/<operation_id>` and can be used to"]
        #[doc = " track creation of the instance partition.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateInstancePartitionMetadata][google.spanner.admin.instance.v1.CreateInstancePartitionMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [InstancePartition][google.spanner.admin.instance.v1.InstancePartition], if"]
        #[doc = " successful."]
        pub async fn create_instance_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstancePartitionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/CreateInstancePartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing instance partition. Requires that the"]
        #[doc = " instance partition is not used by any database or backup and is not the"]
        #[doc = " default instance partition of an instance."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instancePartitions.delete` permission on"]
        #[doc = " the resource"]
        #[doc = " [name][google.spanner.admin.instance.v1.InstancePartition.name]."]
        pub async fn delete_instance_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstancePartitionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/DeleteInstancePartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an instance partition, and begins allocating or releasing resources"]
        #[doc = " as requested. The returned [long-running"]
        #[doc = " operation][google.longrunning.Operation] can be used to track the"]
        #[doc = " progress of updating the instance partition. If the named instance"]
        #[doc = " partition does not exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * For resource types for which a decrease in the instance partition's"]
        #[doc = "   allocation has been requested, billing is based on the newly-requested"]
        #[doc = "   level."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation sets its metadata's"]
        #[doc = "     [cancel_time][google.spanner.admin.instance.v1.UpdateInstancePartitionMetadata.cancel_time],"]
        #[doc = "     and begins restoring resources to their pre-request values. The"]
        #[doc = "     operation is guaranteed to succeed at undoing all resource changes,"]
        #[doc = "     after which point it terminates with a `CANCELLED` status."]
        #[doc = "   * All other attempts to modify the instance partition are rejected."]
        #[doc = "   * Reading the instance partition via the API continues to give the"]
        #[doc = "     pre-request resource levels."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing begins for all successfully-allocated resources (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * All newly-reserved resources are available for serving the instance"]
        #[doc = "     partition's tables."]
        #[doc = "   * The instance partition's new resource levels are readable via the API."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format"]
        #[doc = " `<instance_partition_name>/operations/<operation_id>` and can be used to"]
        #[doc = " track the instance partition modification. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateInstancePartitionMetadata][google.spanner.admin.instance.v1.UpdateInstancePartitionMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [InstancePartition][google.spanner.admin.instance.v1.InstancePartition], if"]
        #[doc = " successful."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instancePartitions.update` permission on"]
        #[doc = " the resource"]
        #[doc = " [name][google.spanner.admin.instance.v1.InstancePartition.name]."]
        pub async fn update_instance_partition(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstancePartitionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/UpdateInstancePartition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists instance partition [long-running"]
        #[doc = " operations][google.longrunning.Operation] in the given instance."]
        #[doc = " An instance partition operation has a name of the form"]
        #[doc = " `projects/<project>/instances/<instance>/instancePartitions/<instance_partition>/operations/<operation>`."]
        #[doc = " The long-running operation"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type"]
        #[doc = " `metadata.type_url` describes the type of the metadata. Operations returned"]
        #[doc = " include those that have completed/failed/canceled within the last 7 days,"]
        #[doc = " and pending operations. Operations returned are ordered by"]
        #[doc = " `operation.metadata.value.start_time` in descending order starting from the"]
        #[doc = " most recently started operation."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instancePartitionOperations.list`"]
        #[doc = " permission on the resource"]
        #[doc = " [parent][google.spanner.admin.instance.v1.ListInstancePartitionOperationsRequest.parent]."]
        pub async fn list_instance_partition_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancePartitionOperationsRequest>,
        ) -> Result<tonic::Response<super::ListInstancePartitionOperationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstancePartitionOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
