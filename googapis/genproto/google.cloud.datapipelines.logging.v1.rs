/// Cloud Logging structured payload for events generated from Data Pipelines API
/// requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLogEntry {
    /// Type of the Data Pipelines API request.
    #[prost(enumeration = "request_log_entry::RequestType", tag = "1")]
    pub request_type: i32,
    /// The resulting status of the Data Pipelines API request.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Cause of the error status.
    #[prost(enumeration = "request_log_entry::ErrorCause", tag = "3")]
    pub error_cause: i32,
}
/// Nested message and enum types in `RequestLogEntry`.
pub mod request_log_entry {
    /// Type of a Data Pipelines API request.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RequestType {
        /// Default value. This value is not used.
        Unspecified = 0,
        /// A Data Pipelines Create Pipeline request.
        CreatePipeline = 1,
        /// A Data Pipelines Update Pipeline request.
        UpdatePipeline = 2,
        /// A Data Pipelines Delete Pipeline request.
        DeletePipeline = 3,
        /// A Data Pipelines List Pipelines request.
        ListPipelines = 4,
        /// A Data Pipelines Get Pipeline request.
        GetPipeline = 5,
        /// A Data Pipelines Stop Pipeline request.
        StopPipeline = 6,
        /// A Data Pipelines Run Pipeline request.
        RunPipeline = 7,
        /// A Data Pipelines List Jobs request.
        ListJobs = 8,
    }
    /// Cause code for a Data Pipelines API request error.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ErrorCause {
        /// Default value. This value is not used.
        Unspecified = 0,
        /// The request is invalid.
        InvalidRequest = 1,
        /// Failed to fetch project number for the provided project id.
        ProjectNumberNotFound = 2,
        /// The given pipeline already exists.
        PipelineIdAlreadyExists = 3,
        /// Failed to allocate a token for the per project pipeline count quota.
        PipelineQuotaAllocationFailed = 4,
        /// The given pipeline is not found.
        PipelineNotFound = 5,
        /// The pipeline's workload is invalid.
        InvalidPipelineWorkload = 6,
        /// The user cannot act as the Dataflow worker service account.
        DataflowWorkerServiceAccountPermissionDenied = 7,
        /// The user cannot act as the Cloud Scheduler service account.
        CloudSchedulerServiceAccountPermissionDenied = 8,
        /// Issues related to the per service per project service account.
        InternalDataPipelinesServiceAccountIssue = 9,
        /// Invalid argument in Cloud Scheduler service call.
        CloudSchedulerInvalidArgument = 10,
        /// Exceeds Cloud Scheduler service quota limit.
        CloudSchedulerResourceExhausted = 11,
        /// Cloud Scheduler job not found.
        CloudSchedulerJobNotFound = 12,
        /// Other Cloud Scheduler related issues.
        OtherCloudSchedulerIssue = 13,
        /// Dataflow job with the same name already exists.
        DataflowJobAlreadyExists = 14,
        /// Invalid argument in Dataflow service call.
        DataflowInvalidArgument = 15,
        /// Exceeds Dataflow service quota limit.
        DataflowResourceExhausted = 16,
        /// Dataflow job not found.
        DataflowJobNotFound = 17,
        /// Other Dataflow related issues.
        OtherDataflowIssue = 18,
        /// Database related issues.
        DatabaseError = 19,
        /// Request with the wrong pipeline type. For example, RunPipeline cannot be
        /// used with a streaming pipeline.
        WrongPipelineType = 20,
        /// Issues related to other Google internal services/systems.
        InternalError = 21,
        /// Cannot find the given pipeline or project.
        PipelineOrProjectNotFound = 22,
    }
}
