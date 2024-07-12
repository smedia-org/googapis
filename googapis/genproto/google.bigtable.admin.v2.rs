/// Encapsulates progress related information for a Cloud Bigtable long
/// running operation.
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
/// Storage media types for persisting Bigtable data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageType {
    /// The user did not specify a storage type.
    Unspecified = 0,
    /// Flash (SSD) storage should be used.
    Ssd = 1,
    /// Magnetic drive (HDD) storage should be used.
    Hdd = 2,
}
/// A collection of Bigtable \[Tables][google.bigtable.admin.v2.Table\] and
/// the resources that serve them.
/// All tables in an instance are served from all
/// \[Clusters][google.bigtable.admin.v2.Cluster\] in the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// The unique name of the instance. Values are of the form
    /// `projects/{project}/instances/\[a-z][a-z0-9\\-]+[a-z0-9\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The descriptive name for this instance as it appears in UIs.
    /// Can be changed at any time, but should be kept globally unique
    /// to avoid confusion.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// (`OutputOnly`)
    /// The current state of the instance.
    #[prost(enumeration = "instance::State", tag = "3")]
    pub state: i32,
    /// The type of the instance. Defaults to `PRODUCTION`.
    #[prost(enumeration = "instance::Type", tag = "4")]
    pub r#type: i32,
    /// Labels are a flexible and lightweight mechanism for organizing cloud
    /// resources into groups that reflect a customer's organizational needs and
    /// deployment strategies. They can be used to filter resources and aggregate
    /// metrics.
    ///
    /// * Label keys must be between 1 and 63 characters long and must conform to
    ///   the regular expression: `\[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}`.
    /// * Label values must be between 0 and 63 characters long and must conform to
    ///   the regular expression: `\[\p{Ll}\p{Lo}\p{N}_-\]{0,63}`.
    /// * No more than 64 labels can be associated with a given resource.
    /// * Keys and values must both be under 128 bytes.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. A server-assigned timestamp representing when this Instance
    /// was created. For instances created before this field was added (August
    /// 2021), this value is `seconds: 0, nanos: 1`.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Reserved for future use.
    #[prost(bool, optional, tag = "8")]
    pub satisfies_pzs: ::core::option::Option<bool>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Possible states of an instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the instance could not be determined.
        NotKnown = 0,
        /// The instance has been successfully created and can serve requests
        /// to its tables.
        Ready = 1,
        /// The instance is currently being created, and may be destroyed
        /// if the creation process encounters an error.
        Creating = 2,
    }
    /// The type of the instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The type of the instance is unspecified. If set when creating an
        /// instance, a `PRODUCTION` instance will be created. If set when updating
        /// an instance, the type will be left unchanged.
        Unspecified = 0,
        /// An instance meant for production use. `serve_nodes` must be set
        /// on the cluster.
        Production = 1,
        /// DEPRECATED: Prefer PRODUCTION for all use cases, as it no longer enforces
        /// a higher minimum node count than DEVELOPMENT.
        Development = 2,
    }
}
/// The Autoscaling targets for a Cluster. These determine the recommended nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingTargets {
    /// The cpu utilization that the Autoscaler should be trying to achieve.
    /// This number is on a scale from 0 (no utilization) to
    /// 100 (total utilization), and is limited between 10 and 80, otherwise it
    /// will return INVALID_ARGUMENT error.
    #[prost(int32, tag = "2")]
    pub cpu_utilization_percent: i32,
    /// The storage utilization that the Autoscaler should be trying to achieve.
    /// This number is limited between 2560 (2.5TiB) and 5120 (5TiB) for a SSD
    /// cluster and between 8192 (8TiB) and 16384 (16TiB) for an HDD cluster,
    /// otherwise it will return INVALID_ARGUMENT error. If this value is set to 0,
    /// it will be treated as if it were set to the default value: 2560 for SSD,
    /// 8192 for HDD.
    #[prost(int32, tag = "3")]
    pub storage_utilization_gib_per_node: i32,
}
/// Limits for the number of nodes a Cluster can autoscale up/down to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingLimits {
    /// Required. Minimum number of nodes to scale down to.
    #[prost(int32, tag = "1")]
    pub min_serve_nodes: i32,
    /// Required. Maximum number of nodes to scale up to.
    #[prost(int32, tag = "2")]
    pub max_serve_nodes: i32,
}
/// A resizable group of nodes in a particular cloud location, capable
/// of serving all \[Tables][google.bigtable.admin.v2.Table\] in the parent
/// \[Instance][google.bigtable.admin.v2.Instance\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// The unique name of the cluster. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/\[a-z][-a-z0-9\]*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The location where this cluster's nodes and storage reside. For
    /// best performance, clients should be located as close as possible to this
    /// cluster. Currently only zones are supported, so values should be of the
    /// form `projects/{project}/locations/{zone}`.
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    /// Output only. The current state of the cluster.
    #[prost(enumeration = "cluster::State", tag = "3")]
    pub state: i32,
    /// The number of nodes allocated to this cluster. More nodes enable higher
    /// throughput and more consistent performance.
    #[prost(int32, tag = "4")]
    pub serve_nodes: i32,
    /// Immutable. The type of storage used by this cluster to serve its
    /// parent instance's tables, unless explicitly overridden.
    #[prost(enumeration = "StorageType", tag = "5")]
    pub default_storage_type: i32,
    /// Immutable. The encryption configuration for CMEK-protected clusters.
    #[prost(message, optional, tag = "6")]
    pub encryption_config: ::core::option::Option<cluster::EncryptionConfig>,
    #[prost(oneof = "cluster::Config", tags = "7")]
    pub config: ::core::option::Option<cluster::Config>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// Autoscaling config for a cluster.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterAutoscalingConfig {
        /// Required. Autoscaling limits for this cluster.
        #[prost(message, optional, tag = "1")]
        pub autoscaling_limits: ::core::option::Option<super::AutoscalingLimits>,
        /// Required. Autoscaling targets for this cluster.
        #[prost(message, optional, tag = "2")]
        pub autoscaling_targets: ::core::option::Option<super::AutoscalingTargets>,
    }
    /// Configuration for a cluster.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterConfig {
        /// Autoscaling configuration for this cluster.
        #[prost(message, optional, tag = "1")]
        pub cluster_autoscaling_config: ::core::option::Option<ClusterAutoscalingConfig>,
    }
    /// Cloud Key Management Service (Cloud KMS) settings for a CMEK-protected
    /// cluster.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EncryptionConfig {
        /// Describes the Cloud KMS encryption key that will be used to protect the
        /// destination Bigtable cluster. The requirements for this key are:
        ///  1) The Cloud Bigtable service account associated with the project that
        ///  contains this cluster must be granted the
        ///  `cloudkms.cryptoKeyEncrypterDecrypter` role on the CMEK key.
        ///  2) Only regional keys can be used and the region of the CMEK key must
        ///  match the region of the cluster.
        ///  3) All clusters within an instance must use the same CMEK key.
        /// Values are of the form
        /// `projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}`
        #[prost(string, tag = "1")]
        pub kms_key_name: ::prost::alloc::string::String,
    }
    /// Possible states of a cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the cluster could not be determined.
        NotKnown = 0,
        /// The cluster has been successfully created and is ready to serve requests.
        Ready = 1,
        /// The cluster is currently being created, and may be destroyed
        /// if the creation process encounters an error.
        /// A cluster may not be able to serve requests while being created.
        Creating = 2,
        /// The cluster is currently being resized, and may revert to its previous
        /// node count if the process encounters an error.
        /// A cluster is still capable of serving requests while being resized,
        /// but may exhibit performance as if its number of allocated nodes is
        /// between the starting and requested states.
        Resizing = 3,
        /// The cluster has no backing nodes. The data (tables) still
        /// exist, but no operations can be performed on the cluster.
        Disabled = 4,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        /// Configuration for this cluster.
        #[prost(message, tag = "7")]
        ClusterConfig(ClusterConfig),
    }
}
/// A configuration object describing how Cloud Bigtable should treat traffic
/// from a particular end user application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppProfile {
    /// The unique name of the app profile. Values are of the form
    /// `projects/{project}/instances/{instance}/appProfiles/\[_a-zA-Z0-9][-_.a-zA-Z0-9\]*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Strongly validated etag for optimistic concurrency control. Preserve the
    /// value returned from `GetAppProfile` when calling `UpdateAppProfile` to
    /// fail the request if there has been a modification in the mean time. The
    /// `update_mask` of the request need not include `etag` for this protection
    /// to apply.
    /// See \[Wikipedia\](<https://en.wikipedia.org/wiki/HTTP_ETag>) and
    /// [RFC 7232](<https://tools.ietf.org/html/rfc7232#section-2.3>) for more
    /// details.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// Long form description of the use case for this AppProfile.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The routing policy for all read/write requests that use this app profile.
    /// A value must be explicitly set.
    #[prost(oneof = "app_profile::RoutingPolicy", tags = "5, 6")]
    pub routing_policy: ::core::option::Option<app_profile::RoutingPolicy>,
    /// Options for isolating this app profile's traffic from other use cases.
    #[prost(oneof = "app_profile::Isolation", tags = "7, 11, 10")]
    pub isolation: ::core::option::Option<app_profile::Isolation>,
}
/// Nested message and enum types in `AppProfile`.
pub mod app_profile {
    /// Read/write requests are routed to the nearest cluster in the instance, and
    /// will fail over to the nearest cluster that is available in the event of
    /// transient errors or delays. Clusters in a region are considered
    /// equidistant. Choosing this option sacrifices read-your-writes consistency
    /// to improve availability.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiClusterRoutingUseAny {
        /// The set of clusters to route to. The order is ignored; clusters will be
        /// tried in order of distance. If left empty, all clusters are eligible.
        #[prost(string, repeated, tag = "1")]
        pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Unconditionally routes all read/write requests to a specific cluster.
    /// This option preserves read-your-writes consistency but does not improve
    /// availability.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingleClusterRouting {
        /// The cluster to which read/write requests should be routed.
        #[prost(string, tag = "1")]
        pub cluster_id: ::prost::alloc::string::String,
        /// Whether or not `CheckAndMutateRow` and `ReadModifyWriteRow` requests are
        /// allowed by this app profile. It is unsafe to send these requests to
        /// the same table/row/column in multiple clusters.
        #[prost(bool, tag = "2")]
        pub allow_transactional_writes: bool,
    }
    /// Standard options for isolating this app profile's traffic from other use
    /// cases.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StandardIsolation {
        /// The priority of requests sent using this app profile.
        #[prost(enumeration = "Priority", tag = "1")]
        pub priority: i32,
    }
    /// Data Boost is a serverless compute capability that lets you run
    /// high-throughput read jobs on your Bigtable data, without impacting the
    /// performance of the clusters that handle your application traffic.
    /// Currently, Data Boost exclusively supports read-only use-cases with
    /// single-cluster routing.
    ///
    /// Data Boost reads are only guaranteed to see the results of writes that
    /// were written at least 30 minutes ago. This means newly written values may
    /// not become visible for up to 30m, and also means that old values may
    /// remain visible for up to 30m after being deleted or overwritten. To
    /// mitigate the staleness of the data, users may either wait 30m, or use
    /// CheckConsistency.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataBoostIsolationReadOnly {
        /// The Compute Billing Owner for this Data Boost App Profile.
        #[prost(
            enumeration = "data_boost_isolation_read_only::ComputeBillingOwner",
            optional,
            tag = "1"
        )]
        pub compute_billing_owner: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `DataBoostIsolationReadOnly`.
    pub mod data_boost_isolation_read_only {
        /// Compute Billing Owner specifies how usage should be accounted when using
        /// Data Boost. Compute Billing Owner also configures which Cloud Project is
        /// charged for relevant quota.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ComputeBillingOwner {
            /// Unspecified value.
            Unspecified = 0,
            /// The host Cloud Project containing the targeted Bigtable Instance /
            /// Table pays for compute.
            HostPays = 1,
        }
    }
    /// Possible priorities for an app profile. Note that higher priority writes
    /// can sometimes queue behind lower priority writes to the same tablet, as
    /// writes must be strictly sequenced in the durability log.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority {
        /// Default value. Mapped to PRIORITY_HIGH (the legacy behavior) on creation.
        Unspecified = 0,
        Low = 1,
        Medium = 2,
        High = 3,
    }
    /// The routing policy for all read/write requests that use this app profile.
    /// A value must be explicitly set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RoutingPolicy {
        /// Use a multi-cluster routing policy.
        #[prost(message, tag = "5")]
        MultiClusterRoutingUseAny(MultiClusterRoutingUseAny),
        /// Use a single-cluster routing policy.
        #[prost(message, tag = "6")]
        SingleClusterRouting(SingleClusterRouting),
    }
    /// Options for isolating this app profile's traffic from other use cases.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Isolation {
        /// This field has been deprecated in favor of `standard_isolation.priority`.
        /// If you set this field, `standard_isolation.priority` will be set instead.
        ///
        /// The priority of requests sent using this app profile.
        #[prost(enumeration = "Priority", tag = "7")]
        Priority(i32),
        /// The standard options used for isolating this app profile's traffic from
        /// other use cases.
        #[prost(message, tag = "11")]
        StandardIsolation(StandardIsolation),
        /// Specifies that this app profile is intended for read-only usage via the
        /// Data Boost feature.
        #[prost(message, tag = "10")]
        DataBoostIsolationReadOnly(DataBoostIsolationReadOnly),
    }
}
/// A tablet is a defined by a start and end key and is explained in
/// <https://cloud.google.com/bigtable/docs/overview#architecture> and
/// <https://cloud.google.com/bigtable/docs/performance#optimization.>
/// A Hot tablet is a tablet that exhibits high average cpu usage during the time
/// interval from start time to end time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotTablet {
    /// The unique name of the hot tablet. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/hotTablets/\[a-zA-Z0-9_-\]*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name of the table that contains the tablet. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/\[_a-zA-Z0-9][-_.a-zA-Z0-9\]*`.
    #[prost(string, tag = "2")]
    pub table_name: ::prost::alloc::string::String,
    /// Output only. The start time of the hot tablet.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The end time of the hot tablet.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Tablet Start Key (inclusive).
    #[prost(string, tag = "5")]
    pub start_key: ::prost::alloc::string::String,
    /// Tablet End Key (inclusive).
    #[prost(string, tag = "6")]
    pub end_key: ::prost::alloc::string::String,
    /// Output only. The average CPU usage spent by a node on this tablet over the
    /// start_time to end_time time range. The percentage is the amount of CPU used
    /// by the node to serve the tablet, from 0% (tablet was not interacted with)
    /// to 100% (the node spent all cycles serving the hot tablet).
    #[prost(float, tag = "7")]
    pub node_cpu_usage_percent: f32,
}
/// Request message for BigtableInstanceAdmin.CreateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The unique name of the project in which to create the new
    /// instance. Values are of the form `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to be used when referring to the new instance within its
    /// project, e.g., just `myinstance` rather than
    /// `projects/myproject/instances/myinstance`.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The instance to create.
    /// Fields marked `OutputOnly` must be left blank.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
    /// Required. The clusters to be created within the instance, mapped by desired
    /// cluster ID, e.g., just `mycluster` rather than
    /// `projects/myproject/instances/myinstance/clusters/mycluster`.
    /// Fields marked `OutputOnly` must be left blank.
    /// Currently, at most four clusters can be specified.
    #[prost(map = "string, message", tag = "4")]
    pub clusters: ::std::collections::HashMap<::prost::alloc::string::String, Cluster>,
}
/// Request message for BigtableInstanceAdmin.GetInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The unique name of the requested instance. Values are of the form
    /// `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BigtableInstanceAdmin.ListInstances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The unique name of the project for which a list of instances is
    /// requested. Values are of the form `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for BigtableInstanceAdmin.ListInstances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of requested instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Locations from which Instance information could not be retrieved,
    /// due to an outage or some other transient condition.
    /// Instances whose Clusters are all in one of the failed locations
    /// may be missing from `instances`, and Instances with at least one
    /// Cluster in a failed location may only have partial information returned.
    /// Values are of the form `projects/<project>/locations/<zone_id>`
    #[prost(string, repeated, tag = "2")]
    pub failed_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for BigtableInstanceAdmin.PartialUpdateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialUpdateInstanceRequest {
    /// Required. The Instance which will (partially) replace the current value.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// Required. The subset of Instance fields which should be replaced.
    /// Must be explicitly set.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for BigtableInstanceAdmin.DeleteInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The unique name of the instance to be deleted.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BigtableInstanceAdmin.CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The unique name of the instance in which to create the new
    /// cluster. Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to be used when referring to the new cluster within its
    /// instance, e.g., just `mycluster` rather than
    /// `projects/myproject/instances/myinstance/clusters/mycluster`.
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The cluster to be created.
    /// Fields marked `OutputOnly` must be left blank.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
}
/// Request message for BigtableInstanceAdmin.GetCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The unique name of the requested cluster. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BigtableInstanceAdmin.ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The unique name of the instance for which a list of clusters is
    /// requested. Values are of the form
    /// `projects/{project}/instances/{instance}`. Use `{instance} = '-'` to list
    /// Clusters for all Instances in a project, e.g.,
    /// `projects/myproject/instances/-`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for BigtableInstanceAdmin.ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// The list of requested clusters.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// Locations from which Cluster information could not be retrieved,
    /// due to an outage or some other transient condition.
    /// Clusters from these locations may be missing from `clusters`,
    /// or may only have partial information returned.
    /// Values are of the form `projects/<project>/locations/<zone_id>`
    #[prost(string, repeated, tag = "2")]
    pub failed_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for BigtableInstanceAdmin.DeleteCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The unique name of the cluster to be deleted. Values are of the
    /// form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The metadata for the Operation returned by CreateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceMetadata {
    /// The request that prompted the initiation of this CreateInstance operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<CreateInstanceRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by UpdateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceMetadata {
    /// The request that prompted the initiation of this UpdateInstance operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<PartialUpdateInstanceRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterMetadata {
    /// The request that prompted the initiation of this CreateCluster operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<CreateClusterRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Keys: the full `name` of each table that existed in the instance when
    /// CreateCluster was first called, i.e.
    /// `projects/<project>/instances/<instance>/tables/<table>`. Any table added
    /// to the instance by a later API call will be created in the new cluster by
    /// that API call, not this one.
    ///
    /// Values: information on how much of a table's data has been copied to the
    /// newly-created cluster so far.
    #[prost(map = "string, message", tag = "4")]
    pub tables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        create_cluster_metadata::TableProgress,
    >,
}
/// Nested message and enum types in `CreateClusterMetadata`.
pub mod create_cluster_metadata {
    /// Progress info for copying a table's data to the new cluster.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableProgress {
        /// Estimate of the size of the table to be copied.
        #[prost(int64, tag = "2")]
        pub estimated_size_bytes: i64,
        /// Estimate of the number of bytes copied so far for this table.
        /// This will eventually reach 'estimated_size_bytes' unless the table copy
        /// is CANCELLED.
        #[prost(int64, tag = "3")]
        pub estimated_copied_bytes: i64,
        #[prost(enumeration = "table_progress::State", tag = "4")]
        pub state: i32,
    }
    /// Nested message and enum types in `TableProgress`.
    pub mod table_progress {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum State {
            Unspecified = 0,
            /// The table has not yet begun copying to the new cluster.
            Pending = 1,
            /// The table is actively being copied to the new cluster.
            Copying = 2,
            /// The table has been fully copied to the new cluster.
            Completed = 3,
            /// The table was deleted before it finished copying to the new cluster.
            /// Note that tables deleted after completion will stay marked as
            /// COMPLETED, not CANCELLED.
            Cancelled = 4,
        }
    }
}
/// The metadata for the Operation returned by UpdateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterMetadata {
    /// The request that prompted the initiation of this UpdateCluster operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<Cluster>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by PartialUpdateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialUpdateClusterMetadata {
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "1")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "2")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The original request for PartialUpdateCluster.
    #[prost(message, optional, tag = "3")]
    pub original_request: ::core::option::Option<PartialUpdateClusterRequest>,
}
/// Request message for BigtableInstanceAdmin.PartialUpdateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialUpdateClusterRequest {
    /// Required. The Cluster which contains the partial updates to be applied,
    /// subject to the update_mask.
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Required. The subset of Cluster fields which should be replaced.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for BigtableInstanceAdmin.CreateAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAppProfileRequest {
    /// Required. The unique name of the instance in which to create the new app
    /// profile. Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to be used when referring to the new app profile within
    /// its instance, e.g., just `myprofile` rather than
    /// `projects/myproject/instances/myinstance/appProfiles/myprofile`.
    #[prost(string, tag = "2")]
    pub app_profile_id: ::prost::alloc::string::String,
    /// Required. The app profile to be created.
    /// Fields marked `OutputOnly` will be ignored.
    #[prost(message, optional, tag = "3")]
    pub app_profile: ::core::option::Option<AppProfile>,
    /// If true, ignore safety checks when creating the app profile.
    #[prost(bool, tag = "4")]
    pub ignore_warnings: bool,
}
/// Request message for BigtableInstanceAdmin.GetAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppProfileRequest {
    /// Required. The unique name of the requested app profile. Values are of the
    /// form `projects/{project}/instances/{instance}/appProfiles/{app_profile}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BigtableInstanceAdmin.ListAppProfiles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppProfilesRequest {
    /// Required. The unique name of the instance for which a list of app profiles
    /// is requested. Values are of the form
    /// `projects/{project}/instances/{instance}`.
    /// Use `{instance} = '-'` to list AppProfiles for all Instances in a project,
    /// e.g., `projects/myproject/instances/-`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of results per page.
    ///
    /// A page_size of zero lets the server choose the number of items to return.
    /// A page_size which is strictly positive will return at most that many items.
    /// A negative page_size will cause an error.
    ///
    /// Following the first request, subsequent paginated calls are not required
    /// to pass a page_size. If a page_size is set in subsequent calls, it must
    /// match the page_size given in the first request.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for BigtableInstanceAdmin.ListAppProfiles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppProfilesResponse {
    /// The list of requested app profiles.
    #[prost(message, repeated, tag = "1")]
    pub app_profiles: ::prost::alloc::vec::Vec<AppProfile>,
    /// Set if not all app profiles could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations from which AppProfile information could not be retrieved,
    /// due to an outage or some other transient condition.
    /// AppProfiles from these locations may be missing from `app_profiles`.
    /// Values are of the form `projects/<project>/locations/<zone_id>`
    #[prost(string, repeated, tag = "3")]
    pub failed_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for BigtableInstanceAdmin.UpdateAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppProfileRequest {
    /// Required. The app profile which will (partially) replace the current value.
    #[prost(message, optional, tag = "1")]
    pub app_profile: ::core::option::Option<AppProfile>,
    /// Required. The subset of app profile fields which should be replaced.
    /// If unset, all fields will be replaced.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If true, ignore safety checks when updating the app profile.
    #[prost(bool, tag = "3")]
    pub ignore_warnings: bool,
}
/// Request message for BigtableInstanceAdmin.DeleteAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAppProfileRequest {
    /// Required. The unique name of the app profile to be deleted. Values are of
    /// the form
    /// `projects/{project}/instances/{instance}/appProfiles/{app_profile}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. If true, ignore safety checks when deleting the app profile.
    #[prost(bool, tag = "2")]
    pub ignore_warnings: bool,
}
/// The metadata for the Operation returned by UpdateAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppProfileMetadata {}
/// Request message for BigtableInstanceAdmin.ListHotTablets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHotTabletsRequest {
    /// Required. The cluster name to list hot tablets.
    /// Value is in the following form:
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The start time to list hot tablets. The hot tablets in the response will
    /// have start times between the requested start time and end time. Start time
    /// defaults to Now if it is unset, and end time defaults to Now - 24 hours if
    /// it is unset. The start time should be less than the end time, and the
    /// maximum allowed time range between start time and end time is 48 hours.
    /// Start time and end time should have values between Now and Now - 14 days.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time to list hot tablets.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Maximum number of results per page.
    ///
    /// A page_size that is empty or zero lets the server choose the number of
    /// items to return. A page_size which is strictly positive will return at most
    /// that many items. A negative page_size will cause an error.
    ///
    /// Following the first request, subsequent paginated calls do not need a
    /// page_size field. If a page_size is set in subsequent calls, it must match
    /// the page_size given in the first request.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for BigtableInstanceAdmin.ListHotTablets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHotTabletsResponse {
    /// List of hot tablets in the tables of the requested cluster that fall
    /// within the requested time range. Hot tablets are ordered by node cpu usage
    /// percent. If there are multiple hot tablets that correspond to the same
    /// tablet within a 15-minute interval, only the hot tablet with the highest
    /// node cpu usage will be included in the response.
    #[prost(message, repeated, tag = "1")]
    pub hot_tablets: ::prost::alloc::vec::Vec<HotTablet>,
    /// Set if not all hot tablets could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod bigtable_instance_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for creating, configuring, and deleting Cloud Bigtable Instances and"]
    #[doc = " Clusters. Provides access to the Instance and Cluster schemas only, not the"]
    #[doc = " tables' metadata or data stored in those tables."]
    #[derive(Debug, Clone)]
    pub struct BigtableInstanceAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableInstanceAdminClient<T>
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
        ) -> BigtableInstanceAdminClient<InterceptedService<T, F>>
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
            BigtableInstanceAdminClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Create an instance within a project."]
        #[doc = ""]
        #[doc = " Note that exactly one of Cluster.serve_nodes and"]
        #[doc = " Cluster.cluster_config.cluster_autoscaling_config can be set. If"]
        #[doc = " serve_nodes is set to non-zero, then the cluster is manually scaled. If"]
        #[doc = " cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is"]
        #[doc = " enabled."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about an instance."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists information about instances in a project."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an instance within a project. This method updates only the display"]
        #[doc = " name and type for an Instance. To update other Instance properties, such as"]
        #[doc = " labels, use PartialUpdateInstance."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::Instance>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Partially updates an instance within a project. This method can modify all"]
        #[doc = " fields of an Instance and is the preferred way to update an Instance."]
        pub async fn partial_update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::PartialUpdateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/PartialUpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete an instance from a project."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a cluster within an instance."]
        #[doc = ""]
        #[doc = " Note that exactly one of Cluster.serve_nodes and"]
        #[doc = " Cluster.cluster_config.cluster_autoscaling_config can be set. If"]
        #[doc = " serve_nodes is set to non-zero, then the cluster is manually scaled. If"]
        #[doc = " cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is"]
        #[doc = " enabled."]
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a cluster."]
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists information about clusters in an instance."]
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a cluster within an instance."]
        #[doc = ""]
        #[doc = " Note that UpdateCluster does not support updating"]
        #[doc = " cluster_config.cluster_autoscaling_config. In order to update it, you"]
        #[doc = " must use PartialUpdateCluster."]
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::Cluster>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Partially updates a cluster within a project. This method is the preferred"]
        #[doc = " way to update a Cluster."]
        #[doc = ""]
        #[doc = " To enable and update autoscaling, set"]
        #[doc = " cluster_config.cluster_autoscaling_config. When autoscaling is enabled,"]
        #[doc = " serve_nodes is treated as an OUTPUT_ONLY field, meaning that updates to it"]
        #[doc = " are ignored. Note that an update cannot simultaneously set serve_nodes to"]
        #[doc = " non-zero and cluster_config.cluster_autoscaling_config to non-empty, and"]
        #[doc = " also specify both in the update_mask."]
        #[doc = ""]
        #[doc = " To disable autoscaling, clear cluster_config.cluster_autoscaling_config,"]
        #[doc = " and explicitly set a serve_node count via the update_mask."]
        pub async fn partial_update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::PartialUpdateClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/PartialUpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a cluster from an instance."]
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an app profile within an instance."]
        pub async fn create_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAppProfileRequest>,
        ) -> Result<tonic::Response<super::AppProfile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/CreateAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about an app profile."]
        pub async fn get_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppProfileRequest>,
        ) -> Result<tonic::Response<super::AppProfile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists information about app profiles in an instance."]
        pub async fn list_app_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppProfilesRequest>,
        ) -> Result<tonic::Response<super::ListAppProfilesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListAppProfiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an app profile within an instance."]
        pub async fn update_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAppProfileRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/UpdateAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an app profile from an instance."]
        pub async fn delete_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAppProfileRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/DeleteAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for an instance resource. Returns an empty"]
        #[doc = " policy if an instance exists but does not have a policy set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on an instance resource. Replaces any"]
        #[doc = " existing policy."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified instance resource."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists hot tablets in a cluster, within the time range provided. Hot"]
        #[doc = " tablets are ordered based on CPU usage."]
        pub async fn list_hot_tablets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHotTabletsRequest>,
        ) -> Result<tonic::Response<super::ListHotTabletsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListHotTablets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// `Type` represents the type of data that is written to, read from, or stored
/// in Bigtable. It is heavily based on the GoogleSQL standard to help maintain
/// familiarity and consistency across products and features.
///
/// For compatibility with Bigtable's existing untyped APIs, each `Type` includes
/// an `Encoding` which describes how to convert to/from the underlying data.
/// This might involve composing a series of steps into an "encoding chain," for
/// example to convert from INT64 -> STRING -> raw bytes. In most cases, a "link"
/// in the encoding chain will be based an on existing GoogleSQL conversion
/// function like `CAST`.
///
/// Each link in the encoding chain also defines the following properties:
///  * Natural sort: Does the encoded value sort consistently with the original
///    typed value? Note that Bigtable will always sort data based on the raw
///    encoded value, *not* the decoded type.
///     - Example: BYTES values sort in the same order as their raw encodings.
///     - Counterexample: Encoding INT64 to a fixed-width STRING does *not*
///       preserve sort order when dealing with negative numbers.
///       INT64(1) > INT64(-1), but STRING("-00001") > STRING("00001).
///     - The overall encoding chain has this property if *every* link does.
///  * Self-delimiting: If we concatenate two encoded values, can we always tell
///    where the first one ends and the second one begins?
///     - Example: If we encode INT64s to fixed-width STRINGs, the first value
///       will always contain exactly N digits, possibly preceded by a sign.
///     - Counterexample: If we concatenate two UTF-8 encoded STRINGs, we have
///       no way to tell where the first one ends.
///     - The overall encoding chain has this property if *any* link does.
///  * Compatibility: Which other systems have matching encoding schemes? For
///    example, does this encoding have a GoogleSQL equivalent? HBase? Java?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    /// The kind of type that this represents.
    #[prost(oneof = "r#type::Kind", tags = "1, 2, 5, 6")]
    pub kind: ::core::option::Option<r#type::Kind>,
}
/// Nested message and enum types in `Type`.
pub mod r#type {
    /// Bytes
    /// Values of type `Bytes` are stored in `Value.bytes_value`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Bytes {
        /// The encoding to use when converting to/from lower level types.
        #[prost(message, optional, tag = "1")]
        pub encoding: ::core::option::Option<bytes::Encoding>,
    }
    /// Nested message and enum types in `Bytes`.
    pub mod bytes {
        /// Rules used to convert to/from lower level types.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Encoding {
            /// Which encoding to use.
            #[prost(oneof = "encoding::Encoding", tags = "1")]
            pub encoding: ::core::option::Option<encoding::Encoding>,
        }
        /// Nested message and enum types in `Encoding`.
        pub mod encoding {
            /// Leaves the value "as-is"
            /// * Natural sort? Yes
            /// * Self-delimiting? No
            /// * Compatibility? N/A
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Raw {}
            /// Which encoding to use.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Encoding {
                /// Use `Raw` encoding.
                #[prost(message, tag = "1")]
                Raw(Raw),
            }
        }
    }
    /// String
    /// Values of type `String` are stored in `Value.string_value`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct String {
        /// The encoding to use when converting to/from lower level types.
        #[prost(message, optional, tag = "1")]
        pub encoding: ::core::option::Option<string::Encoding>,
    }
    /// Nested message and enum types in `String`.
    pub mod string {
        /// Rules used to convert to/from lower level types.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Encoding {
            /// Which encoding to use.
            #[prost(oneof = "encoding::Encoding", tags = "1")]
            pub encoding: ::core::option::Option<encoding::Encoding>,
        }
        /// Nested message and enum types in `Encoding`.
        pub mod encoding {
            /// UTF-8 encoding
            /// * Natural sort? No (ASCII characters only)
            /// * Self-delimiting? No
            /// * Compatibility?
            ///    - BigQuery Federation `TEXT` encoding
            ///    - HBase `Bytes.toBytes`
            ///    - Java `String#getBytes(StandardCharsets.UTF_8)`
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Utf8Raw {}
            /// Which encoding to use.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Encoding {
                /// Use `Utf8Raw` encoding.
                #[prost(message, tag = "1")]
                Utf8Raw(Utf8Raw),
            }
        }
    }
    /// Int64
    /// Values of type `Int64` are stored in `Value.int_value`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Int64 {
        /// The encoding to use when converting to/from lower level types.
        #[prost(message, optional, tag = "1")]
        pub encoding: ::core::option::Option<int64::Encoding>,
    }
    /// Nested message and enum types in `Int64`.
    pub mod int64 {
        /// Rules used to convert to/from lower level types.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Encoding {
            /// Which encoding to use.
            #[prost(oneof = "encoding::Encoding", tags = "1")]
            pub encoding: ::core::option::Option<encoding::Encoding>,
        }
        /// Nested message and enum types in `Encoding`.
        pub mod encoding {
            /// Encodes the value as an 8-byte big endian twos complement `Bytes`
            /// value.
            /// * Natural sort? No (positive values only)
            /// * Self-delimiting? Yes
            /// * Compatibility?
            ///    - BigQuery Federation `BINARY` encoding
            ///    - HBase `Bytes.toBytes`
            ///    - Java `ByteBuffer.putLong()` with `ByteOrder.BIG_ENDIAN`
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct BigEndianBytes {
                /// The underlying `Bytes` type, which may be able to encode further.
                #[prost(message, optional, tag = "1")]
                pub bytes_type: ::core::option::Option<super::super::Bytes>,
            }
            /// Which encoding to use.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Encoding {
                /// Use `BigEndianBytes` encoding.
                #[prost(message, tag = "1")]
                BigEndianBytes(BigEndianBytes),
            }
        }
    }
    /// A value that combines incremental updates into a summarized value.
    ///
    /// Data is never directly written or read using type `Aggregate`. Writes will
    /// provide either the `input_type` or `state_type`, and reads will always
    /// return the `state_type` .
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Aggregate {
        /// Type of the inputs that are accumulated by this `Aggregate`, which must
        /// specify a full encoding.
        /// Use `AddInput` mutations to accumulate new inputs.
        #[prost(message, optional, boxed, tag = "1")]
        pub input_type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        /// Output only. Type that holds the internal accumulator state for the
        /// `Aggregate`. This is a function of the `input_type` and `aggregator`
        /// chosen, and will always specify a full encoding.
        #[prost(message, optional, boxed, tag = "2")]
        pub state_type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        /// Which aggregator function to use. The configured types must match.
        #[prost(oneof = "aggregate::Aggregator", tags = "4")]
        pub aggregator: ::core::option::Option<aggregate::Aggregator>,
    }
    /// Nested message and enum types in `Aggregate`.
    pub mod aggregate {
        /// Computes the sum of the input values.
        /// Allowed input: `Int64`
        /// State: same as input
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Sum {}
        /// Which aggregator function to use. The configured types must match.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Aggregator {
            /// Sum aggregator.
            #[prost(message, tag = "4")]
            Sum(Sum),
        }
    }
    /// The kind of type that this represents.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Bytes
        #[prost(message, tag = "1")]
        BytesType(Bytes),
        /// String
        #[prost(message, tag = "2")]
        StringType(String),
        /// Int64
        #[prost(message, tag = "5")]
        Int64Type(Int64),
        /// Aggregate
        #[prost(message, tag = "6")]
        AggregateType(::prost::alloc::boxed::Box<Aggregate>),
    }
}
/// Information about a table restore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreInfo {
    /// The type of the restore source.
    #[prost(enumeration = "RestoreSourceType", tag = "1")]
    pub source_type: i32,
    /// Information about the source used to restore the table.
    #[prost(oneof = "restore_info::SourceInfo", tags = "2")]
    pub source_info: ::core::option::Option<restore_info::SourceInfo>,
}
/// Nested message and enum types in `RestoreInfo`.
pub mod restore_info {
    /// Information about the source used to restore the table.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceInfo {
        /// Information about the backup used to restore the table. The backup
        /// may no longer exist.
        #[prost(message, tag = "2")]
        BackupInfo(super::BackupInfo),
    }
}
/// Change stream configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStreamConfig {
    /// How long the change stream should be retained. Change stream data older
    /// than the retention period will not be returned when reading the change
    /// stream from the table.
    /// Values must be at least 1 day and at most 7 days, and will be truncated to
    /// microsecond granularity.
    #[prost(message, optional, tag = "1")]
    pub retention_period: ::core::option::Option<::prost_types::Duration>,
}
/// A collection of user data indexed by row, column, and timestamp.
/// Each table is served using the resources of its parent cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// The unique name of the table. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/\[_a-zA-Z0-9][-_.a-zA-Z0-9\]*`.
    /// Views: `NAME_ONLY`, `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Map from cluster ID to per-cluster table state.
    /// If it could not be determined whether or not the table has data in a
    /// particular cluster (for example, if its zone is unavailable), then
    /// there will be an entry for the cluster with UNKNOWN `replication_status`.
    /// Views: `REPLICATION_VIEW`, `ENCRYPTION_VIEW`, `FULL`
    #[prost(map = "string, message", tag = "2")]
    pub cluster_states:
        ::std::collections::HashMap<::prost::alloc::string::String, table::ClusterState>,
    /// The column families configured for this table, mapped by column family ID.
    /// Views: `SCHEMA_VIEW`, `STATS_VIEW`, `FULL`
    #[prost(map = "string, message", tag = "3")]
    pub column_families: ::std::collections::HashMap<::prost::alloc::string::String, ColumnFamily>,
    /// Immutable. The granularity (i.e. `MILLIS`) at which timestamps are stored
    /// in this table. Timestamps not matching the granularity will be rejected. If
    /// unspecified at creation time, the value will be set to `MILLIS`. Views:
    /// `SCHEMA_VIEW`, `FULL`.
    #[prost(enumeration = "table::TimestampGranularity", tag = "4")]
    pub granularity: i32,
    /// Output only. If this table was restored from another data source (e.g. a
    /// backup), this field will be populated with information about the restore.
    #[prost(message, optional, tag = "6")]
    pub restore_info: ::core::option::Option<RestoreInfo>,
    /// If specified, enable the change stream on this table.
    /// Otherwise, the change stream is disabled and the change stream is not
    /// retained.
    #[prost(message, optional, tag = "8")]
    pub change_stream_config: ::core::option::Option<ChangeStreamConfig>,
    /// Set to true to make the table protected against data loss. i.e. deleting
    /// the following resources through Admin APIs are prohibited:
    ///
    /// * The table.
    /// * The column families in the table.
    /// * The instance containing the table.
    ///
    /// Note one can still delete the data stored in the table through Data APIs.
    #[prost(bool, tag = "9")]
    pub deletion_protection: bool,
    #[prost(oneof = "table::AutomatedBackupConfig", tags = "13")]
    pub automated_backup_config: ::core::option::Option<table::AutomatedBackupConfig>,
}
/// Nested message and enum types in `Table`.
pub mod table {
    /// The state of a table's data in a particular cluster.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterState {
        /// Output only. The state of replication for the table in this cluster.
        #[prost(enumeration = "cluster_state::ReplicationState", tag = "1")]
        pub replication_state: i32,
        /// Output only. The encryption information for the table in this cluster.
        /// If the encryption key protecting this resource is customer managed, then
        /// its version can be rotated in Cloud Key Management Service (Cloud KMS).
        /// The primary version of the key and its status will be reflected here when
        /// changes propagate from Cloud KMS.
        #[prost(message, repeated, tag = "2")]
        pub encryption_info: ::prost::alloc::vec::Vec<super::EncryptionInfo>,
    }
    /// Nested message and enum types in `ClusterState`.
    pub mod cluster_state {
        /// Table replication states.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ReplicationState {
            /// The replication state of the table is unknown in this cluster.
            StateNotKnown = 0,
            /// The cluster was recently created, and the table must finish copying
            /// over pre-existing data from other clusters before it can begin
            /// receiving live replication updates and serving Data API requests.
            Initializing = 1,
            /// The table is temporarily unable to serve Data API requests from this
            /// cluster due to planned internal maintenance.
            PlannedMaintenance = 2,
            /// The table is temporarily unable to serve Data API requests from this
            /// cluster due to unplanned or emergency maintenance.
            UnplannedMaintenance = 3,
            /// The table can serve Data API requests from this cluster. Depending on
            /// replication delay, reads may not immediately reflect the state of the
            /// table in other clusters.
            Ready = 4,
            /// The table is fully created and ready for use after a restore, and is
            /// being optimized for performance. When optimizations are complete, the
            /// table will transition to `READY` state.
            ReadyOptimizing = 5,
        }
    }
    /// Defines an automated backup policy for a table
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutomatedBackupPolicy {
        /// Required. How long the automated backups should be retained. The only
        /// supported value at this time is 3 days.
        #[prost(message, optional, tag = "1")]
        pub retention_period: ::core::option::Option<::prost_types::Duration>,
        /// Required. How frequently automated backups should occur. The only
        /// supported value at this time is 24 hours.
        #[prost(message, optional, tag = "2")]
        pub frequency: ::core::option::Option<::prost_types::Duration>,
    }
    /// Possible timestamp granularities to use when keeping multiple versions
    /// of data in a table.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimestampGranularity {
        /// The user did not specify a granularity. Should not be returned.
        /// When specified during table creation, MILLIS will be used.
        Unspecified = 0,
        /// The table keeps data versioned at a granularity of 1ms.
        Millis = 1,
    }
    /// Defines a view over a table's fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum View {
        /// Uses the default view for each method as documented in its request.
        Unspecified = 0,
        /// Only populates `name`.
        NameOnly = 1,
        /// Only populates `name` and fields related to the table's schema.
        SchemaView = 2,
        /// Only populates `name` and fields related to the table's replication
        /// state.
        ReplicationView = 3,
        /// Only populates `name` and fields related to the table's encryption state.
        EncryptionView = 5,
        /// Populates all fields.
        Full = 4,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AutomatedBackupConfig {
        /// If specified, automated backups are enabled for this table.
        /// Otherwise, automated backups are disabled.
        #[prost(message, tag = "13")]
        AutomatedBackupPolicy(AutomatedBackupPolicy),
    }
}
/// AuthorizedViews represent subsets of a particular Cloud Bigtable table. Users
/// can configure access to each Authorized View independently from the table and
/// use the existing Data APIs to access the subset of data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedView {
    /// Identifier. The name of this AuthorizedView.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}/authorizedViews/{authorized_view}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The etag for this AuthorizedView.
    /// If this is provided on update, it must match the server's etag. The server
    /// returns ABORTED error on a mismatched etag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Set to true to make the AuthorizedView protected against deletion.
    /// The parent Table and containing Instance cannot be deleted if an
    /// AuthorizedView has this bit set.
    #[prost(bool, tag = "4")]
    pub deletion_protection: bool,
    /// The type of this AuthorizedView.
    #[prost(oneof = "authorized_view::AuthorizedView", tags = "2")]
    pub authorized_view: ::core::option::Option<authorized_view::AuthorizedView>,
}
/// Nested message and enum types in `AuthorizedView`.
pub mod authorized_view {
    /// Subsets of a column family that are included in this AuthorizedView.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FamilySubsets {
        /// Individual exact column qualifiers to be included in the AuthorizedView.
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub qualifiers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        /// Prefixes for qualifiers to be included in the AuthorizedView. Every
        /// qualifier starting with one of these prefixes is included in the
        /// AuthorizedView. To provide access to all qualifiers, include the empty
        /// string as a prefix
        /// ("").
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub qualifier_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    /// Defines a simple AuthorizedView that is a subset of the underlying Table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubsetView {
        /// Row prefixes to be included in the AuthorizedView.
        /// To provide access to all rows, include the empty string as a prefix ("").
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub row_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        /// Map from column family name to the columns in this family to be included
        /// in the AuthorizedView.
        #[prost(map = "string, message", tag = "2")]
        pub family_subsets:
            ::std::collections::HashMap<::prost::alloc::string::String, FamilySubsets>,
    }
    /// Defines a subset of an AuthorizedView's fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseView {
        /// Uses the default view for each method as documented in the request.
        Unspecified = 0,
        /// Only populates `name`.
        NameOnly = 1,
        /// Only populates the AuthorizedView's basic metadata. This includes:
        /// name, deletion_protection, etag.
        Basic = 2,
        /// Populates every fields.
        Full = 3,
    }
    /// The type of this AuthorizedView.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthorizedView {
        /// An AuthorizedView permitting access to an explicit subset of a Table.
        #[prost(message, tag = "2")]
        SubsetView(SubsetView),
    }
}
/// A set of columns within a table which share a common configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnFamily {
    /// Garbage collection rule specified as a protobuf.
    /// Must serialize to at most 500 bytes.
    ///
    /// NOTE: Garbage collection executes opportunistically in the background, and
    /// so it's possible for reads to return a cell even if it matches the active
    /// GC expression for its family.
    #[prost(message, optional, tag = "1")]
    pub gc_rule: ::core::option::Option<GcRule>,
    /// The type of data stored in each of this family's cell values, including its
    /// full encoding. If omitted, the family only serves raw untyped bytes.
    ///
    /// For now, only the `Aggregate` type is supported.
    ///
    /// `Aggregate` can only be set at family creation and is immutable afterwards.
    ///
    ///
    /// If `value_type` is `Aggregate`, written data must be compatible with:
    ///  * `value_type.input_type` for `AddInput` mutations
    #[prost(message, optional, tag = "3")]
    pub value_type: ::core::option::Option<Type>,
}
/// Rule for determining which cells to delete during garbage collection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcRule {
    /// Garbage collection rules.
    #[prost(oneof = "gc_rule::Rule", tags = "1, 2, 3, 4")]
    pub rule: ::core::option::Option<gc_rule::Rule>,
}
/// Nested message and enum types in `GcRule`.
pub mod gc_rule {
    /// A GcRule which deletes cells matching all of the given rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Intersection {
        /// Only delete cells which would be deleted by every element of `rules`.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::GcRule>,
    }
    /// A GcRule which deletes cells matching any of the given rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Union {
        /// Delete cells which would be deleted by any element of `rules`.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::GcRule>,
    }
    /// Garbage collection rules.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// Delete all cells in a column except the most recent N.
        #[prost(int32, tag = "1")]
        MaxNumVersions(i32),
        /// Delete cells in a column older than the given age.
        /// Values must be at least one millisecond, and will be truncated to
        /// microsecond granularity.
        #[prost(message, tag = "2")]
        MaxAge(::prost_types::Duration),
        /// Delete cells that would be deleted by every nested rule.
        #[prost(message, tag = "3")]
        Intersection(Intersection),
        /// Delete cells that would be deleted by any nested rule.
        #[prost(message, tag = "4")]
        Union(Union),
    }
}
/// Encryption information for a given resource.
/// If this resource is protected with customer managed encryption, the in-use
/// Cloud Key Management Service (Cloud KMS) key version is specified along with
/// its status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionInfo {
    /// Output only. The type of encryption used to protect this resource.
    #[prost(enumeration = "encryption_info::EncryptionType", tag = "3")]
    pub encryption_type: i32,
    /// Output only. The status of encrypt/decrypt calls on underlying data for
    /// this resource. Regardless of status, the existing data is always encrypted
    /// at rest.
    #[prost(message, optional, tag = "4")]
    pub encryption_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. The version of the Cloud KMS key specified in the parent
    /// cluster that is in use for the data underlying this table.
    #[prost(string, tag = "2")]
    pub kms_key_version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EncryptionInfo`.
pub mod encryption_info {
    /// Possible encryption types for a resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EncryptionType {
        /// Encryption type was not specified, though data at rest remains encrypted.
        Unspecified = 0,
        /// The data backing this resource is encrypted at rest with a key that is
        /// fully managed by Google. No key version or status will be populated.
        /// This is the default state.
        GoogleDefaultEncryption = 1,
        /// The data backing this resource is encrypted at rest with a key that is
        /// managed by the customer.
        /// The in-use version of the key and its status are populated for
        /// CMEK-protected tables.
        /// CMEK-protected backups are pinned to the key version that was in use at
        /// the time the backup was taken. This key version is populated but its
        /// status is not tracked and is reported as `UNKNOWN`.
        CustomerManagedEncryption = 2,
    }
}
/// A snapshot of a table at a particular time. A snapshot can be used as a
/// checkpoint for data restoration or a data source for a new table.
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The unique name of the snapshot.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The source table at the time the snapshot was taken.
    #[prost(message, optional, tag = "2")]
    pub source_table: ::core::option::Option<Table>,
    /// Output only. The size of the data in the source table at the time the
    /// snapshot was taken. In some cases, this value may be computed
    /// asynchronously via a background process and a placeholder of 0 will be used
    /// in the meantime.
    #[prost(int64, tag = "3")]
    pub data_size_bytes: i64,
    /// Output only. The time when the snapshot is created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the snapshot will be deleted. The maximum amount of time a
    /// snapshot can stay active is 365 days. If 'ttl' is not specified,
    /// the default maximum of 365 days will be used.
    #[prost(message, optional, tag = "5")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the snapshot.
    #[prost(enumeration = "snapshot::State", tag = "6")]
    pub state: i32,
    /// Description of the snapshot.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Snapshot`.
pub mod snapshot {
    /// Possible states of a snapshot.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the snapshot could not be determined.
        NotKnown = 0,
        /// The snapshot has been successfully created and can serve all requests.
        Ready = 1,
        /// The snapshot is currently being created, and may be destroyed if the
        /// creation process encounters an error. A snapshot may not be restored to a
        /// table while it is being created.
        Creating = 2,
    }
}
/// A backup of a Cloud Bigtable table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// A globally unique identifier for the backup which cannot be
    /// changed. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/
    ///    backups/\[_a-zA-Z0-9][-_.a-zA-Z0-9\]*`
    /// The final segment of the name must be between 1 and 50 characters
    /// in length.
    ///
    /// The backup is stored in the cluster identified by the prefix of the backup
    /// name of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Name of the table from which this backup was created.
    /// This needs to be in the same instance as the backup. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{source_table}`.
    #[prost(string, tag = "2")]
    pub source_table: ::prost::alloc::string::String,
    /// Output only. Name of the backup from which this backup was copied. If a
    /// backup is not created by copying a backup, this field will be empty. Values
    /// are of the form: projects/<project>/instances/<instance>/backups/<backup>.
    #[prost(string, tag = "10")]
    pub source_backup: ::prost::alloc::string::String,
    /// Required. The expiration time of the backup, with microseconds
    /// granularity that must be at least 6 hours and at most 90 days
    /// from the time the request is received. Once the `expire_time`
    /// has passed, Cloud Bigtable will delete the backup and free the
    /// resources used by the backup.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. `start_time` is the time that the backup was started
    /// (i.e. approximately the time the
    /// \[CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup\]
    /// request is received).  The row data in this backup will be no older than
    /// this timestamp.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. `end_time` is the time that the backup was finished. The row
    /// data in the backup will be no newer than this timestamp.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Size of the backup in bytes.
    #[prost(int64, tag = "6")]
    pub size_bytes: i64,
    /// Output only. The current state of the backup.
    #[prost(enumeration = "backup::State", tag = "7")]
    pub state: i32,
    /// Output only. The encryption information for the backup.
    #[prost(message, optional, tag = "9")]
    pub encryption_info: ::core::option::Option<EncryptionInfo>,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// Indicates the current state of the backup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The pending backup is still being created. Operations on the
        /// backup may fail with `FAILED_PRECONDITION` in this state.
        Creating = 1,
        /// The backup is complete and ready for use.
        Ready = 2,
    }
}
/// Information about a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupInfo {
    /// Output only. Name of the backup.
    #[prost(string, tag = "1")]
    pub backup: ::prost::alloc::string::String,
    /// Output only. The time that the backup was started. Row data in the backup
    /// will be no older than this timestamp.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. This time that the backup was finished. Row data in the
    /// backup will be no newer than this timestamp.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Name of the table the backup was created from.
    #[prost(string, tag = "4")]
    pub source_table: ::prost::alloc::string::String,
    /// Output only. Name of the backup from which this backup was copied. If a
    /// backup is not created by copying a backup, this field will be empty. Values
    /// are of the form: projects/<project>/instances/<instance>/backups/<backup>.
    #[prost(string, tag = "10")]
    pub source_backup: ::prost::alloc::string::String,
}
/// Indicates the type of the restore source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RestoreSourceType {
    /// No restore associated.
    Unspecified = 0,
    /// A backup was used as the source of the restore.
    Backup = 1,
}
/// The request for
/// \[RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreTableRequest {
    /// Required. The name of the instance in which to create the restored
    /// table. Values are of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the table to create and restore to. This
    /// table must not already exist. The `table_id` appended to
    /// `parent` forms the full table name of the form
    /// `projects/<project>/instances/<instance>/tables/<table_id>`.
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    /// Required. The source from which to restore.
    #[prost(oneof = "restore_table_request::Source", tags = "3")]
    pub source: ::core::option::Option<restore_table_request::Source>,
}
/// Nested message and enum types in `RestoreTableRequest`.
pub mod restore_table_request {
    /// Required. The source from which to restore.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Name of the backup from which to restore.  Values are of the form
        /// `projects/<project>/instances/<instance>/clusters/<cluster>/backups/<backup>`.
        #[prost(string, tag = "3")]
        Backup(::prost::alloc::string::String),
    }
}
/// Metadata type for the long-running operation returned by
/// \[RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreTableMetadata {
    /// Name of the table being created and restored to.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the restore source.
    #[prost(enumeration = "RestoreSourceType", tag = "2")]
    pub source_type: i32,
    /// If exists, the name of the long-running operation that will be used to
    /// track the post-restore optimization process to optimize the performance of
    /// the restored table. The metadata type of the long-running operation is
    /// \[OptimizeRestoreTableMetadata][\]. The response type is
    /// \[Empty][google.protobuf.Empty\]. This long-running operation may be
    /// automatically created by the system if applicable after the
    /// RestoreTable long-running operation completes successfully. This operation
    /// may not be created if the table is already optimized or the restore was
    /// not successful.
    #[prost(string, tag = "4")]
    pub optimize_table_operation_name: ::prost::alloc::string::String,
    /// The progress of the
    /// \[RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable\]
    /// operation.
    #[prost(message, optional, tag = "5")]
    pub progress: ::core::option::Option<OperationProgress>,
    /// Information about the source used to restore the table, as specified by
    /// `source` in
    /// \[RestoreTableRequest][google.bigtable.admin.v2.RestoreTableRequest\].
    #[prost(oneof = "restore_table_metadata::SourceInfo", tags = "3")]
    pub source_info: ::core::option::Option<restore_table_metadata::SourceInfo>,
}
/// Nested message and enum types in `RestoreTableMetadata`.
pub mod restore_table_metadata {
    /// Information about the source used to restore the table, as specified by
    /// `source` in
    /// \[RestoreTableRequest][google.bigtable.admin.v2.RestoreTableRequest\].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceInfo {
        #[prost(message, tag = "3")]
        BackupInfo(super::BackupInfo),
    }
}
/// Metadata type for the long-running operation used to track the progress
/// of optimizations performed on a newly restored table. This long-running
/// operation is automatically created by the system after the successful
/// completion of a table restore, and cannot be cancelled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizeRestoredTableMetadata {
    /// Name of the restored table being optimized.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The progress of the post-restore optimizations.
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<OperationProgress>,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.CreateTable][google.bigtable.admin.v2.BigtableTableAdmin.CreateTable\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// Required. The unique name of the instance in which to create the table.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The name by which the new table should be referred to within the
    /// parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`.
    /// Maximum 50 characters.
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    /// Required. The Table to create.
    #[prost(message, optional, tag = "3")]
    pub table: ::core::option::Option<Table>,
    /// The optional list of row keys that will be used to initially split the
    /// table into several tablets (tablets are similar to HBase regions).
    /// Given two split keys, `s1` and `s2`, three tablets will be created,
    /// spanning the key ranges: `[, s1), [s1, s2), [s2, )`.
    ///
    /// Example:
    ///
    /// * Row keys := `["a", "apple", "custom", "customer_1", "customer_2",`
    ///                `"other", "zz"]`
    /// * initial_split_keys := `["apple", "customer_1", "customer_2", "other"]`
    /// * Key assignment:
    ///     - Tablet 1 `[, apple)                => {"a"}.`
    ///     - Tablet 2 `[apple, customer_1)      => {"apple", "custom"}.`
    ///     - Tablet 3 `[customer_1, customer_2) => {"customer_1"}.`
    ///     - Tablet 4 `[customer_2, other)      => {"customer_2"}.`
    ///     - Tablet 5 `[other, )                => {"other", "zz"}.`
    #[prost(message, repeated, tag = "4")]
    pub initial_splits: ::prost::alloc::vec::Vec<create_table_request::Split>,
}
/// Nested message and enum types in `CreateTableRequest`.
pub mod create_table_request {
    /// An initial split point for a newly created table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Split {
        /// Row key to use as an initial tablet boundary.
        #[prost(bytes = "vec", tag = "1")]
        pub key: ::prost::alloc::vec::Vec<u8>,
    }
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.CreateTableFromSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.CreateTableFromSnapshot\]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableFromSnapshotRequest {
    /// Required. The unique name of the instance in which to create the table.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The name by which the new table should be referred to within the
    /// parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`.
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    /// Required. The unique name of the snapshot from which to restore the table.
    /// The snapshot and the table must be in the same instance. Values are of the
    /// form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "3")]
    pub source_snapshot: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange][google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropRowRangeRequest {
    /// Required. The unique name of the table on which to drop a range of rows.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Delete all rows or by prefix.
    #[prost(oneof = "drop_row_range_request::Target", tags = "2, 3")]
    pub target: ::core::option::Option<drop_row_range_request::Target>,
}
/// Nested message and enum types in `DropRowRangeRequest`.
pub mod drop_row_range_request {
    /// Delete all rows or by prefix.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Delete all rows that start with this row key prefix. Prefix cannot be
        /// zero length.
        #[prost(bytes, tag = "2")]
        RowKeyPrefix(::prost::alloc::vec::Vec<u8>),
        /// Delete all rows in the table. Setting this to false is a no-op.
        #[prost(bool, tag = "3")]
        DeleteAllDataFromTable(bool),
    }
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ListTables][google.bigtable.admin.v2.BigtableTableAdmin.ListTables\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesRequest {
    /// Required. The unique name of the instance for which tables should be
    /// listed. Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The view to be applied to the returned tables' fields.
    /// NAME_ONLY view (default) and REPLICATION_VIEW are supported.
    #[prost(enumeration = "table::View", tag = "2")]
    pub view: i32,
    /// Maximum number of results per page.
    ///
    /// A page_size of zero lets the server choose the number of items to return.
    /// A page_size which is strictly positive will return at most that many items.
    /// A negative page_size will cause an error.
    ///
    /// Following the first request, subsequent paginated calls are not required
    /// to pass a page_size. If a page_size is set in subsequent calls, it must
    /// match the page_size given in the first request.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ListTables][google.bigtable.admin.v2.BigtableTableAdmin.ListTables\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesResponse {
    /// The tables present in the requested instance.
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
    /// Set if not all tables could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.GetTable][google.bigtable.admin.v2.BigtableTableAdmin.GetTable\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableRequest {
    /// Required. The unique name of the requested table.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The view to be applied to the returned table's fields.
    /// Defaults to `SCHEMA_VIEW` if unspecified.
    #[prost(enumeration = "table::View", tag = "2")]
    pub view: i32,
}
/// The request for
/// \[UpdateTable][google.bigtable.admin.v2.BigtableTableAdmin.UpdateTable\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTableRequest {
    /// Required. The table to update.
    /// The table's `name` field is used to identify the table to update.
    #[prost(message, optional, tag = "1")]
    pub table: ::core::option::Option<Table>,
    /// Required. The list of fields to update.
    /// A mask specifying which fields (e.g. `change_stream_config`) in the `table`
    /// field should be updated. This mask is relative to the `table` field, not to
    /// the request message. The wildcard (*) path is currently not supported.
    /// Currently UpdateTable is only supported for the following fields:
    ///
    /// * `change_stream_config`
    /// * `change_stream_config.retention_period`
    /// * `deletion_protection`
    ///
    /// If `column_families` is set in `update_mask`, it will return an
    /// UNIMPLEMENTED error.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Metadata type for the operation returned by
/// \[UpdateTable][google.bigtable.admin.v2.BigtableTableAdmin.UpdateTable\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTableMetadata {
    /// The name of the table being updated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time at which this operation started.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation finished or was canceled.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.DeleteTable][google.bigtable.admin.v2.BigtableTableAdmin.DeleteTable\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTableRequest {
    /// Required. The unique name of the table to be deleted.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.UndeleteTable][google.bigtable.admin.v2.BigtableTableAdmin.UndeleteTable\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteTableRequest {
    /// Required. The unique name of the table to be restored.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata type for the operation returned by
/// \[google.bigtable.admin.v2.BigtableTableAdmin.UndeleteTable][google.bigtable.admin.v2.BigtableTableAdmin.UndeleteTable\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteTableMetadata {
    /// The name of the table being restored.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time at which this operation started.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation finished or was cancelled.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies][google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyColumnFamiliesRequest {
    /// Required. The unique name of the table whose families should be modified.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Modifications to be atomically applied to the specified table's
    /// families. Entries are applied in order, meaning that earlier modifications
    /// can be masked by later ones (in the case of repeated updates to the same
    /// family, for example).
    #[prost(message, repeated, tag = "2")]
    pub modifications: ::prost::alloc::vec::Vec<modify_column_families_request::Modification>,
    /// Optional. If true, ignore safety checks when modifying the column families.
    #[prost(bool, tag = "3")]
    pub ignore_warnings: bool,
}
/// Nested message and enum types in `ModifyColumnFamiliesRequest`.
pub mod modify_column_families_request {
    /// A create, update, or delete of a particular column family.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Modification {
        /// The ID of the column family to be modified.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Optional. A mask specifying which fields (e.g. `gc_rule`) in the `update`
        /// mod should be updated, ignored for other modification types. If unset or
        /// empty, we treat it as updating `gc_rule` to be backward compatible.
        #[prost(message, optional, tag = "6")]
        pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Column family modifications.
        #[prost(oneof = "modification::Mod", tags = "2, 3, 4")]
        pub r#mod: ::core::option::Option<modification::Mod>,
    }
    /// Nested message and enum types in `Modification`.
    pub mod modification {
        /// Column family modifications.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Mod {
            /// Create a new column family with the specified schema, or fail if
            /// one already exists with the given ID.
            #[prost(message, tag = "2")]
            Create(super::super::ColumnFamily),
            /// Update an existing column family to the specified schema, or fail
            /// if no column family exists with the given ID.
            #[prost(message, tag = "3")]
            Update(super::super::ColumnFamily),
            /// Drop (delete) the column family with the given ID, or fail if no such
            /// family exists.
            #[prost(bool, tag = "4")]
            Drop(bool),
        }
    }
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken][google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConsistencyTokenRequest {
    /// Required. The unique name of the Table for which to create a consistency
    /// token. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken][google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConsistencyTokenResponse {
    /// The generated consistency token.
    #[prost(string, tag = "1")]
    pub consistency_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency][google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyRequest {
    /// Required. The unique name of the Table for which to check replication
    /// consistency. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The token created using GenerateConsistencyToken for the Table.
    #[prost(string, tag = "2")]
    pub consistency_token: ::prost::alloc::string::String,
    /// Which type of read needs to consistently observe which type of write?
    /// Default: `standard_read_remote_writes`
    #[prost(oneof = "check_consistency_request::Mode", tags = "3, 4")]
    pub mode: ::core::option::Option<check_consistency_request::Mode>,
}
/// Nested message and enum types in `CheckConsistencyRequest`.
pub mod check_consistency_request {
    /// Which type of read needs to consistently observe which type of write?
    /// Default: `standard_read_remote_writes`
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Checks that reads using an app profile with `StandardIsolation` can
        /// see all writes committed before the token was created, even if the
        /// read and write target different clusters.
        #[prost(message, tag = "3")]
        StandardReadRemoteWrites(super::StandardReadRemoteWrites),
        /// Checks that reads using an app profile with `DataBoostIsolationReadOnly`
        /// can see all writes committed before the token was created, but only if
        /// the read and write target the same cluster.
        #[prost(message, tag = "4")]
        DataBoostReadLocalWrites(super::DataBoostReadLocalWrites),
    }
}
/// Checks that all writes before the consistency token was generated are
/// replicated in every cluster and readable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardReadRemoteWrites {}
/// Checks that all writes before the consistency token was generated in the same
/// cluster are readable by Databoost.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataBoostReadLocalWrites {}
/// Response message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency][google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyResponse {
    /// True only if the token is consistent. A token is consistent if replication
    /// has caught up with the restrictions specified in the request.
    #[prost(bool, tag = "1")]
    pub consistent: bool,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.SnapshotTable][google.bigtable.admin.v2.BigtableTableAdmin.SnapshotTable\]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotTableRequest {
    /// Required. The unique name of the table to have the snapshot taken.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the cluster where the snapshot will be created in.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "2")]
    pub cluster: ::prost::alloc::string::String,
    /// Required. The ID by which the new snapshot should be referred to within the
    /// parent cluster, e.g., `mysnapshot` of the form:
    /// `\[_a-zA-Z0-9][-_.a-zA-Z0-9\]*` rather than
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/mysnapshot`.
    #[prost(string, tag = "3")]
    pub snapshot_id: ::prost::alloc::string::String,
    /// The amount of time that the new snapshot can stay active after it is
    /// created. Once 'ttl' expires, the snapshot will get deleted. The maximum
    /// amount of time a snapshot can stay active is 7 days. If 'ttl' is not
    /// specified, the default value of 24 hours will be used.
    #[prost(message, optional, tag = "4")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
    /// Description of the snapshot.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.GetSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.GetSnapshot\]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    /// Required. The unique name of the requested snapshot.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots][google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots\]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// Required. The unique name of the cluster for which snapshots should be
    /// listed. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    /// Use `{cluster} = '-'` to list snapshots for all clusters in an instance,
    /// e.g., `projects/{project}/instances/{instance}/clusters/-`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of snapshots to return per page.
    /// CURRENTLY UNIMPLEMENTED AND IGNORED.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots][google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots\]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    /// The snapshots present in the requested cluster.
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
    /// Set if not all snapshots could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.DeleteSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.DeleteSnapshot\]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotRequest {
    /// Required. The unique name of the snapshot to be deleted.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The metadata for the Operation returned by SnapshotTable.
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotTableMetadata {
    /// The request that prompted the initiation of this SnapshotTable operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<SnapshotTableRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by CreateTableFromSnapshot.
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableFromSnapshotMetadata {
    /// The request that prompted the initiation of this CreateTableFromSnapshot
    /// operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<CreateTableFromSnapshotRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request for
/// \[CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. This must be one of the clusters in the instance in which this
    /// table is located. The backup will be stored in this cluster. Values are
    /// of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the backup to be created. The `backup_id` along with
    /// the parent `parent` are combined as {parent}/backups/{backup_id} to create
    /// the full backup name, of the form:
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup_id}`.
    /// This string must be between 1 and 50 characters in length and match the
    /// regex \[_a-zA-Z0-9][-_.a-zA-Z0-9\]*.
    #[prost(string, tag = "2")]
    pub backup_id: ::prost::alloc::string::String,
    /// Required. The backup to create.
    #[prost(message, optional, tag = "3")]
    pub backup: ::core::option::Option<Backup>,
}
/// Metadata type for the operation returned by
/// \[CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupMetadata {
    /// The name of the backup being created.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the table the backup is created from.
    #[prost(string, tag = "2")]
    pub source_table: ::prost::alloc::string::String,
    /// The time at which this operation started.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation finished or was cancelled.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request for
/// \[UpdateBackup][google.bigtable.admin.v2.BigtableTableAdmin.UpdateBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. The backup to update. `backup.name`, and the fields to be updated
    /// as specified by `update_mask` are required. Other fields are ignored.
    /// Update is only supported for the following fields:
    ///
    ///  * `backup.expire_time`.
    #[prost(message, optional, tag = "1")]
    pub backup: ::core::option::Option<Backup>,
    /// Required. A mask specifying which fields (e.g. `expire_time`) in the
    /// Backup resource should be updated. This mask is relative to the Backup
    /// resource, not to the request message. The field mask must always be
    /// specified; this prevents any future fields from being erased accidentally
    /// by clients that do not know about them.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// \[GetBackup][google.bigtable.admin.v2.BigtableTableAdmin.GetBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. Name of the backup.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[DeleteBackup][google.bigtable.admin.v2.BigtableTableAdmin.DeleteBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. Name of the backup to delete.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The cluster to list backups from.  Values are of the
    /// form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    /// Use `{cluster} = '-'` to list backups for all clusters in an instance,
    /// e.g., `projects/{project}/instances/{instance}/clusters/-`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// A filter expression that filters backups listed in the response.
    /// The expression must specify the field name, a comparison operator,
    /// and the value that you want to use for filtering. The value must be a
    /// string, a number, or a boolean. The comparison operator must be
    /// <, >, <=, >=, !=, =, or :. Colon ':' represents a HAS operator which is
    /// roughly synonymous with equality. Filter rules are case insensitive.
    ///
    /// The fields eligible for filtering are:
    ///
    /// * `name`
    /// * `source_table`
    /// * `state`
    /// * `start_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    /// * `end_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    /// * `expire_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    /// * `size_bytes`
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. By default, each expression is an AND expression. However,
    /// you can include AND, OR, and NOT expressions explicitly.
    ///
    /// Some examples of using filters are:
    ///
    /// * `name:"exact"` --> The backup's name is the string "exact".
    /// * `name:howl` --> The backup's name contains the string "howl".
    /// * `source_table:prod`
    ///        --> The source_table's name contains the string "prod".
    /// * `state:CREATING` --> The backup is pending creation.
    /// * `state:READY` --> The backup is fully created and ready for use.
    /// * `(name:howl) AND (start_time < \"2018-03-28T14:50:00Z\")`
    ///        --> The backup name contains the string "howl" and start_time
    ///            of the backup is before 2018-03-28T14:50:00Z.
    /// * `size_bytes > 10000000000` --> The backup's size is greater than 10GB
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// An expression for specifying the sort order of the results of the request.
    /// The string value should specify one or more fields in
    /// \[Backup][google.bigtable.admin.v2.Backup\]. The full syntax is described at
    /// <https://aip.dev/132#ordering.>
    ///
    /// Fields supported are:
    ///
    /// * name
    /// * source_table
    /// * expire_time
    /// * start_time
    /// * end_time
    /// * size_bytes
    /// * state
    ///
    /// For example, "start_time". The default sorting order is ascending.
    /// To specify descending order for the field, a suffix " desc" should
    /// be appended to the field name. For example, "start_time desc".
    /// Redundant space characters in the syntax are insigificant.
    ///
    /// If order_by is empty, results will be sorted by `start_time` in descending
    /// order starting from the most recently created backup.
    #[prost(string, tag = "3")]
    pub order_by: ::prost::alloc::string::String,
    /// Number of backups to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// \[next_page_token][google.bigtable.admin.v2.ListBackupsResponse.next_page_token\]
    /// from a previous
    /// \[ListBackupsResponse][google.bigtable.admin.v2.ListBackupsResponse\] to the
    /// same `parent` and with the same `filter`.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// The list of matching backups.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// `next_page_token` can be sent in a subsequent
    /// \[ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups\] call
    /// to fetch more of the matching backups.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[CopyBackup][google.bigtable.admin.v2.BigtableTableAdmin.CopyBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyBackupRequest {
    /// Required. The name of the destination cluster that will contain the backup
    /// copy. The cluster must already exists. Values are of the form:
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the new backup. The `backup_id` along with `parent`
    /// are combined as {parent}/backups/{backup_id} to create the full backup
    /// name, of the form:
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup_id}`.
    /// This string must be between 1 and 50 characters in length and match the
    /// regex \[_a-zA-Z0-9][-_.a-zA-Z0-9\]*.
    #[prost(string, tag = "2")]
    pub backup_id: ::prost::alloc::string::String,
    /// Required. The source backup to be copied from.
    /// The source backup needs to be in READY state for it to be copied.
    /// Copying a copied backup is not allowed.
    /// Once CopyBackup is in progress, the source backup cannot be deleted or
    /// cleaned up on expiration until CopyBackup is finished.
    /// Values are of the form:
    /// `projects/<project>/instances/<instance>/clusters/<cluster>/backups/<backup>`.
    #[prost(string, tag = "3")]
    pub source_backup: ::prost::alloc::string::String,
    /// Required. Required. The expiration time of the copied backup with
    /// microsecond granularity that must be at least 6 hours and at most 30 days
    /// from the time the request is received. Once the `expire_time` has
    /// passed, Cloud Bigtable will delete the backup and free the resources used
    /// by the backup.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata type for the google.longrunning.Operation returned by
/// \[CopyBackup][google.bigtable.admin.v2.BigtableTableAdmin.CopyBackup\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyBackupMetadata {
    /// The name of the backup being created through the copy operation.
    /// Values are of the form
    /// `projects/<project>/instances/<instance>/clusters/<cluster>/backups/<backup>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the source backup that is being copied from.
    #[prost(message, optional, tag = "2")]
    pub source_backup_info: ::core::option::Option<BackupInfo>,
    /// The progress of the
    /// \[CopyBackup][google.bigtable.admin.v2.BigtableTableAdmin.CopyBackup\]
    /// operation.
    #[prost(message, optional, tag = "3")]
    pub progress: ::core::option::Option<OperationProgress>,
}
/// The request for
/// \[CreateAuthorizedView][google.bigtable.admin.v2.BigtableTableAdmin.CreateAuthorizedView\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizedViewRequest {
    /// Required. This is the name of the table the AuthorizedView belongs to.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id of the AuthorizedView to create. This AuthorizedView must
    /// not already exist. The `authorized_view_id` appended to `parent` forms the
    /// full AuthorizedView name of the form
    /// `projects/{project}/instances/{instance}/tables/{table}/authorizedView/{authorized_view}`.
    #[prost(string, tag = "2")]
    pub authorized_view_id: ::prost::alloc::string::String,
    /// Required. The AuthorizedView to create.
    #[prost(message, optional, tag = "3")]
    pub authorized_view: ::core::option::Option<AuthorizedView>,
}
/// The metadata for the Operation returned by CreateAuthorizedView.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizedViewMetadata {
    /// The request that prompted the initiation of this CreateInstance operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<CreateAuthorizedViewRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ListAuthorizedViews][google.bigtable.admin.v2.BigtableTableAdmin.ListAuthorizedViews\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedViewsRequest {
    /// Required. The unique name of the table for which AuthorizedViews should be
    /// listed. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Maximum number of results per page.
    ///
    /// A page_size of zero lets the server choose the number of items to return.
    /// A page_size which is strictly positive will return at most that many items.
    /// A negative page_size will cause an error.
    ///
    /// Following the first request, subsequent paginated calls are not required
    /// to pass a page_size. If a page_size is set in subsequent calls, it must
    /// match the page_size given in the first request.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The resource_view to be applied to the returned views' fields.
    /// Default to NAME_ONLY.
    #[prost(enumeration = "authorized_view::ResponseView", tag = "4")]
    pub view: i32,
}
/// Response message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.ListAuthorizedViews][google.bigtable.admin.v2.BigtableTableAdmin.ListAuthorizedViews\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizedViewsResponse {
    /// The AuthorizedViews present in the requested table.
    #[prost(message, repeated, tag = "1")]
    pub authorized_views: ::prost::alloc::vec::Vec<AuthorizedView>,
    /// Set if not all tables could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.GetAuthorizedView][google.bigtable.admin.v2.BigtableTableAdmin.GetAuthorizedView\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizedViewRequest {
    /// Required. The unique name of the requested AuthorizedView.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}/authorizedViews/{authorized_view}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The resource_view to be applied to the returned AuthorizedView's
    /// fields. Default to BASIC.
    #[prost(enumeration = "authorized_view::ResponseView", tag = "2")]
    pub view: i32,
}
/// The request for
/// \[UpdateAuthorizedView][google.bigtable.admin.v2.BigtableTableAdmin.UpdateAuthorizedView\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizedViewRequest {
    /// Required. The AuthorizedView to update. The `name` in `authorized_view` is
    /// used to identify the AuthorizedView. AuthorizedView name must in this
    /// format
    /// projects/<project>/instances/<instance>/tables/<table>/authorizedViews/<authorized_view>
    #[prost(message, optional, tag = "1")]
    pub authorized_view: ::core::option::Option<AuthorizedView>,
    /// Optional. The list of fields to update.
    /// A mask specifying which fields in the AuthorizedView resource should be
    /// updated. This mask is relative to the AuthorizedView resource, not to the
    /// request message. A field will be overwritten if it is in the mask. If
    /// empty, all fields set in the request will be overwritten. A special value
    /// `*` means to overwrite all fields (including fields not set in the
    /// request).
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. If true, ignore the safety checks when updating the
    /// AuthorizedView.
    #[prost(bool, tag = "3")]
    pub ignore_warnings: bool,
}
/// Metadata for the google.longrunning.Operation returned by
/// \[UpdateAuthorizedView][google.bigtable.admin.v2.BigtableTableAdmin.UpdateAuthorizedView\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizedViewMetadata {
    /// The request that prompted the initiation of this UpdateAuthorizedView
    /// operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::core::option::Option<UpdateAuthorizedViewRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for
/// \[google.bigtable.admin.v2.BigtableTableAdmin.DeleteAuthorizedView][google.bigtable.admin.v2.BigtableTableAdmin.DeleteAuthorizedView\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAuthorizedViewRequest {
    /// Required. The unique name of the AuthorizedView to be deleted.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}/authorizedViews/{authorized_view}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The current etag of the AuthorizedView.
    /// If an etag is provided and does not match the current etag of the
    /// AuthorizedView, deletion will be blocked and an ABORTED error will be
    /// returned.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod bigtable_table_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for creating, configuring, and deleting Cloud Bigtable tables."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Provides access to the table schemas only, not the data stored within"]
    #[doc = " the tables."]
    #[derive(Debug, Clone)]
    pub struct BigtableTableAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableTableAdminClient<T>
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
        ) -> BigtableTableAdminClient<InterceptedService<T, F>>
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
            BigtableTableAdminClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new table in the specified instance."]
        #[doc = " The table can be created with a full set of initial column families,"]
        #[doc = " specified in the request."]
        pub async fn create_table(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new table from the specified snapshot. The target table must"]
        #[doc = " not exist. The snapshot and the table must be in the same instance."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn create_table_from_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableFromSnapshotRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateTableFromSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all tables served from a specified instance."]
        pub async fn list_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTablesRequest>,
        ) -> Result<tonic::Response<super::ListTablesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListTables",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata information about the specified table."]
        pub async fn get_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a specified table."]
        pub async fn update_table(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTableRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/UpdateTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a specified table and all of its data."]
        pub async fn delete_table(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTableRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores a specified table which was accidentally deleted."]
        pub async fn undelete_table(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteTableRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/UndeleteTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new AuthorizedView in a table."]
        pub async fn create_authorized_view(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuthorizedViewRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateAuthorizedView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all AuthorizedViews from a specific table."]
        pub async fn list_authorized_views(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizedViewsRequest>,
        ) -> Result<tonic::Response<super::ListAuthorizedViewsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListAuthorizedViews",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information from a specified AuthorizedView."]
        pub async fn get_authorized_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthorizedViewRequest>,
        ) -> Result<tonic::Response<super::AuthorizedView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetAuthorizedView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an AuthorizedView in a table."]
        pub async fn update_authorized_view(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAuthorizedViewRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/UpdateAuthorizedView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a specified AuthorizedView."]
        pub async fn delete_authorized_view(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAuthorizedViewRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteAuthorizedView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs a series of column family modifications on the specified table."]
        #[doc = " Either all or none of the modifications will occur before this method"]
        #[doc = " returns, but data requests received prior to that point may see a table"]
        #[doc = " where only some modifications have taken effect."]
        pub async fn modify_column_families(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyColumnFamiliesRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ModifyColumnFamilies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently drop/delete a row range from a specified table. The request can"]
        #[doc = " specify whether to delete all rows in a table, or only those that match a"]
        #[doc = " particular prefix."]
        pub async fn drop_row_range(
            &mut self,
            request: impl tonic::IntoRequest<super::DropRowRangeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DropRowRange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a consistency token for a Table, which can be used in"]
        #[doc = " CheckConsistency to check whether mutations to the table that finished"]
        #[doc = " before this call started have been replicated. The tokens will be available"]
        #[doc = " for 90 days."]
        pub async fn generate_consistency_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateConsistencyTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateConsistencyTokenResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GenerateConsistencyToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Checks replication consistency based on a consistency token, that is, if"]
        #[doc = " replication has caught up based on the conditions specified in the token"]
        #[doc = " and the check request."]
        pub async fn check_consistency(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckConsistencyRequest>,
        ) -> Result<tonic::Response<super::CheckConsistencyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/CheckConsistency",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new snapshot in the specified cluster from the specified"]
        #[doc = " source table. The cluster and the table must be in the same instance."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn snapshot_table(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapshotTableRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/SnapshotTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata information about the specified snapshot."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all snapshots associated with the specified cluster."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListSnapshots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes the specified snapshot."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn delete_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts creating a new Cloud Bigtable Backup.  The returned backup"]
        #[doc = " [long-running operation][google.longrunning.Operation] can be used to"]
        #[doc = " track creation of the backup. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateBackupMetadata][google.bigtable.admin.v2.CreateBackupMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Backup][google.bigtable.admin.v2.Backup], if successful. Cancelling the"]
        #[doc = " returned operation will stop the creation and delete the backup."]
        pub async fn create_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata on a pending or completed Cloud Bigtable Backup."]
        pub async fn get_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a pending or completed Cloud Bigtable Backup."]
        pub async fn update_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/UpdateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a pending or completed Cloud Bigtable backup."]
        pub async fn delete_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Cloud Bigtable backups. Returns both completed and pending"]
        #[doc = " backups."]
        pub async fn list_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupsRequest>,
        ) -> Result<tonic::Response<super::ListBackupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListBackups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new table by restoring from a completed backup.  The"]
        #[doc = " returned table [long-running operation][google.longrunning.Operation] can"]
        #[doc = " be used to track the progress of the operation, and to cancel it.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [RestoreTableMetadata][google.bigtable.admin.RestoreTableMetadata].  The"]
        #[doc = " [response][google.longrunning.Operation.response] type is"]
        #[doc = " [Table][google.bigtable.admin.v2.Table], if successful."]
        pub async fn restore_table(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreTableRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/RestoreTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Copy a Cloud Bigtable backup to a new backup in the destination cluster"]
        #[doc = " located in the destination instance and project."]
        pub async fn copy_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::CopyBackupRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/CopyBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a Table or Backup resource."]
        #[doc = " Returns an empty policy if the resource exists but does not have a policy"]
        #[doc = " set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on a Table or Backup resource."]
        #[doc = " Replaces any existing policy."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified Table or Backup"]
        #[doc = " resource."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
