/// A collection of source attributions for a piece of content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    #[prost(message, repeated, tag = "1")]
    pub citation_sources: ::prost::alloc::vec::Vec<CitationSource>,
}
/// A citation to a source for a portion of a specific response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this
    /// source.
    ///
    /// Index indicates the start of the segment, measured in bytes.
    #[prost(int32, optional, tag = "1")]
    pub start_index: ::core::option::Option<i32>,
    /// Optional. End of the attributed segment, exclusive.
    #[prost(int32, optional, tag = "2")]
    pub end_index: ::core::option::Option<i32>,
    /// Optional. URI that is attributed as a source for a portion of the text.
    #[prost(string, optional, tag = "3")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. License for the GitHub project that is attributed as a source for
    /// segment.
    ///
    /// License info is required for code citations.
    #[prost(string, optional, tag = "4")]
    pub license: ::core::option::Option<::prost::alloc::string::String>,
}
/// The base structured datatype containing multi-part content of a message.
///
/// A `Content` includes a `role` field designating the producer of the `Content`
/// and a `parts` field containing multi-part data that contains the content of
/// the message turn.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    /// Ordered `Parts` that constitute a single message. Parts may have different
    /// MIME types.
    #[prost(message, repeated, tag = "1")]
    pub parts: ::prost::alloc::vec::Vec<Part>,
    /// Optional. The producer of the content. Must be either 'user' or 'model'.
    ///
    /// Useful to set for multi-turn conversations, otherwise can be left blank
    /// or unset.
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
}
/// A datatype containing media that is part of a multi-part `Content` message.
///
/// A `Part` consists of data which has an associated datatype. A `Part` can only
/// contain one of the accepted types in `Part.data`.
///
/// A `Part` must have a fixed IANA MIME type identifying the type and subtype
/// of the media if the `inline_data` field is filled with raw bytes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(oneof = "part::Data", tags = "2, 3")]
    pub data: ::core::option::Option<part::Data>,
}
/// Nested message and enum types in `Part`.
pub mod part {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Inline text.
        #[prost(string, tag = "2")]
        Text(::prost::alloc::string::String),
        /// Inline media bytes.
        #[prost(message, tag = "3")]
        InlineData(super::Blob),
    }
}
/// Raw media bytes.
///
/// Text should not be sent as raw bytes, use the 'text' field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    /// The IANA standard MIME type of the source data.
    /// Examples:
    ///   - image/png
    ///   - image/jpeg
    /// If an unsupported MIME type is provided, an error will be returned. For a
    /// complete list of supported types, see [Supported file
    /// formats](<https://ai.google.dev/gemini-api/docs/prompting_with_media#supported_file_formats>).
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Raw bytes for media formats.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Safety rating for a piece of content.
///
/// The safety rating contains the category of harm and the
/// harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of
/// harm categories and the probability of the harm classification is included
/// here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetyRating {
    /// Required. The category for this rating.
    #[prost(enumeration = "HarmCategory", tag = "3")]
    pub category: i32,
    /// Required. The probability of harm for this content.
    #[prost(enumeration = "safety_rating::HarmProbability", tag = "4")]
    pub probability: i32,
    /// Was this content blocked because of this rating?
    #[prost(bool, tag = "5")]
    pub blocked: bool,
}
/// Nested message and enum types in `SafetyRating`.
pub mod safety_rating {
    /// The probability that a piece of content is harmful.
    ///
    /// The classification system gives the probability of the content being
    /// unsafe. This does not indicate the severity of harm for a piece of content.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HarmProbability {
        /// Probability is unspecified.
        Unspecified = 0,
        /// Content has a negligible chance of being unsafe.
        Negligible = 1,
        /// Content has a low chance of being unsafe.
        Low = 2,
        /// Content has a medium chance of being unsafe.
        Medium = 3,
        /// Content has a high chance of being unsafe.
        High = 4,
    }
}
/// Safety setting, affecting the safety-blocking behavior.
///
/// Passing a safety setting for a category changes the allowed probability that
/// content is blocked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafetySetting {
    /// Required. The category for this setting.
    #[prost(enumeration = "HarmCategory", tag = "3")]
    pub category: i32,
    /// Required. Controls the probability threshold at which harm is blocked.
    #[prost(enumeration = "safety_setting::HarmBlockThreshold", tag = "4")]
    pub threshold: i32,
}
/// Nested message and enum types in `SafetySetting`.
pub mod safety_setting {
    /// Block at and beyond a specified harm probability.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HarmBlockThreshold {
        /// Threshold is unspecified.
        Unspecified = 0,
        /// Content with NEGLIGIBLE will be allowed.
        BlockLowAndAbove = 1,
        /// Content with NEGLIGIBLE and LOW will be allowed.
        BlockMediumAndAbove = 2,
        /// Content with NEGLIGIBLE, LOW, and MEDIUM will be allowed.
        BlockOnlyHigh = 3,
        /// All content will be allowed.
        BlockNone = 4,
    }
}
/// The category of a rating.
///
/// These categories cover various kinds of harms that developers
/// may wish to adjust.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HarmCategory {
    /// Category is unspecified.
    Unspecified = 0,
    /// Negative or harmful comments targeting identity and/or protected attribute.
    Derogatory = 1,
    /// Content that is rude, disrespectful, or profane.
    Toxicity = 2,
    /// Describes scenarios depicting violence against an individual or group, or
    /// general descriptions of gore.
    Violence = 3,
    /// Contains references to sexual acts or other lewd content.
    Sexual = 4,
    /// Promotes unchecked medical advice.
    Medical = 5,
    /// Dangerous content that promotes, facilitates, or encourages harmful acts.
    Dangerous = 6,
    /// Harasment content.
    Harassment = 7,
    /// Hate speech and content.
    HateSpeech = 8,
    /// Sexually explicit content.
    SexuallyExplicit = 9,
    /// Dangerous content.
    DangerousContent = 10,
}
/// Request to generate a completion from the model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContentRequest {
    /// Required. The name of the `Model` to use for generating the completion.
    ///
    /// Format: `name=models/{model}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content of the current conversation with the model.
    ///
    /// For single-turn queries, this is a single instance. For multi-turn queries,
    /// this is a repeated field that contains conversation history + latest
    /// request.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
    /// Optional. A list of unique `SafetySetting` instances for blocking unsafe
    /// content.
    ///
    /// This will be enforced on the `GenerateContentRequest.contents` and
    /// `GenerateContentResponse.candidates`. There should not be more than one
    /// setting for each `SafetyCategory` type. The API will block any contents and
    /// responses that fail to meet the thresholds set by these settings. This list
    /// overrides the default settings for each `SafetyCategory` specified in the
    /// safety_settings. If there is no `SafetySetting` for a given
    /// `SafetyCategory` provided in the list, the API will use the default safety
    /// setting for that category. Harm categories HARM_CATEGORY_HATE_SPEECH,
    /// HARM_CATEGORY_SEXUALLY_EXPLICIT, HARM_CATEGORY_DANGEROUS_CONTENT,
    /// HARM_CATEGORY_HARASSMENT are supported.
    #[prost(message, repeated, tag = "3")]
    pub safety_settings: ::prost::alloc::vec::Vec<SafetySetting>,
    /// Optional. Configuration options for model generation and outputs.
    #[prost(message, optional, tag = "4")]
    pub generation_config: ::core::option::Option<GenerationConfig>,
}
/// Configuration options for model generation and outputs. Not all parameters
/// may be configurable for every model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationConfig {
    /// Optional. Number of generated responses to return.
    ///
    /// Currently, this value can only be set to 1. If unset, this will default
    /// to 1.
    #[prost(int32, optional, tag = "1")]
    pub candidate_count: ::core::option::Option<i32>,
    /// Optional. The set of character sequences (up to 5) that will stop output
    /// generation. If specified, the API will stop at the first appearance of a
    /// stop sequence. The stop sequence will not be included as part of the
    /// response.
    #[prost(string, repeated, tag = "2")]
    pub stop_sequences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The maximum number of tokens to include in a candidate.
    ///
    /// Note: The default value varies by model, see the `Model.output_token_limit`
    /// attribute of the `Model` returned from the `getModel` function.
    #[prost(int32, optional, tag = "4")]
    pub max_output_tokens: ::core::option::Option<i32>,
    /// Optional. Controls the randomness of the output.
    ///
    /// Note: The default value varies by model, see the `Model.temperature`
    /// attribute of the `Model` returned from the `getModel` function.
    ///
    /// Values can range from [0.0, 2.0].
    #[prost(float, optional, tag = "5")]
    pub temperature: ::core::option::Option<f32>,
    /// Optional. The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so that only the
    /// most likely tokens are considered. Top-k sampling directly limits the
    /// maximum number of tokens to consider, while Nucleus sampling limits number
    /// of tokens based on the cumulative probability.
    ///
    /// Note: The default value varies by model, see the `Model.top_p`
    /// attribute of the `Model` returned from the `getModel` function.
    #[prost(float, optional, tag = "6")]
    pub top_p: ::core::option::Option<f32>,
    /// Optional. The maximum number of tokens to consider when sampling.
    ///
    /// Models use nucleus sampling or combined Top-k and nucleus sampling.
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// Models running with nucleus sampling don't allow top_k setting.
    ///
    /// Note: The default value varies by model, see the `Model.top_k`
    /// attribute of the `Model` returned from the `getModel` function. Empty
    /// `top_k` field in `Model` indicates the model doesn't apply top-k sampling
    /// and doesn't allow setting `top_k` on requests.
    #[prost(int32, optional, tag = "7")]
    pub top_k: ::core::option::Option<i32>,
}
/// Response from the model supporting multiple candidates.
///
/// Note on safety ratings and content filtering. They are reported for both
/// prompt in `GenerateContentResponse.prompt_feedback` and for each candidate
/// in `finish_reason` and in `safety_ratings`. The API contract is that:
///  - either all requested candidates are returned or no candidates at all
///  - no candidates are returned only if there was something wrong with the
///    prompt (see `prompt_feedback`)
///  - feedback on each candidate is reported on `finish_reason` and
///    `safety_ratings`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    #[prost(message, repeated, tag = "1")]
    pub candidates: ::prost::alloc::vec::Vec<Candidate>,
    /// Returns the prompt's feedback related to the content filters.
    #[prost(message, optional, tag = "2")]
    pub prompt_feedback: ::core::option::Option<generate_content_response::PromptFeedback>,
    /// Output only. Metadata on the generation requests' token usage.
    #[prost(message, optional, tag = "3")]
    pub usage_metadata: ::core::option::Option<generate_content_response::UsageMetadata>,
}
/// Nested message and enum types in `GenerateContentResponse`.
pub mod generate_content_response {
    /// A set of the feedback metadata the prompt specified in
    /// `GenerateContentRequest.content`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PromptFeedback {
        /// Optional. If set, the prompt was blocked and no candidates are returned.
        /// Rephrase your prompt.
        #[prost(enumeration = "prompt_feedback::BlockReason", tag = "1")]
        pub block_reason: i32,
        /// Ratings for safety of the prompt.
        /// There is at most one rating per category.
        #[prost(message, repeated, tag = "2")]
        pub safety_ratings: ::prost::alloc::vec::Vec<super::SafetyRating>,
    }
    /// Nested message and enum types in `PromptFeedback`.
    pub mod prompt_feedback {
        /// Specifies what was the reason why prompt was blocked.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum BlockReason {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// Prompt was blocked due to safety reasons. You can inspect
            /// `safety_ratings` to understand which safety category blocked it.
            Safety = 1,
            /// Prompt was blocked due to unknown reaasons.
            Other = 2,
        }
    }
    /// Metadata on the generation request's token usage.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UsageMetadata {
        /// Number of tokens in the prompt.
        #[prost(int32, tag = "1")]
        pub prompt_token_count: i32,
        /// Total number of tokens across the generated candidates.
        #[prost(int32, tag = "2")]
        pub candidates_token_count: i32,
        /// Total token count for the generation request (prompt + candidates).
        #[prost(int32, tag = "3")]
        pub total_token_count: i32,
    }
}
/// A response candidate generated from the model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candidate {
    /// Output only. Index of the candidate in the list of candidates.
    #[prost(int32, optional, tag = "3")]
    pub index: ::core::option::Option<i32>,
    /// Output only. Generated content returned from the model.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Output only. The reason why the model stopped generating tokens.
    ///
    /// If empty, the model has not stopped generating the tokens.
    #[prost(enumeration = "candidate::FinishReason", tag = "2")]
    pub finish_reason: i32,
    /// List of ratings for the safety of a response candidate.
    ///
    /// There is at most one rating per category.
    #[prost(message, repeated, tag = "5")]
    pub safety_ratings: ::prost::alloc::vec::Vec<SafetyRating>,
    /// Output only. Citation information for model-generated candidate.
    ///
    /// This field may be populated with recitation information for any text
    /// included in the `content`. These are passages that are "recited" from
    /// copyrighted material in the foundational LLM's training data.
    #[prost(message, optional, tag = "6")]
    pub citation_metadata: ::core::option::Option<CitationMetadata>,
    /// Output only. Token count for this candidate.
    #[prost(int32, tag = "7")]
    pub token_count: i32,
}
/// Nested message and enum types in `Candidate`.
pub mod candidate {
    /// Defines the reason why the model stopped generating tokens.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FinishReason {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Natural stop point of the model or provided stop sequence.
        Stop = 1,
        /// The maximum number of tokens as specified in the request was reached.
        MaxTokens = 2,
        /// The candidate content was flagged for safety reasons.
        Safety = 3,
        /// The candidate content was flagged for recitation reasons.
        Recitation = 4,
        /// Unknown reason.
        Other = 5,
    }
}
/// Request containing the `Content` for the model to embed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedContentRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. The content to embed. Only the `parts.text` fields will be
    /// counted.
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Optional task type for which the embeddings will be used. Can
    /// only be set for `models/embedding-001`.
    #[prost(enumeration = "TaskType", optional, tag = "3")]
    pub task_type: ::core::option::Option<i32>,
    /// Optional. An optional title for the text. Only applicable when TaskType is
    /// `RETRIEVAL_DOCUMENT`.
    ///
    /// Note: Specifying a `title` for `RETRIEVAL_DOCUMENT` provides better quality
    /// embeddings for retrieval.
    #[prost(string, optional, tag = "4")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. Optional reduced dimension for the output embedding. If set,
    /// excessive values in the output embedding are truncated from the end.
    /// Supported by newer models since 2024, and the earlier model
    /// (`models/embedding-001`) cannot specify this value.
    #[prost(int32, optional, tag = "5")]
    pub output_dimensionality: ::core::option::Option<i32>,
}
/// A list of floats representing an embedding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentEmbedding {
    /// The embedding values.
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
/// The response to an `EmbedContentRequest`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbedContentResponse {
    /// Output only. The embedding generated from the input content.
    #[prost(message, optional, tag = "1")]
    pub embedding: ::core::option::Option<ContentEmbedding>,
}
/// Batch request to get embeddings from the model for a list of prompts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedContentsRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Required. Embed requests for the batch. The model in each of these requests
    /// must match the model specified `BatchEmbedContentsRequest.model`.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<EmbedContentRequest>,
}
/// The response to a `BatchEmbedContentsRequest`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchEmbedContentsResponse {
    /// Output only. The embeddings for each request, in the same order as provided
    /// in the batch request.
    #[prost(message, repeated, tag = "1")]
    pub embeddings: ::prost::alloc::vec::Vec<ContentEmbedding>,
}
/// Counts the number of tokens in the `prompt` sent to a model.
///
/// Models may tokenize text differently, so each model may return a different
/// `token_count`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTokensRequest {
    /// Required. The model's resource name. This serves as an ID for the Model to
    /// use.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Optional. The input given to the model as a prompt. This field is ignored
    /// when `generate_content_request` is set.
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<Content>,
    /// Optional. The overall input given to the model. CountTokens will count
    /// prompt, function calling, etc.
    #[prost(message, optional, tag = "3")]
    pub generate_content_request: ::core::option::Option<GenerateContentRequest>,
}
/// A response from `CountTokens`.
///
/// It returns the model's `token_count` for the `prompt`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountTokensResponse {
    /// The number of tokens that the `model` tokenizes the `prompt` into.
    ///
    /// Always non-negative.
    #[prost(int32, tag = "1")]
    pub total_tokens: i32,
}
/// Type of task for which the embedding will be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// Unset value, which will default to one of the other enum values.
    Unspecified = 0,
    /// Specifies the given text is a query in a search/retrieval setting.
    RetrievalQuery = 1,
    /// Specifies the given text is a document from the corpus being searched.
    RetrievalDocument = 2,
    /// Specifies the given text will be used for STS.
    SemanticSimilarity = 3,
    /// Specifies that the given text will be classified.
    Classification = 4,
    /// Specifies that the embeddings will be used for clustering.
    Clustering = 5,
    /// Specifies that the given text will be used for question answering.
    QuestionAnswering = 6,
    /// Specifies that the given text will be used for fact verification.
    FactVerification = 7,
}
#[doc = r" Generated client implementations."]
pub mod generative_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API for using Large Models that generate multimodal content and have"]
    #[doc = " additional capabilities beyond text generation."]
    #[derive(Debug, Clone)]
    pub struct GenerativeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GenerativeServiceClient<T>
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
        ) -> GenerativeServiceClient<InterceptedService<T, F>>
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
            GenerativeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Generates a response from the model given an input"]
        #[doc = " `GenerateContentRequest`."]
        #[doc = ""]
        #[doc = " Input capabilities differ between models, including tuned models. See the"]
        #[doc = " [model guide](https://ai.google.dev/models/gemini) and"]
        #[doc = " [tuning guide](https://ai.google.dev/docs/model_tuning_guidance) for"]
        #[doc = " details."]
        pub async fn generate_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContentRequest>,
        ) -> Result<tonic::Response<super::GenerateContentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ai.generativelanguage.v1.GenerativeService/GenerateContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a streamed response from the model given an input"]
        #[doc = " `GenerateContentRequest`."]
        pub async fn stream_generate_content(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateContentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GenerateContentResponse>>,
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
                "/google.ai.generativelanguage.v1.GenerativeService/StreamGenerateContent",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Generates an embedding from the model given an input `Content`."]
        pub async fn embed_content(
            &mut self,
            request: impl tonic::IntoRequest<super::EmbedContentRequest>,
        ) -> Result<tonic::Response<super::EmbedContentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ai.generativelanguage.v1.GenerativeService/EmbedContent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates multiple embeddings from the model given input text in a"]
        #[doc = " synchronous call."]
        pub async fn batch_embed_contents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEmbedContentsRequest>,
        ) -> Result<tonic::Response<super::BatchEmbedContentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ai.generativelanguage.v1.GenerativeService/BatchEmbedContents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs a model's tokenizer on input content and returns the token count."]
        pub async fn count_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::CountTokensRequest>,
        ) -> Result<tonic::Response<super::CountTokensResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ai.generativelanguage.v1.GenerativeService/CountTokens",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Information about a Generative Language Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Required. The resource name of the `Model`.
    ///
    /// Format: `models/{model}` with a `{model}` naming convention of:
    ///
    /// * "{base_model_id}-{version}"
    ///
    /// Examples:
    ///
    /// * `models/chat-bison-001`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the base model, pass this to the generation request.
    ///
    /// Examples:
    ///
    /// * `chat-bison`
    #[prost(string, tag = "2")]
    pub base_model_id: ::prost::alloc::string::String,
    /// Required. The version number of the model.
    ///
    /// This represents the major version
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// The human-readable name of the model. E.g. "Chat Bison".
    ///
    /// The name can be up to 128 characters long and can consist of any UTF-8
    /// characters.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// A short description of the model.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Maximum number of input tokens allowed for this model.
    #[prost(int32, tag = "6")]
    pub input_token_limit: i32,
    /// Maximum number of output tokens available for this model.
    #[prost(int32, tag = "7")]
    pub output_token_limit: i32,
    /// The model's supported generation methods.
    ///
    /// The method names are defined as Pascal case
    /// strings, such as `generateMessage` which correspond to API methods.
    #[prost(string, repeated, tag = "8")]
    pub supported_generation_methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Controls the randomness of the output.
    ///
    /// Values can range over `\[0.0,1.0\]`, inclusive. A value closer to `1.0` will
    /// produce responses that are more varied, while a value closer to `0.0` will
    /// typically result in less surprising responses from the model.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "9")]
    pub temperature: ::core::option::Option<f32>,
    /// For Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability
    /// sum is at least `top_p`.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    #[prost(float, optional, tag = "10")]
    pub top_p: ::core::option::Option<f32>,
    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// This value specifies default to be used by the backend while making the
    /// call to the model.
    /// If empty, indicates the model doesn't use top-k sampling, and `top_k` isn't
    /// allowed as a generation parameter.
    #[prost(int32, optional, tag = "11")]
    pub top_k: ::core::option::Option<i32>,
}
/// Request for getting information about a specific Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. The resource name of the model.
    ///
    /// This name should match a model name returned by the `ListModels` method.
    ///
    /// Format: `models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing all Models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// The maximum number of `Models` to return (per page).
    ///
    /// The service may return fewer models.
    /// If unspecified, at most 50 models will be returned per page.
    /// This method returns at most 1000 models per page, even if you pass a larger
    /// page_size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListModels` call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the next
    /// request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to `ListModels` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from `ListModel` containing a paginated list of Models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// The returned Models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provides methods for getting metadata information about Generative Models."]
    #[derive(Debug, Clone)]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModelServiceClient<T>
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
        ) -> ModelServiceClient<InterceptedService<T, F>>
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
            ModelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets information about a specific Model."]
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ai.generativelanguage.v1.ModelService/GetModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists models available through the API."]
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ai.generativelanguage.v1.ModelService/ListModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
