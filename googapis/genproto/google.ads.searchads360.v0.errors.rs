// Proto file describing authentication errors.

/// Container for enum describing possible authentication errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationErrorEnum {}
/// Nested message and enum types in `AuthenticationErrorEnum`.
pub mod authentication_error_enum {
    /// Enum describing possible authentication errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AuthenticationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Authentication of the request failed.
        AuthenticationError = 2,
        /// Client customer ID is not a number.
        ClientCustomerIdInvalid = 5,
        /// No customer found for the provided customer ID.
        CustomerNotFound = 8,
        /// Client's Google account is deleted.
        GoogleAccountDeleted = 9,
        /// Account login token in the cookie is invalid.
        GoogleAccountCookieInvalid = 10,
        /// A problem occurred during Google account authentication.
        GoogleAccountAuthenticationFailed = 25,
        /// The user in the Google account login token does not match the user ID in
        /// the cookie.
        GoogleAccountUserAndAdsUserMismatch = 12,
        /// Login cookie is required for authentication.
        LoginCookieRequired = 13,
        /// The Google account that generated the OAuth access
        /// token is not associated with a Search Ads 360 account. Create a new
        /// account, or add the Google account to an existing Search Ads 360 account.
        NotAdsUser = 14,
        /// OAuth token in the header is not valid.
        OauthTokenInvalid = 15,
        /// OAuth token in the header has expired.
        OauthTokenExpired = 16,
        /// OAuth token in the header has been disabled.
        OauthTokenDisabled = 17,
        /// OAuth token in the header has been revoked.
        OauthTokenRevoked = 18,
        /// OAuth token HTTP header is malformed.
        OauthTokenHeaderInvalid = 19,
        /// Login cookie is not valid.
        LoginCookieInvalid = 20,
        /// User ID in the header is not a valid ID.
        UserIdInvalid = 22,
        /// An account administrator changed this account's authentication settings.
        /// To access this account, enable 2-Step Verification in your
        /// Google account at <https://www.google.com/landing/2step.>
        TwoStepVerificationNotEnrolled = 23,
        /// An account administrator changed this account's authentication settings.
        /// To access this account, enable Advanced Protection in your
        /// Google account at <https://landing.google.com/advancedprotection.>
        AdvancedProtectionNotEnrolled = 24,
    }
}
// Proto file describing authorization errors.

/// Container for enum describing possible authorization errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationErrorEnum {}
/// Nested message and enum types in `AuthorizationErrorEnum`.
pub mod authorization_error_enum {
    /// Enum describing possible authorization errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AuthorizationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// User doesn't have permission to access customer. Note: If you're
        /// accessing a client customer, the manager's customer ID must be set in the
        /// `login-customer-id` header. Learn more at
        /// <https://developers.google.com/search-ads/reporting/concepts/call-structure#login_customer_id_header>
        UserPermissionDenied = 2,
        /// The Google Cloud project sent in the request does not have permission to
        /// access the api.
        ProjectDisabled = 5,
        /// Authorization of the client failed.
        AuthorizationError = 6,
        /// The user does not have permission to perform this action
        /// (for example, ADD, UPDATE, REMOVE) on the resource or call a method.
        ActionNotPermitted = 7,
        /// Signup not complete.
        IncompleteSignup = 8,
        /// The customer account can't be accessed because it is not yet enabled or
        /// has been deactivated.
        CustomerNotEnabled = 24,
        /// The developer must sign the terms of service. They can be found here:
        /// <https://developers.google.com/terms>
        MissingTos = 9,
        /// The login customer specified does not have access to the account
        /// specified, so the request is invalid.
        InvalidLoginCustomerIdServingCustomerIdCombination = 11,
        /// The developer specified does not have access to the service.
        ServiceAccessDenied = 12,
        /// The customer (or login customer) isn't allowed in Search Ads 360 API. It
        /// belongs to another ads system.
        AccessDeniedForAccountType = 25,
        /// The developer does not have access to the metrics queried.
        MetricAccessDenied = 26,
    }
}
// Proto file describing CustomColumn errors.

/// Container for enum describing possible custom column errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomColumnErrorEnum {}
/// Nested message and enum types in `CustomColumnErrorEnum`.
pub mod custom_column_error_enum {
    /// Enum describing possible custom column errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomColumnError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The custom column has not been found.
        CustomColumnNotFound = 2,
        /// The custom column is not available.
        CustomColumnNotAvailable = 3,
    }
}
// Proto file describing date errors.

/// Container for enum describing possible date errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateErrorEnum {}
/// Nested message and enum types in `DateErrorEnum`.
pub mod date_error_enum {
    /// Enum describing possible date errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DateError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Given field values do not correspond to a valid date.
        InvalidFieldValuesInDate = 2,
        /// Given field values do not correspond to a valid date time.
        InvalidFieldValuesInDateTime = 3,
        /// The string date's format should be yyyy-mm-dd.
        InvalidStringDate = 4,
        /// The string date time's format should be yyyy-mm-dd hh:mm:ss.ssssss.
        InvalidStringDateTimeMicros = 6,
        /// The string date time's format should be yyyy-mm-dd hh:mm:ss.
        InvalidStringDateTimeSeconds = 11,
        /// The string date time's format should be yyyy-mm-dd hh:mm:ss+|-hh:mm.
        InvalidStringDateTimeSecondsWithOffset = 12,
        /// Date is before allowed minimum.
        EarlierThanMinimumDate = 7,
        /// Date is after allowed maximum.
        LaterThanMaximumDate = 8,
        /// Date range bounds are not in order.
        DateRangeMinimumDateLaterThanMaximumDate = 9,
        /// Both dates in range are null.
        DateRangeMinimumAndMaximumDatesBothNull = 10,
    }
}
// Proto file describing date range errors.

/// Container for enum describing possible date range errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRangeErrorEnum {}
/// Nested message and enum types in `DateRangeErrorEnum`.
pub mod date_range_error_enum {
    /// Enum describing possible date range errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DateRangeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Invalid date.
        InvalidDate = 2,
        /// The start date was after the end date.
        StartDateAfterEndDate = 3,
        /// Cannot set date to past time
        CannotSetDateToPast = 4,
        /// A date was used that is past the system "last" date.
        AfterMaximumAllowableDate = 5,
        /// Trying to change start date on a resource that has started.
        CannotModifyStartDateIfAlreadyStarted = 6,
    }
}
// Proto file describing distinct errors.

/// Container for enum describing possible distinct errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistinctErrorEnum {}
/// Nested message and enum types in `DistinctErrorEnum`.
pub mod distinct_error_enum {
    /// Enum describing possible distinct errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DistinctError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Duplicate element.
        DuplicateElement = 2,
        /// Duplicate type.
        DuplicateType = 3,
    }
}
// Proto file describing header errors.

/// Container for enum describing possible header errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderErrorEnum {}
/// Nested message and enum types in `HeaderErrorEnum`.
pub mod header_error_enum {
    /// Enum describing possible header errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HeaderError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The user selected customer ID could not be validated.
        InvalidUserSelectedCustomerId = 2,
        /// The login customer ID could not be validated.
        InvalidLoginCustomerId = 3,
    }
}
// Proto file describing internal errors.

/// Container for enum describing possible internal errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalErrorEnum {}
/// Nested message and enum types in `InternalErrorEnum`.
pub mod internal_error_enum {
    /// Enum describing possible internal errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InternalError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// API encountered unexpected internal error.
        InternalError = 2,
        /// The intended error code doesn't exist in specified API version. It will
        /// be released in a future API version.
        ErrorCodeNotPublished = 3,
        /// API encountered an unexpected transient error. The user
        /// should retry their request in these cases.
        TransientError = 4,
        /// The request took longer than a deadline.
        DeadlineExceeded = 5,
    }
}
// Proto file describing request errors.

/// Container for enum describing possible errors from an invalid parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidParameterErrorEnum {}
/// Nested message and enum types in `InvalidParameterErrorEnum`.
pub mod invalid_parameter_error_enum {
    /// Enum describing possible parameter errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InvalidParameterError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The specified currency code is invalid.
        InvalidCurrencyCode = 2,
    }
}
// Proto file describing query errors.

/// Container for enum describing possible query errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorEnum {}
/// Nested message and enum types in `QueryErrorEnum`.
pub mod query_error_enum {
    /// Enum describing possible query errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryError {
        /// Name unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Returned if all other query error reasons are not applicable.
        QueryError = 50,
        /// A condition used in the query references an invalid enum constant.
        BadEnumConstant = 18,
        /// Query contains an invalid escape sequence.
        BadEscapeSequence = 7,
        /// Field name is invalid.
        BadFieldName = 12,
        /// Limit value is invalid (for example, not a number)
        BadLimitValue = 15,
        /// Encountered number can not be parsed.
        BadNumber = 5,
        /// Invalid operator encountered.
        BadOperator = 3,
        /// Parameter unknown or not supported.
        BadParameterName = 61,
        /// Parameter have invalid value.
        BadParameterValue = 62,
        /// Invalid resource type was specified in the FROM clause.
        BadResourceTypeInFromClause = 45,
        /// Non-ASCII symbol encountered outside of strings.
        BadSymbol = 2,
        /// Value is invalid.
        BadValue = 4,
        /// Date filters fail to restrict date to a range smaller than 31 days.
        /// Applicable if the query is segmented by date.
        DateRangeTooWide = 36,
        /// Filters on date/week/month/quarter have a start date after
        /// end date.
        DateRangeTooNarrow = 60,
        /// Expected AND between values with BETWEEN operator.
        ExpectedAnd = 30,
        /// Expecting ORDER BY to have BY.
        ExpectedBy = 14,
        /// There was no dimension field selected.
        ExpectedDimensionFieldInSelectClause = 37,
        /// Missing filters on date related fields.
        ExpectedFiltersOnDateRange = 55,
        /// Missing FROM clause.
        ExpectedFrom = 44,
        /// The operator used in the conditions requires the value to be a list.
        ExpectedList = 41,
        /// Fields used in WHERE or ORDER BY clauses are missing from the SELECT
        /// clause.
        ExpectedReferencedFieldInSelectClause = 16,
        /// SELECT is missing at the beginning of query.
        ExpectedSelect = 13,
        /// A list was passed as a value to a condition whose operator expects a
        /// single value.
        ExpectedSingleValue = 42,
        /// Missing one or both values with BETWEEN operator.
        ExpectedValueWithBetweenOperator = 29,
        /// Invalid date format. Expected 'YYYY-MM-DD'.
        InvalidDateFormat = 38,
        /// Misaligned date value for the filter. The date should be the start of a
        /// week/month/quarter if the filtered field is
        /// segments.week/segments.month/segments.quarter.
        MisalignedDateForFilter = 64,
        /// Value passed was not a string when it should have been. For example, it
        /// was a number or unquoted literal.
        InvalidStringValue = 57,
        /// A String value passed to the BETWEEN operator does not parse as a date.
        InvalidValueWithBetweenOperator = 26,
        /// The value passed to the DURING operator is not a Date range literal
        InvalidValueWithDuringOperator = 22,
        /// A value was passed to the LIKE operator.
        InvalidValueWithLikeOperator = 56,
        /// An operator was provided that is inapplicable to the field being
        /// filtered.
        OperatorFieldMismatch = 35,
        /// A Condition was found with an empty list.
        ProhibitedEmptyListInCondition = 28,
        /// A condition used in the query references an unsupported enum constant.
        ProhibitedEnumConstant = 54,
        /// Fields that are not allowed to be selected together were included in
        /// the SELECT clause.
        ProhibitedFieldCombinationInSelectClause = 31,
        /// A field that is not orderable was included in the ORDER BY clause.
        ProhibitedFieldInOrderByClause = 40,
        /// A field that is not selectable was included in the SELECT clause.
        ProhibitedFieldInSelectClause = 23,
        /// A field that is not filterable was included in the WHERE clause.
        ProhibitedFieldInWhereClause = 24,
        /// Resource type specified in the FROM clause is not supported by this
        /// service.
        ProhibitedResourceTypeInFromClause = 43,
        /// A field that comes from an incompatible resource was included in the
        /// SELECT clause.
        ProhibitedResourceTypeInSelectClause = 48,
        /// A field that comes from an incompatible resource was included in the
        /// WHERE clause.
        ProhibitedResourceTypeInWhereClause = 58,
        /// A metric incompatible with the main resource or other selected
        /// segmenting resources was included in the SELECT or WHERE clause.
        ProhibitedMetricInSelectOrWhereClause = 49,
        /// A segment incompatible with the main resource or other selected
        /// segmenting resources was included in the SELECT or WHERE clause.
        ProhibitedSegmentInSelectOrWhereClause = 51,
        /// A segment in the SELECT clause is incompatible with a metric in the
        /// SELECT or WHERE clause.
        ProhibitedSegmentWithMetricInSelectOrWhereClause = 53,
        /// The value passed to the limit clause is too low.
        LimitValueTooLow = 25,
        /// Query has a string containing a newline character.
        ProhibitedNewlineInString = 8,
        /// List contains values of different types.
        ProhibitedValueCombinationInList = 10,
        /// The values passed to the BETWEEN operator are not of the same type.
        ProhibitedValueCombinationWithBetweenOperator = 21,
        /// Query contains unterminated string.
        StringNotTerminated = 6,
        /// Too many segments are specified in SELECT clause.
        TooManySegments = 34,
        /// Query is incomplete and cannot be parsed.
        UnexpectedEndOfQuery = 9,
        /// FROM clause cannot be specified in this query.
        UnexpectedFromClause = 47,
        /// Query contains one or more unrecognized fields.
        UnrecognizedField = 32,
        /// Query has an unexpected extra part.
        UnexpectedInput = 11,
        /// Metrics cannot be requested for a manager account. To retrieve metrics,
        /// issue separate requests against each client account under the manager
        /// account.
        RequestedMetricsForManager = 59,
        /// The number of values (right-hand-side operands) in a filter exceeds the
        /// limit.
        FilterHasTooManyValues = 63,
    }
}
// Proto file describing quota errors.

/// Container for enum describing possible quota errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaErrorEnum {}
/// Nested message and enum types in `QuotaErrorEnum`.
pub mod quota_error_enum {
    /// Enum describing possible quota errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QuotaError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Too many requests.
        ResourceExhausted = 2,
        /// Too many requests in a short amount of time.
        ResourceTemporarilyExhausted = 4,
    }
}
// Proto file describing request errors.

/// Container for enum describing possible request errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestErrorEnum {}
/// Nested message and enum types in `RequestErrorEnum`.
pub mod request_error_enum {
    /// Enum describing possible request errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RequestError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Resource name is required for this request.
        ResourceNameMissing = 3,
        /// Resource name provided is malformed.
        ResourceNameMalformed = 4,
        /// Resource name provided is malformed.
        BadResourceId = 17,
        /// Product name is invalid.
        InvalidProductName = 35,
        /// Customer ID is invalid.
        InvalidCustomerId = 16,
        /// Mutate operation should have either create, update, or remove specified.
        OperationRequired = 5,
        /// Requested resource not found.
        ResourceNotFound = 6,
        /// Next page token specified in user request is invalid.
        InvalidPageToken = 7,
        /// Next page token specified in user request has expired.
        ExpiredPageToken = 8,
        /// Page size specified in user request is invalid.
        InvalidPageSize = 22,
        /// Required field is missing.
        RequiredFieldMissing = 9,
        /// The field cannot be modified because it's immutable. It's also possible
        /// that the field can be modified using 'create' operation but not 'update'.
        ImmutableField = 11,
        /// Received too many entries in request.
        TooManyMutateOperations = 13,
        /// Request cannot be executed by a manager account.
        CannotBeExecutedByManagerAccount = 14,
        /// Mutate request was attempting to modify a readonly field.
        /// For instance, Budget fields can be requested for Ad Group,
        /// but are read-only for adGroups:mutate.
        CannotModifyForeignField = 15,
        /// Enum value is not permitted.
        InvalidEnumValue = 18,
        /// The login-customer-id parameter is required for this request.
        LoginCustomerIdParameterMissing = 20,
        /// Either login-customer-id or linked-customer-id parameter is required for
        /// this request.
        LoginOrLinkedCustomerIdParameterRequired = 34,
        /// page_token is set in the validate only request
        ValidateOnlyRequestHasPageToken = 21,
        /// return_summary_row cannot be enabled if request did not select any
        /// metrics field.
        CannotReturnSummaryRowForRequestWithoutMetrics = 29,
        /// return_summary_row should not be enabled for validate only requests.
        CannotReturnSummaryRowForValidateOnlyRequests = 30,
        /// return_summary_row parameter value should be the same between requests
        /// with page_token field set and their original request.
        InconsistentReturnSummaryRowValue = 31,
        /// The total results count cannot be returned if it was not requested in the
        /// original request.
        TotalResultsCountNotOriginallyRequested = 32,
        /// Deadline specified by the client was too short.
        RpcDeadlineTooShort = 33,
        /// The product associated with the request is not supported for the current
        /// request.
        ProductNotSupported = 37,
    }
}
// Proto file describing size limit errors.

/// Container for enum describing possible size limit errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeLimitErrorEnum {}
/// Nested message and enum types in `SizeLimitErrorEnum`.
pub mod size_limit_error_enum {
    /// Enum describing possible size limit errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SizeLimitError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The number of entries in the request exceeds the system limit, or the
        /// contents of the operations exceed transaction limits due to their size
        /// or complexity. Try reducing the number of entries per request.
        RequestSizeLimitExceeded = 2,
        /// The number of entries in the response exceeds the system limit.
        ResponseSizeLimitExceeded = 3,
    }
}
// Proto file describing the common error protos

/// Describes how a Search Ads 360 API call failed. It's returned inside
/// google.rpc.Status.details when a call fails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360Failure {
    /// The list of errors that occurred.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<SearchAds360Error>,
    /// The unique ID of the request that is used for debugging purposes.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// SearchAds360-specific error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360Error {
    /// An enum value that indicates which error occurred.
    #[prost(message, optional, tag = "1")]
    pub error_code: ::core::option::Option<ErrorCode>,
    /// A human-readable description of the error.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// The value that triggered the error.
    #[prost(message, optional, tag = "3")]
    pub trigger: ::core::option::Option<super::common::Value>,
    /// Describes the part of the request proto that caused the error.
    #[prost(message, optional, tag = "4")]
    pub location: ::core::option::Option<ErrorLocation>,
    /// Additional error details, which are returned by certain error codes. Most
    /// error codes do not include details.
    #[prost(message, optional, tag = "5")]
    pub details: ::core::option::Option<ErrorDetails>,
}
/// The error reason represented by type and enum.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorCode {
    /// The list of error enums
    #[prost(
        oneof = "error_code::ErrorCode",
        tags = "1, 5, 9, 10, 11, 17, 33, 34, 35, 66, 118, 144, 175"
    )]
    pub error_code: ::core::option::Option<error_code::ErrorCode>,
}
/// Nested message and enum types in `ErrorCode`.
pub mod error_code {
    /// The list of error enums
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ErrorCode {
        /// An error caused by the request
        #[prost(enumeration = "super::request_error_enum::RequestError", tag = "1")]
        RequestError(i32),
        /// An error with the query
        #[prost(enumeration = "super::query_error_enum::QueryError", tag = "5")]
        QueryError(i32),
        /// An error encountered when trying to authorize a user.
        #[prost(
            enumeration = "super::authorization_error_enum::AuthorizationError",
            tag = "9"
        )]
        AuthorizationError(i32),
        /// An unexpected server-side error.
        #[prost(enumeration = "super::internal_error_enum::InternalError", tag = "10")]
        InternalError(i32),
        /// An error with the amount of quota remaining.
        #[prost(enumeration = "super::quota_error_enum::QuotaError", tag = "11")]
        QuotaError(i32),
        /// Indicates failure to properly authenticate user.
        #[prost(
            enumeration = "super::authentication_error_enum::AuthenticationError",
            tag = "17"
        )]
        AuthenticationError(i32),
        /// The reasons for the date error
        #[prost(enumeration = "super::date_error_enum::DateError", tag = "33")]
        DateError(i32),
        /// The reasons for the date range error
        #[prost(
            enumeration = "super::date_range_error_enum::DateRangeError",
            tag = "34"
        )]
        DateRangeError(i32),
        /// The reasons for the distinct error
        #[prost(enumeration = "super::distinct_error_enum::DistinctError", tag = "35")]
        DistinctError(i32),
        /// The reasons for the header error.
        #[prost(enumeration = "super::header_error_enum::HeaderError", tag = "66")]
        HeaderError(i32),
        /// The reasons for the size limit error
        #[prost(
            enumeration = "super::size_limit_error_enum::SizeLimitError",
            tag = "118"
        )]
        SizeLimitError(i32),
        /// The reasons for the custom column error
        #[prost(
            enumeration = "super::custom_column_error_enum::CustomColumnError",
            tag = "144"
        )]
        CustomColumnError(i32),
        /// The reasons for invalid parameter errors.
        #[prost(
            enumeration = "super::invalid_parameter_error_enum::InvalidParameterError",
            tag = "175"
        )]
        InvalidParameterError(i32),
    }
}
/// Describes the part of the request proto that caused the error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLocation {
    /// A field path that indicates which field was invalid in the request.
    #[prost(message, repeated, tag = "2")]
    pub field_path_elements: ::prost::alloc::vec::Vec<error_location::FieldPathElement>,
}
/// Nested message and enum types in `ErrorLocation`.
pub mod error_location {
    /// A part of a field path.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldPathElement {
        /// The name of a field or a oneof
        #[prost(string, tag = "1")]
        pub field_name: ::prost::alloc::string::String,
        /// If field_name is a repeated field, this is the element that failed
        #[prost(int32, optional, tag = "3")]
        pub index: ::core::option::Option<i32>,
    }
}
/// Additional error details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    /// The error code that should have been returned, but wasn't. This is used
    /// when the error code is not published in the client specified version.
    #[prost(string, tag = "1")]
    pub unpublished_error_code: ::prost::alloc::string::String,
    /// Details on the quota error, including the scope (account or developer), the
    /// rate bucket name and the retry delay.
    #[prost(message, optional, tag = "4")]
    pub quota_error_details: ::core::option::Option<QuotaErrorDetails>,
}
/// Additional quota error details when there is QuotaError.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaErrorDetails {
    /// The rate scope of the quota limit.
    #[prost(enumeration = "quota_error_details::QuotaRateScope", tag = "1")]
    pub rate_scope: i32,
    /// The high level description of the quota bucket.
    /// Examples are "Get requests for standard access" or "Requests per account".
    #[prost(string, tag = "2")]
    pub rate_name: ::prost::alloc::string::String,
    /// Backoff period that customers should wait before sending next request.
    #[prost(message, optional, tag = "3")]
    pub retry_delay: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `QuotaErrorDetails`.
pub mod quota_error_details {
    /// Enum of possible scopes that quota buckets belong to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QuotaRateScope {
        /// Unspecified enum
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Per customer account quota
        Account = 2,
        /// Per project quota
        Developer = 3,
    }
}
