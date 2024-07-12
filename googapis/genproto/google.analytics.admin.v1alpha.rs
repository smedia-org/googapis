/// Dimensions are attributes of your data. For example, the dimension
/// `userEmail` indicates the email of the user that accessed reporting data.
/// Dimension values in report responses are strings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDimension {
    /// The API name of the dimension. See [Data Access
    /// Schema](<https://developers.google.com/analytics/devguides/config/admin/v1/access-api-schema>)
    /// for the list of dimensions supported in this API.
    ///
    /// Dimensions are referenced by name in `dimensionFilter` and `orderBys`.
    #[prost(string, tag = "1")]
    pub dimension_name: ::prost::alloc::string::String,
}
/// The quantitative measurements of a report. For example, the metric
/// `accessCount` is the total number of data access records.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessMetric {
    /// The API name of the metric. See [Data Access
    /// Schema](<https://developers.google.com/analytics/devguides/config/admin/v1/access-api-schema>)
    /// for the list of metrics supported in this API.
    ///
    /// Metrics are referenced by name in `metricFilter` & `orderBys`.
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
}
/// A contiguous range of days: startDate, startDate + 1, ..., endDate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDateRange {
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot
    /// be after `endDate`. The format `NdaysAgo`, `yesterday`, or `today` is also
    /// accepted, and in that case, the date is inferred based on the current time
    /// in the request's time zone.
    #[prost(string, tag = "1")]
    pub start_date: ::prost::alloc::string::String,
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot
    /// be before `startDate`. The format `NdaysAgo`, `yesterday`, or `today` is
    /// also accepted, and in that case, the date is inferred based on the current
    /// time in the request's time zone.
    #[prost(string, tag = "2")]
    pub end_date: ::prost::alloc::string::String,
}
/// Expresses dimension or metric filters. The fields in the same expression need
/// to be either all dimensions or all metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessFilterExpression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[prost(oneof = "access_filter_expression::OneExpression", tags = "1, 2, 3, 4")]
    pub one_expression: ::core::option::Option<access_filter_expression::OneExpression>,
}
/// Nested message and enum types in `AccessFilterExpression`.
pub mod access_filter_expression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneExpression {
        /// Each of the FilterExpressions in the and_group has an AND relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::AccessFilterExpressionList),
        /// Each of the FilterExpressions in the or_group has an OR relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::AccessFilterExpressionList),
        /// The FilterExpression is NOT of not_expression.
        #[prost(message, tag = "3")]
        NotExpression(::prost::alloc::boxed::Box<super::AccessFilterExpression>),
        /// A primitive filter. In the same FilterExpression, all of the filter's
        /// field names need to be either all dimensions or all metrics.
        #[prost(message, tag = "4")]
        AccessFilter(super::AccessFilter),
    }
}
/// A list of filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessFilterExpressionList {
    /// A list of filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<AccessFilterExpression>,
}
/// An expression to filter dimension or metric values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessFilter {
    /// The dimension name or metric name.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "access_filter::OneFilter", tags = "2, 3, 4, 5")]
    pub one_filter: ::core::option::Option<access_filter::OneFilter>,
}
/// Nested message and enum types in `AccessFilter`.
pub mod access_filter {
    /// Specify one type of filter for `Filter`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "2")]
        StringFilter(super::AccessStringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "3")]
        InListFilter(super::AccessInListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "4")]
        NumericFilter(super::AccessNumericFilter),
        /// A filter for two values.
        #[prost(message, tag = "5")]
        BetweenFilter(super::AccessBetweenFilter),
    }
}
/// The filter for strings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessStringFilter {
    /// The match type for this filter.
    #[prost(enumeration = "access_string_filter::MatchType", tag = "1")]
    pub match_type: i32,
    /// The string value used for the matching.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// If true, the string value is case sensitive.
    #[prost(bool, tag = "3")]
    pub case_sensitive: bool,
}
/// Nested message and enum types in `AccessStringFilter`.
pub mod access_string_filter {
    /// The match type of a string filter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchType {
        /// Unspecified
        Unspecified = 0,
        /// Exact match of the string value.
        Exact = 1,
        /// Begins with the string value.
        BeginsWith = 2,
        /// Ends with the string value.
        EndsWith = 3,
        /// Contains the string value.
        Contains = 4,
        /// Full match for the regular expression with the string value.
        FullRegexp = 5,
        /// Partial match for the regular expression with the string value.
        PartialRegexp = 6,
    }
}
/// The result needs to be in a list of string values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessInListFilter {
    /// The list of string values. Must be non-empty.
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, the string value is case sensitive.
    #[prost(bool, tag = "2")]
    pub case_sensitive: bool,
}
/// Filters for numeric or date values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessNumericFilter {
    /// The operation type for this filter.
    #[prost(enumeration = "access_numeric_filter::Operation", tag = "1")]
    pub operation: i32,
    /// A numeric value or a date value.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<NumericValue>,
}
/// Nested message and enum types in `AccessNumericFilter`.
pub mod access_numeric_filter {
    /// The operation applied to a numeric filter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Unspecified.
        Unspecified = 0,
        /// Equal
        Equal = 1,
        /// Less than
        LessThan = 2,
        /// Less than or equal
        LessThanOrEqual = 3,
        /// Greater than
        GreaterThan = 4,
        /// Greater than or equal
        GreaterThanOrEqual = 5,
    }
}
/// To express that the result needs to be between two numbers (inclusive).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessBetweenFilter {
    /// Begins with this number.
    #[prost(message, optional, tag = "1")]
    pub from_value: ::core::option::Option<NumericValue>,
    /// Ends with this number.
    #[prost(message, optional, tag = "2")]
    pub to_value: ::core::option::Option<NumericValue>,
}
/// To represent a number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericValue {
    /// One of a numeric value
    #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
    pub one_value: ::core::option::Option<numeric_value::OneValue>,
}
/// Nested message and enum types in `NumericValue`.
pub mod numeric_value {
    /// One of a numeric value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Integer value
        #[prost(int64, tag = "1")]
        Int64Value(i64),
        /// Double value
        #[prost(double, tag = "2")]
        DoubleValue(f64),
    }
}
/// Order bys define how rows will be sorted in the response. For example,
/// ordering rows by descending access count is one ordering, and ordering rows
/// by the country string is a different ordering.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessOrderBy {
    /// If true, sorts by descending order. If false or unspecified, sorts in
    /// ascending order.
    #[prost(bool, tag = "3")]
    pub desc: bool,
    /// Specify one type of order by for `OrderBy`.
    #[prost(oneof = "access_order_by::OneOrderBy", tags = "1, 2")]
    pub one_order_by: ::core::option::Option<access_order_by::OneOrderBy>,
}
/// Nested message and enum types in `AccessOrderBy`.
pub mod access_order_by {
    /// Sorts by metric values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricOrderBy {
        /// A metric name in the request to order by.
        #[prost(string, tag = "1")]
        pub metric_name: ::prost::alloc::string::String,
    }
    /// Sorts by dimension values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionOrderBy {
        /// A dimension name in the request to order by.
        #[prost(string, tag = "1")]
        pub dimension_name: ::prost::alloc::string::String,
        /// Controls the rule for dimension value ordering.
        #[prost(enumeration = "dimension_order_by::OrderType", tag = "2")]
        pub order_type: i32,
    }
    /// Nested message and enum types in `DimensionOrderBy`.
    pub mod dimension_order_by {
        /// Rule to order the string dimension values by.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum OrderType {
            /// Unspecified.
            Unspecified = 0,
            /// Alphanumeric sort by Unicode code point. For example, "2" < "A" < "X" <
            /// "b" < "z".
            Alphanumeric = 1,
            /// Case insensitive alphanumeric sort by lower case Unicode code point.
            /// For example, "2" < "A" < "b" < "X" < "z".
            CaseInsensitiveAlphanumeric = 2,
            /// Dimension values are converted to numbers before sorting. For example
            /// in NUMERIC sort, "25" < "100", and in `ALPHANUMERIC` sort, "100" <
            /// "25". Non-numeric dimension values all have equal ordering value below
            /// all numeric values.
            Numeric = 3,
        }
    }
    /// Specify one type of order by for `OrderBy`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneOrderBy {
        /// Sorts results by a metric's values.
        #[prost(message, tag = "1")]
        Metric(MetricOrderBy),
        /// Sorts results by a dimension's values.
        #[prost(message, tag = "2")]
        Dimension(DimensionOrderBy),
    }
}
/// Describes a dimension column in the report. Dimensions requested in a report
/// produce column entries within rows and DimensionHeaders. However, dimensions
/// used exclusively within filters or expressions do not produce columns in a
/// report; correspondingly, those dimensions do not produce headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDimensionHeader {
    /// The dimension's name; for example 'userEmail'.
    #[prost(string, tag = "1")]
    pub dimension_name: ::prost::alloc::string::String,
}
/// Describes a metric column in the report. Visible metrics requested in a
/// report produce column entries within rows and MetricHeaders. However,
/// metrics used exclusively within filters or expressions do not produce columns
/// in a report; correspondingly, those metrics do not produce headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessMetricHeader {
    /// The metric's name; for example 'accessCount'.
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
}
/// Access report data for each row.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessRow {
    /// List of dimension values. These values are in the same order as specified
    /// in the request.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::prost::alloc::vec::Vec<AccessDimensionValue>,
    /// List of metric values. These values are in the same order as specified
    /// in the request.
    #[prost(message, repeated, tag = "2")]
    pub metric_values: ::prost::alloc::vec::Vec<AccessMetricValue>,
}
/// The value of a dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDimensionValue {
    /// The dimension value. For example, this value may be 'France' for the
    /// 'country' dimension.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// The value of a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessMetricValue {
    /// The measurement value. For example, this value may be '13'.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Current state of all quotas for this Analytics property. If any quota for a
/// property is exhausted, all requests to that property will return Resource
/// Exhausted errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessQuota {
    /// Properties can use 250,000 tokens per day. Most requests consume fewer than
    /// 10 tokens.
    #[prost(message, optional, tag = "1")]
    pub tokens_per_day: ::core::option::Option<AccessQuotaStatus>,
    /// Properties can use 50,000 tokens per hour. An API request consumes a single
    /// number of tokens, and that number is deducted from all of the hourly,
    /// daily, and per project hourly quotas.
    #[prost(message, optional, tag = "2")]
    pub tokens_per_hour: ::core::option::Option<AccessQuotaStatus>,
    /// Properties can use up to 50 concurrent requests.
    #[prost(message, optional, tag = "3")]
    pub concurrent_requests: ::core::option::Option<AccessQuotaStatus>,
    /// Properties and cloud project pairs can have up to 50 server errors per
    /// hour.
    #[prost(message, optional, tag = "4")]
    pub server_errors_per_project_per_hour: ::core::option::Option<AccessQuotaStatus>,
    /// Properties can use up to 25% of their tokens per project per hour. This
    /// amounts to Analytics 360 Properties can use 12,500 tokens per project per
    /// hour. An API request consumes a single number of tokens, and that number is
    /// deducted from all of the hourly, daily, and per project hourly quotas.
    #[prost(message, optional, tag = "5")]
    pub tokens_per_project_per_hour: ::core::option::Option<AccessQuotaStatus>,
}
/// Current state for a particular quota group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessQuotaStatus {
    /// Quota consumed by this request.
    #[prost(int32, tag = "1")]
    pub consumed: i32,
    /// Quota remaining after this request.
    #[prost(int32, tag = "2")]
    pub remaining: i32,
}
/// A specific filter for a single dimension or metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceDimensionOrMetricFilter {
    /// Required. Immutable. The dimension name or metric name to filter. If the
    /// field name refers to a custom dimension or metric, a scope prefix will be
    /// added to the front of the custom dimensions or metric name. For more on
    /// scope prefixes or custom dimensions/metrics, reference the [Google
    /// Analytics Data API documentation]
    /// (<https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#custom_dimensions>).
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Optional. Indicates whether this filter needs dynamic evaluation or not. If
    /// set to true, users join the Audience if they ever met the condition (static
    /// evaluation). If unset or set to false, user evaluation for an Audience is
    /// dynamic; users are added to an Audience when they meet the conditions and
    /// then removed when they no longer meet them.
    ///
    /// This can only be set when Audience scope is ACROSS_ALL_SESSIONS.
    #[prost(bool, tag = "6")]
    pub at_any_point_in_time: bool,
    /// Optional. If set, specifies the time window for which to evaluate data in
    /// number of days. If not set, then audience data is evaluated against
    /// lifetime data (For example, infinite time window).
    ///
    /// For example, if set to 1 day, only the current day's data is evaluated. The
    /// reference point is the current day when at_any_point_in_time is unset or
    /// false.
    ///
    /// It can only be set when Audience scope is ACROSS_ALL_SESSIONS and cannot be
    /// greater than 60 days.
    #[prost(int32, tag = "7")]
    pub in_any_n_day_period: i32,
    /// One of the above filters.
    #[prost(
        oneof = "audience_dimension_or_metric_filter::OneFilter",
        tags = "2, 3, 4, 5"
    )]
    pub one_filter: ::core::option::Option<audience_dimension_or_metric_filter::OneFilter>,
}
/// Nested message and enum types in `AudienceDimensionOrMetricFilter`.
pub mod audience_dimension_or_metric_filter {
    /// A filter for a string-type dimension that matches a particular pattern.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// Required. The match type for the string filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// Required. The string value to be matched against.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        /// Optional. If true, the match is case-sensitive. If false, the match is
        /// case-insensitive.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
    /// Nested message and enum types in `StringFilter`.
    pub mod string_filter {
        /// The match type for the string filter.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MatchType {
            /// Unspecified
            Unspecified = 0,
            /// Exact match of the string value.
            Exact = 1,
            /// Begins with the string value.
            BeginsWith = 2,
            /// Ends with the string value.
            EndsWith = 3,
            /// Contains the string value.
            Contains = 4,
            /// Full regular expression matches with the string value.
            FullRegexp = 5,
        }
    }
    /// A filter for a string dimension that matches a particular list of options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InListFilter {
        /// Required. The list of possible string values to match against. Must be
        /// non-empty.
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. If true, the match is case-sensitive. If false, the match is
        /// case-insensitive.
        #[prost(bool, tag = "2")]
        pub case_sensitive: bool,
    }
    /// To represent a number.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NumericValue {
        /// One of a numeric value.
        #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
        pub one_value: ::core::option::Option<numeric_value::OneValue>,
    }
    /// Nested message and enum types in `NumericValue`.
    pub mod numeric_value {
        /// One of a numeric value.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OneValue {
            /// Integer value.
            #[prost(int64, tag = "1")]
            Int64Value(i64),
            /// Double value.
            #[prost(double, tag = "2")]
            DoubleValue(f64),
        }
    }
    /// A filter for numeric or date values on a dimension or metric.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NumericFilter {
        /// Required. The operation applied to a numeric filter.
        #[prost(enumeration = "numeric_filter::Operation", tag = "1")]
        pub operation: i32,
        /// Required. The numeric or date value to match against.
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<NumericValue>,
    }
    /// Nested message and enum types in `NumericFilter`.
    pub mod numeric_filter {
        /// The operation applied to a numeric filter.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Operation {
            /// Unspecified.
            Unspecified = 0,
            /// Equal.
            Equal = 1,
            /// Less than.
            LessThan = 2,
            /// Greater than.
            GreaterThan = 4,
        }
    }
    /// A filter for numeric or date values between certain values on a dimension
    /// or metric.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BetweenFilter {
        /// Required. Begins with this number, inclusive.
        #[prost(message, optional, tag = "1")]
        pub from_value: ::core::option::Option<NumericValue>,
        /// Required. Ends with this number, inclusive.
        #[prost(message, optional, tag = "2")]
        pub to_value: ::core::option::Option<NumericValue>,
    }
    /// One of the above filters.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// A filter for a string-type dimension that matches a particular pattern.
        #[prost(message, tag = "2")]
        StringFilter(StringFilter),
        /// A filter for a string dimension that matches a particular list of
        /// options.
        #[prost(message, tag = "3")]
        InListFilter(InListFilter),
        /// A filter for numeric or date values on a dimension or metric.
        #[prost(message, tag = "4")]
        NumericFilter(NumericFilter),
        /// A filter for numeric or date values between certain values on a dimension
        /// or metric.
        #[prost(message, tag = "5")]
        BetweenFilter(BetweenFilter),
    }
}
/// A filter that matches events of a single event name. If an event parameter
/// is specified, only the subset of events that match both the single event name
/// and the parameter filter expressions match this event filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceEventFilter {
    /// Required. Immutable. The name of the event to match against.
    #[prost(string, tag = "1")]
    pub event_name: ::prost::alloc::string::String,
    /// Optional. If specified, this filter matches events that match both the
    /// single event name and the parameter filter expressions. AudienceEventFilter
    /// inside the parameter filter expression cannot be set (For example, nested
    /// event filters are not supported). This should be a single and_group of
    /// dimension_or_metric_filter or not_expression; ANDs of ORs are not
    /// supported. Also, if it includes a filter for "eventCount", only that one
    /// will be considered; all the other filters will be ignored.
    #[prost(message, optional, boxed, tag = "2")]
    pub event_parameter_filter_expression:
        ::core::option::Option<::prost::alloc::boxed::Box<AudienceFilterExpression>>,
}
/// A logical expression of Audience dimension, metric, or event filters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceFilterExpression {
    /// The expression applied to a filter.
    #[prost(oneof = "audience_filter_expression::Expr", tags = "1, 2, 3, 4, 5")]
    pub expr: ::core::option::Option<audience_filter_expression::Expr>,
}
/// Nested message and enum types in `AudienceFilterExpression`.
pub mod audience_filter_expression {
    /// The expression applied to a filter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// A list of expressions to be AND’ed together. It can only contain
        /// AudienceFilterExpressions with or_group. This must be set for the top
        /// level AudienceFilterExpression.
        #[prost(message, tag = "1")]
        AndGroup(super::AudienceFilterExpressionList),
        /// A list of expressions to OR’ed together. It cannot contain
        /// AudienceFilterExpressions with and_group or or_group.
        #[prost(message, tag = "2")]
        OrGroup(super::AudienceFilterExpressionList),
        /// A filter expression to be NOT'ed (For example, inverted, complemented).
        /// It can only include a dimension_or_metric_filter. This cannot be set on
        /// the top level AudienceFilterExpression.
        #[prost(message, tag = "3")]
        NotExpression(::prost::alloc::boxed::Box<super::AudienceFilterExpression>),
        /// A filter on a single dimension or metric. This cannot be set on the top
        /// level AudienceFilterExpression.
        #[prost(message, tag = "4")]
        DimensionOrMetricFilter(super::AudienceDimensionOrMetricFilter),
        /// Creates a filter that matches a specific event. This cannot be set on the
        /// top level AudienceFilterExpression.
        #[prost(message, tag = "5")]
        EventFilter(::prost::alloc::boxed::Box<super::AudienceEventFilter>),
    }
}
/// A list of Audience filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceFilterExpressionList {
    /// A list of Audience filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub filter_expressions: ::prost::alloc::vec::Vec<AudienceFilterExpression>,
}
/// Defines a simple filter that a user must satisfy to be a member of the
/// Audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceSimpleFilter {
    /// Required. Immutable. Specifies the scope for this filter.
    #[prost(enumeration = "AudienceFilterScope", tag = "1")]
    pub scope: i32,
    /// Required. Immutable. A logical expression of Audience dimension, metric, or
    /// event filters.
    #[prost(message, optional, tag = "2")]
    pub filter_expression: ::core::option::Option<AudienceFilterExpression>,
}
/// Defines filters that must occur in a specific order for the user to be a
/// member of the Audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceSequenceFilter {
    /// Required. Immutable. Specifies the scope for this filter.
    #[prost(enumeration = "AudienceFilterScope", tag = "1")]
    pub scope: i32,
    /// Optional. Defines the time period in which the whole sequence must occur.
    #[prost(message, optional, tag = "2")]
    pub sequence_maximum_duration: ::core::option::Option<::prost_types::Duration>,
    /// Required. An ordered sequence of steps. A user must complete each step in
    /// order to join the sequence filter.
    #[prost(message, repeated, tag = "3")]
    pub sequence_steps: ::prost::alloc::vec::Vec<audience_sequence_filter::AudienceSequenceStep>,
}
/// Nested message and enum types in `AudienceSequenceFilter`.
pub mod audience_sequence_filter {
    /// A condition that must occur in the specified step order for this user
    /// to match the sequence.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudienceSequenceStep {
        /// Required. Immutable. Specifies the scope for this step.
        #[prost(enumeration = "super::AudienceFilterScope", tag = "1")]
        pub scope: i32,
        /// Optional. If true, the event satisfying this step must be the very next
        /// event after the event satisfying the last step. If unset or false, this
        /// step indirectly follows the prior step; for example, there may be
        /// events between the prior step and this step. It is ignored for the
        /// first step.
        #[prost(bool, tag = "2")]
        pub immediately_follows: bool,
        /// Optional. When set, this step must be satisfied within the
        /// constraint_duration of the previous step (For example,  t\[i\] - t\[i-1\] <=
        /// constraint_duration). If not set, there is no duration requirement (the
        /// duration is effectively unlimited). It is ignored for the first step.
        #[prost(message, optional, tag = "3")]
        pub constraint_duration: ::core::option::Option<::prost_types::Duration>,
        /// Required. Immutable. A logical expression of Audience dimension, metric,
        /// or event filters in each step.
        #[prost(message, optional, tag = "4")]
        pub filter_expression: ::core::option::Option<super::AudienceFilterExpression>,
    }
}
/// A clause for defining either a simple or sequence filter. A filter can be
/// inclusive (For example, users satisfying the filter clause are included in
/// the Audience) or exclusive (For example, users satisfying the filter clause
/// are excluded from the Audience).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceFilterClause {
    /// Required. Specifies whether this is an include or exclude filter clause.
    #[prost(enumeration = "audience_filter_clause::AudienceClauseType", tag = "1")]
    pub clause_type: i32,
    #[prost(oneof = "audience_filter_clause::Filter", tags = "2, 3")]
    pub filter: ::core::option::Option<audience_filter_clause::Filter>,
}
/// Nested message and enum types in `AudienceFilterClause`.
pub mod audience_filter_clause {
    /// Specifies whether this is an include or exclude filter clause.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AudienceClauseType {
        /// Unspecified clause type.
        Unspecified = 0,
        /// Users will be included in the Audience if the filter clause is met.
        Include = 1,
        /// Users will be excluded from the Audience if the filter clause is met.
        Exclude = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// A simple filter that a user must satisfy to be a member of the Audience.
        #[prost(message, tag = "2")]
        SimpleFilter(super::AudienceSimpleFilter),
        /// Filters that must occur in a specific order for the user to be a member
        /// of the Audience.
        #[prost(message, tag = "3")]
        SequenceFilter(super::AudienceSequenceFilter),
    }
}
/// Specifies an event to log when a user joins the Audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceEventTrigger {
    /// Required. The event name that will be logged.
    #[prost(string, tag = "1")]
    pub event_name: ::prost::alloc::string::String,
    /// Required. When to log the event.
    #[prost(enumeration = "audience_event_trigger::LogCondition", tag = "2")]
    pub log_condition: i32,
}
/// Nested message and enum types in `AudienceEventTrigger`.
pub mod audience_event_trigger {
    /// Determines when to log the event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogCondition {
        /// Log condition is not specified.
        Unspecified = 0,
        /// The event should be logged only when a user is joined.
        AudienceJoined = 1,
        /// The event should be logged whenever the Audience condition is met, even
        /// if the user is already a member of the Audience.
        AudienceMembershipRenewed = 2,
    }
}
/// A resource message representing a GA4 Audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    /// Output only. The resource name for this Audience resource.
    /// Format: properties/{propertyId}/audiences/{audienceId}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the Audience.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The description of the Audience.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. The duration a user should stay in an Audience. It
    /// cannot be set to more than 540 days.
    #[prost(int32, tag = "4")]
    pub membership_duration_days: i32,
    /// Output only. It is automatically set by GA to false if this is an NPA
    /// Audience and is excluded from ads personalization.
    #[prost(bool, tag = "5")]
    pub ads_personalization_enabled: bool,
    /// Optional. Specifies an event to log when a user joins the Audience. If not
    /// set, no event is logged when a user joins the Audience.
    #[prost(message, optional, tag = "6")]
    pub event_trigger: ::core::option::Option<AudienceEventTrigger>,
    /// Immutable. Specifies how long an exclusion lasts for users that meet the
    /// exclusion filter. It is applied to all EXCLUDE filter clauses and is
    /// ignored when there is no EXCLUDE filter clause in the Audience.
    #[prost(enumeration = "audience::AudienceExclusionDurationMode", tag = "7")]
    pub exclusion_duration_mode: i32,
    /// Required. Immutable. Unordered list. Filter clauses that define the
    /// Audience. All clauses will be AND’ed together.
    #[prost(message, repeated, tag = "8")]
    pub filter_clauses: ::prost::alloc::vec::Vec<AudienceFilterClause>,
}
/// Nested message and enum types in `Audience`.
pub mod audience {
    /// Specifies how long an exclusion lasts for users that meet the exclusion
    /// filter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AudienceExclusionDurationMode {
        /// Not specified.
        Unspecified = 0,
        /// Exclude users from the Audience during periods when they meet the
        /// filter clause.
        ExcludeTemporarily = 1,
        /// Exclude users from the Audience if they've ever met the filter clause.
        ExcludePermanently = 2,
    }
}
/// Specifies how to evaluate users for joining an Audience.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudienceFilterScope {
    /// Scope is not specified.
    Unspecified = 0,
    /// User joins the Audience if the filter condition is met within one
    /// event.
    WithinSameEvent = 1,
    /// User joins the Audience if the filter condition is met within one
    /// session.
    WithinSameSession = 2,
    /// User joins the Audience if the filter condition is met by any event
    /// across any session.
    AcrossAllSessions = 3,
}
/// A specific filter for a single dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGroupFilter {
    /// Required. Immutable. The dimension name to filter.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// A StringFilter or InListFilter that defines this filters behavior.
    #[prost(oneof = "channel_group_filter::ValueFilter", tags = "2, 3")]
    pub value_filter: ::core::option::Option<channel_group_filter::ValueFilter>,
}
/// Nested message and enum types in `ChannelGroupFilter`.
pub mod channel_group_filter {
    /// Filter where the field value is a String. The match is case insensitive.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// Required. The match type for the string filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// Required. The string value to be matched against.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `StringFilter`.
    pub mod string_filter {
        /// How the filter will be used to determine a match.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MatchType {
            /// Default match type.
            Unspecified = 0,
            /// Exact match of the string value.
            Exact = 1,
            /// Begins with the string value.
            BeginsWith = 2,
            /// Ends with the string value.
            EndsWith = 3,
            /// Contains the string value.
            Contains = 4,
            /// Full regular expression match with the string value.
            FullRegexp = 5,
            /// Partial regular expression match with the string value.
            PartialRegexp = 6,
        }
    }
    /// A filter for a string dimension that matches a particular list of options.
    /// The match is case insensitive.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InListFilter {
        /// Required. The list of possible string values to match against. Must be
        /// non-empty.
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A StringFilter or InListFilter that defines this filters behavior.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueFilter {
        /// A filter for a string-type dimension that matches a particular pattern.
        #[prost(message, tag = "2")]
        StringFilter(StringFilter),
        /// A filter for a string dimension that matches a particular list of
        /// options.
        #[prost(message, tag = "3")]
        InListFilter(InListFilter),
    }
}
/// A logical expression of Channel Group dimension filters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGroupFilterExpression {
    /// The expression applied to a filter.
    #[prost(oneof = "channel_group_filter_expression::Expr", tags = "1, 2, 3, 4")]
    pub expr: ::core::option::Option<channel_group_filter_expression::Expr>,
}
/// Nested message and enum types in `ChannelGroupFilterExpression`.
pub mod channel_group_filter_expression {
    /// The expression applied to a filter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// A list of expressions to be AND’ed together. It can only contain
        /// ChannelGroupFilterExpressions with or_group. This must be set for the
        /// top level ChannelGroupFilterExpression.
        #[prost(message, tag = "1")]
        AndGroup(super::ChannelGroupFilterExpressionList),
        /// A list of expressions to OR’ed together. It cannot contain
        /// ChannelGroupFilterExpressions with and_group or or_group.
        #[prost(message, tag = "2")]
        OrGroup(super::ChannelGroupFilterExpressionList),
        /// A filter expression to be NOT'ed (that is inverted, complemented). It
        /// can only include a dimension_or_metric_filter. This cannot be set on the
        /// top level ChannelGroupFilterExpression.
        #[prost(message, tag = "3")]
        NotExpression(::prost::alloc::boxed::Box<super::ChannelGroupFilterExpression>),
        /// A filter on a single dimension. This cannot be set on the top
        /// level ChannelGroupFilterExpression.
        #[prost(message, tag = "4")]
        Filter(super::ChannelGroupFilter),
    }
}
/// A list of Channel Group filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGroupFilterExpressionList {
    /// A list of Channel Group filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub filter_expressions: ::prost::alloc::vec::Vec<ChannelGroupFilterExpression>,
}
/// The rules that govern how traffic is grouped into one channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupingRule {
    /// Required. Customer defined display name for the channel.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The Filter Expression that defines the Grouping Rule.
    #[prost(message, optional, tag = "2")]
    pub expression: ::core::option::Option<ChannelGroupFilterExpression>,
}
/// A resource message representing a Channel Group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelGroup {
    /// Output only. The resource name for this Channel Group resource.
    /// Format: properties/{property}/channelGroups/{channel_group}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the Channel Group. Max length of 80
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the Channel Group. Max length of 256 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. The grouping rules of channels. Maximum number of rules is 50.
    #[prost(message, repeated, tag = "4")]
    pub grouping_rule: ::prost::alloc::vec::Vec<GroupingRule>,
    /// Output only. If true, then this channel group is the Default Channel Group
    /// predefined by Google Analytics. Display name and grouping rules cannot be
    /// updated for this channel group.
    #[prost(bool, tag = "5")]
    pub system_defined: bool,
}
/// Defines an event parameter to mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterMutation {
    /// Required. The name of the parameter to mutate.
    /// This value must:
    /// * be less than 40 characters.
    /// * be unique across across all mutations within the rule
    /// * consist only of letters, digits or _ (underscores)
    /// For event edit rules, the name may also be set to 'event_name' to modify
    /// the event_name in place.
    #[prost(string, tag = "1")]
    pub parameter: ::prost::alloc::string::String,
    /// Required. The value mutation to perform.
    /// * Must be less than 100 characters.
    /// * To specify a constant value for the param, use the value's string.
    /// * To copy value from another parameter, use syntax like
    /// "\[[other_parameter]\]" For more details, see this [help center
    /// article](<https://support.google.com/analytics/answer/10085872#modify-an-event&zippy=%2Cin-this-article%2Cmodify-parameters>).
    #[prost(string, tag = "2")]
    pub parameter_value: ::prost::alloc::string::String,
}
/// An Event Create Rule defines conditions that will trigger the creation
/// of an entirely new event based upon matched criteria of a source event.
/// Additional mutations of the parameters from the source event can be defined.
///
/// Unlike Event Edit rules, Event Creation Rules have no defined order.  They
/// will all be run independently.
///
/// Event Edit and Event Create rules can't be used to modify an event created
/// from an Event Create rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateRule {
    /// Output only. Resource name for this EventCreateRule resource.
    /// Format:
    /// properties/{property}/dataStreams/{data_stream}/eventCreateRules/{event_create_rule}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the new event to be created.
    ///
    /// This value must:
    /// * be less than 40 characters
    /// * consist only of letters, digits or _ (underscores)
    /// * start with a letter
    #[prost(string, tag = "2")]
    pub destination_event: ::prost::alloc::string::String,
    /// Required. Must have at least one condition, and can have up to 10 max.
    /// Conditions on the source event must match for this rule to be applied.
    #[prost(message, repeated, tag = "3")]
    pub event_conditions: ::prost::alloc::vec::Vec<MatchingCondition>,
    /// If true, the source parameters are copied to the new event.
    /// If false, or unset, all non-internal parameters are not copied from the
    /// source event. Parameter mutations are applied after the parameters have
    /// been copied.
    #[prost(bool, tag = "4")]
    pub source_copy_parameters: bool,
    /// Parameter mutations define parameter behavior on the new event, and
    /// are applied in order.
    /// A maximum of 20 mutations can be applied.
    #[prost(message, repeated, tag = "5")]
    pub parameter_mutations: ::prost::alloc::vec::Vec<ParameterMutation>,
}
/// Defines a condition for when an Event Edit or Event Creation rule applies to
/// an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingCondition {
    /// Required. The name of the field that is compared against for the condition.
    /// If 'event_name' is specified this condition will apply to the name of the
    /// event.  Otherwise the condition will apply to a parameter with the
    /// specified name.
    ///
    /// This value cannot contain spaces.
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    /// Required. The type of comparison to be applied to the value.
    #[prost(enumeration = "matching_condition::ComparisonType", tag = "2")]
    pub comparison_type: i32,
    /// Required. The value being compared against for this condition.  The runtime
    /// implementation may perform type coercion of this value to evaluate this
    /// condition based on the type of the parameter value.
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    /// Whether or not the result of the comparison should be negated. For example,
    /// if `negated` is true, then 'equals' comparisons would function as 'not
    /// equals'.
    #[prost(bool, tag = "4")]
    pub negated: bool,
}
/// Nested message and enum types in `MatchingCondition`.
pub mod matching_condition {
    /// Comparison type for matching condition
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ComparisonType {
        /// Unknown
        Unspecified = 0,
        /// Equals, case sensitive
        Equals = 1,
        /// Equals, case insensitive
        EqualsCaseInsensitive = 2,
        /// Contains, case sensitive
        Contains = 3,
        /// Contains, case insensitive
        ContainsCaseInsensitive = 4,
        /// Starts with, case sensitive
        StartsWith = 5,
        /// Starts with, case insensitive
        StartsWithCaseInsensitive = 6,
        /// Ends with, case sensitive
        EndsWith = 7,
        /// Ends with, case insensitive
        EndsWithCaseInsensitive = 8,
        /// Greater than
        GreaterThan = 9,
        /// Greater than or equal
        GreaterThanOrEqual = 10,
        /// Less than
        LessThan = 11,
        /// Less than or equal
        LessThanOrEqual = 12,
        /// regular expression. Only supported for web streams.
        RegularExpression = 13,
        /// regular expression, case insensitive. Only supported for web streams.
        RegularExpressionCaseInsensitive = 14,
    }
}
/// A specific filter for a single dimension
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedDataSetFilter {
    /// Required. The dimension name to filter.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// One of the above filters.
    #[prost(oneof = "expanded_data_set_filter::OneFilter", tags = "2, 3")]
    pub one_filter: ::core::option::Option<expanded_data_set_filter::OneFilter>,
}
/// Nested message and enum types in `ExpandedDataSetFilter`.
pub mod expanded_data_set_filter {
    /// A filter for a string-type dimension that matches a particular pattern.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// Required. The match type for the string filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// Required. The string value to be matched against.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        /// Optional. If true, the match is case-sensitive. If false, the match is
        /// case-insensitive.
        /// Must be true when match_type is EXACT.
        /// Must be false when match_type is CONTAINS.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
    /// Nested message and enum types in `StringFilter`.
    pub mod string_filter {
        /// The match type for the string filter.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MatchType {
            /// Unspecified
            Unspecified = 0,
            /// Exact match of the string value.
            Exact = 1,
            /// Contains the string value.
            Contains = 2,
        }
    }
    /// A filter for a string dimension that matches a particular list of options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InListFilter {
        /// Required. The list of possible string values to match against. Must be
        /// non-empty.
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. If true, the match is case-sensitive. If false, the match is
        /// case-insensitive.
        /// Must be true.
        #[prost(bool, tag = "2")]
        pub case_sensitive: bool,
    }
    /// One of the above filters.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// A filter for a string-type dimension that matches a particular pattern.
        #[prost(message, tag = "2")]
        StringFilter(StringFilter),
        /// A filter for a string dimension that matches a particular list of
        /// options.
        #[prost(message, tag = "3")]
        InListFilter(InListFilter),
    }
}
/// A logical expression of EnhancedDataSet dimension filters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedDataSetFilterExpression {
    /// The expression applied to a filter.
    #[prost(oneof = "expanded_data_set_filter_expression::Expr", tags = "1, 2, 3")]
    pub expr: ::core::option::Option<expanded_data_set_filter_expression::Expr>,
}
/// Nested message and enum types in `ExpandedDataSetFilterExpression`.
pub mod expanded_data_set_filter_expression {
    /// The expression applied to a filter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// A list of expressions to be AND’ed together. It must contain a
        /// ExpandedDataSetFilterExpression with either not_expression or
        /// dimension_filter. This must be set for the top level
        /// ExpandedDataSetFilterExpression.
        #[prost(message, tag = "1")]
        AndGroup(super::ExpandedDataSetFilterExpressionList),
        /// A filter expression to be NOT'ed (that is, inverted, complemented). It
        /// must include a dimension_filter. This cannot be set on the
        /// top level ExpandedDataSetFilterExpression.
        #[prost(message, tag = "2")]
        NotExpression(::prost::alloc::boxed::Box<super::ExpandedDataSetFilterExpression>),
        /// A filter on a single dimension. This cannot be set on the top
        /// level ExpandedDataSetFilterExpression.
        #[prost(message, tag = "3")]
        Filter(super::ExpandedDataSetFilter),
    }
}
/// A list of ExpandedDataSet filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedDataSetFilterExpressionList {
    /// A list of ExpandedDataSet filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub filter_expressions: ::prost::alloc::vec::Vec<ExpandedDataSetFilterExpression>,
}
/// A resource message representing a GA4 ExpandedDataSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedDataSet {
    /// Output only. The resource name for this ExpandedDataSet resource.
    /// Format: properties/{property_id}/expandedDataSets/{expanded_data_set}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the ExpandedDataSet.
    /// Max 200 chars.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The description of the ExpandedDataSet.
    /// Max 50 chars.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Immutable. The list of dimensions included in the ExpandedDataSet.
    /// See the [API
    /// Dimensions](<https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions>)
    /// for the list of dimension names.
    #[prost(string, repeated, tag = "4")]
    pub dimension_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Immutable. The list of metrics included in the ExpandedDataSet.
    /// See the [API
    /// Metrics](<https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics>)
    /// for the list of dimension names.
    #[prost(string, repeated, tag = "5")]
    pub metric_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Immutable. A logical expression of ExpandedDataSet filters applied to
    /// dimension included in the ExpandedDataSet. This filter is used to reduce
    /// the number of rows and thus the chance of encountering `other` row.
    #[prost(message, optional, tag = "6")]
    pub dimension_filter_expression: ::core::option::Option<ExpandedDataSetFilterExpression>,
    /// Output only. Time when expanded data set began (or will begin) collecing
    /// data.
    #[prost(message, optional, tag = "7")]
    pub data_collection_start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A resource message representing a Google Analytics account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Output only. Resource name of this account.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time when this account was originally created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when account payload fields were last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Human-readable display name for this account.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Country of business. Must be a Unicode CLDR region code.
    #[prost(string, tag = "5")]
    pub region_code: ::prost::alloc::string::String,
    /// Output only. Indicates whether this Account is soft-deleted or not. Deleted
    /// accounts are excluded from List results unless specifically requested.
    #[prost(bool, tag = "6")]
    pub deleted: bool,
}
/// A resource message representing a Google Analytics GA4 property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Output only. Resource name of this property.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The property type for this Property resource. When creating a
    /// property, if the type is "PROPERTY_TYPE_UNSPECIFIED", then
    /// "ORDINARY_PROPERTY" will be implied.
    #[prost(enumeration = "PropertyType", tag = "14")]
    pub property_type: i32,
    /// Output only. Time when the entity was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when entity payload fields were last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account}, properties/{property}
    /// Example: "accounts/100", "properties/101"
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this property.
    ///
    /// The max allowed display name length is 100 UTF-16 code units.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Industry associated with this property
    /// Example: AUTOMOTIVE, FOOD_AND_DRINK
    #[prost(enumeration = "IndustryCategory", tag = "6")]
    pub industry_category: i32,
    /// Required. Reporting Time Zone, used as the day boundary for reports,
    /// regardless of where the data originates. If the time zone honors DST,
    /// Analytics will automatically adjust for the changes.
    ///
    /// NOTE: Changing the time zone only affects data going forward, and is not
    /// applied retroactively.
    ///
    /// Format: <https://www.iana.org/time-zones>
    /// Example: "America/Los_Angeles"
    #[prost(string, tag = "7")]
    pub time_zone: ::prost::alloc::string::String,
    /// The currency type used in reports involving monetary values.
    ///
    ///
    /// Format: <https://en.wikipedia.org/wiki/ISO_4217>
    /// Examples: "USD", "EUR", "JPY"
    #[prost(string, tag = "8")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. The Google Analytics service level that applies to this
    /// property.
    #[prost(enumeration = "ServiceLevel", tag = "10")]
    pub service_level: i32,
    /// Output only. If set, the time at which this property was trashed. If not
    /// set, then this property is not currently in the trash can.
    #[prost(message, optional, tag = "11")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If set, the time at which this trashed property will be
    /// permanently deleted. If not set, then this property is not currently in the
    /// trash can and is not slated to be deleted.
    #[prost(message, optional, tag = "12")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. The resource name of the parent account
    /// Format: accounts/{account_id}
    /// Example: "accounts/123"
    #[prost(string, tag = "13")]
    pub account: ::prost::alloc::string::String,
}
/// A resource message representing a data stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataStream {
    /// Output only. Resource name of this Data Stream.
    /// Format: properties/{property_id}/dataStreams/{stream_id}
    /// Example: "properties/1000/dataStreams/2000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The type of this DataStream resource.
    #[prost(enumeration = "data_stream::DataStreamType", tag = "2")]
    pub r#type: i32,
    /// Human-readable display name for the Data Stream.
    ///
    /// Required for web data streams.
    ///
    /// The max allowed display name length is 255 UTF-16 code units.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Time when this stream was originally created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when stream payload fields were last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Data for specific data stream types. The message that will be
    /// set corresponds to the type of this stream.
    #[prost(oneof = "data_stream::StreamData", tags = "6, 7, 8")]
    pub stream_data: ::core::option::Option<data_stream::StreamData>,
}
/// Nested message and enum types in `DataStream`.
pub mod data_stream {
    /// Data specific to web streams.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebStreamData {
        /// Output only. Analytics Measurement ID.
        ///
        /// Example: "G-1A2BCD345E"
        #[prost(string, tag = "1")]
        pub measurement_id: ::prost::alloc::string::String,
        /// Output only. ID of the corresponding web app in Firebase, if any.
        /// This ID can change if the web app is deleted and recreated.
        #[prost(string, tag = "2")]
        pub firebase_app_id: ::prost::alloc::string::String,
        /// Domain name of the web app being measured, or empty.
        /// Example: "<http://www.google.com",> "<https://www.google.com">
        #[prost(string, tag = "3")]
        pub default_uri: ::prost::alloc::string::String,
    }
    /// Data specific to Android app streams.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AndroidAppStreamData {
        /// Output only. ID of the corresponding Android app in Firebase, if any.
        /// This ID can change if the Android app is deleted and recreated.
        #[prost(string, tag = "1")]
        pub firebase_app_id: ::prost::alloc::string::String,
        /// Immutable. The package name for the app being measured.
        /// Example: "com.example.myandroidapp"
        #[prost(string, tag = "2")]
        pub package_name: ::prost::alloc::string::String,
    }
    /// Data specific to iOS app streams.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IosAppStreamData {
        /// Output only. ID of the corresponding iOS app in Firebase, if any.
        /// This ID can change if the iOS app is deleted and recreated.
        #[prost(string, tag = "1")]
        pub firebase_app_id: ::prost::alloc::string::String,
        /// Required. Immutable. The Apple App Store Bundle ID for the app
        /// Example: "com.example.myiosapp"
        #[prost(string, tag = "2")]
        pub bundle_id: ::prost::alloc::string::String,
    }
    /// The type of the data stream.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataStreamType {
        /// Type unknown or not specified.
        Unspecified = 0,
        /// Web data stream.
        WebDataStream = 1,
        /// Android app data stream.
        AndroidAppDataStream = 2,
        /// iOS app data stream.
        IosAppDataStream = 3,
    }
    /// Data for specific data stream types. The message that will be
    /// set corresponds to the type of this stream.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamData {
        /// Data specific to web streams. Must be populated if type is
        /// WEB_DATA_STREAM.
        #[prost(message, tag = "6")]
        WebStreamData(WebStreamData),
        /// Data specific to Android app streams. Must be populated if type is
        /// ANDROID_APP_DATA_STREAM.
        #[prost(message, tag = "7")]
        AndroidAppStreamData(AndroidAppStreamData),
        /// Data specific to iOS app streams. Must be populated if type is
        /// IOS_APP_DATA_STREAM.
        #[prost(message, tag = "8")]
        IosAppStreamData(IosAppStreamData),
    }
}
/// A link between a GA4 property and a Firebase project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirebaseLink {
    /// Output only. Example format: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Firebase project resource name. When creating a FirebaseLink,
    /// you may provide this resource name using either a project number or project
    /// ID. Once this resource has been created, returned FirebaseLinks will always
    /// have a project_name that contains a project number.
    ///
    /// Format: 'projects/{project number}'
    /// Example: 'projects/1234'
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Output only. Time when this FirebaseLink was originally created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Read-only resource with the tag for sending data from a website to a
/// DataStream. Only present for web DataStream resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSiteTag {
    /// Output only. Resource name for this GlobalSiteTag resource.
    /// Format: properties/{property_id}/dataStreams/{stream_id}/globalSiteTag
    /// Example: "properties/123/dataStreams/456/globalSiteTag"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. JavaScript code snippet to be pasted as the first item into the
    /// head tag of every webpage to measure.
    #[prost(string, tag = "2")]
    pub snippet: ::prost::alloc::string::String,
}
/// A link between a GA4 property and a Google Ads account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsLink {
    /// Output only. Format:
    /// properties/{propertyId}/googleAdsLinks/{googleAdsLinkId}
    ///
    /// Note: googleAdsLinkId is not the Google Ads customer ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Google Ads customer ID.
    #[prost(string, tag = "3")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. If true, this link is for a Google Ads manager account.
    #[prost(bool, tag = "4")]
    pub can_manage_clients: bool,
    /// Enable personalized advertising features with this integration.
    /// Automatically publish my Google Analytics audience lists and Google
    /// Analytics remarketing events/parameters to the linked Google Ads account.
    /// If this field is not set on create/update, it will be defaulted to true.
    #[prost(message, optional, tag = "5")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Output only. Time when this link was originally created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this link was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the user that created the link.
    /// An empty string will be returned if the email address can't be retrieved.
    #[prost(string, tag = "9")]
    pub creator_email_address: ::prost::alloc::string::String,
}
/// A resource message representing data sharing settings of a Google Analytics
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSharingSettings {
    /// Output only. Resource name.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Allows Google support to access the data in order to help troubleshoot
    /// issues.
    #[prost(bool, tag = "2")]
    pub sharing_with_google_support_enabled: bool,
    /// Allows Google sales teams that are assigned to the customer to access the
    /// data in order to suggest configuration changes to improve results.
    /// Sales team restrictions still apply when enabled.
    #[prost(bool, tag = "3")]
    pub sharing_with_google_assigned_sales_enabled: bool,
    /// Allows any of Google sales to access the data in order to suggest
    /// configuration changes to improve results.
    #[prost(bool, tag = "4")]
    pub sharing_with_google_any_sales_enabled: bool,
    /// Allows Google to use the data to improve other Google products or services.
    #[prost(bool, tag = "5")]
    pub sharing_with_google_products_enabled: bool,
    /// Allows Google to share the data anonymously in aggregate form with others.
    #[prost(bool, tag = "6")]
    pub sharing_with_others_enabled: bool,
}
/// A virtual resource representing an overview of an account and
/// all its child GA4 properties.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSummary {
    /// Resource name for this account summary.
    /// Format: accountSummaries/{account_id}
    /// Example: "accountSummaries/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Resource name of account referred to by this account summary
    /// Format: accounts/{account_id}
    /// Example: "accounts/1000"
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// Display name for the account referred to in this account summary.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// List of summaries for child accounts of this account.
    #[prost(message, repeated, tag = "4")]
    pub property_summaries: ::prost::alloc::vec::Vec<PropertySummary>,
}
/// A virtual resource representing metadata for a GA4 property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertySummary {
    /// Resource name of property referred to by this property summary
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Display name for the property referred to in this property summary.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The property's property type.
    #[prost(enumeration = "PropertyType", tag = "3")]
    pub property_type: i32,
    /// Resource name of this property's logical parent.
    ///
    /// Note: The Property-Moving UI can be used to change the parent.
    /// Format: accounts/{account}, properties/{property}
    /// Example: "accounts/100", "properties/200"
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
}
/// A secret value used for sending hits to Measurement Protocol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementProtocolSecret {
    /// Output only. Resource name of this secret. This secret may be a child of
    /// any type of stream. Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Human-readable display name for this secret.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The measurement protocol secret value. Pass this value to the
    /// api_secret field of the Measurement Protocol API when sending hits to this
    /// secret's parent property.
    #[prost(string, tag = "3")]
    pub secret_value: ::prost::alloc::string::String,
}
/// SKAdNetwork conversion value schema of an iOS stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkAdNetworkConversionValueSchema {
    /// Output only. Resource name of the schema. This will be child of ONLY an iOS
    /// stream, and there can be at most one such child under an iOS stream.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/sKAdNetworkConversionValueSchema
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The conversion value settings for the first postback window.
    /// These differ from values for postback window two and three in that they
    /// contain a "Fine" grained conversion value (a numeric value).
    ///
    /// Conversion values for this postback window must be set.  The other windows
    /// are optional and may inherit this window's settings if unset or disabled.
    #[prost(message, optional, tag = "2")]
    pub postback_window_one: ::core::option::Option<PostbackWindow>,
    /// The conversion value settings for the second postback window.
    ///
    /// This field should only be configured if there is a need to define different
    /// conversion values for this postback window.
    ///
    /// If enable_postback_window_settings is set to false for this postback
    /// window, the values from postback_window_one will be used.
    #[prost(message, optional, tag = "3")]
    pub postback_window_two: ::core::option::Option<PostbackWindow>,
    /// The conversion value settings for the third postback window.
    ///
    /// This field should only be set if the user chose to define different
    /// conversion values for this postback window. It is allowed to configure
    /// window 3 without setting window 2. In case window 1 & 2 settings are set
    /// and enable_postback_window_settings for this postback window is set to
    /// false, the schema will inherit settings from postback_window_two.
    #[prost(message, optional, tag = "4")]
    pub postback_window_three: ::core::option::Option<PostbackWindow>,
    /// If enabled, the GA SDK will set conversion values using this schema
    /// definition, and schema will be exported to any Google Ads accounts linked
    /// to this property. If disabled, the GA SDK will not automatically set
    /// conversion values, and also the schema will not be exported to Ads.
    #[prost(bool, tag = "5")]
    pub apply_conversion_values: bool,
}
/// Settings for a SKAdNetwork conversion postback window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostbackWindow {
    /// Ordering of the repeated field will be used to prioritize the conversion
    /// value settings. Lower indexed entries are prioritized higher. The first
    /// conversion value setting that evaluates to true will be selected. It must
    /// have at least one entry if enable_postback_window_settings is set to
    /// true. It can have maximum of 128 entries.
    #[prost(message, repeated, tag = "1")]
    pub conversion_values: ::prost::alloc::vec::Vec<ConversionValues>,
    /// If enable_postback_window_settings is true, conversion_values
    /// must be populated and will be used for determining when and how to set the
    /// Conversion Value on a client device and exporting schema to linked Ads
    /// accounts. If false, the settings are not used, but are retained in case
    /// they may be used in the future. This must always be true for
    /// postback_window_one.
    #[prost(bool, tag = "2")]
    pub postback_window_settings_enabled: bool,
}
/// Conversion value settings for a postback window for SKAdNetwork conversion
/// value schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValues {
    /// Display name of the SKAdNetwork conversion value.
    /// The max allowed display name length is 50 UTF-16 code units.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// The fine-grained conversion value.  This is applicable only to the first
    /// postback window. Its valid values are \[0,63\], both inclusive. It must be
    /// set for postback window 1, and must not be set for postback window 2 & 3.
    /// This value is not guaranteed to be unique.
    ///
    /// If the configuration for the first postback window is re-used for second or
    /// third postback windows this field has no effect.
    #[prost(int32, optional, tag = "2")]
    pub fine_value: ::core::option::Option<i32>,
    /// Required. A coarse grained conversion value.
    ///
    /// This value is not guaranteed to be unique.
    #[prost(enumeration = "CoarseValue", tag = "3")]
    pub coarse_value: i32,
    /// Event conditions that must be met for this Conversion Value to be achieved.
    /// The conditions in this list are ANDed together. It must have minimum of 1
    /// entry and maximum of 3 entries, if the postback window is enabled.
    #[prost(message, repeated, tag = "4")]
    pub event_mappings: ::prost::alloc::vec::Vec<EventMapping>,
    /// If true, the SDK should lock to this conversion value for the current
    /// postback window.
    #[prost(bool, tag = "5")]
    pub lock_enabled: bool,
}
/// Event setting conditions to match an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMapping {
    /// Required. Name of the GA4 event. It must always be set.
    /// The max allowed display name length is 40 UTF-16 code units.
    #[prost(string, tag = "1")]
    pub event_name: ::prost::alloc::string::String,
    /// At least one of the following four min/max values must be set. The
    /// values set will be ANDed together to qualify an event.
    /// The minimum number of times the event occurred. If not set, minimum event
    /// count won't be checked.
    #[prost(int64, optional, tag = "2")]
    pub min_event_count: ::core::option::Option<i64>,
    /// The maximum number of times the event occurred. If not set, maximum event
    /// count won't be checked.
    #[prost(int64, optional, tag = "3")]
    pub max_event_count: ::core::option::Option<i64>,
    /// The minimum revenue generated due to the event. Revenue currency will be
    /// defined at the property level. If not set, minimum event value won't be
    /// checked.
    #[prost(double, optional, tag = "4")]
    pub min_event_value: ::core::option::Option<f64>,
    /// The maximum revenue generated due to the event. Revenue currency will be
    /// defined at the property level. If not set, maximum event value won't be
    /// checked.
    #[prost(double, optional, tag = "5")]
    pub max_event_value: ::core::option::Option<f64>,
}
/// A set of changes within a Google Analytics account or its child properties
/// that resulted from the same cause. Common causes would be updates made in the
/// Google Analytics UI, changes from customer support, or automatic Google
/// Analytics system changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHistoryEvent {
    /// ID of this change history event. This ID is unique across Google Analytics.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Time when change was made.
    #[prost(message, optional, tag = "2")]
    pub change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The type of actor that made this change.
    #[prost(enumeration = "ActorType", tag = "3")]
    pub actor_type: i32,
    /// Email address of the Google account that made the change. This will be a
    /// valid email address if the actor field is set to USER, and empty otherwise.
    /// Google accounts that have been deleted will cause an error.
    #[prost(string, tag = "4")]
    pub user_actor_email: ::prost::alloc::string::String,
    /// If true, then the list of changes returned was filtered, and does not
    /// represent all changes that occurred in this event.
    #[prost(bool, tag = "5")]
    pub changes_filtered: bool,
    /// A list of changes made in this change history event that fit the filters
    /// specified in SearchChangeHistoryEventsRequest.
    #[prost(message, repeated, tag = "6")]
    pub changes: ::prost::alloc::vec::Vec<ChangeHistoryChange>,
}
/// A description of a change to a single Google Analytics resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHistoryChange {
    /// Resource name of the resource whose changes are described by this entry.
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// The type of action that changed this resource.
    #[prost(enumeration = "ActionType", tag = "2")]
    pub action: i32,
    /// Resource contents from before the change was made. If this resource was
    /// created in this change, this field will be missing.
    #[prost(message, optional, tag = "3")]
    pub resource_before_change:
        ::core::option::Option<change_history_change::ChangeHistoryResource>,
    /// Resource contents from after the change was made. If this resource was
    /// deleted in this change, this field will be missing.
    #[prost(message, optional, tag = "4")]
    pub resource_after_change: ::core::option::Option<change_history_change::ChangeHistoryResource>,
}
/// Nested message and enum types in `ChangeHistoryChange`.
pub mod change_history_change {
    /// A snapshot of a resource as before or after the result of a change in
    /// change history.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeHistoryResource {
        #[prost(
            oneof = "change_history_resource::Resource",
            tags = "1, 2, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31"
        )]
        pub resource: ::core::option::Option<change_history_resource::Resource>,
    }
    /// Nested message and enum types in `ChangeHistoryResource`.
    pub mod change_history_resource {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Resource {
            /// A snapshot of an Account resource in change history.
            #[prost(message, tag = "1")]
            Account(super::super::Account),
            /// A snapshot of a Property resource in change history.
            #[prost(message, tag = "2")]
            Property(super::super::Property),
            /// A snapshot of a FirebaseLink resource in change history.
            #[prost(message, tag = "6")]
            FirebaseLink(super::super::FirebaseLink),
            /// A snapshot of a GoogleAdsLink resource in change history.
            #[prost(message, tag = "7")]
            GoogleAdsLink(super::super::GoogleAdsLink),
            /// A snapshot of a GoogleSignalsSettings resource in change history.
            #[prost(message, tag = "8")]
            GoogleSignalsSettings(super::super::GoogleSignalsSettings),
            /// A snapshot of a DisplayVideo360AdvertiserLink resource in change
            /// history.
            #[prost(message, tag = "9")]
            DisplayVideo360AdvertiserLink(super::super::DisplayVideo360AdvertiserLink),
            /// A snapshot of a DisplayVideo360AdvertiserLinkProposal resource in
            /// change history.
            #[prost(message, tag = "10")]
            DisplayVideo360AdvertiserLinkProposal(
                super::super::DisplayVideo360AdvertiserLinkProposal,
            ),
            /// A snapshot of a ConversionEvent resource in change history.
            #[prost(message, tag = "11")]
            ConversionEvent(super::super::ConversionEvent),
            /// A snapshot of a MeasurementProtocolSecret resource in change history.
            #[prost(message, tag = "12")]
            MeasurementProtocolSecret(super::super::MeasurementProtocolSecret),
            /// A snapshot of a CustomDimension resource in change history.
            #[prost(message, tag = "13")]
            CustomDimension(super::super::CustomDimension),
            /// A snapshot of a CustomMetric resource in change history.
            #[prost(message, tag = "14")]
            CustomMetric(super::super::CustomMetric),
            /// A snapshot of a data retention settings resource in change history.
            #[prost(message, tag = "15")]
            DataRetentionSettings(super::super::DataRetentionSettings),
            /// A snapshot of a SearchAds360Link resource in change history.
            #[prost(message, tag = "16")]
            SearchAds360Link(super::super::SearchAds360Link),
            /// A snapshot of a DataStream resource in change history.
            #[prost(message, tag = "18")]
            DataStream(super::super::DataStream),
            /// A snapshot of AttributionSettings resource in change history.
            #[prost(message, tag = "20")]
            AttributionSettings(super::super::AttributionSettings),
            /// A snapshot of an ExpandedDataSet resource in change history.
            #[prost(message, tag = "21")]
            ExpandedDataSet(super::super::ExpandedDataSet),
            /// A snapshot of a ChannelGroup resource in change history.
            #[prost(message, tag = "22")]
            ChannelGroup(super::super::ChannelGroup),
            /// A snapshot of a BigQuery link resource in change history.
            #[prost(message, tag = "23")]
            BigqueryLink(super::super::BigQueryLink),
            /// A snapshot of EnhancedMeasurementSettings resource in change history.
            #[prost(message, tag = "24")]
            EnhancedMeasurementSettings(super::super::EnhancedMeasurementSettings),
            /// A snapshot of DataRedactionSettings resource in change history.
            #[prost(message, tag = "25")]
            DataRedactionSettings(super::super::DataRedactionSettings),
            /// A snapshot of SKAdNetworkConversionValueSchema resource in change
            /// history.
            #[prost(message, tag = "26")]
            SkadnetworkConversionValueSchema(super::super::SkAdNetworkConversionValueSchema),
            /// A snapshot of an AdSenseLink resource in change history.
            #[prost(message, tag = "27")]
            AdsenseLink(super::super::AdSenseLink),
            /// A snapshot of an Audience resource in change history.
            #[prost(message, tag = "28")]
            Audience(super::super::Audience),
            /// A snapshot of an EventCreateRule resource in change history.
            #[prost(message, tag = "29")]
            EventCreateRule(super::super::EventCreateRule),
            /// A snapshot of a CalculatedMetric resource in change history.
            #[prost(message, tag = "31")]
            CalculatedMetric(super::super::CalculatedMetric),
        }
    }
}
/// A link between a GA4 property and a Display & Video 360 advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayVideo360AdvertiserLink {
    /// Output only. The resource name for this DisplayVideo360AdvertiserLink
    /// resource. Format:
    /// properties/{propertyId}/displayVideo360AdvertiserLinks/{linkId}
    ///
    /// Note: linkId is not the Display & Video 360 Advertiser ID
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The Display & Video 360 Advertiser's advertiser ID.
    #[prost(string, tag = "2")]
    pub advertiser_id: ::prost::alloc::string::String,
    /// Output only. The display name of the Display & Video 360 Advertiser.
    #[prost(string, tag = "3")]
    pub advertiser_display_name: ::prost::alloc::string::String,
    /// Enables personalized advertising features with this integration.
    /// If this field is not set on create/update, it will be defaulted to true.
    #[prost(message, optional, tag = "4")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of campaign data from Display & Video 360
    /// into the GA4 property. After link creation, this can only be updated from
    /// the Display & Video 360 product. If this field is not set on create, it
    /// will be defaulted to true.
    #[prost(message, optional, tag = "5")]
    pub campaign_data_sharing_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of cost data from Display & Video 360 into
    /// the GA4 property. This can only be enabled if campaign_data_sharing_enabled
    /// is enabled. After link creation, this can only be updated from the Display
    /// & Video 360 product. If this field is not set on create, it will be
    /// defaulted to true.
    #[prost(message, optional, tag = "6")]
    pub cost_data_sharing_enabled: ::core::option::Option<bool>,
}
/// A proposal for a link between a GA4 property and a Display & Video 360
/// advertiser.
///
/// A proposal is converted to a DisplayVideo360AdvertiserLink once approved.
/// Google Analytics admins approve inbound proposals while Display & Video 360
/// admins approve outbound proposals.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayVideo360AdvertiserLinkProposal {
    /// Output only. The resource name for this
    /// DisplayVideo360AdvertiserLinkProposal resource. Format:
    /// properties/{propertyId}/displayVideo360AdvertiserLinkProposals/{proposalId}
    ///
    /// Note: proposalId is not the Display & Video 360 Advertiser ID
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The Display & Video 360 Advertiser's advertiser ID.
    #[prost(string, tag = "2")]
    pub advertiser_id: ::prost::alloc::string::String,
    /// Output only. The status information for this link proposal.
    #[prost(message, optional, tag = "3")]
    pub link_proposal_status_details: ::core::option::Option<LinkProposalStatusDetails>,
    /// Output only. The display name of the Display & Video Advertiser.
    /// Only populated for proposals that originated from Display & Video 360.
    #[prost(string, tag = "4")]
    pub advertiser_display_name: ::prost::alloc::string::String,
    /// Input only. On a proposal being sent to Display & Video 360, this field
    /// must be set to the email address of an admin on the target advertiser. This
    /// is used to verify that the Google Analytics admin is aware of at least one
    /// admin on the Display & Video 360 Advertiser. This does not restrict
    /// approval of the proposal to a single user. Any admin on the Display & Video
    /// 360 Advertiser may approve the proposal.
    #[prost(string, tag = "5")]
    pub validation_email: ::prost::alloc::string::String,
    /// Immutable. Enables personalized advertising features with this integration.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "6")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of campaign data from Display & Video 360.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "7")]
    pub campaign_data_sharing_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of cost data from Display & Video 360.
    /// This can only be enabled if campaign_data_sharing_enabled is enabled.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "8")]
    pub cost_data_sharing_enabled: ::core::option::Option<bool>,
}
/// A link between a GA4 property and a Search Ads 360 entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360Link {
    /// Output only. The resource name for this SearchAds360Link resource.
    /// Format: properties/{propertyId}/searchAds360Links/{linkId}
    ///
    /// Note: linkId is not the Search Ads 360 advertiser ID
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. This field represents the Advertiser ID of the Search Ads 360
    /// Advertiser. that has been linked.
    #[prost(string, tag = "2")]
    pub advertiser_id: ::prost::alloc::string::String,
    /// Immutable. Enables the import of campaign data from Search Ads 360 into the
    /// GA4 property. After link creation, this can only be updated from the Search
    /// Ads 360 product.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "3")]
    pub campaign_data_sharing_enabled: ::core::option::Option<bool>,
    /// Immutable. Enables the import of cost data from Search Ads 360 to the GA4
    /// property. This can only be enabled if campaign_data_sharing_enabled is
    /// enabled. After link creation, this can only be updated from
    /// the Search Ads 360 product.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "4")]
    pub cost_data_sharing_enabled: ::core::option::Option<bool>,
    /// Output only. The display name of the Search Ads 360 Advertiser.
    /// Allows users to easily identify the linked resource.
    #[prost(string, tag = "5")]
    pub advertiser_display_name: ::prost::alloc::string::String,
    /// Enables personalized advertising features with this integration.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "6")]
    pub ads_personalization_enabled: ::core::option::Option<bool>,
    /// Enables export of site stats with this integration.
    /// If this field is not set on create, it will be defaulted to true.
    #[prost(message, optional, tag = "7")]
    pub site_stats_sharing_enabled: ::core::option::Option<bool>,
}
/// Status information for a link proposal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkProposalStatusDetails {
    /// Output only. The source of this proposal.
    #[prost(enumeration = "LinkProposalInitiatingProduct", tag = "1")]
    pub link_proposal_initiating_product: i32,
    /// Output only. The email address of the user that proposed this linkage.
    #[prost(string, tag = "2")]
    pub requestor_email: ::prost::alloc::string::String,
    /// Output only. The state of this proposal.
    #[prost(enumeration = "LinkProposalState", tag = "3")]
    pub link_proposal_state: i32,
}
/// A conversion event in a Google Analytics property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionEvent {
    /// Output only. Resource name of this conversion event.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The event name for this conversion event.
    /// Examples: 'click', 'purchase'
    #[prost(string, tag = "2")]
    pub event_name: ::prost::alloc::string::String,
    /// Output only. Time when this conversion event was created in the property.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. If set, this event can currently be deleted with
    /// DeleteConversionEvent.
    #[prost(bool, tag = "4")]
    pub deletable: bool,
    /// Output only. If set to true, this conversion event refers to a custom
    /// event.  If set to false, this conversion event refers to a default event in
    /// GA. Default events typically have special meaning in GA. Default events are
    /// usually created for you by the GA system, but in some cases can be created
    /// by property admins. Custom events count towards the maximum number of
    /// custom conversion events that may be created per property.
    #[prost(bool, tag = "5")]
    pub custom: bool,
    /// Optional. The method by which conversions will be counted across multiple
    /// events within a session. If this value is not provided, it will be set to
    /// `ONCE_PER_EVENT`.
    #[prost(enumeration = "conversion_event::ConversionCountingMethod", tag = "6")]
    pub counting_method: i32,
    /// Optional. Defines a default value/currency for a conversion event.
    #[prost(message, optional, tag = "7")]
    pub default_conversion_value: ::core::option::Option<conversion_event::DefaultConversionValue>,
}
/// Nested message and enum types in `ConversionEvent`.
pub mod conversion_event {
    /// Defines a default value/currency for a conversion event. Both value and
    /// currency must be provided.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefaultConversionValue {
        /// This value will be used to populate the value for all conversions
        /// of the specified event_name where the event "value" parameter is unset.
        #[prost(double, optional, tag = "1")]
        pub value: ::core::option::Option<f64>,
        /// When a conversion event for this event_name has no set currency,
        /// this currency will be applied as the default. Must be in ISO 4217
        /// currency code format. See <https://en.wikipedia.org/wiki/ISO_4217> for
        /// more information.
        #[prost(string, optional, tag = "2")]
        pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// The method by which conversions will be counted across multiple events
    /// within a session.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionCountingMethod {
        /// Counting method not specified.
        Unspecified = 0,
        /// Each Event instance is considered a Conversion.
        OncePerEvent = 1,
        /// An Event instance is considered a Conversion at most once per session per
        /// user.
        OncePerSession = 2,
    }
}
/// Settings values for Google Signals.  This is a singleton resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleSignalsSettings {
    /// Output only. Resource name of this setting.
    /// Format: properties/{property_id}/googleSignalsSettings
    /// Example: "properties/1000/googleSignalsSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Status of this setting.
    #[prost(enumeration = "GoogleSignalsState", tag = "3")]
    pub state: i32,
    /// Output only. Terms of Service acceptance.
    #[prost(enumeration = "GoogleSignalsConsent", tag = "4")]
    pub consent: i32,
}
/// A definition for a CustomDimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomDimension {
    /// Output only. Resource name for this CustomDimension resource.
    /// Format: properties/{property}/customDimensions/{customDimension}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Tagging parameter name for this custom dimension.
    ///
    /// If this is a user-scoped dimension, then this is the user property name.
    /// If this is an event-scoped dimension, then this is the event parameter
    /// name.
    ///
    /// If this is an item-scoped dimension, then this is the parameter
    /// name found in the eCommerce items array.
    ///
    /// May only contain alphanumeric and underscore characters, starting with a
    /// letter. Max length of 24 characters for user-scoped dimensions, 40
    /// characters for event-scoped dimensions.
    #[prost(string, tag = "2")]
    pub parameter_name: ::prost::alloc::string::String,
    /// Required. Display name for this custom dimension as shown in the Analytics
    /// UI. Max length of 82 characters, alphanumeric plus space and underscore
    /// starting with a letter. Legacy system-generated display names may contain
    /// square brackets, but updates to this field will never permit square
    /// brackets.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description for this custom dimension. Max length of 150
    /// characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. The scope of this dimension.
    #[prost(enumeration = "custom_dimension::DimensionScope", tag = "5")]
    pub scope: i32,
    /// Optional. If set to true, sets this dimension as NPA and excludes it from
    /// ads personalization.
    ///
    /// This is currently only supported by user-scoped custom dimensions.
    #[prost(bool, tag = "6")]
    pub disallow_ads_personalization: bool,
}
/// Nested message and enum types in `CustomDimension`.
pub mod custom_dimension {
    /// Valid values for the scope of this dimension.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DimensionScope {
        /// Scope unknown or not specified.
        Unspecified = 0,
        /// Dimension scoped to an event.
        Event = 1,
        /// Dimension scoped to a user.
        User = 2,
        /// Dimension scoped to eCommerce items
        Item = 3,
    }
}
/// A definition for a custom metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomMetric {
    /// Output only. Resource name for this CustomMetric resource.
    /// Format: properties/{property}/customMetrics/{customMetric}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Tagging name for this custom metric.
    ///
    /// If this is an event-scoped metric, then this is the event parameter
    /// name.
    ///
    /// May only contain alphanumeric and underscore charactes, starting with a
    /// letter. Max length of 40 characters for event-scoped metrics.
    #[prost(string, tag = "2")]
    pub parameter_name: ::prost::alloc::string::String,
    /// Required. Display name for this custom metric as shown in the Analytics UI.
    /// Max length of 82 characters, alphanumeric plus space and underscore
    /// starting with a letter. Legacy system-generated display names may contain
    /// square brackets, but updates to this field will never permit square
    /// brackets.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description for this custom dimension.
    /// Max length of 150 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Required. The type for the custom metric's value.
    #[prost(enumeration = "custom_metric::MeasurementUnit", tag = "5")]
    pub measurement_unit: i32,
    /// Required. Immutable. The scope of this custom metric.
    #[prost(enumeration = "custom_metric::MetricScope", tag = "6")]
    pub scope: i32,
    /// Optional. Types of restricted data that this metric may contain. Required
    /// for metrics with CURRENCY measurement unit. Must be empty for metrics with
    /// a non-CURRENCY measurement unit.
    #[prost(
        enumeration = "custom_metric::RestrictedMetricType",
        repeated,
        packed = "false",
        tag = "8"
    )]
    pub restricted_metric_type: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `CustomMetric`.
pub mod custom_metric {
    /// Possible types of representing the custom metric's value.
    ///
    /// Currency representation may change in the future, requiring a breaking API
    /// change.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MeasurementUnit {
        /// MeasurementUnit unspecified or missing.
        Unspecified = 0,
        /// This metric uses default units.
        Standard = 1,
        /// This metric measures a currency.
        Currency = 2,
        /// This metric measures feet.
        Feet = 3,
        /// This metric measures meters.
        Meters = 4,
        /// This metric measures kilometers.
        Kilometers = 5,
        /// This metric measures miles.
        Miles = 6,
        /// This metric measures milliseconds.
        Milliseconds = 7,
        /// This metric measures seconds.
        Seconds = 8,
        /// This metric measures minutes.
        Minutes = 9,
        /// This metric measures hours.
        Hours = 10,
    }
    /// The scope of this metric.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MetricScope {
        /// Scope unknown or not specified.
        Unspecified = 0,
        /// Metric scoped to an event.
        Event = 1,
    }
    /// Labels that mark the data in this custom metric as data that should be
    /// restricted to specific users.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RestrictedMetricType {
        /// Type unknown or unspecified.
        Unspecified = 0,
        /// Metric reports cost data.
        CostData = 1,
        /// Metric reports revenue data.
        RevenueData = 2,
    }
}
/// A definition for a calculated metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculatedMetric {
    /// Output only. Resource name for this CalculatedMetric.
    /// Format: 'properties/{property_id}/calculatedMetrics/{calculated_metric_id}'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Description for this calculated metric.
    /// Max length of 4096 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Display name for this calculated metric as shown in the
    /// Google Analytics UI. Max length 82 characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The ID to use for the calculated metric. In the UI, this is
    /// referred to as the "API name."
    ///
    /// The calculated_metric_id is used when referencing this calculated metric
    /// from external APIs. For example, "calcMetric:{calculated_metric_id}".
    #[prost(string, tag = "4")]
    pub calculated_metric_id: ::prost::alloc::string::String,
    /// Required. The type for the calculated metric's value.
    #[prost(enumeration = "calculated_metric::MetricUnit", tag = "5")]
    pub metric_unit: i32,
    /// Output only. Types of restricted data that this metric contains.
    #[prost(
        enumeration = "calculated_metric::RestrictedMetricType",
        repeated,
        packed = "false",
        tag = "6"
    )]
    pub restricted_metric_type: ::prost::alloc::vec::Vec<i32>,
    /// Required. The calculated metric's definition. Maximum number of unique
    /// referenced custom metrics is 5. Formulas supports the following operations:
    /// + (addition),  - (subtraction), - (negative),  * (multiplication), /
    /// (division), () (parenthesis). Any valid real numbers are acceptable that
    /// fit in a Long (64bit integer) or a Double (64 bit floating point number).
    /// Example formula:
    ///   "( customEvent:parameter_name + cartPurchaseQuantity ) / 2.0"
    #[prost(string, tag = "7")]
    pub formula: ::prost::alloc::string::String,
    /// Output only. If true, this calculated metric has a invalid metric
    /// reference. Anything using a calculated metric with invalid_metric_reference
    /// set to true may fail, produce warnings, or produce unexpected results.
    #[prost(bool, tag = "9")]
    pub invalid_metric_reference: bool,
}
/// Nested message and enum types in `CalculatedMetric`.
pub mod calculated_metric {
    /// Possible types of representing the calculated metric's value.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MetricUnit {
        /// MetricUnit unspecified or missing.
        Unspecified = 0,
        /// This metric uses default units.
        Standard = 1,
        /// This metric measures a currency.
        Currency = 2,
        /// This metric measures feet.
        Feet = 3,
        /// This metric measures miles.
        Miles = 4,
        /// This metric measures meters.
        Meters = 5,
        /// This metric measures kilometers.
        Kilometers = 6,
        /// This metric measures milliseconds.
        Milliseconds = 7,
        /// This metric measures seconds.
        Seconds = 8,
        /// This metric measures minutes.
        Minutes = 9,
        /// This metric measures hours.
        Hours = 10,
    }
    /// Labels that mark the data in calculated metric used in conjunction with
    /// user roles that restrict access to cost and/or revenue metrics.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RestrictedMetricType {
        /// Type unknown or unspecified.
        Unspecified = 0,
        /// Metric reports cost data.
        CostData = 1,
        /// Metric reports revenue data.
        RevenueData = 2,
    }
}
/// Settings values for data retention. This is a singleton resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRetentionSettings {
    /// Output only. Resource name for this DataRetentionSetting resource.
    /// Format: properties/{property}/dataRetentionSettings
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The length of time that event-level data is retained.
    #[prost(enumeration = "data_retention_settings::RetentionDuration", tag = "2")]
    pub event_data_retention: i32,
    /// If true, reset the retention period for the user identifier with every
    /// event from that user.
    #[prost(bool, tag = "3")]
    pub reset_user_data_on_new_activity: bool,
}
/// Nested message and enum types in `DataRetentionSettings`.
pub mod data_retention_settings {
    /// Valid values for the data retention duration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RetentionDuration {
        /// Data retention time duration is not specified.
        Unspecified = 0,
        /// The data retention time duration is 2 months.
        TwoMonths = 1,
        /// The data retention time duration is 14 months.
        FourteenMonths = 3,
        /// The data retention time duration is 26 months.
        /// Available to 360 properties only.
        TwentySixMonths = 4,
        /// The data retention time duration is 38 months.
        /// Available to 360 properties only.
        ThirtyEightMonths = 5,
        /// The data retention time duration is 50 months.
        /// Available to 360 properties only.
        FiftyMonths = 6,
    }
}
/// The attribution settings used for a given property. This is a singleton
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributionSettings {
    /// Output only. Resource name of this attribution settings resource.
    /// Format: properties/{property_id}/attributionSettings
    /// Example: "properties/1000/attributionSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The lookback window configuration for acquisition conversion
    /// events. The default window size is 30 days.
    #[prost(
        enumeration = "attribution_settings::AcquisitionConversionEventLookbackWindow",
        tag = "2"
    )]
    pub acquisition_conversion_event_lookback_window: i32,
    /// Required. The lookback window for all other, non-acquisition conversion
    /// events. The default window size is 90 days.
    #[prost(
        enumeration = "attribution_settings::OtherConversionEventLookbackWindow",
        tag = "3"
    )]
    pub other_conversion_event_lookback_window: i32,
    /// Required. The reporting attribution model used to calculate conversion
    /// credit in this property's reports.
    ///
    /// Changing the attribution model will apply to both historical and future
    /// data. These changes will be reflected in reports with conversion and
    /// revenue data. User and session data will be unaffected.
    #[prost(
        enumeration = "attribution_settings::ReportingAttributionModel",
        tag = "4"
    )]
    pub reporting_attribution_model: i32,
    /// Required. The Conversion Export Scope for data exported to linked Ads
    /// Accounts.
    #[prost(
        enumeration = "attribution_settings::AdsWebConversionDataExportScope",
        tag = "5"
    )]
    pub ads_web_conversion_data_export_scope: i32,
}
/// Nested message and enum types in `AttributionSettings`.
pub mod attribution_settings {
    /// How far back in time events should be considered for inclusion in a
    /// converting path which leads to the first install of an app or the first
    /// visit to a site.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AcquisitionConversionEventLookbackWindow {
        /// Lookback window size unspecified.
        Unspecified = 0,
        /// 7-day lookback window.
        AcquisitionConversionEventLookbackWindow7Days = 1,
        /// 30-day lookback window.
        AcquisitionConversionEventLookbackWindow30Days = 2,
    }
    /// How far back in time events should be considered for inclusion in a
    /// converting path for all conversions other than first app install/first site
    /// visit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OtherConversionEventLookbackWindow {
        /// Lookback window size unspecified.
        Unspecified = 0,
        /// 30-day lookback window.
        OtherConversionEventLookbackWindow30Days = 1,
        /// 60-day lookback window.
        OtherConversionEventLookbackWindow60Days = 2,
        /// 90-day lookback window.
        OtherConversionEventLookbackWindow90Days = 3,
    }
    /// The reporting attribution model used to calculate conversion credit in this
    /// property's reports.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReportingAttributionModel {
        /// Reporting attribution model unspecified.
        Unspecified = 0,
        /// Data-driven attribution distributes credit for the conversion based on
        /// data for each conversion event. Each Data-driven model is specific to
        /// each advertiser and each conversion event.
        /// Previously CROSS_CHANNEL_DATA_DRIVEN
        PaidAndOrganicChannelsDataDriven = 1,
        /// Ignores direct traffic and attributes 100% of the conversion value to the
        /// last channel that the customer clicked through (or engaged view through
        /// for YouTube) before converting.
        /// Previously CROSS_CHANNEL_LAST_CLICK
        PaidAndOrganicChannelsLastClick = 2,
        /// Attributes 100% of the conversion value to the last Google Paid channel
        /// that the customer clicked through before converting.
        /// Previously ADS_PREFERRED_LAST_CLICK
        GooglePaidChannelsLastClick = 7,
    }
    /// The Conversion Export Scope for data exported to linked Ads Accounts.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdsWebConversionDataExportScope {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// No data export scope selected yet.
        /// Export scope can never be changed back to this value.
        NotSelectedYet = 1,
        /// Paid and organic channels are eligible to receive conversion credit, but
        /// only credit assigned to Google Ads channels will appear in your Ads
        /// accounts. To learn more, see [Paid and Organic
        /// channels](<https://support.google.com/analytics/answer/10632359>).
        PaidAndOrganicChannels = 2,
        /// Only Google Ads paid channels are eligible to receive conversion credit.
        /// To learn more, see [Google Paid
        /// channels](<https://support.google.com/analytics/answer/10632359>).
        GooglePaidChannels = 3,
    }
}
/// A binding of a user to a set of roles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessBinding {
    /// Output only. Resource name of this binding.
    ///
    /// Format: accounts/{account}/accessBindings/{access_binding} or
    /// properties/{property}/accessBindings/{access_binding}
    ///
    /// Example:
    /// "accounts/100/accessBindings/200"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of roles for to grant to the parent resource.
    ///
    /// Valid values:
    /// predefinedRoles/viewer
    /// predefinedRoles/analyst
    /// predefinedRoles/editor
    /// predefinedRoles/admin
    /// predefinedRoles/no-cost-data
    /// predefinedRoles/no-revenue-data
    ///
    /// For users, if an empty list of roles is set, this AccessBinding will be
    /// deleted.
    #[prost(string, repeated, tag = "3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The target for which to set roles for.
    #[prost(oneof = "access_binding::AccessTarget", tags = "2")]
    pub access_target: ::core::option::Option<access_binding::AccessTarget>,
}
/// Nested message and enum types in `AccessBinding`.
pub mod access_binding {
    /// The target for which to set roles for.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessTarget {
        /// If set, the email address of the user to set roles for.
        /// Format: "someuser@gmail.com"
        #[prost(string, tag = "2")]
        User(::prost::alloc::string::String),
    }
}
/// A link between a GA4 Property and BigQuery project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryLink {
    /// Output only. Resource name of this BigQuery link.
    /// Format: 'properties/{property_id}/bigQueryLinks/{bigquery_link_id}'
    /// Format: 'properties/1234/bigQueryLinks/abc567'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The linked Google Cloud project. When creating a BigQueryLink,
    /// you may provide this resource name using either a project number or project
    /// ID. Once this resource has been created, the returned project will always
    /// have a project that contains a project number.
    /// Format: 'projects/{project number}'
    /// Example: 'projects/1234'
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Output only. Time when the link was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set true, enables daily data export to the linked Google Cloud project.
    #[prost(bool, tag = "4")]
    pub daily_export_enabled: bool,
    /// If set true, enables streaming export to the linked Google Cloud project.
    #[prost(bool, tag = "5")]
    pub streaming_export_enabled: bool,
    /// If set true, enables fresh daily export to the linked Google Cloud project.
    #[prost(bool, tag = "9")]
    pub fresh_daily_export_enabled: bool,
    /// If set true, exported data will include advertising identifiers for mobile
    /// app streams.
    #[prost(bool, tag = "6")]
    pub include_advertising_id: bool,
    /// The list of streams under the parent property for which data will be
    /// exported.
    /// Format: properties/{property_id}/dataStreams/{stream_id}
    /// Example: \['properties/1000/dataStreams/2000'\]
    #[prost(string, repeated, tag = "7")]
    pub export_streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of event names that will be excluded from exports.
    #[prost(string, repeated, tag = "8")]
    pub excluded_events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Singleton resource under a web DataStream, configuring measurement of
/// additional site interactions and content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhancedMeasurementSettings {
    /// Output only. Resource name of the Enhanced Measurement Settings.
    /// Format:
    /// properties/{property_id}/dataStreams/{data_stream}/enhancedMeasurementSettings
    /// Example: "properties/1000/dataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates whether Enhanced Measurement Settings will be used to
    /// automatically measure interactions and content on this web stream.
    ///
    /// Changing this value does not affect the settings themselves, but determines
    /// whether they are respected.
    #[prost(bool, tag = "2")]
    pub stream_enabled: bool,
    /// If enabled, capture scroll events each time a visitor gets to the bottom of
    /// a page.
    #[prost(bool, tag = "3")]
    pub scrolls_enabled: bool,
    /// If enabled, capture an outbound click event each time a visitor clicks a
    /// link that leads them away from your domain.
    #[prost(bool, tag = "4")]
    pub outbound_clicks_enabled: bool,
    /// If enabled, capture a view search results event each time a visitor
    /// performs a search on your site (based on a query parameter).
    #[prost(bool, tag = "5")]
    pub site_search_enabled: bool,
    /// If enabled, capture video play, progress, and complete events as visitors
    /// view embedded videos on your site.
    #[prost(bool, tag = "6")]
    pub video_engagement_enabled: bool,
    /// If enabled, capture a file download event each time a link is clicked with
    /// a common document, compressed file, application, video, or audio extension.
    #[prost(bool, tag = "7")]
    pub file_downloads_enabled: bool,
    /// If enabled, capture a page view event each time the website changes the
    /// browser history state.
    #[prost(bool, tag = "8")]
    pub page_changes_enabled: bool,
    /// If enabled, capture a form interaction event each time a visitor interacts
    /// with a form on your website.
    /// False by default.
    #[prost(bool, tag = "9")]
    pub form_interactions_enabled: bool,
    /// Required. URL query parameters to interpret as site search parameters.
    /// Max length is 1024 characters. Must not be empty.
    #[prost(string, tag = "10")]
    pub search_query_parameter: ::prost::alloc::string::String,
    /// Additional URL query parameters.
    /// Max length is 1024 characters.
    #[prost(string, tag = "11")]
    pub uri_query_parameter: ::prost::alloc::string::String,
}
/// Configuration for a specific Connected Site Tag.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedSiteTag {
    /// Required. User-provided display name for the connected site tag. Must be
    /// less than 256 characters.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. "Tag ID to forward events to. Also known as the Measurement ID,
    /// or the "G-ID"  (For example: G-12345).
    #[prost(string, tag = "2")]
    pub tag_id: ::prost::alloc::string::String,
}
/// Settings for client-side data redaction. Singleton resource under a Web
/// Stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRedactionSettings {
    /// Output only. Name of this Data Redaction Settings resource.
    /// Format:
    /// properties/{property_id}/dataStreams/{data_stream}/dataRedactionSettings
    /// Example: "properties/1000/dataStreams/2000/dataRedactionSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If enabled, any event parameter or user property values that look like an
    /// email will be redacted.
    #[prost(bool, tag = "2")]
    pub email_redaction_enabled: bool,
    /// Query Parameter redaction removes the key and value portions of a
    /// query parameter if it is in the configured set of query parameters.
    ///
    /// If enabled, URL query replacement logic will be run for the Stream. Any
    /// query parameters defined in query_parameter_keys will be redacted.
    #[prost(bool, tag = "3")]
    pub query_parameter_redaction_enabled: bool,
    /// The query parameter keys to apply redaction logic to if present in the URL.
    /// Query parameter matching is case-insensitive.
    ///
    /// Must contain at least one element if query_parameter_replacement_enabled
    /// is true. Keys cannot contain commas.
    #[prost(string, repeated, tag = "4")]
    pub query_parameter_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A link between a GA4 Property and an AdSense for Content ad client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdSenseLink {
    /// Output only. The resource name for this AdSense Link resource.
    /// Format: properties/{propertyId}/adSenseLinks/{linkId}
    /// Example: properties/1234/adSenseLinks/6789
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The AdSense ad client code that the GA4 property is linked to.
    /// Example format: "ca-pub-1234567890"
    #[prost(string, tag = "2")]
    pub ad_client_code: ::prost::alloc::string::String,
}
/// A link that references a source property under the parent rollup property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollupPropertySourceLink {
    /// Output only. Resource name of this RollupPropertySourceLink.
    /// Format:
    /// 'properties/{property_id}/rollupPropertySourceLinks/{rollup_property_source_link}'
    /// Format: 'properties/123/rollupPropertySourceLinks/456'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Resource name of the source property.
    /// Format: properties/{property_id}
    /// Example: "properties/789"
    #[prost(string, tag = "2")]
    pub source_property: ::prost::alloc::string::String,
}
/// The category selected for this property, used for industry benchmarking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndustryCategory {
    /// Industry category unspecified
    Unspecified = 0,
    /// Automotive
    Automotive = 1,
    /// Business and industrial markets
    BusinessAndIndustrialMarkets = 2,
    /// Finance
    Finance = 3,
    /// Healthcare
    Healthcare = 4,
    /// Technology
    Technology = 5,
    /// Travel
    Travel = 6,
    /// Other
    Other = 7,
    /// Arts and entertainment
    ArtsAndEntertainment = 8,
    /// Beauty and fitness
    BeautyAndFitness = 9,
    /// Books and literature
    BooksAndLiterature = 10,
    /// Food and drink
    FoodAndDrink = 11,
    /// Games
    Games = 12,
    /// Hobbies and leisure
    HobbiesAndLeisure = 13,
    /// Home and garden
    HomeAndGarden = 14,
    /// Internet and telecom
    InternetAndTelecom = 15,
    /// Law and government
    LawAndGovernment = 16,
    /// News
    News = 17,
    /// Online communities
    OnlineCommunities = 18,
    /// People and society
    PeopleAndSociety = 19,
    /// Pets and animals
    PetsAndAnimals = 20,
    /// Real estate
    RealEstate = 21,
    /// Reference
    Reference = 22,
    /// Science
    Science = 23,
    /// Sports
    Sports = 24,
    /// Jobs and education
    JobsAndEducation = 25,
    /// Shopping
    Shopping = 26,
}
/// Various levels of service for Google Analytics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceLevel {
    /// Service level not specified or invalid.
    Unspecified = 0,
    /// The standard version of Google Analytics.
    GoogleAnalyticsStandard = 1,
    /// The paid, premium version of Google Analytics.
    GoogleAnalytics360 = 2,
}
/// Different kinds of actors that can make changes to Google Analytics
/// resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActorType {
    /// Unknown or unspecified actor type.
    Unspecified = 0,
    /// Changes made by the user specified in actor_email.
    User = 1,
    /// Changes made by the Google Analytics system.
    System = 2,
    /// Changes made by Google Analytics support team staff.
    Support = 3,
}
/// Types of actions that may change a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    /// Action type unknown or not specified.
    Unspecified = 0,
    /// Resource was created in this change.
    Created = 1,
    /// Resource was updated in this change.
    Updated = 2,
    /// Resource was deleted in this change.
    Deleted = 3,
}
/// Types of resources whose changes may be returned from change history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeHistoryResourceType {
    /// Resource type unknown or not specified.
    Unspecified = 0,
    /// Account resource
    Account = 1,
    /// Property resource
    Property = 2,
    /// FirebaseLink resource
    FirebaseLink = 6,
    /// GoogleAdsLink resource
    GoogleAdsLink = 7,
    /// GoogleSignalsSettings resource
    GoogleSignalsSettings = 8,
    /// ConversionEvent resource
    ConversionEvent = 9,
    /// MeasurementProtocolSecret resource
    MeasurementProtocolSecret = 10,
    /// CustomDimension resource
    CustomDimension = 11,
    /// CustomMetric resource
    CustomMetric = 12,
    /// DataRetentionSettings resource
    DataRetentionSettings = 13,
    /// DisplayVideo360AdvertiserLink resource
    DisplayVideo360AdvertiserLink = 14,
    /// DisplayVideo360AdvertiserLinkProposal resource
    DisplayVideo360AdvertiserLinkProposal = 15,
    /// SearchAds360Link resource
    SearchAds360Link = 16,
    /// DataStream resource
    DataStream = 18,
    /// AttributionSettings resource
    AttributionSettings = 20,
    /// ExpandedDataSet resource
    ExpandedDataSet = 21,
    /// ChannelGroup resource
    ChannelGroup = 22,
    /// EnhancedMeasurementSettings resource
    EnhancedMeasurementSettings = 24,
    /// DataRedactionSettings resource
    DataRedactionSettings = 25,
    /// SKAdNetworkConversionValueSchema resource
    SkadnetworkConversionValueSchema = 26,
    /// AdSenseLink resource
    AdsenseLink = 27,
    /// Audience resource
    Audience = 28,
    /// EventCreateRule resource
    EventCreateRule = 29,
    /// CalculatedMetric resource
    CalculatedMetric = 31,
}
/// Status of the Google Signals settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GoogleSignalsState {
    /// Google Signals status defaults to GOOGLE_SIGNALS_STATE_UNSPECIFIED to
    /// represent that the user has not made an explicit choice.
    Unspecified = 0,
    /// Google Signals is enabled.
    GoogleSignalsEnabled = 1,
    /// Google Signals is disabled.
    GoogleSignalsDisabled = 2,
}
/// Consent field of the Google Signals settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GoogleSignalsConsent {
    /// Google Signals consent value defaults to
    /// GOOGLE_SIGNALS_CONSENT_UNSPECIFIED.  This will be treated as
    /// GOOGLE_SIGNALS_CONSENT_NOT_CONSENTED.
    Unspecified = 0,
    /// Terms of service have been accepted
    Consented = 2,
    /// Terms of service have not been accepted
    NotConsented = 1,
}
/// An indication of which product the user initiated a link proposal from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LinkProposalInitiatingProduct {
    /// Unspecified product.
    Unspecified = 0,
    /// This proposal was created by a user from Google Analytics.
    GoogleAnalytics = 1,
    /// This proposal was created by a user from a linked product (not Google
    /// Analytics).
    LinkedProduct = 2,
}
/// The state of a link proposal resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LinkProposalState {
    /// Unspecified state
    Unspecified = 0,
    /// This proposal is awaiting review from a Google Analytics user. This
    /// proposal will automatically expire after some time.
    AwaitingReviewFromGoogleAnalytics = 1,
    /// This proposal is awaiting review from a user of a linked product. This
    /// proposal will automatically expire after some time.
    AwaitingReviewFromLinkedProduct = 2,
    /// This proposal has been withdrawn by an admin on the initiating product.
    /// This proposal will be automatically deleted after some time.
    Withdrawn = 3,
    /// This proposal has been declined by an admin on the receiving product. This
    /// proposal will be automatically deleted after some time.
    Declined = 4,
    /// This proposal expired due to lack of response from an admin on the
    /// receiving product. This proposal will be automatically deleted after some
    /// time.
    Expired = 5,
    /// This proposal has become obsolete because a link was directly created to
    /// the same external product resource that this proposal specifies. This
    /// proposal will be automatically deleted after some time.
    Obsolete = 6,
}
/// Types of Property resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PropertyType {
    /// Unknown or unspecified property type
    Unspecified = 0,
    /// Ordinary GA4 property
    Ordinary = 1,
    /// GA4 subproperty
    Subproperty = 2,
    /// GA4 rollup property
    Rollup = 3,
}
/// The coarse conversion value set on the updatePostbackConversionValue SDK call
/// when a ConversionValues.event_mappings conditions are satisfied. For
/// more information, see
/// \[SKAdNetwork.CoarseConversionValue\](<https://developer.apple.com/documentation/storekit/skadnetwork/coarseconversionvalue>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoarseValue {
    /// Coarse value not specified.
    Unspecified = 0,
    /// Coarse value of low.
    Low = 1,
    /// Coarse value of medium.
    Medium = 2,
    /// Coarse value of high.
    High = 3,
}
/// A specific filter expression
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpropertyEventFilterCondition {
    /// Required. The field that is being filtered.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    #[prost(oneof = "subproperty_event_filter_condition::OneFilter", tags = "2, 3")]
    pub one_filter: ::core::option::Option<subproperty_event_filter_condition::OneFilter>,
}
/// Nested message and enum types in `SubpropertyEventFilterCondition`.
pub mod subproperty_event_filter_condition {
    /// A filter for a string-type dimension that matches a particular pattern.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// Required. The match type for the string filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// Required. The string value used for the matching.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        /// Optional. If true, the string value is case sensitive. If false, the
        /// match is case-insensitive.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
    /// Nested message and enum types in `StringFilter`.
    pub mod string_filter {
        /// How the filter will be used to determine a match.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MatchType {
            /// Match type unknown or not specified.
            Unspecified = 0,
            /// Exact match of the string value.
            Exact = 1,
            /// Begins with the string value.
            BeginsWith = 2,
            /// Ends with the string value.
            EndsWith = 3,
            /// Contains the string value.
            Contains = 4,
            /// Full regular expression matches with the string value.
            FullRegexp = 5,
            /// Partial regular expression matches with the string value.
            PartialRegexp = 6,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// A filter for null values.
        #[prost(bool, tag = "2")]
        NullFilter(bool),
        /// A filter for a string-type dimension that matches a particular pattern.
        #[prost(message, tag = "3")]
        StringFilter(StringFilter),
    }
}
/// A logical expression of Subproperty event filters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpropertyEventFilterExpression {
    /// The expression applied to a filter.
    #[prost(oneof = "subproperty_event_filter_expression::Expr", tags = "1, 2, 3")]
    pub expr: ::core::option::Option<subproperty_event_filter_expression::Expr>,
}
/// Nested message and enum types in `SubpropertyEventFilterExpression`.
pub mod subproperty_event_filter_expression {
    /// The expression applied to a filter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// A list of expressions to OR’ed together. Must only contain
        /// not_expression or filter_condition expressions.
        #[prost(message, tag = "1")]
        OrGroup(super::SubpropertyEventFilterExpressionList),
        /// A filter expression to be NOT'ed (inverted, complemented). It can only
        /// include a filter. This cannot be set on the top level
        /// SubpropertyEventFilterExpression.
        #[prost(message, tag = "2")]
        NotExpression(::prost::alloc::boxed::Box<super::SubpropertyEventFilterExpression>),
        /// Creates a filter that matches a specific event. This cannot be set on the
        /// top level SubpropertyEventFilterExpression.
        #[prost(message, tag = "3")]
        FilterCondition(super::SubpropertyEventFilterCondition),
    }
}
/// A list of Subproperty event filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpropertyEventFilterExpressionList {
    /// Required. Unordered list. A list of Subproperty event filter expressions
    #[prost(message, repeated, tag = "1")]
    pub filter_expressions: ::prost::alloc::vec::Vec<SubpropertyEventFilterExpression>,
}
/// A clause for defining a filter. A filter may be inclusive (events satisfying
/// the filter clause are included in the subproperty's data) or exclusive
/// (events satisfying the filter clause are excluded from the subproperty's
/// data).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpropertyEventFilterClause {
    /// Required. The type for the filter clause.
    #[prost(
        enumeration = "subproperty_event_filter_clause::FilterClauseType",
        tag = "1"
    )]
    pub filter_clause_type: i32,
    /// Required. The logical expression for what events are sent to the
    /// subproperty.
    #[prost(message, optional, tag = "2")]
    pub filter_expression: ::core::option::Option<SubpropertyEventFilterExpression>,
}
/// Nested message and enum types in `SubpropertyEventFilterClause`.
pub mod subproperty_event_filter_clause {
    /// Specifies whether this is an include or exclude filter clause.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FilterClauseType {
        /// Filter clause type unknown or not specified.
        Unspecified = 0,
        /// Events will be included in the Sub property if the filter clause is met.
        Include = 1,
        /// Events will be excluded from the Sub property if the filter clause is
        /// met.
        Exclude = 2,
    }
}
/// A resource message representing a GA4 Subproperty event filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpropertyEventFilter {
    /// Output only. Format:
    /// properties/{ordinary_property_id}/subpropertyEventFilters/{sub_property_event_filter}
    /// Example: properties/1234/subpropertyEventFilters/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Resource name of the Subproperty that uses this filter.
    #[prost(string, optional, tag = "2")]
    pub apply_to_property: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. Unordered list. Filter clauses that define the
    /// SubpropertyEventFilter. All clauses are AND'ed together to determine what
    /// data is sent to the subproperty.
    #[prost(message, repeated, tag = "3")]
    pub filter_clauses: ::prost::alloc::vec::Vec<SubpropertyEventFilterClause>,
}
/// The request for a Data Access Record Report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAccessReportRequest {
    /// The Data Access Report supports requesting at the property level or account
    /// level. If requested at the account level, Data Access Reports include all
    /// access for all properties under that account.
    ///
    /// To request at the property level, entity should be for example
    /// 'properties/123' if "123" is your GA4 property ID. To request at the
    /// account level, entity should be for example 'accounts/1234' if "1234" is
    /// your GA4 Account ID.
    #[prost(string, tag = "1")]
    pub entity: ::prost::alloc::string::String,
    /// The dimensions requested and displayed in the response. Requests are
    /// allowed up to 9 dimensions.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<AccessDimension>,
    /// The metrics requested and displayed in the response. Requests are allowed
    /// up to 10 metrics.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<AccessMetric>,
    /// Date ranges of access records to read. If multiple date ranges are
    /// requested, each response row will contain a zero based date range index. If
    /// two date ranges overlap, the access records for the overlapping days is
    /// included in the response rows for both date ranges. Requests are allowed up
    /// to 2 date ranges.
    #[prost(message, repeated, tag = "4")]
    pub date_ranges: ::prost::alloc::vec::Vec<AccessDateRange>,
    /// Dimension filters let you restrict report response to specific
    /// dimension values which match the filter. For example, filtering on access
    /// records of a single user. To learn more, see [Fundamentals of Dimension
    /// Filters](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters>)
    /// for examples. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "5")]
    pub dimension_filter: ::core::option::Option<AccessFilterExpression>,
    /// Metric filters allow you to restrict report response to specific metric
    /// values which match the filter. Metric filters are applied after aggregating
    /// the report's rows, similar to SQL having-clause. Dimensions cannot be used
    /// in this filter.
    #[prost(message, optional, tag = "6")]
    pub metric_filter: ::core::option::Option<AccessFilterExpression>,
    /// The row count of the start row. The first row is counted as row 0. If
    /// offset is unspecified, it is treated as 0. If offset is zero, then this
    /// method will return the first page of results with `limit` entries.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "7")]
    pub offset: i64,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The
    /// API returns a maximum of 100,000 rows per request, no matter how many you
    /// ask for. `limit` must be positive.
    ///
    /// The API may return fewer rows than the requested `limit`, if there aren't
    /// as many remaining rows as the `limit`. For instance, there are fewer than
    /// 300 possible values for the dimension `country`, so when reporting on only
    /// `country`, you can't get more than 300 rows, even if you set `limit` to a
    /// higher value.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "8")]
    pub limit: i64,
    /// This request's time zone if specified. If unspecified, the property's time
    /// zone is used. The request's time zone is used to interpret the start & end
    /// dates of the report.
    ///
    /// Formatted as strings from the IANA Time Zone database
    /// (<https://www.iana.org/time-zones>); for example "America/New_York" or
    /// "Asia/Tokyo".
    #[prost(string, tag = "9")]
    pub time_zone: ::prost::alloc::string::String,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "10")]
    pub order_bys: ::prost::alloc::vec::Vec<AccessOrderBy>,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in \[AccessQuota\](#AccessQuota). For account-level
    /// requests, this field must be false.
    #[prost(bool, tag = "11")]
    pub return_entity_quota: bool,
    /// Optional. Determines whether to include users who have never made an API
    /// call in the response. If true, all users with access to the specified
    /// property or account are included in the response, regardless of whether
    /// they have made an API call or not. If false, only the users who have made
    /// an API call will be included.
    #[prost(bool, tag = "12")]
    pub include_all_users: bool,
    /// Optional. Decides whether to return the users within user groups. This
    /// field works only when include_all_users is set to true. If true, it will
    /// return all users with access to the specified property or account.
    /// If false, only the users with direct access will be returned.
    #[prost(bool, tag = "13")]
    pub expand_groups: bool,
}
/// The customized Data Access Record Report response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAccessReportResponse {
    /// The header for a column in the report that corresponds to a specific
    /// dimension. The number of DimensionHeaders and ordering of DimensionHeaders
    /// matches the dimensions present in rows.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::prost::alloc::vec::Vec<AccessDimensionHeader>,
    /// The header for a column in the report that corresponds to a specific
    /// metric. The number of MetricHeaders and ordering of MetricHeaders matches
    /// the metrics present in rows.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::prost::alloc::vec::Vec<AccessMetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<AccessRow>,
    /// The total number of rows in the query result. `rowCount` is independent of
    /// the number of rows returned in the response, the `limit` request
    /// parameter, and the `offset` request parameter. For example if a query
    /// returns 175 rows and includes `limit` of 50 in the API request, the
    /// response will contain `rowCount` of 175 but only 50 rows.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int32, tag = "4")]
    pub row_count: i32,
    /// The quota state for this Analytics property including this request. This
    /// field doesn't work with account-level requests.
    #[prost(message, optional, tag = "5")]
    pub quota: ::core::option::Option<AccessQuota>,
}
/// Request message for GetAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    /// Required. The name of the account to lookup.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsRequest {
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccounts` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccounts` must
    /// match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Accounts in the
    /// results. Accounts can be inspected to determine whether they are deleted or
    /// not.
    #[prost(bool, tag = "3")]
    pub show_deleted: bool,
}
/// Request message for ListAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccountRequest {
    /// Required. The name of the Account to soft-delete.
    /// Format: accounts/{account}
    /// Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAccount RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccountRequest {
    /// Required. The account to update.
    /// The account's `name` field is used to identify the account.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (for example, "field_to_update"). Omitted fields will not be updated.
    /// To replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketRequest {
    /// The account to create.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<Account>,
    /// Redirect URI where the user will be sent after accepting Terms of Service.
    /// Must be configured in Cloud Console as a Redirect URI.
    #[prost(string, tag = "2")]
    pub redirect_uri: ::prost::alloc::string::String,
}
/// Response message for ProvisionAccountTicket RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisionAccountTicketResponse {
    /// The param to be passed in the ToS link.
    #[prost(string, tag = "1")]
    pub account_ticket_id: ::prost::alloc::string::String,
}
/// Request message for GetProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyRequest {
    /// Required. The name of the property to lookup.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListProperties RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesRequest {
    /// Required. An expression for filtering the results of the request.
    /// Fields eligible for filtering are:
    /// `parent:`(The resource name of the parent account/property) or
    /// `ancestor:`(The resource name of the parent account) or
    /// `firebase_project:`(The id or number of the linked firebase project).
    /// Some examples of filters:
    ///
    /// ```
    /// | Filter                      | Description                               |
    /// |-----------------------------|-------------------------------------------|
    /// | parent:accounts/123         | The account with account id: 123.       |
    /// | parent:properties/123       | The property with property id: 123.       |
    /// | ancestor:accounts/123       | The account with account id: 123.         |
    /// | firebase_project:project-id | The firebase project with id: project-id. |
    /// | firebase_project:123        | The firebase project with number: 123.    |
    /// ```
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListProperties` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListProperties` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether to include soft-deleted (ie: "trashed") Properties in the
    /// results. Properties can be inspected to determine whether they are deleted
    /// or not.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// Response message for ListProperties RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPropertiesResponse {
    /// Results that matched the filter criteria and were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for UpdateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePropertyRequest {
    /// Required. The property to update.
    /// The property's `name` field is used to identify the property to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<Property>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePropertyRequest {
    /// Required. The property to create.
    /// Note: the supplied property must specify its parent.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<Property>,
}
/// Request message for DeleteProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePropertyRequest {
    /// Required. The name of the Property to soft-delete.
    /// Format: properties/{property_id}
    /// Example: "properties/1000"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Firebase link to create.
    #[prost(message, optional, tag = "2")]
    pub firebase_link: ::core::option::Option<FirebaseLink>,
}
/// Request message for DeleteFirebaseLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFirebaseLinkRequest {
    /// Required. Format: properties/{property_id}/firebaseLinks/{firebase_link_id}
    /// Example: properties/1234/firebaseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListFirebaseLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListFirebaseLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListFirebaseLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFirebaseLinksResponse {
    /// List of FirebaseLinks. This will have at most one value.
    #[prost(message, repeated, tag = "1")]
    pub firebase_links: ::prost::alloc::vec::Vec<FirebaseLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    /// Currently, Google Analytics supports only one FirebaseLink per property,
    /// so this will never be populated.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetGlobalSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGlobalSiteTagRequest {
    /// Required. The name of the site tag to lookup.
    /// Note that site tags are singletons and do not have unique IDs.
    /// Format: properties/{property_id}/dataStreams/{stream_id}/globalSiteTag
    /// Example: "properties/123/dataStreams/456/globalSiteTag"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The GoogleAdsLink to create.
    #[prost(message, optional, tag = "2")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
}
/// Request message for UpdateGoogleAdsLink RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleAdsLinkRequest {
    /// The GoogleAdsLink to update
    #[prost(message, optional, tag = "1")]
    pub google_ads_link: ::core::option::Option<GoogleAdsLink>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteGoogleAdsLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGoogleAdsLinkRequest {
    /// Required. Example format: properties/1234/googleAdsLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListGoogleAdsLinks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListGoogleAdsLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListGoogleAdsLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGoogleAdsLinksResponse {
    /// List of GoogleAdsLinks.
    #[prost(message, repeated, tag = "1")]
    pub google_ads_links: ::prost::alloc::vec::Vec<GoogleAdsLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDataSharingSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataSharingSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format: accounts/{account}/dataSharingSettings
    /// Example: "accounts/1000/dataSharingSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAccountSummaries RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesRequest {
    /// The maximum number of AccountSummary resources to return. The service may
    /// return fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccountSummaries` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccountSummaries`
    /// must match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAccountSummaries RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountSummariesResponse {
    /// Account summaries of all accounts the caller has access to.
    #[prost(message, repeated, tag = "1")]
    pub account_summaries: ::prost::alloc::vec::Vec<AccountSummary>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for AcknowledgeUserDataCollection RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeUserDataCollectionRequest {
    /// Required. The property for which to acknowledge user data collection.
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Required. An acknowledgement that the caller of this method understands the
    /// terms of user data collection.
    ///
    /// This field must contain the exact value:
    /// "I acknowledge that I have the necessary privacy disclosures and rights
    /// from my end users for the collection and processing of their data,
    /// including the association of such data with the visitation information
    /// Google Analytics collects from my site and/or app property."
    #[prost(string, tag = "2")]
    pub acknowledgement: ::prost::alloc::string::String,
}
/// Response message for AcknowledgeUserDataCollection RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeUserDataCollectionResponse {}
/// Request message for SearchChangeHistoryEvents RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChangeHistoryEventsRequest {
    /// Required. The account resource for which to return change history
    /// resources. Format: accounts/{account} Example: "accounts/100"
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// Optional. Resource name for a child property. If set, only return changes
    /// made to this property or its child resources.
    /// Format: properties/{propertyId}
    /// Example: "properties/100"
    #[prost(string, tag = "2")]
    pub property: ::prost::alloc::string::String,
    /// Optional. If set, only return changes if they are for a resource that
    /// matches at least one of these types.
    #[prost(
        enumeration = "ChangeHistoryResourceType",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub resource_type: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If set, only return changes that match one or more of these types
    /// of actions.
    #[prost(enumeration = "ActionType", repeated, packed = "false", tag = "4")]
    pub action: ::prost::alloc::vec::Vec<i32>,
    /// Optional. If set, only return changes if they are made by a user in this
    /// list.
    #[prost(string, repeated, tag = "5")]
    pub actor_email: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If set, only return changes made after this time (inclusive).
    #[prost(message, optional, tag = "6")]
    pub earliest_change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. If set, only return changes made before this time (inclusive).
    #[prost(message, optional, tag = "7")]
    pub latest_change_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The maximum number of ChangeHistoryEvent items to return.
    /// The service may return fewer than this value, even if there are additional
    /// pages. If unspecified, at most 50 items will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "8")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `SearchChangeHistoryEvents` call. Provide this to retrieve the subsequent
    /// page. When paginating, all other parameters provided to
    /// `SearchChangeHistoryEvents` must match the call that provided the page
    /// token.
    #[prost(string, tag = "9")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for SearchAccounts RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchChangeHistoryEventsResponse {
    /// Results that were accessible to the caller.
    #[prost(message, repeated, tag = "1")]
    pub change_history_events: ::prost::alloc::vec::Vec<ChangeHistoryEvent>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetMeasurementProtocolSecret RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeasurementProtocolSecretRequest {
    /// Required. The name of the measurement protocol secret to lookup.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMeasurementProtocolSecretRequest {
    /// Required. The parent resource where this secret will be created.
    /// Format: properties/{property}/dataStreams/{dataStream}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The measurement protocol secret to create.
    #[prost(message, optional, tag = "2")]
    pub measurement_protocol_secret: ::core::option::Option<MeasurementProtocolSecret>,
}
/// Request message for DeleteMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMeasurementProtocolSecretRequest {
    /// Required. The name of the MeasurementProtocolSecret to delete.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMeasurementProtocolSecretRequest {
    /// Required. The measurement protocol secret to update.
    #[prost(message, optional, tag = "1")]
    pub measurement_protocol_secret: ::core::option::Option<MeasurementProtocolSecret>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeasurementProtocolSecretsRequest {
    /// Required. The resource name of the parent stream.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 10 resources will be returned.
    /// The maximum value is 10. Higher values will be coerced to the maximum.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMeasurementProtocolSecrets`
    /// call. Provide this to retrieve the subsequent page. When paginating, all
    /// other parameters provided to `ListMeasurementProtocolSecrets` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListMeasurementProtocolSecret RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMeasurementProtocolSecretsResponse {
    /// A list of secrets for the parent stream specified in the request.
    #[prost(message, repeated, tag = "1")]
    pub measurement_protocol_secrets: ::prost::alloc::vec::Vec<MeasurementProtocolSecret>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetSKAdNetworkConversionValueSchema RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSkAdNetworkConversionValueSchemaRequest {
    /// Required. The resource name of SKAdNetwork conversion value schema to look
    /// up. Format:
    /// properties/{property}/dataStreams/{dataStream}/sKAdNetworkConversionValueSchema/{skadnetwork_conversion_value_schema}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateSKAdNetworkConversionValueSchema RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSkAdNetworkConversionValueSchemaRequest {
    /// Required. The parent resource where this schema will be created.
    /// Format: properties/{property}/dataStreams/{dataStream}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. SKAdNetwork conversion value schema to create.
    #[prost(message, optional, tag = "2")]
    pub skadnetwork_conversion_value_schema:
        ::core::option::Option<SkAdNetworkConversionValueSchema>,
}
/// Request message for DeleteSKAdNetworkConversionValueSchema RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSkAdNetworkConversionValueSchemaRequest {
    /// Required. The name of the SKAdNetworkConversionValueSchema to delete.
    /// Format:
    /// properties/{property}/dataStreams/{dataStream}/sKAdNetworkConversionValueSchema/{skadnetwork_conversion_value_schema}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateSKAdNetworkConversionValueSchema RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSkAdNetworkConversionValueSchemaRequest {
    /// Required. SKAdNetwork conversion value schema to update.
    #[prost(message, optional, tag = "1")]
    pub skadnetwork_conversion_value_schema:
        ::core::option::Option<SkAdNetworkConversionValueSchema>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListSKAdNetworkConversionValueSchemas RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkAdNetworkConversionValueSchemasRequest {
    /// Required. The DataStream resource to list schemas for.
    /// Format:
    /// properties/{property_id}/dataStreams/{dataStream}
    /// Example: properties/1234/dataStreams/5678
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// `ListSKAdNetworkConversionValueSchemas` call. Provide this to retrieve the
    /// subsequent page. When paginating, all other parameters provided to
    /// `ListSKAdNetworkConversionValueSchema` must match the call that provided
    /// the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListSKAdNetworkConversionValueSchemas RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkAdNetworkConversionValueSchemasResponse {
    /// List of SKAdNetworkConversionValueSchemas. This will have at most one
    /// value.
    #[prost(message, repeated, tag = "1")]
    pub skadnetwork_conversion_value_schemas:
        ::prost::alloc::vec::Vec<SkAdNetworkConversionValueSchema>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    /// Currently, Google Analytics supports only one
    /// SKAdNetworkConversionValueSchema per dataStream, so this will never be
    /// populated.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetGoogleSignalsSettings RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleSignalsSettingsRequest {
    /// Required. The name of the google signals settings to retrieve.
    /// Format: properties/{property}/googleSignalsSettings
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateGoogleSignalsSettings RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGoogleSignalsSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub google_signals_settings: ::core::option::Option<GoogleSignalsSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConversionEventRequest {
    /// Required. The conversion event to create.
    #[prost(message, optional, tag = "1")]
    pub conversion_event: ::core::option::Option<ConversionEvent>,
    /// Required. The resource name of the parent property where this conversion
    /// event will be created. Format: properties/123
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for UpdateConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversionEventRequest {
    /// Required. The conversion event to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub conversion_event: ::core::option::Option<ConversionEvent>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversionEventRequest {
    /// Required. The resource name of the conversion event to retrieve.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    /// Example: "properties/123/conversionEvents/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteConversionEvent RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConversionEventRequest {
    /// Required. The resource name of the conversion event to delete.
    /// Format: properties/{property}/conversionEvents/{conversion_event}
    /// Example: "properties/123/conversionEvents/456"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListConversionEvents RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionEventsRequest {
    /// Required. The resource name of the parent property.
    /// Example: 'properties/123'
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConversionEvents` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListConversionEvents`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListConversionEvents RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConversionEventsResponse {
    /// The requested conversion events
    #[prost(message, repeated, tag = "1")]
    pub conversion_events: ::prost::alloc::vec::Vec<ConversionEvent>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDisplayVideo360AdvertiserLinkRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLink to get.
    /// Example format: properties/1234/displayVideo360AdvertiserLink/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDisplayVideo360AdvertiserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDisplayVideo360AdvertiserLinks`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListDisplayVideo360AdvertiserLinks` must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDisplayVideo360AdvertiserLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinksResponse {
    /// List of DisplayVideo360AdvertiserLinks.
    #[prost(message, repeated, tag = "1")]
    pub display_video_360_advertiser_links: ::prost::alloc::vec::Vec<DisplayVideo360AdvertiserLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisplayVideo360AdvertiserLinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DisplayVideo360AdvertiserLink to create.
    #[prost(message, optional, tag = "2")]
    pub display_video_360_advertiser_link: ::core::option::Option<DisplayVideo360AdvertiserLink>,
}
/// Request message for DeleteDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDisplayVideo360AdvertiserLinkRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLink to delete.
    /// Example format: properties/1234/displayVideo360AdvertiserLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDisplayVideo360AdvertiserLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDisplayVideo360AdvertiserLinkRequest {
    /// The DisplayVideo360AdvertiserLink to update
    #[prost(message, optional, tag = "1")]
    pub display_video_360_advertiser_link: ::core::option::Option<DisplayVideo360AdvertiserLink>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to get.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDisplayVideo360AdvertiserLinkProposals RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinkProposalsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous
    /// `ListDisplayVideo360AdvertiserLinkProposals` call. Provide this to retrieve
    /// the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListDisplayVideo360AdvertiserLinkProposals` must match the call that
    /// provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDisplayVideo360AdvertiserLinkProposals RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDisplayVideo360AdvertiserLinkProposalsResponse {
    /// List of DisplayVideo360AdvertiserLinkProposals.
    #[prost(message, repeated, tag = "1")]
    pub display_video_360_advertiser_link_proposals:
        ::prost::alloc::vec::Vec<DisplayVideo360AdvertiserLinkProposal>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DisplayVideo360AdvertiserLinkProposal to create.
    #[prost(message, optional, tag = "2")]
    pub display_video_360_advertiser_link_proposal:
        ::core::option::Option<DisplayVideo360AdvertiserLinkProposal>,
}
/// Request message for DeleteDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to delete.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ApproveDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to approve.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for ApproveDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveDisplayVideo360AdvertiserLinkProposalResponse {
    /// The DisplayVideo360AdvertiserLink created as a result of approving the
    /// proposal.
    #[prost(message, optional, tag = "1")]
    pub display_video_360_advertiser_link: ::core::option::Option<DisplayVideo360AdvertiserLink>,
}
/// Request message for CancelDisplayVideo360AdvertiserLinkProposal RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDisplayVideo360AdvertiserLinkProposalRequest {
    /// Required. The name of the DisplayVideo360AdvertiserLinkProposal to cancel.
    /// Example format: properties/1234/displayVideo360AdvertiserLinkProposals/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetSearchAds360Link RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSearchAds360LinkRequest {
    /// Required. The name of the SearchAds360Link to get.
    /// Example format: properties/1234/SearchAds360Link/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListSearchAds360Links RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSearchAds360LinksRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListSearchAds360Links`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListSearchAds360Links` must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListSearchAds360Links RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSearchAds360LinksResponse {
    /// List of SearchAds360Links.
    #[prost(message, repeated, tag = "1")]
    pub search_ads_360_links: ::prost::alloc::vec::Vec<SearchAds360Link>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateSearchAds360Link RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSearchAds360LinkRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SearchAds360Link to create.
    #[prost(message, optional, tag = "2")]
    pub search_ads_360_link: ::core::option::Option<SearchAds360Link>,
}
/// Request message for DeleteSearchAds360Link RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSearchAds360LinkRequest {
    /// Required. The name of the SearchAds360Link to delete.
    /// Example format: properties/1234/SearchAds360Links/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateSearchAds360Link RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSearchAds360LinkRequest {
    /// The SearchAds360Link to update
    #[prost(message, optional, tag = "1")]
    pub search_ads_360_link: ::core::option::Option<SearchAds360Link>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomDimensionRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomDimension to create.
    #[prost(message, optional, tag = "2")]
    pub custom_dimension: ::core::option::Option<CustomDimension>,
}
/// Request message for UpdateCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomDimensionRequest {
    /// The CustomDimension to update
    #[prost(message, optional, tag = "1")]
    pub custom_dimension: ::core::option::Option<CustomDimension>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCustomDimensions RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomDimensionsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomDimensions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomDimensions`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCustomDimensions RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomDimensionsResponse {
    /// List of CustomDimensions.
    #[prost(message, repeated, tag = "1")]
    pub custom_dimensions: ::prost::alloc::vec::Vec<CustomDimension>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ArchiveCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveCustomDimensionRequest {
    /// Required. The name of the CustomDimension to archive.
    /// Example format: properties/1234/customDimensions/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetCustomDimension RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomDimensionRequest {
    /// Required. The name of the CustomDimension to get.
    /// Example format: properties/1234/customDimensions/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomMetricRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomMetric to create.
    #[prost(message, optional, tag = "2")]
    pub custom_metric: ::core::option::Option<CustomMetric>,
}
/// Request message for UpdateCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCustomMetricRequest {
    /// The CustomMetric to update
    #[prost(message, optional, tag = "1")]
    pub custom_metric: ::core::option::Option<CustomMetric>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListCustomMetrics RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomMetricsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListCustomMetrics` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomMetrics` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCustomMetrics RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomMetricsResponse {
    /// List of CustomMetrics.
    #[prost(message, repeated, tag = "1")]
    pub custom_metrics: ::prost::alloc::vec::Vec<CustomMetric>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for ArchiveCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveCustomMetricRequest {
    /// Required. The name of the CustomMetric to archive.
    /// Example format: properties/1234/customMetrics/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetCustomMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomMetricRequest {
    /// Required. The name of the CustomMetric to get.
    /// Example format: properties/1234/customMetrics/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateCalculatedMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCalculatedMetricRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the calculated metric which will become the
    /// final component of the calculated metric's resource name.
    ///
    /// This value should be 1-80 characters and valid characters are
    /// /\[a-zA-Z0-9_\]/, no spaces allowed. calculated_metric_id must be unique
    /// between all calculated metrics under a property. The calculated_metric_id
    /// is used when referencing this calculated metric from external APIs, for
    /// example, "calcMetric:{calculated_metric_id}".
    #[prost(string, tag = "2")]
    pub calculated_metric_id: ::prost::alloc::string::String,
    /// Required. The CalculatedMetric to create.
    #[prost(message, optional, tag = "3")]
    pub calculated_metric: ::core::option::Option<CalculatedMetric>,
}
/// Request message for UpdateCalculatedMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCalculatedMetricRequest {
    /// Required. The CalculatedMetric to update
    #[prost(message, optional, tag = "1")]
    pub calculated_metric: ::core::option::Option<CalculatedMetric>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteCalculatedMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCalculatedMetricRequest {
    /// Required. The name of the CalculatedMetric to delete.
    /// Format: properties/{property_id}/calculatedMetrics/{calculated_metric_id}
    /// Example: properties/1234/calculatedMetrics/Metric01
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListCalculatedMetrics RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCalculatedMetricsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCalculatedMetrics`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCalculatedMetrics`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListCalculatedMetrics RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCalculatedMetricsResponse {
    /// List of CalculatedMetrics.
    #[prost(message, repeated, tag = "1")]
    pub calculated_metrics: ::prost::alloc::vec::Vec<CalculatedMetric>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetCalculatedMetric RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCalculatedMetricRequest {
    /// Required. The name of the CalculatedMetric to get.
    /// Format: properties/{property_id}/calculatedMetrics/{calculated_metric_id}
    /// Example: properties/1234/calculatedMetrics/Metric01
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetDataRetentionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRetentionSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property}/dataRetentionSettings
    /// Example: "properties/1000/dataRetentionSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDataRetentionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRetentionSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub data_retention_settings: ::core::option::Option<DataRetentionSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataStreamRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DataStream to create.
    #[prost(message, optional, tag = "2")]
    pub data_stream: ::core::option::Option<DataStream>,
}
/// Request message for DeleteDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataStreamRequest {
    /// Required. The name of the DataStream to delete.
    /// Example format: properties/1234/dataStreams/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataStreamRequest {
    /// The DataStream to update
    #[prost(message, optional, tag = "1")]
    pub data_stream: ::core::option::Option<DataStream>,
    /// Required. The list of fields to be updated. Omitted fields will not be
    /// updated. To replace the entire entity, use one path with the string "*" to
    /// match all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ListDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStreamsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDataStreams` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDataStreams` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListDataStreams RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStreamsResponse {
    /// List of DataStreams.
    #[prost(message, repeated, tag = "1")]
    pub data_streams: ::prost::alloc::vec::Vec<DataStream>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetDataStream RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataStreamRequest {
    /// Required. The name of the DataStream to get.
    /// Example format: properties/1234/dataStreams/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetAudience RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAudienceRequest {
    /// Required. The name of the Audience to get.
    /// Example format: properties/1234/audiences/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAudiences RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAudiencesRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAudiences` call. Provide this
    /// to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAudiences` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAudiences RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAudiencesResponse {
    /// List of Audiences.
    #[prost(message, repeated, tag = "1")]
    pub audiences: ::prost::alloc::vec::Vec<Audience>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateAudience RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAudienceRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The audience to create.
    #[prost(message, optional, tag = "2")]
    pub audience: ::core::option::Option<Audience>,
}
/// Request message for UpdateAudience RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAudienceRequest {
    /// Required. The audience to update.
    /// The audience's `name` field is used to identify the audience to be updated.
    #[prost(message, optional, tag = "1")]
    pub audience: ::core::option::Option<Audience>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for ArchiveAudience RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveAudienceRequest {
    /// Required. Example format: properties/1234/audiences/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetAttributionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAttributionSettingsRequest {
    /// Required. The name of the attribution settings to retrieve.
    /// Format: properties/{property}/attributionSettings
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateAttributionSettings RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAttributionSettingsRequest {
    /// Required. The attribution settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub attribution_settings: ::core::option::Option<AttributionSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetAccessBinding RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccessBindingRequest {
    /// Required. The name of the access binding to retrieve.
    /// Formats:
    /// - accounts/{account}/accessBindings/{accessBinding}
    /// - properties/{property}/accessBindings/{accessBinding}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchGetAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAccessBindingsRequest {
    /// Required. The account or property that owns the access bindings. The parent
    /// of all provided values for the 'names' field must match this field.
    /// Formats:
    /// - accounts/{account}
    /// - properties/{property}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names of the access bindings to retrieve.
    /// A maximum of 1000 access bindings can be retrieved in a batch.
    /// Formats:
    /// - accounts/{account}/accessBindings/{accessBinding}
    /// - properties/{property}/accessBindings/{accessBinding}
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for BatchGetAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAccessBindingsResponse {
    /// The requested access bindings.
    #[prost(message, repeated, tag = "1")]
    pub access_bindings: ::prost::alloc::vec::Vec<AccessBinding>,
}
/// Request message for ListAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessBindingsRequest {
    /// Required. Formats:
    /// - accounts/{account}
    /// - properties/{property}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of access bindings to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 200 access bindings will be returned.
    /// The maximum value is 500; values above 500 will be coerced to 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListAccessBindings` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListAccessBindings` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessBindingsResponse {
    /// List of AccessBindings. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub access_bindings: ::prost::alloc::vec::Vec<AccessBinding>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateAccessBinding RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccessBindingRequest {
    /// Required. Formats:
    /// - accounts/{account}
    /// - properties/{property}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The access binding to create.
    #[prost(message, optional, tag = "2")]
    pub access_binding: ::core::option::Option<AccessBinding>,
}
/// Request message for BatchCreateAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateAccessBindingsRequest {
    /// Required. The account or property that owns the access bindings. The parent
    /// field in the CreateAccessBindingRequest messages must either be empty or
    /// match this field. Formats:
    /// - accounts/{account}
    /// - properties/{property}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the access bindings to create.
    /// A maximum of 1000 access bindings can be created in a batch.
    #[prost(message, repeated, tag = "3")]
    pub requests: ::prost::alloc::vec::Vec<CreateAccessBindingRequest>,
}
/// Response message for BatchCreateAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateAccessBindingsResponse {
    /// The access bindings created.
    #[prost(message, repeated, tag = "1")]
    pub access_bindings: ::prost::alloc::vec::Vec<AccessBinding>,
}
/// Request message for UpdateAccessBinding RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessBindingRequest {
    /// Required. The access binding to update.
    #[prost(message, optional, tag = "1")]
    pub access_binding: ::core::option::Option<AccessBinding>,
}
/// Request message for BatchUpdateAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateAccessBindingsRequest {
    /// Required. The account or property that owns the access bindings. The parent
    /// of all provided AccessBinding in UpdateAccessBindingRequest messages must
    /// match this field.
    /// Formats:
    /// - accounts/{account}
    /// - properties/{property}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the access bindings to update.
    /// A maximum of 1000 access bindings can be updated in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateAccessBindingRequest>,
}
/// Response message for BatchUpdateAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateAccessBindingsResponse {
    /// The access bindings updated.
    #[prost(message, repeated, tag = "1")]
    pub access_bindings: ::prost::alloc::vec::Vec<AccessBinding>,
}
/// Request message for DeleteAccessBinding RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccessBindingRequest {
    /// Required. Formats:
    /// - accounts/{account}/accessBindings/{accessBinding}
    /// - properties/{property}/accessBindings/{accessBinding}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BatchDeleteAccessBindings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteAccessBindingsRequest {
    /// Required. The account or property that owns the access bindings. The parent
    /// of all provided values for the 'names' field in DeleteAccessBindingRequest
    /// messages must match this field. Formats:
    /// - accounts/{account}
    /// - properties/{property}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The requests specifying the access bindings to delete.
    /// A maximum of 1000 access bindings can be deleted in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<DeleteAccessBindingRequest>,
}
/// Request message for CreateExpandedDataSet RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExpandedDataSetRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ExpandedDataSet to create.
    #[prost(message, optional, tag = "2")]
    pub expanded_data_set: ::core::option::Option<ExpandedDataSet>,
}
/// Request message for UpdateExpandedDataSet RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateExpandedDataSetRequest {
    /// Required. The ExpandedDataSet to update.
    /// The resource's `name` field is used to identify the ExpandedDataSet to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub expanded_data_set: ::core::option::Option<ExpandedDataSet>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteExpandedDataSet RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExpandedDataSetRequest {
    /// Required. Example format: properties/1234/expandedDataSets/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetExpandedDataSet RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExpandedDataSetRequest {
    /// Required. The name of the ExpandedDataSet to get.
    /// Example format: properties/1234/expandedDataSets/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListExpandedDataSets RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExpandedDataSetsRequest {
    /// Required. Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListExpandedDataSets` call. Provide
    /// this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListExpandedDataSet`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListExpandedDataSets RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExpandedDataSetsResponse {
    /// List of ExpandedDataSet. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub expanded_data_sets: ::prost::alloc::vec::Vec<ExpandedDataSet>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateChannelGroup RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelGroupRequest {
    /// Required. The property for which to create a ChannelGroup.
    /// Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ChannelGroup to create.
    #[prost(message, optional, tag = "2")]
    pub channel_group: ::core::option::Option<ChannelGroup>,
}
/// Request message for UpdateChannelGroup RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelGroupRequest {
    /// Required. The ChannelGroup to update.
    /// The resource's `name` field is used to identify the ChannelGroup to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub channel_group: ::core::option::Option<ChannelGroup>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteChannelGroup RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelGroupRequest {
    /// Required. The ChannelGroup to delete.
    /// Example format: properties/1234/channelGroups/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetChannelGroup RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelGroupRequest {
    /// Required. The ChannelGroup to get.
    /// Example format: properties/1234/channelGroups/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListChannelGroups RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelGroupsRequest {
    /// Required. The property for which to list ChannelGroups.
    /// Example format: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListChannelGroups` call. Provide
    /// this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListChannelGroups`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListChannelGroups RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelGroupsResponse {
    /// List of ChannelGroup. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub channel_groups: ::prost::alloc::vec::Vec<ChannelGroup>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for setting the opt out status for the automated GA4 setup process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAutomatedGa4ConfigurationOptOutRequest {
    /// Required. The UA property to set the opt out status. Note this request uses
    /// the internal property ID, not the tracking ID of the form UA-XXXXXX-YY.
    /// Format: properties/{internalWebPropertyId}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// The status to set.
    #[prost(bool, tag = "2")]
    pub opt_out: bool,
}
/// Response message for setting the opt out status for the automated GA4 setup
/// process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAutomatedGa4ConfigurationOptOutResponse {}
/// Request for fetching the opt out status for the automated GA4 setup process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchAutomatedGa4ConfigurationOptOutRequest {
    /// Required. The UA property to get the opt out status. Note this request uses
    /// the internal property ID, not the tracking ID of the form UA-XXXXXX-YY.
    /// Format: properties/{internalWebPropertyId}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
}
/// Response message for fetching the opt out status for the automated GA4 setup
/// process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchAutomatedGa4ConfigurationOptOutResponse {
    /// The opt out status for the UA property.
    #[prost(bool, tag = "1")]
    pub opt_out: bool,
}
/// Request message for GetBigQueryLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBigQueryLinkRequest {
    /// Required. The name of the BigQuery link to lookup.
    /// Format: properties/{property_id}/bigQueryLinks/{bigquery_link_id}
    /// Example: properties/123/bigQueryLinks/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListBigQueryLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBigQueryLinksRequest {
    /// Required. The name of the property to list BigQuery links under.
    /// Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListBigQueryLinks` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListBigQueryLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListBigQueryLinks RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBigQueryLinksResponse {
    /// List of BigQueryLinks.
    #[prost(message, repeated, tag = "1")]
    pub bigquery_links: ::prost::alloc::vec::Vec<BigQueryLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnhancedMeasurementSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property}/dataStreams/{data_stream}/enhancedMeasurementSettings
    /// Example: "properties/1000/dataStreams/2000/enhancedMeasurementSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateEnhancedMeasurementSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnhancedMeasurementSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub enhanced_measurement_settings: ::core::option::Option<EnhancedMeasurementSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetDataRedactionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataRedactionSettingsRequest {
    /// Required. The name of the settings to lookup.
    /// Format:
    /// properties/{property}/dataStreams/{data_stream}/dataRedactionSettings
    /// Example: "properties/1000/dataStreams/2000/dataRedactionSettings"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateDataRedactionSettings RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataRedactionSettingsRequest {
    /// Required. The settings to update.
    /// The `name` field is used to identify the settings to be updated.
    #[prost(message, optional, tag = "1")]
    pub data_redaction_settings: ::core::option::Option<DataRedactionSettings>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for CreateConnectedSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectedSiteTagRequest {
    /// The Universal Analytics property to create connected site tags for.
    /// This API does not support GA4 properties.
    /// Format: properties/{universalAnalyticsPropertyId}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Required. The tag to add to the Universal Analytics property
    #[prost(message, optional, tag = "2")]
    pub connected_site_tag: ::core::option::Option<ConnectedSiteTag>,
}
/// Response message for CreateConnectedSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectedSiteTagResponse {}
/// Request message for DeleteConnectedSiteTag RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectedSiteTagRequest {
    /// The Universal Analytics property to delete connected site tags for.
    /// This API does not support GA4 properties.
    /// Format: properties/{universalAnalyticsPropertyId}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Tag ID to forward events to. Also known as the Measurement ID, or the
    /// "G-ID"  (For example: G-12345).
    #[prost(string, tag = "2")]
    pub tag_id: ::prost::alloc::string::String,
}
/// Request message for ListConnectedSiteTags RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectedSiteTagsRequest {
    /// The Universal Analytics property to fetch connected site tags for.
    /// This does not work on GA4 properties. A maximum of 20 connected site tags
    /// will be returned.
    /// Example Format: `properties/1234`
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
}
/// Response message for ListConnectedSiteTags RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectedSiteTagsResponse {
    /// The site tags for the Universal Analytics property. A maximum of 20
    /// connected site tags will be returned.
    #[prost(message, repeated, tag = "1")]
    pub connected_site_tags: ::prost::alloc::vec::Vec<ConnectedSiteTag>,
}
/// Request message to be passed to CreateAdSenseLink method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAdSenseLinkRequest {
    /// Required. The property for which to create an AdSense Link.
    /// Format: properties/{propertyId}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The AdSense Link to create
    #[prost(message, optional, tag = "2")]
    pub adsense_link: ::core::option::Option<AdSenseLink>,
}
/// Request message to be passed to GetAdSenseLink method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdSenseLinkRequest {
    /// Required. Unique identifier for the AdSense Link requested.
    /// Format: properties/{propertyId}/adSenseLinks/{linkId}
    /// Example: properties/1234/adSenseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to be passed to DeleteAdSenseLink method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAdSenseLinkRequest {
    /// Required. Unique identifier for the AdSense Link to be deleted.
    /// Format: properties/{propertyId}/adSenseLinks/{linkId}
    /// Example: properties/1234/adSenseLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to be passed to ListAdSenseLinks method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdSenseLinksRequest {
    /// Required. Resource name of the parent property.
    /// Format: properties/{propertyId}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous `ListAdSenseLinks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAdSenseLinks` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListAdSenseLinks method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdSenseLinksResponse {
    /// List of AdSenseLinks.
    #[prost(message, repeated, tag = "1")]
    pub adsense_links: ::prost::alloc::vec::Vec<AdSenseLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for looking up GA4 property connected to a UA property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchConnectedGa4PropertyRequest {
    /// Required. The UA property for which to look up the connected GA4 property.
    /// Note this request uses the
    /// internal property ID, not the tracking ID of the form UA-XXXXXX-YY.
    /// Format: properties/{internal_web_property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
}
/// Response for looking up GA4 property connected to a UA property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchConnectedGa4PropertyResponse {
    /// The GA4 property connected to the UA property. An empty string is returned
    /// when there is no connected GA4 property.
    /// Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
}
/// Request message for CreateEventCreateRule RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventCreateRuleRequest {
    /// Required. Example format: properties/123/dataStreams/456
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The EventCreateRule to create.
    #[prost(message, optional, tag = "2")]
    pub event_create_rule: ::core::option::Option<EventCreateRule>,
}
/// Request message for UpdateEventCreateRule RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEventCreateRuleRequest {
    /// Required. The EventCreateRule to update.
    /// The resource's `name` field is used to identify the EventCreateRule to be
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub event_create_rule: ::core::option::Option<EventCreateRule>,
    /// Required. The list of fields to be updated. Field names must be in snake
    /// case (e.g., "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteEventCreateRule RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventCreateRuleRequest {
    /// Required. Example format:
    /// properties/123/dataStreams/456/eventCreateRules/789
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GetEventCreateRule RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventCreateRuleRequest {
    /// Required. The name of the EventCreateRule to get.
    /// Example format: properties/123/dataStreams/456/eventCreateRules/789
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListEventCreateRules RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventCreateRulesRequest {
    /// Required. Example format: properties/123/dataStreams/456
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of resources to return.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200 (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListEventCreateRules` call. Provide
    /// this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListEventCreateRules`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListEventCreateRules RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventCreateRulesResponse {
    /// List of EventCreateRules. These will be ordered stably, but in an arbitrary
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub event_create_rules: ::prost::alloc::vec::Vec<EventCreateRule>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateRollupProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRollupPropertyRequest {
    /// Required. The roll-up property to create.
    #[prost(message, optional, tag = "1")]
    pub rollup_property: ::core::option::Option<Property>,
    /// Optional. The resource names of properties that will be sources to the
    /// created roll-up property.
    #[prost(string, repeated, tag = "2")]
    pub source_properties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for CreateRollupProperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRollupPropertyResponse {
    /// The created roll-up property.
    #[prost(message, optional, tag = "1")]
    pub rollup_property: ::core::option::Option<Property>,
    /// The created roll-up property source links.
    #[prost(message, repeated, tag = "2")]
    pub rollup_property_source_links: ::prost::alloc::vec::Vec<RollupPropertySourceLink>,
}
/// Request message for GetRollupPropertySourceLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRollupPropertySourceLinkRequest {
    /// Required. The name of the roll-up property source link to lookup.
    /// Format:
    /// properties/{property_id}/rollupPropertySourceLinks/{rollup_property_source_link_id}
    /// Example: properties/123/rollupPropertySourceLinks/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListRollupPropertySourceLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRollupPropertySourceLinksRequest {
    /// Required. The name of the roll-up property to list roll-up property source
    /// links under. Format: properties/{property_id} Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages.
    /// If unspecified, at most 50 resources will be returned.
    /// The maximum value is 200; (higher values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListRollupPropertySourceLinks` call. Provide this to retrieve the
    /// subsequent page. When paginating, all other parameters provided to
    /// `ListRollupPropertySourceLinks` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListRollupPropertySourceLinks RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRollupPropertySourceLinksResponse {
    /// List of RollupPropertySourceLinks.
    #[prost(message, repeated, tag = "1")]
    pub rollup_property_source_links: ::prost::alloc::vec::Vec<RollupPropertySourceLink>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateRollupPropertySourceLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRollupPropertySourceLinkRequest {
    /// Required. Format: properties/{property_id}
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The roll-up property source link to create.
    #[prost(message, optional, tag = "2")]
    pub rollup_property_source_link: ::core::option::Option<RollupPropertySourceLink>,
}
/// Request message for DeleteRollupPropertySourceLink RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRollupPropertySourceLinkRequest {
    /// Required. Format:
    /// properties/{property_id}/rollupPropertySourceLinks/{rollup_property_source_link_id}
    /// Example: properties/1234/rollupPropertySourceLinks/5678
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateSubproperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubpropertyRequest {
    /// Required. The ordinary property for which to create a subproperty.
    /// Format: properties/property_id
    /// Example: properties/123
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The subproperty to create.
    #[prost(message, optional, tag = "2")]
    pub subproperty: ::core::option::Option<Property>,
    /// Optional. The subproperty event filter to create on an ordinary property.
    #[prost(message, optional, tag = "3")]
    pub subproperty_event_filter: ::core::option::Option<SubpropertyEventFilter>,
}
/// Response message for CreateSubproperty RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubpropertyResponse {
    /// The created subproperty.
    #[prost(message, optional, tag = "1")]
    pub subproperty: ::core::option::Option<Property>,
    /// The created subproperty event filter.
    #[prost(message, optional, tag = "2")]
    pub subproperty_event_filter: ::core::option::Option<SubpropertyEventFilter>,
}
/// Request message for CreateSubpropertyEventFilter RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubpropertyEventFilterRequest {
    /// Required. The ordinary property for which to create a subproperty event
    /// filter. Format: properties/property_id Example: properties/123
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The subproperty event filter to create.
    #[prost(message, optional, tag = "2")]
    pub subproperty_event_filter: ::core::option::Option<SubpropertyEventFilter>,
}
/// Request message for GetSubpropertyEventFilter RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubpropertyEventFilterRequest {
    /// Required. Resource name of the subproperty event filter to lookup.
    /// Format:
    /// properties/property_id/subpropertyEventFilters/subproperty_event_filter
    /// Example: properties/123/subpropertyEventFilters/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListSubpropertyEventFilters RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubpropertyEventFiltersRequest {
    /// Required. Resource name of the ordinary property.
    /// Format: properties/property_id
    /// Example: properties/123
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of resources to return. The service may return
    /// fewer than this value, even if there are additional pages. If unspecified,
    /// at most 50 resources will be returned. The maximum value is 200; (higher
    /// values will be coerced to the maximum)
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListSubpropertyEventFilters` call. Provide this to retrieve the subsequent
    /// page. When paginating, all other parameters provided to
    /// `ListSubpropertyEventFilters` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListSubpropertyEventFilter RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubpropertyEventFiltersResponse {
    /// List of subproperty event filters.
    #[prost(message, repeated, tag = "1")]
    pub subproperty_event_filters: ::prost::alloc::vec::Vec<SubpropertyEventFilter>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If
    /// this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for UpdateSubpropertyEventFilter RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubpropertyEventFilterRequest {
    /// Required. The subproperty event filter to update.
    #[prost(message, optional, tag = "1")]
    pub subproperty_event_filter: ::core::option::Option<SubpropertyEventFilter>,
    /// Required. The list of fields to update. Field names must be in snake case
    /// (for example, "field_to_update"). Omitted fields will not be updated. To
    /// replace the entire entity, use one path with the string "*" to match all
    /// fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteSubpropertyEventFilter RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubpropertyEventFilterRequest {
    /// Required. Resource name of the subproperty event filter to delete.
    /// Format:
    /// properties/property_id/subpropertyEventFilters/subproperty_event_filter
    /// Example: properties/123/subpropertyEventFilters/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod analytics_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service Interface for the Analytics Admin API (GA4)."]
    #[derive(Debug, Clone)]
    pub struct AnalyticsAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AnalyticsAdminServiceClient<T>
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
        ) -> AnalyticsAdminServiceClient<InterceptedService<T, F>>
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
            AnalyticsAdminServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lookup for a single Account."]
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all accounts accessible by the caller."]
        #[doc = ""]
        #[doc = " Note that these accounts might not currently have GA4 properties."]
        #[doc = " Soft-deleted (ie: \"trashed\") accounts are excluded by default."]
        #[doc = " Returns an empty list if no relevant accounts are found."]
        pub async fn list_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountsRequest>,
        ) -> Result<tonic::Response<super::ListAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks target Account as soft-deleted (ie: \"trashed\") and returns it."]
        #[doc = ""]
        #[doc = " This API does not have a method to restore soft-deleted accounts."]
        #[doc = " However, they can be restored using the Trash Can UI."]
        #[doc = ""]
        #[doc = " If the accounts are not restored before the expiration time, the account"]
        #[doc = " and all child resources (eg: Properties, GoogleAdsLinks, Streams,"]
        #[doc = " AccessBindings) will be permanently purged."]
        #[doc = " https://support.google.com/analytics/answer/6154772"]
        #[doc = ""]
        #[doc = " Returns an error if the target is not found."]
        pub async fn delete_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an account."]
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccountRequest>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Requests a ticket for creating an account."]
        pub async fn provision_account_ticket(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvisionAccountTicketRequest>,
        ) -> Result<tonic::Response<super::ProvisionAccountTicketResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ProvisionAccountTicket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns summaries of all accounts accessible by the caller."]
        pub async fn list_account_summaries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountSummariesRequest>,
        ) -> Result<tonic::Response<super::ListAccountSummariesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccountSummaries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single \"GA4\" Property."]
        pub async fn get_property(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child Properties under the specified parent Account."]
        #[doc = ""]
        #[doc = " Only \"GA4\" properties will be returned."]
        #[doc = " Properties will be excluded if the caller does not have access."]
        #[doc = " Soft-deleted (ie: \"trashed\") properties are excluded by default."]
        #[doc = " Returns an empty list if no relevant properties are found."]
        pub async fn list_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPropertiesRequest>,
        ) -> Result<tonic::Response<super::ListPropertiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListProperties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an \"GA4\" property with the specified location and attributes."]
        pub async fn create_property(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks target Property as soft-deleted (ie: \"trashed\") and returns it."]
        #[doc = ""]
        #[doc = " This API does not have a method to restore soft-deleted properties."]
        #[doc = " However, they can be restored using the Trash Can UI."]
        #[doc = ""]
        #[doc = " If the properties are not restored before the expiration time, the Property"]
        #[doc = " and all child resources (eg: GoogleAdsLinks, Streams, AccessBindings)"]
        #[doc = " will be permanently purged."]
        #[doc = " https://support.google.com/analytics/answer/6154772"]
        #[doc = ""]
        #[doc = " Returns an error if the target is not found, or is not a GA4 Property."]
        pub async fn delete_property(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a property."]
        pub async fn update_property(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePropertyRequest>,
        ) -> Result<tonic::Response<super::Property>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a FirebaseLink."]
        #[doc = ""]
        #[doc = " Properties can have at most one FirebaseLink."]
        pub async fn create_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFirebaseLinkRequest>,
        ) -> Result<tonic::Response<super::FirebaseLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a FirebaseLink on a property"]
        pub async fn delete_firebase_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFirebaseLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteFirebaseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists FirebaseLinks on a property."]
        #[doc = " Properties can have at most one FirebaseLink."]
        pub async fn list_firebase_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFirebaseLinksRequest>,
        ) -> Result<tonic::Response<super::ListFirebaseLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListFirebaseLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the Site Tag for the specified web stream."]
        #[doc = " Site Tags are immutable singletons."]
        pub async fn get_global_site_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGlobalSiteTagRequest>,
        ) -> Result<tonic::Response<super::GlobalSiteTag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetGlobalSiteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a GoogleAdsLink."]
        pub async fn create_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a GoogleAdsLink on a property"]
        pub async fn update_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<super::GoogleAdsLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a GoogleAdsLink on a property"]
        pub async fn delete_google_ads_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGoogleAdsLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteGoogleAdsLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists GoogleAdsLinks on a property."]
        pub async fn list_google_ads_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGoogleAdsLinksRequest>,
        ) -> Result<tonic::Response<super::ListGoogleAdsLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListGoogleAdsLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get data sharing settings on an account."]
        #[doc = " Data sharing settings are singletons."]
        pub async fn get_data_sharing_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataSharingSettingsRequest>,
        ) -> Result<tonic::Response<super::DataSharingSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataSharingSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single \"GA4\" MeasurementProtocolSecret."]
        pub async fn get_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeasurementProtocolSecretRequest>,
        ) -> Result<tonic::Response<super::MeasurementProtocolSecret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/GetMeasurementProtocolSecret") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns child MeasurementProtocolSecrets under the specified parent"]
        #[doc = " Property."]
        pub async fn list_measurement_protocol_secrets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMeasurementProtocolSecretsRequest>,
        ) -> Result<tonic::Response<super::ListMeasurementProtocolSecretsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/ListMeasurementProtocolSecrets") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a measurement protocol secret."]
        pub async fn create_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMeasurementProtocolSecretRequest>,
        ) -> Result<tonic::Response<super::MeasurementProtocolSecret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateMeasurementProtocolSecret") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes target MeasurementProtocolSecret."]
        pub async fn delete_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMeasurementProtocolSecretRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteMeasurementProtocolSecret") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a measurement protocol secret."]
        pub async fn update_measurement_protocol_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMeasurementProtocolSecretRequest>,
        ) -> Result<tonic::Response<super::MeasurementProtocolSecret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateMeasurementProtocolSecret") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Acknowledges the terms of user data collection for the specified property."]
        #[doc = ""]
        #[doc = " This acknowledgement must be completed (either in the Google Analytics UI"]
        #[doc = " or through this API) before MeasurementProtocolSecret resources may be"]
        #[doc = " created."]
        pub async fn acknowledge_user_data_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeUserDataCollectionRequest>,
        ) -> Result<tonic::Response<super::AcknowledgeUserDataCollectionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/AcknowledgeUserDataCollection") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Looks up a single SKAdNetworkConversionValueSchema."]
        pub async fn get_sk_ad_network_conversion_value_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSkAdNetworkConversionValueSchemaRequest>,
        ) -> Result<tonic::Response<super::SkAdNetworkConversionValueSchema>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/GetSKAdNetworkConversionValueSchema") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a SKAdNetworkConversionValueSchema."]
        pub async fn create_sk_ad_network_conversion_value_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSkAdNetworkConversionValueSchemaRequest>,
        ) -> Result<tonic::Response<super::SkAdNetworkConversionValueSchema>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateSKAdNetworkConversionValueSchema") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes target SKAdNetworkConversionValueSchema."]
        pub async fn delete_sk_ad_network_conversion_value_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSkAdNetworkConversionValueSchemaRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteSKAdNetworkConversionValueSchema") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a SKAdNetworkConversionValueSchema."]
        pub async fn update_sk_ad_network_conversion_value_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSkAdNetworkConversionValueSchemaRequest>,
        ) -> Result<tonic::Response<super::SkAdNetworkConversionValueSchema>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateSKAdNetworkConversionValueSchema") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists SKAdNetworkConversionValueSchema on a stream."]
        #[doc = " Properties can have at most one SKAdNetworkConversionValueSchema."]
        pub async fn list_sk_ad_network_conversion_value_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSkAdNetworkConversionValueSchemasRequest>,
        ) -> Result<
            tonic::Response<super::ListSkAdNetworkConversionValueSchemasResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/ListSKAdNetworkConversionValueSchemas") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches through all changes to an account or its children given the"]
        #[doc = " specified set of filters."]
        pub async fn search_change_history_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchChangeHistoryEventsRequest>,
        ) -> Result<tonic::Response<super::SearchChangeHistoryEventsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/SearchChangeHistoryEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for Google Signals settings for a property."]
        pub async fn get_google_signals_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGoogleSignalsSettingsRequest>,
        ) -> Result<tonic::Response<super::GoogleSignalsSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetGoogleSignalsSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates Google Signals settings for a property."]
        pub async fn update_google_signals_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGoogleSignalsSettingsRequest>,
        ) -> Result<tonic::Response<super::GoogleSignalsSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateGoogleSignalsSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a conversion event with the specified attributes."]
        pub async fn create_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversionEventRequest>,
        ) -> Result<tonic::Response<super::ConversionEvent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a conversion event with the specified attributes."]
        pub async fn update_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversionEventRequest>,
        ) -> Result<tonic::Response<super::ConversionEvent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve a single conversion event."]
        pub async fn get_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversionEventRequest>,
        ) -> Result<tonic::Response<super::ConversionEvent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a conversion event in a property."]
        pub async fn delete_conversion_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConversionEventRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteConversionEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of conversion events in the specified parent property."]
        #[doc = ""]
        #[doc = " Returns an empty list if no conversion events are found."]
        pub async fn list_conversion_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversionEventsRequest>,
        ) -> Result<tonic::Response<super::ListConversionEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListConversionEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Look up a single DisplayVideo360AdvertiserLink"]
        pub async fn get_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDisplayVideo360AdvertiserLinkRequest>,
        ) -> Result<tonic::Response<super::DisplayVideo360AdvertiserLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDisplayVideo360AdvertiserLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all DisplayVideo360AdvertiserLinks on a property."]
        pub async fn list_display_video360_advertiser_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDisplayVideo360AdvertiserLinksRequest>,
        ) -> Result<tonic::Response<super::ListDisplayVideo360AdvertiserLinksResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/ListDisplayVideo360AdvertiserLinks") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a DisplayVideo360AdvertiserLink."]
        #[doc = " This can only be utilized by users who have proper authorization both on"]
        #[doc = " the Google Analytics property and on the Display & Video 360 advertiser."]
        #[doc = " Users who do not have access to the Display & Video 360 advertiser should"]
        #[doc = " instead seek to create a DisplayVideo360LinkProposal."]
        pub async fn create_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDisplayVideo360AdvertiserLinkRequest>,
        ) -> Result<tonic::Response<super::DisplayVideo360AdvertiserLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateDisplayVideo360AdvertiserLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a DisplayVideo360AdvertiserLink on a property."]
        pub async fn delete_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDisplayVideo360AdvertiserLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteDisplayVideo360AdvertiserLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a DisplayVideo360AdvertiserLink on a property."]
        pub async fn update_display_video360_advertiser_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDisplayVideo360AdvertiserLinkRequest>,
        ) -> Result<tonic::Response<super::DisplayVideo360AdvertiserLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateDisplayVideo360AdvertiserLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single DisplayVideo360AdvertiserLinkProposal."]
        pub async fn get_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDisplayVideo360AdvertiserLinkProposalRequest>,
        ) -> Result<tonic::Response<super::DisplayVideo360AdvertiserLinkProposal>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDisplayVideo360AdvertiserLinkProposal") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists DisplayVideo360AdvertiserLinkProposals on a property."]
        pub async fn list_display_video360_advertiser_link_proposals(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDisplayVideo360AdvertiserLinkProposalsRequest>,
        ) -> Result<
            tonic::Response<super::ListDisplayVideo360AdvertiserLinkProposalsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/ListDisplayVideo360AdvertiserLinkProposals") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a DisplayVideo360AdvertiserLinkProposal."]
        pub async fn create_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDisplayVideo360AdvertiserLinkProposalRequest>,
        ) -> Result<tonic::Response<super::DisplayVideo360AdvertiserLinkProposal>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateDisplayVideo360AdvertiserLinkProposal") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a DisplayVideo360AdvertiserLinkProposal on a property."]
        #[doc = " This can only be used on cancelled proposals."]
        pub async fn delete_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDisplayVideo360AdvertiserLinkProposalRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteDisplayVideo360AdvertiserLinkProposal") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Approves a DisplayVideo360AdvertiserLinkProposal."]
        #[doc = " The DisplayVideo360AdvertiserLinkProposal will be deleted and a new"]
        #[doc = " DisplayVideo360AdvertiserLink will be created."]
        pub async fn approve_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveDisplayVideo360AdvertiserLinkProposalRequest>,
        ) -> Result<
            tonic::Response<super::ApproveDisplayVideo360AdvertiserLinkProposalResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/ApproveDisplayVideo360AdvertiserLinkProposal") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a DisplayVideo360AdvertiserLinkProposal."]
        #[doc = " Cancelling can mean either:"]
        #[doc = " - Declining a proposal initiated from Display & Video 360"]
        #[doc = " - Withdrawing a proposal initiated from Google Analytics"]
        #[doc = " After being cancelled, a proposal will eventually be deleted automatically."]
        pub async fn cancel_display_video360_advertiser_link_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelDisplayVideo360AdvertiserLinkProposalRequest>,
        ) -> Result<tonic::Response<super::DisplayVideo360AdvertiserLinkProposal>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CancelDisplayVideo360AdvertiserLinkProposal") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a CustomDimension."]
        pub async fn create_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomDimensionRequest>,
        ) -> Result<tonic::Response<super::CustomDimension>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a CustomDimension on a property."]
        pub async fn update_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomDimensionRequest>,
        ) -> Result<tonic::Response<super::CustomDimension>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists CustomDimensions on a property."]
        pub async fn list_custom_dimensions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomDimensionsRequest>,
        ) -> Result<tonic::Response<super::ListCustomDimensionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListCustomDimensions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Archives a CustomDimension on a property."]
        pub async fn archive_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveCustomDimensionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ArchiveCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single CustomDimension."]
        pub async fn get_custom_dimension(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomDimensionRequest>,
        ) -> Result<tonic::Response<super::CustomDimension>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetCustomDimension",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a CustomMetric."]
        pub async fn create_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomMetricRequest>,
        ) -> Result<tonic::Response<super::CustomMetric>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a CustomMetric on a property."]
        pub async fn update_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomMetricRequest>,
        ) -> Result<tonic::Response<super::CustomMetric>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists CustomMetrics on a property."]
        pub async fn list_custom_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomMetricsRequest>,
        ) -> Result<tonic::Response<super::ListCustomMetricsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListCustomMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Archives a CustomMetric on a property."]
        pub async fn archive_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveCustomMetricRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ArchiveCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single CustomMetric."]
        pub async fn get_custom_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomMetricRequest>,
        ) -> Result<tonic::Response<super::CustomMetric>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetCustomMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the singleton data retention settings for this property."]
        pub async fn get_data_retention_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataRetentionSettingsRequest>,
        ) -> Result<tonic::Response<super::DataRetentionSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataRetentionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the singleton data retention settings for this property."]
        pub async fn update_data_retention_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataRetentionSettingsRequest>,
        ) -> Result<tonic::Response<super::DataRetentionSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateDataRetentionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a DataStream."]
        pub async fn create_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataStreamRequest>,
        ) -> Result<tonic::Response<super::DataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a DataStream on a property."]
        pub async fn delete_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataStreamRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a DataStream on a property."]
        pub async fn update_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataStreamRequest>,
        ) -> Result<tonic::Response<super::DataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists DataStreams on a property."]
        pub async fn list_data_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataStreamsRequest>,
        ) -> Result<tonic::Response<super::ListDataStreamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListDataStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single DataStream."]
        pub async fn get_data_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataStreamRequest>,
        ) -> Result<tonic::Response<super::DataStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single Audience."]
        #[doc = " Audiences created before 2020 may not be supported."]
        #[doc = " Default audiences will not show filter definitions."]
        pub async fn get_audience(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAudienceRequest>,
        ) -> Result<tonic::Response<super::Audience>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAudience",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Audiences on a property."]
        #[doc = " Audiences created before 2020 may not be supported."]
        #[doc = " Default audiences will not show filter definitions."]
        pub async fn list_audiences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAudiencesRequest>,
        ) -> Result<tonic::Response<super::ListAudiencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAudiences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an Audience."]
        pub async fn create_audience(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAudienceRequest>,
        ) -> Result<tonic::Response<super::Audience>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateAudience",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an Audience on a property."]
        pub async fn update_audience(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAudienceRequest>,
        ) -> Result<tonic::Response<super::Audience>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAudience",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Archives an Audience on a property."]
        pub async fn archive_audience(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveAudienceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ArchiveAudience",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Look up a single SearchAds360Link"]
        pub async fn get_search_ads360_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSearchAds360LinkRequest>,
        ) -> Result<tonic::Response<super::SearchAds360Link>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetSearchAds360Link",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all SearchAds360Links on a property."]
        pub async fn list_search_ads360_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSearchAds360LinksRequest>,
        ) -> Result<tonic::Response<super::ListSearchAds360LinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListSearchAds360Links",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a SearchAds360Link."]
        pub async fn create_search_ads360_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSearchAds360LinkRequest>,
        ) -> Result<tonic::Response<super::SearchAds360Link>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateSearchAds360Link",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a SearchAds360Link on a property."]
        pub async fn delete_search_ads360_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSearchAds360LinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteSearchAds360Link",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a SearchAds360Link on a property."]
        pub async fn update_search_ads360_link(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSearchAds360LinkRequest>,
        ) -> Result<tonic::Response<super::SearchAds360Link>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateSearchAds360Link",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a AttributionSettings singleton."]
        pub async fn get_attribution_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAttributionSettingsRequest>,
        ) -> Result<tonic::Response<super::AttributionSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAttributionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates attribution settings on a property."]
        pub async fn update_attribution_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAttributionSettingsRequest>,
        ) -> Result<tonic::Response<super::AttributionSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAttributionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a customized report of data access records. The report provides"]
        #[doc = " records of each time a user reads Google Analytics reporting data. Access"]
        #[doc = " records are retained for up to 2 years."]
        #[doc = ""]
        #[doc = " Data Access Reports can be requested for a property. Reports may be"]
        #[doc = " requested for any property, but dimensions that aren't related to quota can"]
        #[doc = " only be requested on Google Analytics 360 properties. This method is only"]
        #[doc = " available to Administrators."]
        #[doc = ""]
        #[doc = " These data access records include GA4 UI Reporting, GA4 UI Explorations,"]
        #[doc = " GA4 Data API, and other products like Firebase & Admob that can retrieve"]
        #[doc = " data from Google Analytics through a linkage. These records don't include"]
        #[doc = " property configuration changes like adding a stream or changing a"]
        #[doc = " property's time zone. For configuration change history, see"]
        #[doc = " [searchChangeHistoryEvents](https://developers.google.com/analytics/devguides/config/admin/v1/rest/v1alpha/accounts/searchChangeHistoryEvents)."]
        pub async fn run_access_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunAccessReportRequest>,
        ) -> Result<tonic::Response<super::RunAccessReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/RunAccessReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an access binding on an account or property."]
        pub async fn create_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccessBindingRequest>,
        ) -> Result<tonic::Response<super::AccessBinding>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about an access binding."]
        pub async fn get_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccessBindingRequest>,
        ) -> Result<tonic::Response<super::AccessBinding>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an access binding on an account or property."]
        pub async fn update_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccessBindingRequest>,
        ) -> Result<tonic::Response<super::AccessBinding>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an access binding on an account or property."]
        pub async fn delete_access_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccessBindingRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAccessBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all access bindings on an account or property."]
        pub async fn list_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessBindingsRequest>,
        ) -> Result<tonic::Response<super::ListAccessBindingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates information about multiple access bindings to an account or"]
        #[doc = " property."]
        #[doc = ""]
        #[doc = " This method is transactional. If any AccessBinding cannot be created, none"]
        #[doc = " of the AccessBindings will be created."]
        pub async fn batch_create_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateAccessBindingsRequest>,
        ) -> Result<tonic::Response<super::BatchCreateAccessBindingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchCreateAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about multiple access bindings to an account or property."]
        pub async fn batch_get_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetAccessBindingsRequest>,
        ) -> Result<tonic::Response<super::BatchGetAccessBindingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchGetAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates information about multiple access bindings to an account or"]
        #[doc = " property."]
        pub async fn batch_update_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateAccessBindingsRequest>,
        ) -> Result<tonic::Response<super::BatchUpdateAccessBindingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchUpdateAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes information about multiple users' links to an account or property."]
        pub async fn batch_delete_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteAccessBindingsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/BatchDeleteAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single ExpandedDataSet."]
        pub async fn get_expanded_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExpandedDataSetRequest>,
        ) -> Result<tonic::Response<super::ExpandedDataSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetExpandedDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ExpandedDataSets on a property."]
        pub async fn list_expanded_data_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExpandedDataSetsRequest>,
        ) -> Result<tonic::Response<super::ListExpandedDataSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListExpandedDataSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a ExpandedDataSet."]
        pub async fn create_expanded_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExpandedDataSetRequest>,
        ) -> Result<tonic::Response<super::ExpandedDataSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateExpandedDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a ExpandedDataSet on a property."]
        pub async fn update_expanded_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateExpandedDataSetRequest>,
        ) -> Result<tonic::Response<super::ExpandedDataSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateExpandedDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a ExpandedDataSet on a property."]
        pub async fn delete_expanded_data_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExpandedDataSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteExpandedDataSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single ChannelGroup."]
        pub async fn get_channel_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChannelGroupRequest>,
        ) -> Result<tonic::Response<super::ChannelGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetChannelGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ChannelGroups on a property."]
        pub async fn list_channel_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelGroupsRequest>,
        ) -> Result<tonic::Response<super::ListChannelGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListChannelGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a ChannelGroup."]
        pub async fn create_channel_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChannelGroupRequest>,
        ) -> Result<tonic::Response<super::ChannelGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateChannelGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a ChannelGroup."]
        pub async fn update_channel_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChannelGroupRequest>,
        ) -> Result<tonic::Response<super::ChannelGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateChannelGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a ChannelGroup on a property."]
        pub async fn delete_channel_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteChannelGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteChannelGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the opt out status for the automated GA4 setup process for a UA"]
        #[doc = " property."]
        #[doc = " Note: this has no effect on GA4 property."]
        pub async fn set_automated_ga4_configuration_opt_out(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAutomatedGa4ConfigurationOptOutRequest>,
        ) -> Result<tonic::Response<super::SetAutomatedGa4ConfigurationOptOutResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/SetAutomatedGa4ConfigurationOptOut") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches the opt out status for the automated GA4 setup process for a UA"]
        #[doc = " property."]
        #[doc = " Note: this has no effect on GA4 property."]
        pub async fn fetch_automated_ga4_configuration_opt_out(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchAutomatedGa4ConfigurationOptOutRequest>,
        ) -> Result<
            tonic::Response<super::FetchAutomatedGa4ConfigurationOptOutResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/FetchAutomatedGa4ConfigurationOptOut") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single BigQuery Link."]
        pub async fn get_big_query_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBigQueryLinkRequest>,
        ) -> Result<tonic::Response<super::BigQueryLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetBigQueryLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists BigQuery Links on a property."]
        pub async fn list_big_query_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBigQueryLinksRequest>,
        ) -> Result<tonic::Response<super::ListBigQueryLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListBigQueryLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the enhanced measurement settings for this data stream."]
        #[doc = " Note that the stream must enable enhanced measurement for these settings to"]
        #[doc = " take effect."]
        pub async fn get_enhanced_measurement_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnhancedMeasurementSettingsRequest>,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/GetEnhancedMeasurementSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the enhanced measurement settings for this data stream."]
        #[doc = " Note that the stream must enable enhanced measurement for these settings to"]
        #[doc = " take effect."]
        pub async fn update_enhanced_measurement_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnhancedMeasurementSettingsRequest>,
        ) -> Result<tonic::Response<super::EnhancedMeasurementSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateEnhancedMeasurementSettings") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a connected site tag for a Universal Analytics property. You can"]
        #[doc = " create a maximum of 20 connected site tags per property."]
        #[doc = " Note: This API cannot be used on GA4 properties."]
        pub async fn create_connected_site_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectedSiteTagRequest>,
        ) -> Result<tonic::Response<super::CreateConnectedSiteTagResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateConnectedSiteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a connected site tag for a Universal Analytics property."]
        #[doc = " Note: this has no effect on GA4 properties."]
        pub async fn delete_connected_site_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectedSiteTagRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteConnectedSiteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the connected site tags for a Universal Analytics property. A maximum"]
        #[doc = " of 20 connected site tags will be returned. Note: this has no effect on GA4"]
        #[doc = " property."]
        pub async fn list_connected_site_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectedSiteTagsRequest>,
        ) -> Result<tonic::Response<super::ListConnectedSiteTagsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListConnectedSiteTags",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Given a specified UA property, looks up the GA4 property connected to it."]
        #[doc = " Note: this cannot be used with GA4 properties."]
        pub async fn fetch_connected_ga4_property(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchConnectedGa4PropertyRequest>,
        ) -> Result<tonic::Response<super::FetchConnectedGa4PropertyResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/FetchConnectedGa4Property",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Looks up a single AdSenseLink."]
        pub async fn get_ad_sense_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdSenseLinkRequest>,
        ) -> Result<tonic::Response<super::AdSenseLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetAdSenseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an AdSenseLink."]
        pub async fn create_ad_sense_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAdSenseLinkRequest>,
        ) -> Result<tonic::Response<super::AdSenseLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateAdSenseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an AdSenseLink."]
        pub async fn delete_ad_sense_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAdSenseLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteAdSenseLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists AdSenseLinks on a property."]
        pub async fn list_ad_sense_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAdSenseLinksRequest>,
        ) -> Result<tonic::Response<super::ListAdSenseLinksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListAdSenseLinks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single EventCreateRule."]
        pub async fn get_event_create_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventCreateRuleRequest>,
        ) -> Result<tonic::Response<super::EventCreateRule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetEventCreateRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists EventCreateRules on a web data stream."]
        pub async fn list_event_create_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventCreateRulesRequest>,
        ) -> Result<tonic::Response<super::ListEventCreateRulesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListEventCreateRules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an EventCreateRule."]
        pub async fn create_event_create_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventCreateRuleRequest>,
        ) -> Result<tonic::Response<super::EventCreateRule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateEventCreateRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an EventCreateRule."]
        pub async fn update_event_create_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEventCreateRuleRequest>,
        ) -> Result<tonic::Response<super::EventCreateRule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateEventCreateRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an EventCreateRule."]
        pub async fn delete_event_create_rule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventCreateRuleRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteEventCreateRule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a DataRedactionSettings on a property."]
        pub async fn update_data_redaction_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataRedactionSettingsRequest>,
        ) -> Result<tonic::Response<super::DataRedactionSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateDataRedactionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single DataRedactionSettings."]
        pub async fn get_data_redaction_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataRedactionSettingsRequest>,
        ) -> Result<tonic::Response<super::DataRedactionSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetDataRedactionSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single CalculatedMetric."]
        pub async fn get_calculated_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCalculatedMetricRequest>,
        ) -> Result<tonic::Response<super::CalculatedMetric>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetCalculatedMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a CalculatedMetric."]
        pub async fn create_calculated_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCalculatedMetricRequest>,
        ) -> Result<tonic::Response<super::CalculatedMetric>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateCalculatedMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists CalculatedMetrics on a property."]
        pub async fn list_calculated_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCalculatedMetricsRequest>,
        ) -> Result<tonic::Response<super::ListCalculatedMetricsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListCalculatedMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a CalculatedMetric on a property."]
        pub async fn update_calculated_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCalculatedMetricRequest>,
        ) -> Result<tonic::Response<super::CalculatedMetric>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateCalculatedMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a CalculatedMetric on a property."]
        pub async fn delete_calculated_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCalculatedMetricRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteCalculatedMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a roll-up property and all roll-up property source links."]
        pub async fn create_rollup_property(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRollupPropertyRequest>,
        ) -> Result<tonic::Response<super::CreateRollupPropertyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateRollupProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single roll-up property source Link."]
        #[doc = " Only roll-up properties can have source links, so this method will throw an"]
        #[doc = " error if used on other types of properties."]
        pub async fn get_rollup_property_source_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRollupPropertySourceLinkRequest>,
        ) -> Result<tonic::Response<super::RollupPropertySourceLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetRollupPropertySourceLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists roll-up property source Links on a property."]
        #[doc = " Only roll-up properties can have source links, so this method will throw an"]
        #[doc = " error if used on other types of properties."]
        pub async fn list_rollup_property_source_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRollupPropertySourceLinksRequest>,
        ) -> Result<tonic::Response<super::ListRollupPropertySourceLinksResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/ListRollupPropertySourceLinks") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a roll-up property source link."]
        #[doc = " Only roll-up properties can have source links, so this method will throw an"]
        #[doc = " error if used on other types of properties."]
        pub async fn create_rollup_property_source_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRollupPropertySourceLinkRequest>,
        ) -> Result<tonic::Response<super::RollupPropertySourceLink>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateRollupPropertySourceLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a roll-up property source link."]
        #[doc = " Only roll-up properties can have source links, so this method will throw an"]
        #[doc = " error if used on other types of properties."]
        pub async fn delete_rollup_property_source_link(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRollupPropertySourceLinkRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteRollupPropertySourceLink") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a subproperty and a subproperty event filter that applies to the"]
        #[doc = " created subproperty."]
        pub async fn create_subproperty(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSubpropertyRequest>,
        ) -> Result<tonic::Response<super::CreateSubpropertyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateSubproperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a subproperty Event Filter."]
        pub async fn create_subproperty_event_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSubpropertyEventFilterRequest>,
        ) -> Result<tonic::Response<super::SubpropertyEventFilter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/CreateSubpropertyEventFilter") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup for a single subproperty Event Filter."]
        pub async fn get_subproperty_event_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubpropertyEventFilterRequest>,
        ) -> Result<tonic::Response<super::SubpropertyEventFilter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/GetSubpropertyEventFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all subproperty Event Filters on a property."]
        pub async fn list_subproperty_event_filters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubpropertyEventFiltersRequest>,
        ) -> Result<tonic::Response<super::ListSubpropertyEventFiltersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.admin.v1alpha.AnalyticsAdminService/ListSubpropertyEventFilters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a subproperty Event Filter."]
        pub async fn update_subproperty_event_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSubpropertyEventFilterRequest>,
        ) -> Result<tonic::Response<super::SubpropertyEventFilter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/UpdateSubpropertyEventFilter") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a subproperty event filter."]
        pub async fn delete_subproperty_event_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSubpropertyEventFilterRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.analytics.admin.v1alpha.AnalyticsAdminService/DeleteSubpropertyEventFilter") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
