/// Represents the status for a request to either invoke or submit a
/// \[dialog\](<https://developers.google.com/workspace/chat/dialogs>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionStatus {
    /// The status code.
    #[prost(enumeration = "super::super::rpc::Code", tag = "1")]
    pub status_code: i32,
    /// The message to send users about the status of their request.
    /// If unset, a generic message based on the `status_code` is sent.
    #[prost(string, tag = "2")]
    pub user_facing_message: ::prost::alloc::string::String,
}
/// An attachment in Google Chat.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attachment {
    /// Resource name of the attachment, in the form
    /// `spaces/{space}/messages/{message}/attachments/{attachment}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The original file name for the content, not the full path.
    #[prost(string, tag = "2")]
    pub content_name: ::prost::alloc::string::String,
    /// Output only. The content type (MIME type) of the file.
    #[prost(string, tag = "3")]
    pub content_type: ::prost::alloc::string::String,
    /// Output only. The thumbnail URL which should be used to preview the
    /// attachment to a human user. Chat apps shouldn't use this URL to download
    /// attachment content.
    #[prost(string, tag = "5")]
    pub thumbnail_uri: ::prost::alloc::string::String,
    /// Output only. The download URL which should be used to allow a human user to
    /// download the attachment. Chat apps shouldn't use this URL to download
    /// attachment content.
    #[prost(string, tag = "6")]
    pub download_uri: ::prost::alloc::string::String,
    /// Output only. The source of the attachment.
    #[prost(enumeration = "attachment::Source", tag = "9")]
    pub source: i32,
    /// The data reference to the attachment.
    #[prost(oneof = "attachment::DataRef", tags = "4, 7")]
    pub data_ref: ::core::option::Option<attachment::DataRef>,
}
/// Nested message and enum types in `Attachment`.
pub mod attachment {
    /// The source of the attachment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Source {
        /// Reserved.
        Unspecified = 0,
        /// The file is a Google Drive file.
        DriveFile = 1,
        /// The file is uploaded to Chat.
        UploadedContent = 2,
    }
    /// The data reference to the attachment.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataRef {
        /// A reference to the attachment data. This field is used with the media API
        /// to download the attachment data.
        #[prost(message, tag = "4")]
        AttachmentDataRef(super::AttachmentDataRef),
        /// Output only. A reference to the Google Drive attachment. This field is
        /// used with the Google Drive API.
        #[prost(message, tag = "7")]
        DriveDataRef(super::DriveDataRef),
    }
}
/// A reference to the data of a drive attachment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveDataRef {
    /// The ID for the drive file. Use with the Drive API.
    #[prost(string, tag = "2")]
    pub drive_file_id: ::prost::alloc::string::String,
}
/// A reference to the attachment data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachmentDataRef {
    /// The resource name of the attachment data. This field is used with the media
    /// API to download the attachment data.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Opaque token containing a reference to an uploaded attachment. Treated by
    /// clients as an opaque string and used to create or update Chat messages with
    /// attachments.
    #[prost(string, tag = "2")]
    pub attachment_upload_token: ::prost::alloc::string::String,
}
/// Request to get an attachment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAttachmentRequest {
    /// Required. Resource name of the attachment, in the form
    /// `spaces/{space}/messages/{message}/attachments/{attachment}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to upload an attachment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadAttachmentRequest {
    /// Required. Resource name of the Chat space in which the attachment is
    /// uploaded. Format "spaces/{space}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filename of the attachment, including the file extension.
    #[prost(string, tag = "4")]
    pub filename: ::prost::alloc::string::String,
}
/// Response of uploading an attachment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadAttachmentResponse {
    /// Reference to the uploaded attachment.
    #[prost(message, optional, tag = "1")]
    pub attachment_data_ref: ::core::option::Option<AttachmentDataRef>,
}
/// A user in Google Chat.
/// When returned as an output from a request, if your Chat app [authenticates as
/// a
/// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
/// the output for a `User` resource only populates the user's `name` and `type`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Resource name for a Google Chat \[user][google.chat.v1.User\].
    ///
    /// Format: `users/{user}`. `users/app` can be used as an alias for the calling
    /// app \[bot][google.chat.v1.User.Type.BOT\] user.
    ///
    /// For [human users]\[google.chat.v1.User.Type.HUMAN\], `{user}` is the same
    /// user identifier as:
    ///
    /// - the `id` for the
    /// \[Person\](<https://developers.google.com/people/api/rest/v1/people>) in the
    /// People API. For example, `users/123456789` in Chat API represents the same
    /// person as the `123456789` Person profile ID in People API.
    ///
    /// - the `id` for a
    /// \[user\](<https://developers.google.com/admin-sdk/directory/reference/rest/v1/users>)
    /// in the Admin SDK Directory API.
    ///
    /// - the user's email address can be used as an alias for `{user}` in API
    /// requests. For example, if the People API Person profile ID for
    /// `user@example.com` is `123456789`, you can use `users/user@example.com` as
    /// an alias to reference `users/123456789`. Only the canonical resource name
    /// (for example `users/123456789`) will be returned from the API.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The user's display name.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Unique identifier of the user's Google Workspace domain.
    #[prost(string, tag = "6")]
    pub domain_id: ::prost::alloc::string::String,
    /// User type.
    #[prost(enumeration = "user::Type", tag = "5")]
    pub r#type: i32,
    /// Output only. When `true`, the user is deleted or their profile is not
    /// visible.
    #[prost(bool, tag = "7")]
    pub is_anonymous: bool,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value for the enum. DO NOT USE.
        Unspecified = 0,
        /// Human user.
        Human = 1,
        /// Chat app user.
        Bot = 2,
    }
}
/// Output only. Annotations associated with the plain-text body of the message.
/// To add basic formatting to a text message, see
/// [Format text
/// messages](<https://developers.google.com/workspace/chat/format-messages>).
///
/// Example plain-text message body:
/// ```
/// Hello @FooBot how are you!"
/// ```
///
/// The corresponding annotations metadata:
/// ```
/// "annotations":[{
///   "type":"USER_MENTION",
///   "startIndex":6,
///   "length":7,
///   "userMention": {
///     "user": {
///       "name":"users/{user}",
///       "displayName":"FooBot",
///       "avatarUrl":"<https://goo.gl/aeDtrS",>
///       "type":"BOT"
///     },
///     "type":"MENTION"
///    }
/// }]
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    /// The type of this annotation.
    #[prost(enumeration = "AnnotationType", tag = "1")]
    pub r#type: i32,
    /// Start index (0-based, inclusive) in the plain-text message body this
    /// annotation corresponds to.
    #[prost(int32, optional, tag = "2")]
    pub start_index: ::core::option::Option<i32>,
    /// Length of the substring in the plain-text message body this annotation
    /// corresponds to.
    #[prost(int32, tag = "3")]
    pub length: i32,
    /// Additional metadata about the annotation.
    #[prost(oneof = "annotation::Metadata", tags = "4, 5, 6")]
    pub metadata: ::core::option::Option<annotation::Metadata>,
}
/// Nested message and enum types in `Annotation`.
pub mod annotation {
    /// Additional metadata about the annotation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// The metadata of user mention.
        #[prost(message, tag = "4")]
        UserMention(super::UserMentionMetadata),
        /// The metadata for a slash command.
        #[prost(message, tag = "5")]
        SlashCommand(super::SlashCommandMetadata),
        /// The metadata for a rich link.
        #[prost(message, tag = "6")]
        RichLinkMetadata(super::RichLinkMetadata),
    }
}
/// Annotation metadata for user mentions (@).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserMentionMetadata {
    /// The user mentioned.
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    /// The type of user mention.
    #[prost(enumeration = "user_mention_metadata::Type", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `UserMentionMetadata`.
pub mod user_mention_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value for the enum. Don't use.
        Unspecified = 0,
        /// Add user to space.
        Add = 1,
        /// Mention user in space.
        Mention = 2,
    }
}
/// Annotation metadata for slash commands (/).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlashCommandMetadata {
    /// The Chat app whose command was invoked.
    #[prost(message, optional, tag = "1")]
    pub bot: ::core::option::Option<User>,
    /// The type of slash command.
    #[prost(enumeration = "slash_command_metadata::Type", tag = "2")]
    pub r#type: i32,
    /// The name of the invoked slash command.
    #[prost(string, tag = "3")]
    pub command_name: ::prost::alloc::string::String,
    /// The command ID of the invoked slash command.
    #[prost(int64, tag = "4")]
    pub command_id: i64,
    /// Indicates whether the slash command is for a dialog.
    #[prost(bool, tag = "5")]
    pub triggers_dialog: bool,
}
/// Nested message and enum types in `SlashCommandMetadata`.
pub mod slash_command_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value for the enum. Don't use.
        Unspecified = 0,
        /// Add Chat app to space.
        Add = 1,
        /// Invoke slash command in space.
        Invoke = 2,
    }
}
/// A rich link to a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichLinkMetadata {
    /// The URI of this link.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// The rich link type.
    #[prost(enumeration = "rich_link_metadata::RichLinkType", tag = "2")]
    pub rich_link_type: i32,
    /// Data for the linked resource.
    #[prost(oneof = "rich_link_metadata::Data", tags = "3")]
    pub data: ::core::option::Option<rich_link_metadata::Data>,
}
/// Nested message and enum types in `RichLinkMetadata`.
pub mod rich_link_metadata {
    /// The rich link type. More types might be added in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RichLinkType {
        /// Default value for the enum. Don't use.
        Unspecified = 0,
        /// A Google Drive rich link type.
        DriveFile = 1,
    }
    /// Data for the linked resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Data for a drive link.
        #[prost(message, tag = "3")]
        DriveLinkData(super::DriveLinkData),
    }
}
/// Data for Google Drive links.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DriveLinkData {
    /// A
    /// \[DriveDataRef\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages.attachments#drivedataref>)
    /// which references a Google Drive file.
    #[prost(message, optional, tag = "1")]
    pub drive_data_ref: ::core::option::Option<DriveDataRef>,
    /// The mime type of the linked Google Drive resource.
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Type of the annotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationType {
    /// Default value for the enum. Don't use.
    Unspecified = 0,
    /// A user is mentioned.
    UserMention = 1,
    /// A slash command is invoked.
    SlashCommand = 2,
    /// A rich link annotation.
    RichLink = 3,
}
/// A Google Group in Google Chat.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Resource name for a Google Group.
    ///
    /// Represents a
    /// \[group\](<https://cloud.google.com/identity/docs/reference/rest/v1/groups>) in
    /// Cloud Identity Groups API.
    ///
    /// Format: groups/{group}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents a membership relation in Google Chat, such as whether a user or
/// Chat app is invited to, part of, or absent from a space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Membership {
    /// Resource name of the membership, assigned by the server.
    ///
    /// Format: `spaces/{space}/members/{member}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. State of the membership.
    #[prost(enumeration = "membership::MembershipState", tag = "2")]
    pub state: i32,
    /// Optional. User's role within a Chat space, which determines their permitted
    /// actions in the space.
    ///
    /// This field can only be used as input in `UpdateMembership`.
    #[prost(enumeration = "membership::MembershipRole", tag = "7")]
    pub role: i32,
    /// Optional. Immutable. The creation time of the membership, such as when a
    /// member joined or was invited to join a space. This field is output only,
    /// except when used to import historical memberships in import mode spaces.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Immutable. The deletion time of the membership, such as when a
    /// member left or was removed from a space. This field is output only, except
    /// when used to import historical memberships in import mode spaces.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Member associated with this membership. Other member types might be
    /// supported in the future.
    #[prost(oneof = "membership::MemberType", tags = "3, 5")]
    pub member_type: ::core::option::Option<membership::MemberType>,
}
/// Nested message and enum types in `Membership`.
pub mod membership {
    /// Specifies the member's relationship with a space. Other membership states
    /// might be supported in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MembershipState {
        /// Default value. Don't use.
        Unspecified = 0,
        /// The user is added to the space, and can participate in the space.
        Joined = 1,
        /// The user is invited to join the space, but hasn't joined it.
        Invited = 2,
        /// The user doesn't belong to the space and doesn't have a pending
        /// invitation to join the space.
        NotAMember = 3,
    }
    /// Represents a user's permitted actions in a Chat space. More enum values
    /// might be added in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MembershipRole {
        /// Default value. For \[users][google.chat.v1.Membership.member\]: they
        /// aren't a member of the space, but can be invited. For
        /// [Google Groups]\[google.chat.v1.Membership.group_member\]: they're always
        ///  assigned this role (other enum values might be used in the future).
        Unspecified = 0,
        /// A member of the space. The user has basic permissions, like sending
        /// messages to the space. In 1:1 and unnamed group conversations, everyone
        /// has this role.
        RoleMember = 1,
        /// A space manager. The user has all basic permissions plus administrative
        /// permissions that let them manage the space, like adding or removing
        /// members. Only supported in
        /// \[SpaceType.SPACE][google.chat.v1.Space.SpaceType\].
        RoleManager = 2,
    }
    /// Member associated with this membership. Other member types might be
    /// supported in the future.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MemberType {
        /// The Google Chat user or app the membership corresponds to.
        /// If your Chat app [authenticates as a
        /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
        /// the output populates the
        /// \[user\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/User>)
        /// `name` and `type`.
        #[prost(message, tag = "3")]
        Member(super::User),
        /// The Google Group the membership corresponds to.
        #[prost(message, tag = "5")]
        GroupMember(super::Group),
    }
}
/// Request message for creating a membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMembershipRequest {
    /// Required. The resource name of the space for which to create the
    /// membership.
    ///
    /// Format: spaces/{space}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The membership relation to create.
    /// The `memberType` field must contain a user with the `user.name` and
    /// `user.type` fields populated. The server will assign a resource name
    /// and overwrite anything specified.
    /// When a Chat app creates a membership relation for a human user, it must use
    /// the `chat.memberships` scope, set `user.type` to `HUMAN`, and set
    /// `user.name` with format `users/{user}`, where `{user}` can be the email
    /// address for the user. For users in the same Workspace organization `{user}`
    /// can also be the `id` of the
    /// \[person\](<https://developers.google.com/people/api/rest/v1/people>) from the
    /// People API, or the `id` for the user in the Directory API. For example, if
    /// the People API Person profile ID for `user@example.com` is `123456789`, you
    /// can add the user to the space by setting the `membership.member.name` to
    /// `users/user@example.com` or `users/123456789`. When a Chat app creates a
    /// membership relation for itself, it must use the `chat.memberships.app`
    /// scope, set `user.type` to `BOT`, and set `user.name` to `users/app`.
    #[prost(message, optional, tag = "2")]
    pub membership: ::core::option::Option<Membership>,
}
/// Request message for updating a membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMembershipRequest {
    /// Required. The membership to update. Only fields specified by `update_mask`
    /// are updated.
    #[prost(message, optional, tag = "1")]
    pub membership: ::core::option::Option<Membership>,
    /// Required. The field paths to update. Separate multiple values with commas
    /// or use `*` to update all field paths.
    ///
    /// Currently supported field paths:
    ///
    /// - `role`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for listing memberships.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMembershipsRequest {
    /// Required. The resource name of the space for which to fetch a membership
    /// list.
    ///
    /// Format: spaces/{space}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of memberships to return. The service might
    /// return fewer than this value.
    ///
    /// If unspecified, at most 100 memberships are returned.
    ///
    /// The maximum value is 1000. If you use a value more than 1000, it's
    /// automatically changed to 1000.
    ///
    /// Negative values return an `INVALID_ARGUMENT` error.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous call to list memberships.
    /// Provide this parameter to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided should match the call that
    /// provided the page token. Passing different values to the other parameters
    /// might lead to unexpected results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A query filter.
    ///
    /// You can filter memberships by a member's role
    /// (\[`role`\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.members#membershiprole>))
    /// and type
    /// (\[`member.type`\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/User#type>)).
    ///
    /// To filter by role, set `role` to `ROLE_MEMBER` or `ROLE_MANAGER`.
    ///
    /// To filter by type, set `member.type` to `HUMAN` or `BOT`. Developer
    /// Preview: You can also filter for `member.type` using the `!=` operator.
    ///
    /// To filter by both role and type, use the `AND` operator. To filter by
    /// either role or type, use the `OR` operator.
    ///
    /// Either `member.type = "HUMAN"` or `member.type != "BOT"` is required
    /// when `use_admin_access` is set to true. Other member type filters will be
    /// rejected.
    ///
    /// For example, the following queries are valid:
    ///
    /// ```
    /// role = "ROLE_MANAGER" OR role = "ROLE_MEMBER"
    /// member.type = "HUMAN" AND role = "ROLE_MANAGER"
    ///
    /// member.type != "BOT"
    /// ```
    ///
    /// The following queries are invalid:
    ///
    /// ```
    /// member.type = "HUMAN" AND member.type = "BOT"
    /// role = "ROLE_MANAGER" AND role = "ROLE_MEMBER"
    /// ```
    ///
    /// Invalid queries are rejected by the server with an `INVALID_ARGUMENT`
    /// error.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. When `true`, also returns memberships associated with a
    /// [Google Group]\[google.chat.v1.Membership.group_member\], in
    /// addition to other types of memberships. If a
    /// \[filter][google.chat.v1.ListMembershipsRequest.filter\] is set,
    /// [Google Group]\[google.chat.v1.Membership.group_member\]
    /// memberships that don't match the filter criteria aren't returned.
    #[prost(bool, tag = "6")]
    pub show_groups: bool,
    /// Optional. When `true`, also returns memberships associated with
    /// \[invited][google.chat.v1.Membership.MembershipState.INVITED\] members, in
    /// addition to other types of memberships. If a
    /// filter is set,
    /// \[invited][google.chat.v1.Membership.MembershipState.INVITED\] memberships
    /// that don't match the filter criteria aren't returned.
    ///
    /// Currently requires [user
    /// authentication](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>).
    #[prost(bool, tag = "7")]
    pub show_invited: bool,
}
/// Response to list memberships of the space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMembershipsResponse {
    /// Unordered list. List of memberships in the requested (or first) page.
    #[prost(message, repeated, tag = "1")]
    pub memberships: ::prost::alloc::vec::Vec<Membership>,
    /// A token that you can send as `pageToken` to retrieve the next page of
    /// results. If empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to get a membership of a space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMembershipRequest {
    /// Required. Resource name of the membership to retrieve.
    ///
    /// To get the app's own membership [by using user
    /// authentication](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
    /// you can optionally use `spaces/{space}/members/app`.
    ///
    /// Format: `spaces/{space}/members/{member}` or `spaces/{space}/members/app`
    ///
    /// When [authenticated as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
    /// you can use the user's email as an alias for `{member}`. For example,
    /// `spaces/{space}/members/example@gmail.com` where `example@gmail.com` is the
    /// email of the Google Chat user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to delete a membership in a space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMembershipRequest {
    /// Required. Resource name of the membership to delete. Chat apps can delete
    /// human users' or their own memberships. Chat apps can't delete other apps'
    /// memberships.
    ///
    /// When deleting a human membership, requires the `chat.memberships` scope and
    /// `spaces/{space}/members/{member}` format. You can use the email as an
    /// alias for `{member}`. For example,
    /// `spaces/{space}/members/example@gmail.com` where `example@gmail.com` is the
    /// email of the Google Chat user.
    ///
    /// When deleting an app membership, requires the `chat.memberships.app` scope
    /// and `spaces/{space}/members/app` format.
    ///
    /// Format: `spaces/{space}/members/{member}` or `spaces/{space}/members/app`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A widget is a UI element that presents text and images.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WidgetMarkup {
    /// A list of buttons. Buttons is also `oneof data` and only one of these
    /// fields should be set.
    #[prost(message, repeated, tag = "6")]
    pub buttons: ::prost::alloc::vec::Vec<widget_markup::Button>,
    /// A `WidgetMarkup` can only have one of the following items. You can use
    /// multiple `WidgetMarkup` fields to display more items.
    #[prost(oneof = "widget_markup::Data", tags = "1, 2, 3")]
    pub data: ::core::option::Option<widget_markup::Data>,
}
/// Nested message and enum types in `WidgetMarkup`.
pub mod widget_markup {
    /// A paragraph of text. Formatted text supported. For more information
    /// about formatting text, see
    /// [Formatting text in Google Chat
    /// apps](<https://developers.google.com/workspace/chat/format-messages#card-formatting>)
    /// and
    /// [Formatting
    /// text in Google Workspace
    /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextParagraph {
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
    }
    /// A button. Can be a text button or an image button.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Button {
        #[prost(oneof = "button::Type", tags = "1, 2")]
        pub r#type: ::core::option::Option<button::Type>,
    }
    /// Nested message and enum types in `Button`.
    pub mod button {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A button with text and `onclick` action.
            #[prost(message, tag = "1")]
            TextButton(super::TextButton),
            /// A button with image and `onclick` action.
            #[prost(message, tag = "2")]
            ImageButton(super::ImageButton),
        }
    }
    /// A button with text and `onclick` action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextButton {
        /// The text of the button.
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
        /// The `onclick` action of the button.
        #[prost(message, optional, tag = "2")]
        pub on_click: ::core::option::Option<OnClick>,
    }
    /// A UI element contains a key (label) and a value (content). This
    /// element can also contain some actions such as `onclick` button.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValue {
        /// The text of the top label. Formatted text supported. For more information
        /// about formatting text, see
        /// [Formatting text in Google Chat
        /// apps](<https://developers.google.com/workspace/chat/format-messages#card-formatting>)
        /// and
        /// [Formatting
        /// text in Google Workspace
        /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
        #[prost(string, tag = "3")]
        pub top_label: ::prost::alloc::string::String,
        /// The text of the content. Formatted text supported and always required.
        /// For more information
        /// about formatting text, see
        /// [Formatting text in Google Chat
        /// apps](<https://developers.google.com/workspace/chat/format-messages#card-formatting>)
        /// and
        /// [Formatting
        /// text in Google Workspace
        /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
        #[prost(string, tag = "4")]
        pub content: ::prost::alloc::string::String,
        /// If the content should be multiline.
        #[prost(bool, tag = "9")]
        pub content_multiline: bool,
        /// The text of the bottom label. Formatted text supported. For more
        /// information about formatting text, see [Formatting text in Google Chat
        /// apps](<https://developers.google.com/workspace/chat/format-messages#card-formatting>)
        /// and
        /// [Formatting
        /// text in Google Workspace
        /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
        #[prost(string, tag = "5")]
        pub bottom_label: ::prost::alloc::string::String,
        /// The `onclick` action. Only the top label, bottom label, and content
        /// region are clickable.
        #[prost(message, optional, tag = "6")]
        pub on_click: ::core::option::Option<OnClick>,
        /// At least one of icons, `top_label` and `bottom_label` must be defined.
        #[prost(oneof = "key_value::Icons", tags = "1, 2")]
        pub icons: ::core::option::Option<key_value::Icons>,
        /// A control widget. You can set either `button` or `switch_widget`,
        /// but not both.
        #[prost(oneof = "key_value::Control", tags = "7")]
        pub control: ::core::option::Option<key_value::Control>,
    }
    /// Nested message and enum types in `KeyValue`.
    pub mod key_value {
        /// At least one of icons, `top_label` and `bottom_label` must be defined.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Icons {
            /// An enum value that's replaced by the Chat API with the
            /// corresponding icon image.
            #[prost(enumeration = "super::Icon", tag = "1")]
            Icon(i32),
            /// The icon specified by a URL.
            #[prost(string, tag = "2")]
            IconUrl(::prost::alloc::string::String),
        }
        /// A control widget. You can set either `button` or `switch_widget`,
        /// but not both.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Control {
            /// A button that can be clicked to trigger an action.
            #[prost(message, tag = "7")]
            Button(super::Button),
        }
    }
    /// An image that's specified by a URL and can have an `onclick` action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Image {
        /// The URL of the image.
        #[prost(string, tag = "1")]
        pub image_url: ::prost::alloc::string::String,
        /// The `onclick` action.
        #[prost(message, optional, tag = "2")]
        pub on_click: ::core::option::Option<OnClick>,
        /// The aspect ratio of this image (width and height). This field lets you
        /// reserve the right height for the image while waiting for it to load.
        /// It's not meant to override the built-in aspect ratio of the image.
        /// If unset, the server fills it by prefetching the image.
        #[prost(double, tag = "3")]
        pub aspect_ratio: f64,
    }
    /// An image button with an `onclick` action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImageButton {
        /// The `onclick` action.
        #[prost(message, optional, tag = "2")]
        pub on_click: ::core::option::Option<OnClick>,
        /// The name of this `image_button` that's used for accessibility.
        /// Default value is provided if this name isn't specified.
        #[prost(string, tag = "4")]
        pub name: ::prost::alloc::string::String,
        /// The icon can be specified by an `Icon` `enum` or a URL.
        #[prost(oneof = "image_button::Icons", tags = "1, 3")]
        pub icons: ::core::option::Option<image_button::Icons>,
    }
    /// Nested message and enum types in `ImageButton`.
    pub mod image_button {
        /// The icon can be specified by an `Icon` `enum` or a URL.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Icons {
            /// The icon specified by an `enum` that indices to an icon provided by
            /// Chat API.
            #[prost(enumeration = "super::Icon", tag = "1")]
            Icon(i32),
            /// The icon specified by a URL.
            #[prost(string, tag = "3")]
            IconUrl(::prost::alloc::string::String),
        }
    }
    /// An `onclick` action (for example, open a link).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OnClick {
        #[prost(oneof = "on_click::Data", tags = "1, 2")]
        pub data: ::core::option::Option<on_click::Data>,
    }
    /// Nested message and enum types in `OnClick`.
    pub mod on_click {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Data {
            /// A form action is triggered by this `onclick` action if specified.
            #[prost(message, tag = "1")]
            Action(super::FormAction),
            /// This `onclick` action triggers an open link action if specified.
            #[prost(message, tag = "2")]
            OpenLink(super::OpenLink),
        }
    }
    /// A link that opens a new window.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OpenLink {
        /// The URL to open.
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
    }
    /// A form action describes the behavior when the form is submitted.
    /// For example, you can invoke Apps Script to handle the form.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FormAction {
        /// The method name is used to identify which part of the form triggered the
        /// form submission. This information is echoed back to the Chat app as part
        /// of the card click event. You can use the same method name for several
        /// elements that trigger a common behavior.
        #[prost(string, tag = "1")]
        pub action_method_name: ::prost::alloc::string::String,
        /// List of action parameters.
        #[prost(message, repeated, tag = "2")]
        pub parameters: ::prost::alloc::vec::Vec<form_action::ActionParameter>,
    }
    /// Nested message and enum types in `FormAction`.
    pub mod form_action {
        /// List of string parameters to supply when the action method is invoked.
        /// For example, consider three snooze buttons: snooze now, snooze one day,
        /// snooze next week. You might use `action method = snooze()`, passing the
        /// snooze type and snooze time in the list of string parameters.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ActionParameter {
            /// The name of the parameter for the action script.
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            /// The value of the parameter.
            #[prost(string, tag = "2")]
            pub value: ::prost::alloc::string::String,
        }
    }
    /// The set of supported icons.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Icon {
        Unspecified = 0,
        Airplane = 1,
        Bookmark = 26,
        Bus = 25,
        Car = 9,
        Clock = 2,
        ConfirmationNumberIcon = 12,
        Dollar = 14,
        Description = 27,
        Email = 10,
        EventPerformer = 20,
        EventSeat = 21,
        FlightArrival = 16,
        FlightDeparture = 15,
        Hotel = 6,
        HotelRoomType = 17,
        Invite = 19,
        MapPin = 3,
        Membership = 24,
        MultiplePeople = 18,
        Offer = 30,
        Person = 11,
        Phone = 13,
        RestaurantIcon = 7,
        ShoppingCart = 8,
        Star = 5,
        Store = 22,
        Ticket = 4,
        Train = 23,
        VideoCamera = 28,
        VideoPlay = 29,
    }
    /// A `WidgetMarkup` can only have one of the following items. You can use
    /// multiple `WidgetMarkup` fields to display more items.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Display a text paragraph in this widget.
        #[prost(message, tag = "1")]
        TextParagraph(TextParagraph),
        /// Display an image in this widget.
        #[prost(message, tag = "2")]
        Image(Image),
        /// Display a key value item in this widget.
        #[prost(message, tag = "3")]
        KeyValue(KeyValue),
    }
}
/// The markup for developers to specify the contents of a contextual AddOn.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextualAddOnMarkup {}
/// Nested message and enum types in `ContextualAddOnMarkup`.
pub mod contextual_add_on_markup {
    /// A card is a UI element that can contain UI widgets such as text and
    /// images.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Card {
        /// The header of the card. A header usually contains a title and an image.
        #[prost(message, optional, tag = "1")]
        pub header: ::core::option::Option<card::CardHeader>,
        /// Sections are separated by a line divider.
        #[prost(message, repeated, tag = "2")]
        pub sections: ::prost::alloc::vec::Vec<card::Section>,
        /// The actions of this card.
        #[prost(message, repeated, tag = "3")]
        pub card_actions: ::prost::alloc::vec::Vec<card::CardAction>,
        /// Name of the card.
        #[prost(string, tag = "4")]
        pub name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Card`.
    pub mod card {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CardHeader {
            /// The title must be specified. The header has a fixed height: if both a
            /// title and subtitle is specified, each takes up one line. If only the
            /// title is specified, it takes up both lines.
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            /// The subtitle of the card header.
            #[prost(string, tag = "2")]
            pub subtitle: ::prost::alloc::string::String,
            /// The image's type (for example, square border or circular border).
            #[prost(enumeration = "card_header::ImageStyle", tag = "3")]
            pub image_style: i32,
            /// The URL of the image in the card header.
            #[prost(string, tag = "4")]
            pub image_url: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `CardHeader`.
        pub mod card_header {
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum ImageStyle {
                Unspecified = 0,
                /// Square border.
                Image = 1,
                /// Circular border.
                Avatar = 2,
            }
        }
        /// A section contains a collection of widgets that are rendered
        /// (vertically) in the order that they are specified. Across all platforms,
        /// cards have a narrow fixed width, so
        /// there's currently no need for layout properties (for example, float).
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Section {
            /// The header of the section. Formatted text is
            /// supported. For more information
            /// about formatting text, see
            /// [Formatting text in Google Chat
            /// apps](<https://developers.google.com/workspace/chat/format-messages#card-formatting>)
            /// and
            /// [Formatting
            /// text in Google Workspace
            /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
            #[prost(string, tag = "1")]
            pub header: ::prost::alloc::string::String,
            /// A section must contain at least one widget.
            #[prost(message, repeated, tag = "2")]
            pub widgets: ::prost::alloc::vec::Vec<super::super::WidgetMarkup>,
        }
        /// A card action is
        /// the action associated with the card. For an invoice card, a
        /// typical action would be: delete invoice, email invoice or open the
        /// invoice in browser.
        ///
        /// Not supported by Google Chat apps.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CardAction {
            /// The label used to be displayed in the action menu item.
            #[prost(string, tag = "1")]
            pub action_label: ::prost::alloc::string::String,
            /// The onclick action for this action item.
            #[prost(message, optional, tag = "2")]
            pub on_click: ::core::option::Option<super::super::widget_markup::OnClick>,
        }
    }
}
/// Information about a deleted message. A message is deleted when `delete_time`
/// is set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletionMetadata {
    /// Indicates who deleted the message.
    #[prost(enumeration = "deletion_metadata::DeletionType", tag = "1")]
    pub deletion_type: i32,
}
/// Nested message and enum types in `DeletionMetadata`.
pub mod deletion_metadata {
    /// Who deleted the message and how it was deleted.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DeletionType {
        /// This value is unused.
        Unspecified = 0,
        /// User deleted their own message.
        Creator = 1,
        /// The space owner deleted the message.
        SpaceOwner = 2,
        /// A Google Workspace admin deleted the message.
        Admin = 3,
        /// A Chat app deleted its own message when it expired.
        AppMessageExpiry = 4,
        /// A Chat app deleted the message on behalf of the user.
        CreatorViaApp = 5,
        /// A Chat app deleted the message on behalf of the space owner.
        SpaceOwnerViaApp = 6,
    }
}
/// A matched URL in a Chat message. Chat apps can preview matched URLs. For more
/// information, see [Preview
/// links](<https://developers.google.com/chat/how-tos/preview-links>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchedUrl {
    /// Output only. The URL that was matched.
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
/// A reaction to a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reaction {
    /// The resource name of the reaction.
    ///
    /// Format: `spaces/{space}/messages/{message}/reactions/{reaction}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The user who created the reaction.
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<User>,
    /// The emoji used in the reaction.
    #[prost(message, optional, tag = "3")]
    pub emoji: ::core::option::Option<Emoji>,
}
/// An emoji that is used as a reaction to a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Emoji {
    /// The content of the emoji.
    #[prost(oneof = "emoji::Content", tags = "1, 2")]
    pub content: ::core::option::Option<emoji::Content>,
}
/// Nested message and enum types in `Emoji`.
pub mod emoji {
    /// The content of the emoji.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// A basic emoji represented by a unicode string.
        #[prost(string, tag = "1")]
        Unicode(::prost::alloc::string::String),
        /// Output only. A custom emoji.
        #[prost(message, tag = "2")]
        CustomEmoji(super::CustomEmoji),
    }
}
/// Represents a custom emoji.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomEmoji {
    /// Output only. Unique key for the custom emoji resource.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
}
/// The number of people who reacted to a message with a specific emoji.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmojiReactionSummary {
    /// Emoji associated with the reactions.
    #[prost(message, optional, tag = "1")]
    pub emoji: ::core::option::Option<Emoji>,
    /// The total number of reactions using the associated emoji.
    #[prost(int32, optional, tag = "2")]
    pub reaction_count: ::core::option::Option<i32>,
}
/// Creates a reaction to a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReactionRequest {
    /// Required. The message where the reaction is created.
    ///
    /// Format: `spaces/{space}/messages/{message}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The reaction to create.
    #[prost(message, optional, tag = "2")]
    pub reaction: ::core::option::Option<Reaction>,
}
/// Lists reactions to a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReactionsRequest {
    /// Required. The message users reacted to.
    ///
    /// Format: `spaces/{space}/messages/{message}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of reactions returned. The service can return
    /// fewer reactions than this value. If unspecified, the default value is 25.
    /// The maximum value is 200; values above 200 are changed to 200.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. (If resuming from a previous query.)
    ///
    /// A page token received from a previous list reactions call. Provide this
    /// to retrieve the subsequent page.
    ///
    /// When paginating, the filter value should match the call that provided the
    /// page token. Passing a different value might lead to unexpected results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A query filter.
    ///
    /// You can filter reactions by
    /// \[emoji\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/Emoji>)
    /// (either `emoji.unicode` or `emoji.custom_emoji.uid`) and
    /// \[user\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/User>)
    /// (`user.name`).
    ///
    /// To filter reactions for multiple emojis or users, join similar fields
    /// with the `OR` operator, such as `emoji.unicode = "🙂" OR emoji.unicode =
    /// "👍"` and `user.name = "users/AAAAAA" OR user.name = "users/BBBBBB"`.
    ///
    /// To filter reactions by emoji and user, use the `AND` operator, such as
    /// `emoji.unicode = "🙂" AND user.name = "users/AAAAAA"`.
    ///
    /// If your query uses both `AND` and `OR`, group them with parentheses.
    ///
    /// For example, the following queries are valid:
    ///
    /// ```
    /// user.name = "users/{user}"
    /// emoji.unicode = "🙂"
    /// emoji.custom_emoji.uid = "{uid}"
    /// emoji.unicode = "🙂" OR emoji.unicode = "👍"
    /// emoji.unicode = "🙂" OR emoji.custom_emoji.uid = "{uid}"
    /// emoji.unicode = "🙂" AND user.name = "users/{user}"
    /// (emoji.unicode = "🙂" OR emoji.custom_emoji.uid = "{uid}")
    /// AND user.name = "users/{user}"
    /// ```
    ///
    /// The following queries are invalid:
    ///
    /// ```
    /// emoji.unicode = "🙂" AND emoji.unicode = "👍"
    /// emoji.unicode = "🙂" AND emoji.custom_emoji.uid = "{uid}"
    /// emoji.unicode = "🙂" OR user.name = "users/{user}"
    /// emoji.unicode = "🙂" OR emoji.custom_emoji.uid = "{uid}" OR
    /// user.name = "users/{user}"
    /// emoji.unicode = "🙂" OR emoji.custom_emoji.uid = "{uid}"
    /// AND user.name = "users/{user}"
    /// ```
    ///
    /// Invalid queries are rejected by the server with an `INVALID_ARGUMENT`
    /// error.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response to a list reactions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReactionsResponse {
    /// List of reactions in the requested (or first) page.
    #[prost(message, repeated, tag = "1")]
    pub reactions: ::prost::alloc::vec::Vec<Reaction>,
    /// Continuation token to retrieve the next page of results. It's empty
    /// for the last page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Deletes a reaction to a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReactionRequest {
    /// Required. Name of the reaction to delete.
    ///
    /// Format: `spaces/{space}/messages/{message}/reactions/{reaction}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A [slash
/// command](<https://developers.google.com/workspace/chat/slash-commands>) in
/// Google Chat.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlashCommand {
    /// The ID of the slash command invoked.
    #[prost(int64, tag = "1")]
    pub command_id: i64,
}
/// The history state for messages and spaces. Specifies how long messages and
/// conversation threads are kept after creation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HistoryState {
    /// Default value. Do not use.
    Unspecified = 0,
    /// History off. [Messages and threads are kept for 24
    /// hours](<https://support.google.com/chat/answer/7664687>).
    HistoryOff = 1,
    /// History on. The organization's [Vault retention
    /// rules](<https://support.google.com/vault/answer/7657597>) specify for
    /// how long messages and threads are kept.
    HistoryOn = 2,
}
/// A space in Google Chat. Spaces are conversations between two or more users
/// or 1:1 messages between a user and a Chat app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Space {
    /// Resource name of the space.
    ///
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Deprecated: Use `space_type` instead.
    /// The type of a space.
    #[deprecated]
    #[prost(enumeration = "space::Type", tag = "2")]
    pub r#type: i32,
    /// The type of space. Required when creating a space or updating the space
    /// type of a space. Output only for other usage.
    #[prost(enumeration = "space::SpaceType", tag = "10")]
    pub space_type: i32,
    /// Optional. Whether the space is a DM between a Chat app and a single
    /// human.
    #[prost(bool, tag = "4")]
    pub single_user_bot_dm: bool,
    /// Output only. Deprecated: Use `spaceThreadingState` instead.
    /// Whether messages are threaded in this space.
    #[deprecated]
    #[prost(bool, tag = "5")]
    pub threaded: bool,
    /// The space's display name. Required when [creating a
    /// space](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/create>).
    /// If you receive the error message `ALREADY_EXISTS` when creating a space or
    /// updating the `displayName`, try a different `displayName`. An
    /// existing space within the Google Workspace organization might already use
    /// this display name.
    ///
    /// For direct messages, this field might be empty.
    ///
    /// Supports up to 128 characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Immutable. Whether this space permits any Google Chat user as a member.
    /// Input when creating a space in a Google Workspace organization. Omit this
    /// field when creating spaces in the following conditions:
    ///
    ///   * The authenticated user uses a consumer account (unmanaged user
    ///     account). By default, a space created by a consumer account permits any
    ///     Google Chat user.
    ///
    ///   * The space is used to [import data to Google Chat]
    ///     (<https://developers.google.com/chat/api/guides/import-data-overview>)
    ///     because import mode spaces must only permit members from the same
    ///     Google Workspace organization. However, as part of the [Google
    ///     Workspace Developer Preview
    ///     Program](<https://developers.google.com/workspace/preview>), import mode
    ///     spaces can permit any Google Chat user so this field can then be set
    ///     for import mode spaces.
    ///
    /// For existing spaces, this field is output only.
    #[prost(bool, tag = "8")]
    pub external_user_allowed: bool,
    /// Output only. The threading state in the Chat space.
    #[prost(enumeration = "space::SpaceThreadingState", tag = "9")]
    pub space_threading_state: i32,
    /// Details about the space including description and rules.
    #[prost(message, optional, tag = "11")]
    pub space_details: ::core::option::Option<space::SpaceDetails>,
    /// The message history state for messages and threads in this space.
    #[prost(enumeration = "HistoryState", tag = "13")]
    pub space_history_state: i32,
    /// Optional. Whether this space is created in `Import Mode` as part of a data
    /// migration into Google Workspace. While spaces are being imported, they
    /// aren't visible to users until the import is complete.
    #[prost(bool, tag = "16")]
    pub import_mode: bool,
    /// Optional. Immutable. For spaces created in Chat, the time the space was
    /// created. This field is output only, except when used in import mode spaces.
    ///
    /// For import mode spaces, set this field to the historical timestamp at which
    /// the space was created in the source in order to preserve the original
    /// creation time.
    ///
    /// Only populated in the output when `spaceType` is `GROUP_CHAT` or `SPACE`.
    #[prost(message, optional, tag = "17")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For direct message (DM) spaces with a Chat app, whether the
    /// space was created by a Google Workspace administrator. Administrators can
    /// install and set up a direct message with a Chat app on behalf of users in
    /// their organization.
    ///
    /// To support admin install, your Chat app must feature direct messaging.
    #[prost(bool, tag = "19")]
    pub admin_installed: bool,
    /// Optional. Specifies the [access
    /// setting](<https://support.google.com/chat/answer/11971020>) of the space.
    /// Only populated when the `space_type` is `SPACE`.
    #[prost(message, optional, tag = "23")]
    pub access_settings: ::core::option::Option<space::AccessSettings>,
    /// Output only. The URI for a user to access the space.
    #[prost(string, tag = "25")]
    pub space_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Space`.
pub mod space {
    /// Details about the space including description and rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpaceDetails {
        /// Optional. A description of the space. For example, describe the space's
        /// discussion topic, functional purpose, or participants.
        ///
        /// Supports up to 150 characters.
        #[prost(string, tag = "1")]
        pub description: ::prost::alloc::string::String,
        /// Optional. The space's rules, expectations, and etiquette.
        ///
        /// Supports up to 5,000 characters.
        #[prost(string, tag = "2")]
        pub guidelines: ::prost::alloc::string::String,
    }
    /// Represents the [access
    /// setting](<https://support.google.com/chat/answer/11971020>) of the space.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessSettings {
        /// Output only. Indicates the access state of the space.
        #[prost(enumeration = "access_settings::AccessState", tag = "1")]
        pub access_state: i32,
        /// Optional. The resource name of the [target
        /// audience](<https://support.google.com/a/answer/9934697>) who can discover
        /// the space, join the space, and preview the messages in the space. For
        /// details, see [Make a space discoverable to a target
        /// audience](<https://developers.google.com/workspace/chat/space-target-audience>).
        ///
        /// Format: `audiences/{audience}`
        ///
        /// To use the default target audience for the Google Workspace organization,
        /// set to `audiences/default`.
        #[prost(string, tag = "3")]
        pub audience: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `AccessSettings`.
    pub mod access_settings {
        /// Represents the access state of the space.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum AccessState {
            /// Access state is unknown or not supported in this API.
            Unspecified = 0,
            /// Space is discoverable by added or invited members or groups.
            Private = 1,
            /// Space is discoverable by the selected [target
            /// audience](<https://support.google.com/a/answer/9934697>), as well as
            /// added or invited members or groups.
            Discoverable = 2,
        }
    }
    /// Deprecated: Use `SpaceType` instead.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Reserved.
        Unspecified = 0,
        /// Conversations between two or more humans.
        Room = 1,
        /// 1:1 Direct Message between a human and a Chat app, where all messages are
        /// flat. Note that this doesn't include direct messages between two humans.
        Dm = 2,
    }
    /// The type of space. Required when creating or updating a space. Output only
    /// for other usage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpaceType {
        /// Reserved.
        Unspecified = 0,
        /// A place where people send messages, share files, and collaborate.
        /// A `SPACE` can include Chat apps.
        Space = 1,
        /// Group conversations between 3 or more people.
        /// A `GROUP_CHAT` can include Chat apps.
        GroupChat = 2,
        /// 1:1 messages between two humans or a human and a Chat app.
        DirectMessage = 3,
    }
    /// Specifies the type of threading state in the Chat space.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpaceThreadingState {
        /// Reserved.
        Unspecified = 0,
        /// Named spaces that support message threads. When users respond to a
        /// message, they can reply in-thread, which keeps their response in the
        /// context of the original message.
        ThreadedMessages = 2,
        /// Named spaces where the conversation is organized by topic. Topics and
        /// their replies are grouped together.
        GroupedMessages = 3,
        /// Direct messages (DMs) between two people and group conversations between
        /// 3 or more people.
        UnthreadedMessages = 4,
    }
}
/// A request to create a named space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpaceRequest {
    /// Required. The `displayName` and `spaceType` fields must be populated.  Only
    /// `SpaceType.SPACE` is supported.
    ///
    /// If you receive the error message `ALREADY_EXISTS` when creating a space,
    /// try a different `displayName`. An existing space within the Google
    /// Workspace organization might already use this display name.
    ///
    /// The space `name` is assigned on the server so anything specified in this
    /// field will be ignored.
    #[prost(message, optional, tag = "1")]
    pub space: ::core::option::Option<Space>,
    /// Optional. A unique identifier for this request.
    /// A random UUID is recommended.
    /// Specifying an existing request ID returns the space created with that ID
    /// instead of creating a new space.
    /// Specifying an existing request ID from the same Chat app with a different
    /// authenticated user returns an error.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// A request to list the spaces the caller is a member of.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpacesRequest {
    /// Optional. The maximum number of spaces to return. The service might return
    /// fewer than this value.
    ///
    /// If unspecified, at most 100 spaces are returned.
    ///
    /// The maximum value is 1000. If you use a value more than 1000, it's
    /// automatically changed to 1000.
    ///
    /// Negative values return an `INVALID_ARGUMENT` error.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous list spaces call.
    /// Provide this parameter to retrieve the subsequent page.
    ///
    /// When paginating, the filter value should match the call that provided the
    /// page token. Passing a different value may lead to unexpected results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A query filter.
    ///
    /// You can filter spaces by the space type
    /// (\[`space_type`\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces#spacetype>)).
    ///
    /// To filter by space type, you must specify valid enum value, such as
    /// `SPACE` or `GROUP_CHAT` (the `space_type` can't be
    /// `SPACE_TYPE_UNSPECIFIED`). To query for multiple space types, use the `OR`
    /// operator.
    ///
    /// For example, the following queries are valid:
    ///
    /// ```
    /// space_type = "SPACE"
    /// spaceType = "GROUP_CHAT" OR spaceType = "DIRECT_MESSAGE"
    /// ```
    ///
    /// Invalid queries are rejected by the server with an `INVALID_ARGUMENT`
    /// error.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// The response for a list spaces request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpacesResponse {
    /// List of spaces in the requested (or first) page.
    #[prost(message, repeated, tag = "1")]
    pub spaces: ::prost::alloc::vec::Vec<Space>,
    /// You can send a token as `pageToken` to retrieve the next page of
    /// results. If empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to return a single space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpaceRequest {
    /// Required. Resource name of the space, in the form `spaces/{space}`.
    ///
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to get direct message space based on the user resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindDirectMessageRequest {
    /// Required. Resource name of the user to find direct message with.
    ///
    /// Format: `users/{user}`, where `{user}` is either the `id` for the
    /// \[person\](<https://developers.google.com/people/api/rest/v1/people>) from the
    /// People API, or the `id` for the
    /// \[user\](<https://developers.google.com/admin-sdk/directory/reference/rest/v1/users>)
    /// in the Directory API. For example, if the People API profile ID is
    /// `123456789`, you can find a direct message with that person by using
    /// `users/123456789` as the `name`. When [authenticated as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
    /// you can use the email as an alias for `{user}`. For example,
    /// `users/example@gmail.com` where `example@gmail.com` is the email of the
    /// Google Chat user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to update a single space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpaceRequest {
    /// Required. Space with fields to be updated. `Space.name` must be
    /// populated in the form of `spaces/{space}`. Only fields
    /// specified by `update_mask` are updated.
    #[prost(message, optional, tag = "1")]
    pub space: ::core::option::Option<Space>,
    /// Required. The updated field paths, comma separated if there are
    /// multiple.
    ///
    /// Currently supported field paths:
    ///
    /// - `display_name` (Only supports changing the display name of a space with
    /// the `SPACE` type, or when also including the `space_type` mask to change a
    /// `GROUP_CHAT` space type to `SPACE`. Trying to update the display name of a
    /// `GROUP_CHAT` or a `DIRECT_MESSAGE` space results in an invalid argument
    /// error. If you receive the error message `ALREADY_EXISTS` when updating the
    /// `displayName`, try a different `displayName`. An existing space within the
    /// Google Workspace organization might already use this display name.)
    ///
    /// - `space_type` (Only supports changing a `GROUP_CHAT` space type to
    /// `SPACE`. Include `display_name` together
    /// with `space_type` in the update mask and ensure that the specified space
    /// has a non-empty display name and the `SPACE` space type. Including the
    /// `space_type` mask and the `SPACE` type in the specified space when updating
    /// the display name is optional if the existing space already has the `SPACE`
    /// type. Trying to update the space type in other ways results in an invalid
    /// argument error).
    /// `space_type` is not supported with admin access.
    ///
    /// - `space_details`
    ///
    /// - `space_history_state` (Supports [turning history on or off for the
    /// space](<https://support.google.com/chat/answer/7664687>) if [the organization
    /// allows users to change their history
    /// setting](<https://support.google.com/a/answer/7664184>).
    /// Warning: mutually exclusive with all other field paths.)
    /// `space_history_state` is not supported with admin access.
    ///
    /// - `access_settings.audience` (Supports changing the [access
    /// setting](<https://support.google.com/chat/answer/11971020>) of who can
    /// discover the space, join the space, and preview the messages in space. If
    /// no audience is specified in the access setting, the space's access setting
    /// is updated to private. Warning: mutually exclusive with all other field
    /// paths.)
    /// `access_settings.audience` is not supported with admin access.
    ///
    /// - Developer Preview: Supports changing the [permission
    /// settings](<https://support.google.com/chat/answer/13340792>) of a space,
    /// supported field paths
    /// include: `permission_settings.manage_members_and_groups`,
    /// `permission_settings.modify_space_details`,
    /// `permission_settings.toggle_history`,
    /// `permission_settings.use_at_mention_all`,
    /// `permission_settings.manage_apps`, `permission_settings.manage_webhooks`,
    /// `permission_settings.reply_messages`
    ///  (Warning: mutually exclusive with all other non-permission settings field
    /// paths). `permission_settings` is not supported with admin access.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for deleting a space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSpaceRequest {
    /// Required. Resource name of the space to delete.
    ///
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for completing the import process for a space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteImportSpaceRequest {
    /// Required. Resource name of the import mode space.
    ///
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for completing the import process for a space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteImportSpaceResponse {
    /// The import mode space.
    #[prost(message, optional, tag = "1")]
    pub space: ::core::option::Option<Space>,
}
/// A message in a Google Chat space.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// Resource name of the message.
    ///
    /// Format: `spaces/{space}/messages/{message}`
    ///
    ///
    /// Where `{space}` is the ID of the space where the message is posted and
    /// `{message}` is a system-assigned ID for the message. For example,
    /// `spaces/AAAAAAAAAAA/messages/BBBBBBBBBBB.BBBBBBBBBBB`.
    ///
    /// If you set a custom ID when you create a message, you can use this ID to
    /// specify the message in a request by replacing `{message}` with the value
    /// from the `clientAssignedMessageId` field. For example,
    /// `spaces/AAAAAAAAAAA/messages/client-custom-name`. For details, see [Name
    /// a
    /// message](<https://developers.google.com/workspace/chat/create-messages#name_a_created_message>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The user who created the message.
    /// If your Chat app [authenticates as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
    /// the output populates the
    /// \[user\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/User>)
    /// `name` and `type`.
    #[prost(message, optional, tag = "2")]
    pub sender: ::core::option::Option<User>,
    /// Optional. Immutable. For spaces created in Chat, the time at which the
    /// message was created. This field is output only, except when used in import
    /// mode spaces.
    ///
    /// For import mode spaces, set this field to the historical timestamp at which
    /// the message was created in the source in order to preserve the original
    /// creation time.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the message was last edited by a user. If
    /// the message has never been edited, this field is empty.
    #[prost(message, optional, tag = "23")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the message was deleted in
    /// Google Chat. If the message is never deleted, this field is empty.
    #[prost(message, optional, tag = "26")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Plain-text body of the message. The first link to an image, video, or web
    /// page generates a
    /// [preview chip](<https://developers.google.com/workspace/chat/preview-links>).
    /// You can also [@mention a Google Chat
    /// user](<https://developers.google.com/workspace/chat/format-messages#messages-@mention>),
    /// or everyone in the space.
    ///
    /// To learn about creating text messages, see [Send a text
    /// message](<https://developers.google.com/workspace/chat/create-messages#create-text-messages>).
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// Output only. Contains the message `text` with markups added to communicate
    /// formatting. This field might not capture all formatting visible in the UI,
    /// but includes the following:
    ///
    /// * [Markup
    /// syntax](<https://developers.google.com/workspace/chat/format-messages>)
    /// for bold, italic, strikethrough, monospace, monospace block, and bulleted
    /// list.
    ///
    /// * [User
    /// mentions](<https://developers.google.com/workspace/chat/format-messages#messages-@mention>)
    /// using the format `<users/{user}>`.
    ///
    /// * Custom hyperlinks using the format `<{url}|{rendered_text}>` where the
    /// first string is the URL and the second is the rendered text—for example,
    /// `<<http://example.com|custom> text>`.
    ///
    /// * Custom emoji using the format `:{emoji_name}:`—for example, `:smile:`.
    /// This doesn't apply to Unicode emoji, such as `U+1F600` for a grinning
    /// face emoji.
    ///
    /// For more information, see [View text formatting sent in a
    /// message](<https://developers.google.com/workspace/chat/format-messages#view_text_formatting_sent_in_a_message>)
    #[prost(string, tag = "43")]
    pub formatted_text: ::prost::alloc::string::String,
    /// Deprecated: Use `cards_v2` instead.
    ///
    /// Rich, formatted, and interactive cards that you can use to display UI
    /// elements such as: formatted texts, buttons, and clickable images. Cards are
    /// normally displayed below the plain-text body of the message. `cards` and
    /// `cards_v2` can have a maximum size of 32 KB.
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub cards: ::prost::alloc::vec::Vec<contextual_add_on_markup::Card>,
    /// An array of
    /// \[cards\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/cards>).
    ///
    /// Only Chat apps can create cards. If your Chat app [authenticates as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
    /// the messages can't contain cards.
    ///
    /// To learn about cards and how to create them, see [Send card
    /// messages](<https://developers.google.com/workspace/chat/create-messages#create>).
    ///
    /// [Card builder](<https://addons.gsuite.google.com/uikit/builder>)
    #[prost(message, repeated, tag = "22")]
    pub cards_v2: ::prost::alloc::vec::Vec<CardWithId>,
    /// Output only. Annotations associated with the `text` in this message.
    #[prost(message, repeated, tag = "10")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// The thread the message belongs to. For example usage, see
    /// [Start or reply to a message
    /// thread](<https://developers.google.com/workspace/chat/create-messages#create-message-thread>).
    #[prost(message, optional, tag = "11")]
    pub thread: ::core::option::Option<Thread>,
    /// If your Chat app [authenticates as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
    /// the output populates the
    /// \[space\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces>)
    /// `name`.
    #[prost(message, optional, tag = "12")]
    pub space: ::core::option::Option<Space>,
    /// A plain-text description of the message's cards, used when the actual cards
    /// can't be displayed—for example, mobile notifications.
    #[prost(string, tag = "13")]
    pub fallback_text: ::prost::alloc::string::String,
    /// Input only. Parameters that a Chat app can use to configure how its
    /// response is posted.
    #[prost(message, optional, tag = "14")]
    pub action_response: ::core::option::Option<ActionResponse>,
    /// Output only. Plain-text body of the message with all Chat app mentions
    /// stripped out.
    #[prost(string, tag = "15")]
    pub argument_text: ::prost::alloc::string::String,
    /// Output only. Slash command information, if applicable.
    #[prost(message, optional, tag = "17")]
    pub slash_command: ::core::option::Option<SlashCommand>,
    /// User-uploaded attachment.
    #[prost(message, repeated, tag = "18")]
    pub attachment: ::prost::alloc::vec::Vec<Attachment>,
    /// Output only. A URL in `spaces.messages.text` that matches a link preview
    /// pattern. For more information, see [Preview
    /// links](<https://developers.google.com/workspace/chat/preview-links>).
    #[prost(message, optional, tag = "20")]
    pub matched_url: ::core::option::Option<MatchedUrl>,
    /// Output only. When `true`, the message is a response in a reply thread. When
    /// `false`, the message is visible in the space's top-level conversation as
    /// either the first message of a thread or a message with no threaded replies.
    ///
    /// If the space doesn't support reply in threads, this field is always
    /// `false`.
    #[prost(bool, tag = "25")]
    pub thread_reply: bool,
    /// Optional. A custom ID for the message. You can use field to identify a
    /// message, or to get, delete, or update a message. To set a custom ID,
    /// specify the
    /// \[`messageId`\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages/create#body.QUERY_PARAMETERS.message_id>)
    /// field when you create the message. For details, see [Name a
    /// message](<https://developers.google.com/workspace/chat/create-messages#name_a_created_message>).
    #[prost(string, tag = "32")]
    pub client_assigned_message_id: ::prost::alloc::string::String,
    /// Output only. The list of emoji reaction summaries on the message.
    #[prost(message, repeated, tag = "33")]
    pub emoji_reaction_summaries: ::prost::alloc::vec::Vec<EmojiReactionSummary>,
    /// Immutable. Input for creating a message, otherwise output only. The user
    /// that can view the message. When set, the message is private and only
    /// visible to the specified user and the Chat app. Link previews and
    /// attachments aren't supported for private messages.
    ///
    /// Only Chat apps can send private messages. If your Chat app [authenticates
    /// as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>)
    /// to send a message, the message can't be private and must omit this field.
    ///
    /// For details, see [Send private messages to Google Chat
    /// users](<https://developers.google.com/workspace/chat/private-messages>).
    #[prost(message, optional, tag = "36")]
    pub private_message_viewer: ::core::option::Option<User>,
    /// Output only. Information about a deleted message. A message is deleted when
    /// `delete_time` is set.
    #[prost(message, optional, tag = "38")]
    pub deletion_metadata: ::core::option::Option<DeletionMetadata>,
    /// Output only. Information about a message that's quoted by a Google Chat
    /// user in a space. Google Chat users can quote a message to reply to it.
    #[prost(message, optional, tag = "39")]
    pub quoted_message_metadata: ::core::option::Option<QuotedMessageMetadata>,
    /// Output only. GIF images that are attached to the message.
    #[prost(message, repeated, tag = "42")]
    pub attached_gifs: ::prost::alloc::vec::Vec<AttachedGif>,
    /// One or more interactive widgets that appear at the bottom of a message.
    /// You can add accessory widgets to messages that contain text, cards, or both
    /// text and cards. Not supported for messages that contain dialogs. For
    /// details, see [Add interactive widgets at the bottom of a
    /// message](<https://developers.google.com/workspace/chat/create-messages#add-accessory-widgets>).
    ///
    /// Creating a message with accessory widgets requires [app
    /// authentication]
    /// (<https://developers.google.com/workspace/chat/authenticate-authorize-chat-app>).
    #[prost(message, repeated, tag = "44")]
    pub accessory_widgets: ::prost::alloc::vec::Vec<AccessoryWidget>,
}
/// A GIF image that's specified by a URL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachedGif {
    /// Output only. The URL that hosts the GIF image.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Information about a quoted message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotedMessageMetadata {
    /// Output only. Resource name of the quoted message.
    ///
    /// Format: `spaces/{space}/messages/{message}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the quoted message was created or when the
    /// quoted message was last updated.
    #[prost(message, optional, tag = "2")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A thread in a Google Chat space. For example usage, see
/// [Start or reply to a message
/// thread](<https://developers.google.com/workspace/chat/create-messages#create-message-thread>).
///
/// If you specify a thread when creating a message, you can set the
/// \[`messageReplyOption`\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages/create#messagereplyoption>)
/// field to determine what happens if no matching thread is found.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thread {
    /// Output only. Resource name of the thread.
    ///
    /// Example: `spaces/{space}/threads/{thread}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Input for creating or updating a thread. Otherwise, output only.
    /// ID for the thread. Supports up to 4000 characters.
    ///
    /// This ID is unique to the Chat app that sets it. For example, if
    /// multiple Chat apps create a message using the same thread key,
    /// the messages are posted in different threads. To reply in a
    /// thread created by a person or another Chat app, specify the thread `name`
    /// field instead.
    #[prost(string, tag = "3")]
    pub thread_key: ::prost::alloc::string::String,
}
/// Parameters that a Chat app can use to configure how its response is posted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionResponse {
    /// Input only. The type of Chat app response.
    #[prost(enumeration = "action_response::ResponseType", tag = "1")]
    pub r#type: i32,
    /// Input only. URL for users to authenticate or configure. (Only for
    /// `REQUEST_CONFIG` response types.)
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// Input only. A response to an interaction event related to a
    /// \[dialog\](<https://developers.google.com/workspace/chat/dialogs>). Must be
    /// accompanied by `ResponseType.Dialog`.
    #[prost(message, optional, tag = "3")]
    pub dialog_action: ::core::option::Option<DialogAction>,
    /// Input only. The response of the updated widget.
    #[prost(message, optional, tag = "4")]
    pub updated_widget: ::core::option::Option<action_response::UpdatedWidget>,
}
/// Nested message and enum types in `ActionResponse`.
pub mod action_response {
    /// List of widget autocomplete results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelectionItems {
        /// An array of the SelectionItem objects.
        #[prost(message, repeated, tag = "1")]
        pub items: ::prost::alloc::vec::Vec<
            super::super::super::apps::card::v1::selection_input::SelectionItem,
        >,
    }
    /// The response of the updated widget.
    /// Used to provide autocomplete options for a widget.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdatedWidget {
        /// The ID of the updated widget. The ID must match the one for the
        /// widget that triggered the update request.
        #[prost(string, tag = "2")]
        pub widget: ::prost::alloc::string::String,
        /// The widget updated in response to a user action.
        #[prost(oneof = "updated_widget::UpdatedWidget", tags = "1")]
        pub updated_widget: ::core::option::Option<updated_widget::UpdatedWidget>,
    }
    /// Nested message and enum types in `UpdatedWidget`.
    pub mod updated_widget {
        /// The widget updated in response to a user action.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum UpdatedWidget {
            /// List of widget autocomplete results
            #[prost(message, tag = "1")]
            Suggestions(super::SelectionItems),
        }
    }
    /// The type of Chat app response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseType {
        /// Default type that's handled as `NEW_MESSAGE`.
        TypeUnspecified = 0,
        /// Post as a new message in the topic.
        NewMessage = 1,
        /// Update the Chat app's message. This is only permitted on a `CARD_CLICKED`
        /// event where the message sender type is `BOT`.
        UpdateMessage = 2,
        /// Update the cards on a user's message. This is only permitted as a
        /// response to a `MESSAGE` event with a matched url, or a `CARD_CLICKED`
        /// event where the message sender type is `HUMAN`. Text is ignored.
        UpdateUserMessageCards = 6,
        /// Privately ask the user for additional authentication or configuration.
        RequestConfig = 3,
        /// Presents a
        /// \[dialog\](<https://developers.google.com/workspace/chat/dialogs>).
        Dialog = 4,
        /// Widget text autocomplete options query.
        UpdateWidget = 7,
    }
}
/// One or more interactive widgets that appear at the bottom of a message. For
/// details, see [Add interactive widgets at the bottom of a
/// message](<https://developers.google.com/workspace/chat/create-messages#add-accessory-widgets>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessoryWidget {
    /// The type of action.
    #[prost(oneof = "accessory_widget::Action", tags = "1")]
    pub action: ::core::option::Option<accessory_widget::Action>,
}
/// Nested message and enum types in `AccessoryWidget`.
pub mod accessory_widget {
    /// The type of action.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// A list of buttons.
        #[prost(message, tag = "1")]
        ButtonList(super::super::super::apps::card::v1::ButtonList),
    }
}
/// Request to get a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageRequest {
    /// Required. Resource name of the message.
    ///
    /// Format: `spaces/{space}/messages/{message}`
    ///
    /// If you've set a custom ID for your message, you can use the value from the
    /// `clientAssignedMessageId` field for `{message}`. For details, see [Name a
    /// message]
    /// (<https://developers.google.com/workspace/chat/create-messages#name_a_created_message>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to delete a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMessageRequest {
    /// Required. Resource name of the message.
    ///
    /// Format: `spaces/{space}/messages/{message}`
    ///
    /// If you've set a custom ID for your message, you can use the value from the
    /// `clientAssignedMessageId` field for `{message}`. For details, see [Name a
    /// message]
    /// (<https://developers.google.com/workspace/chat/create-messages#name_a_created_message>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// When `true`, deleting a message also deletes its threaded replies. When
    /// `false`, if a message has threaded replies, deletion fails.
    ///
    /// Only applies when [authenticating as a
    /// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>).
    /// Has no effect when [authenticating as a Chat app]
    /// (<https://developers.google.com/workspace/chat/authenticate-authorize-chat-app>).
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request to update a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMessageRequest {
    /// Required. Message with fields updated.
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Message>,
    /// Required. The field paths to update. Separate multiple values with commas
    /// or use `*` to update all field paths.
    ///
    /// Currently supported field paths:
    ///
    /// - `text`
    ///
    /// - `attachment`
    ///
    /// - `cards` (Requires [app
    /// authentication](/chat/api/guides/auth/service-accounts).)
    ///
    /// - `cards_v2`  (Requires [app
    /// authentication](/chat/api/guides/auth/service-accounts).)
    ///
    /// - `accessory_widgets`  (Requires [app
    /// authentication](/chat/api/guides/auth/service-accounts).)
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. If `true` and the message isn't found, a new message is created
    /// and `updateMask` is ignored. The specified message ID must be
    /// \[client-assigned\](<https://developers.google.com/workspace/chat/create-messages#name_a_created_message>)
    /// or the request fails.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Creates a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMessageRequest {
    /// Required. The resource name of the space in which to create a message.
    ///
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Message body.
    #[prost(message, optional, tag = "4")]
    pub message: ::core::option::Option<Message>,
    /// Optional. Deprecated: Use
    /// \[thread.thread_key][google.chat.v1.Thread.thread_key\] instead. ID for the
    /// thread. Supports up to 4000 characters. To start or add to a thread, create
    /// a message and specify a `threadKey` or the
    /// \[thread.name][google.chat.v1.Thread.name\]. For example usage, see [Start or
    /// reply to a message
    /// thread](<https://developers.google.com/workspace/chat/create-messages#create-message-thread>).
    #[deprecated]
    #[prost(string, tag = "6")]
    pub thread_key: ::prost::alloc::string::String,
    /// Optional. A unique request ID for this message. Specifying an existing
    /// request ID returns the message created with that ID instead of creating a
    /// new message.
    #[prost(string, tag = "7")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. Specifies whether a message starts a thread or replies to one.
    /// Only supported in named spaces.
    #[prost(enumeration = "create_message_request::MessageReplyOption", tag = "8")]
    pub message_reply_option: i32,
    /// Optional. A custom ID for a message. Lets Chat apps get, update, or delete
    /// a message without needing to store the system-assigned ID in the message's
    /// resource name (represented in the message `name` field).
    ///
    /// The value for this field must meet the following requirements:
    ///
    /// * Begins with `client-`. For example, `client-custom-name` is a valid
    ///   custom ID, but `custom-name` is not.
    /// * Contains up to 63 characters and only lowercase letters, numbers, and
    ///   hyphens.
    /// * Is unique within a space. A Chat app can't use the same custom ID for
    /// different messages.
    ///
    /// For details, see [Name a
    /// message](<https://developers.google.com/workspace/chat/create-messages#name_a_created_message>).
    #[prost(string, tag = "9")]
    pub message_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CreateMessageRequest`.
pub mod create_message_request {
    /// Specifies how to reply to a message.
    /// More states might be added in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageReplyOption {
        /// Default. Starts a new thread. Using this option ignores any [thread
        /// ID]\[google.chat.v1.Thread.name\] or
        /// \[`thread_key`][google.chat.v1.Thread.thread_key\] that's included.
        Unspecified = 0,
        /// Creates the message as a reply to the thread specified by [thread
        /// ID]\[google.chat.v1.Thread.name\] or
        /// \[`thread_key`][google.chat.v1.Thread.thread_key\]. If it fails, the
        /// message starts a new thread instead.
        ReplyMessageFallbackToNewThread = 1,
        /// Creates the message as a reply to the thread specified by [thread
        /// ID]\[google.chat.v1.Thread.name\] or
        /// \[`thread_key`][google.chat.v1.Thread.thread_key\]. If a new `thread_key`
        /// is used, a new thread is created. If the message creation fails, a
        /// `NOT_FOUND` error is returned instead.
        ReplyMessageOrFail = 2,
    }
}
/// Lists messages in the specified space, that the user is a member of.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMessagesRequest {
    /// Required. The resource name of the space to list messages from.
    ///
    /// Format: `spaces/{space}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of messages returned. The service might return fewer
    /// messages than this value.
    ///
    /// If unspecified, at most 25 are returned.
    ///
    /// The maximum value is 1000. If you use a value more than 1000, it's
    /// automatically changed to 1000.
    ///
    /// Negative values return an `INVALID_ARGUMENT` error.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional, if resuming from a previous query.
    ///
    /// A page token received from a previous list messages call. Provide this
    /// parameter to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided should match the call that
    /// provided the page token. Passing different values to the other parameters
    /// might lead to unexpected results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A query filter.
    ///
    /// You can filter messages by date (`create_time`) and thread (`thread.name`).
    ///
    /// To filter messages by the date they were created, specify the `create_time`
    /// with a timestamp in \[RFC-3339\](<https://www.rfc-editor.org/rfc/rfc3339>)
    /// format and double quotation marks. For example,
    /// `"2023-04-21T11:30:00-04:00"`. You can use the greater than operator `>` to
    /// list messages that were created after a timestamp, or the less than
    /// operator `<` to list messages that were created before a timestamp. To
    /// filter messages within a time interval, use the `AND` operator between two
    /// timestamps.
    ///
    /// To filter by thread, specify the `thread.name`, formatted as
    /// `spaces/{space}/threads/{thread}`. You can only specify one
    /// `thread.name` per query.
    ///
    /// To filter by both thread and date, use the `AND` operator in your query.
    ///
    /// For example, the following queries are valid:
    ///
    /// ```
    /// create_time > "2012-04-21T11:30:00-04:00"
    ///
    /// create_time > "2012-04-21T11:30:00-04:00" AND
    ///   thread.name = spaces/AAAAAAAAAAA/threads/123
    ///
    /// create_time > "2012-04-21T11:30:00+00:00" AND
    ///
    /// create_time < "2013-01-01T00:00:00+00:00" AND
    ///   thread.name = spaces/AAAAAAAAAAA/threads/123
    ///
    /// thread.name = spaces/AAAAAAAAAAA/threads/123
    /// ```
    ///
    /// Invalid queries are rejected by the server with an `INVALID_ARGUMENT`
    /// error.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional, if resuming from a previous query.
    ///
    /// How the list of messages is ordered. Specify a value to order by an
    /// ordering operation. Valid ordering operation values are as follows:
    ///
    /// - `ASC` for ascending.
    ///
    /// - `DESC` for descending.
    ///
    /// The default ordering is `create_time ASC`.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Whether to include deleted messages. Deleted messages include deleted time
    /// and metadata about their deletion, but message content is unavailable.
    #[prost(bool, tag = "6")]
    pub show_deleted: bool,
}
/// Response message for listing messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMessagesResponse {
    /// List of messages.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
    /// You can send a token as `pageToken` to retrieve the next page of
    /// results. If empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Contains a
/// \[dialog\](<https://developers.google.com/workspace/chat/dialogs>) and request
/// status code.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogAction {
    /// Input only. Status for a request to either invoke or submit a
    /// \[dialog\](<https://developers.google.com/workspace/chat/dialogs>). Displays
    /// a status and message to users, if necessary.
    /// For example, in case of an error or success.
    #[prost(message, optional, tag = "2")]
    pub action_status: ::core::option::Option<ActionStatus>,
    /// Action to perform.
    #[prost(oneof = "dialog_action::Action", tags = "1")]
    pub action: ::core::option::Option<dialog_action::Action>,
}
/// Nested message and enum types in `DialogAction`.
pub mod dialog_action {
    /// Action to perform.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Input only.
        /// \[Dialog\](<https://developers.google.com/workspace/chat/dialogs>) for the
        /// request.
        #[prost(message, tag = "1")]
        Dialog(super::Dialog),
    }
}
/// Wrapper around the card body of the dialog.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dialog {
    /// Input only. Body of the dialog, which is rendered in a modal.
    /// Google Chat apps don't support the following card entities:
    /// `DateTimePicker`, `OnChangeAction`.
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<super::super::apps::card::v1::Card>,
}
/// A
/// \[card\](<https://developers.google.com/workspace/chat/api/reference/rest/v1/cards>)
/// in a Google Chat message.
///
/// Only Chat apps can create cards. If your Chat app [authenticates as a
/// user](<https://developers.google.com/workspace/chat/authenticate-authorize-chat-user>),
/// the message can't contain cards.
///
/// [Card builder](<https://addons.gsuite.google.com/uikit/builder>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardWithId {
    /// Required if the message contains multiple cards. A unique identifier for
    /// a card in a message.
    #[prost(string, tag = "1")]
    pub card_id: ::prost::alloc::string::String,
    /// A card. Maximum size is 32 KB.
    #[prost(message, optional, tag = "2")]
    pub card: ::core::option::Option<super::super::apps::card::v1::Card>,
}
/// A user's read state within a space, used to identify read and unread
/// messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpaceReadState {
    /// Resource name of the space read state.
    ///
    /// Format: `users/{user}/spaces/{space}/spaceReadState`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The time when the user's space read state was updated. Usually
    /// this corresponds with either the timestamp of the last read message, or a
    /// timestamp specified by the user to mark the last read position in a space.
    #[prost(message, optional, tag = "2")]
    pub last_read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for GetSpaceReadState API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpaceReadStateRequest {
    /// Required. Resource name of the space read state to retrieve.
    ///
    /// Only supports getting read state for the calling user.
    ///
    /// To refer to the calling user, set one of the following:
    ///
    /// - The `me` alias. For example, `users/me/spaces/{space}/spaceReadState`.
    ///
    /// - Their Workspace email address. For example,
    /// `users/user@example.com/spaces/{space}/spaceReadState`.
    ///
    /// - Their user id. For example,
    /// `users/123456789/spaces/{space}/spaceReadState`.
    ///
    /// Format: users/{user}/spaces/{space}/spaceReadState
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateSpaceReadState API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpaceReadStateRequest {
    /// Required. The space read state and fields to update.
    ///
    /// Only supports updating read state for the calling user.
    ///
    /// To refer to the calling user, set one of the following:
    ///
    /// - The `me` alias. For example, `users/me/spaces/{space}/spaceReadState`.
    ///
    /// - Their Workspace email address. For example,
    /// `users/user@example.com/spaces/{space}/spaceReadState`.
    ///
    /// - Their user id. For example,
    /// `users/123456789/spaces/{space}/spaceReadState`.
    ///
    /// Format: users/{user}/spaces/{space}/spaceReadState
    #[prost(message, optional, tag = "1")]
    pub space_read_state: ::core::option::Option<SpaceReadState>,
    /// Required. The field paths to update. Currently supported field paths:
    ///
    /// - `last_read_time`
    ///
    /// When the `last_read_time` is before the latest message create time, the
    /// space appears as unread in the UI.
    ///
    /// To mark the space as read, set `last_read_time` to any value later (larger)
    /// than the latest message create time. The `last_read_time` is coerced to
    /// match the latest message create time. Note that the space read state only
    /// affects the read state of messages that are visible in the space's
    /// top-level conversation. Replies in threads are unaffected by this
    /// timestamp, and instead rely on the thread read state.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to create a space and add specified users to it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpSpaceRequest {
    /// Required. The `Space.spaceType` field is required.
    ///
    /// To create a space, set `Space.spaceType` to `SPACE` and set
    /// `Space.displayName`. If you receive the error message `ALREADY_EXISTS` when
    /// setting up a space, try a different `displayName`. An existing space
    /// within the Google Workspace organization might already use this display
    /// name.
    ///
    /// To create a group chat, set `Space.spaceType` to
    /// `GROUP_CHAT`. Don't set `Space.displayName`.
    ///
    /// To create a 1:1 conversation between humans,
    /// set `Space.spaceType` to `DIRECT_MESSAGE` and set
    /// `Space.singleUserBotDm` to `false`. Don't set `Space.displayName` or
    /// `Space.spaceDetails`.
    ///
    /// To create an 1:1 conversation between a human and the calling Chat app, set
    /// `Space.spaceType` to `DIRECT_MESSAGE` and
    /// `Space.singleUserBotDm` to `true`. Don't set `Space.displayName` or
    /// `Space.spaceDetails`.
    ///
    /// If a `DIRECT_MESSAGE` space already exists, that space is returned instead
    /// of creating a new space.
    #[prost(message, optional, tag = "1")]
    pub space: ::core::option::Option<Space>,
    /// Optional. A unique identifier for this request.
    /// A random UUID is recommended.
    /// Specifying an existing request ID returns the space created with that ID
    /// instead of creating a new space.
    /// Specifying an existing request ID from the same Chat app with a different
    /// authenticated user returns an error.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. The Google Chat users or groups to invite to join the space. Omit
    /// the calling user, as they are added automatically.
    ///
    /// The set currently allows up to 20 memberships (in addition to the caller).
    ///
    /// For human membership, the `Membership.member` field must contain a `user`
    /// with `name` populated (format: `users/{user}`) and `type` set to
    /// `User.Type.HUMAN`. You can only add human users when setting up a space
    /// (adding Chat apps is only supported for direct message setup with the
    /// calling app). You can also add members using the user's email as an alias
    /// for {user}. For example, the `user.name` can be `users/example@gmail.com`.
    /// To invite Gmail users or users from external Google Workspace domains,
    /// user's email must be used for `{user}`.
    ///
    /// For Google group membership, the `Membership.group_member` field must
    /// contain a `group` with `name` populated (format `groups/{group}`). You
    /// can only add Google groups when setting `Space.spaceType` to `SPACE`.
    ///
    /// Optional when setting `Space.spaceType` to `SPACE`.
    ///
    /// Required when setting `Space.spaceType` to `GROUP_CHAT`, along with at
    /// least two memberships.
    ///
    /// Required when setting `Space.spaceType` to `DIRECT_MESSAGE` with a human
    /// user, along with exactly one membership.
    ///
    /// Must be empty when creating a 1:1 conversation between a human and the
    /// calling Chat app (when setting `Space.spaceType` to
    /// `DIRECT_MESSAGE` and `Space.singleUserBotDm` to `true`).
    #[prost(message, repeated, tag = "4")]
    pub memberships: ::prost::alloc::vec::Vec<Membership>,
}
/// A user's read state within a thread, used to identify read and unread
/// messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreadReadState {
    /// Resource name of the thread read state.
    ///
    /// Format: `users/{user}/spaces/{space}/threads/{thread}/threadReadState`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The time when the user's thread read state was updated. Usually this
    /// corresponds with the timestamp of the last read message in a thread.
    #[prost(message, optional, tag = "2")]
    pub last_read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for GetThreadReadStateRequest API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetThreadReadStateRequest {
    /// Required. Resource name of the thread read state to retrieve.
    ///
    /// Only supports getting read state for the calling user.
    ///
    /// To refer to the calling user, set one of the following:
    ///
    /// - The `me` alias. For example,
    /// `users/me/spaces/{space}/threads/{thread}/threadReadState`.
    ///
    /// - Their Workspace email address. For example,
    /// `users/user@example.com/spaces/{space}/threads/{thread}/threadReadState`.
    ///
    /// - Their user id. For example,
    /// `users/123456789/spaces/{space}/threads/{thread}/threadReadState`.
    ///
    /// Format: users/{user}/spaces/{space}/threads/{thread}/threadReadState
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod chat_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enables developers to build Chat apps and"]
    #[doc = " integrations on Google Chat Platform."]
    #[derive(Debug, Clone)]
    pub struct ChatServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ChatServiceClient<T>
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
        ) -> ChatServiceClient<InterceptedService<T, F>>
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
            ChatServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a message in a Google Chat space. The maximum message size,"]
        #[doc = " including text and cards, is 32,000 bytes. For an example, see [Send a"]
        #[doc = " message](https://developers.google.com/workspace/chat/create-messages)."]
        #[doc = ""]
        #[doc = " Calling this method requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)"]
        #[doc = " and supports the following authentication types:"]
        #[doc = ""]
        #[doc = " - For text messages, user authentication or app authentication are"]
        #[doc = " supported."]
        #[doc = " - For card messages, only app authentication is supported. (Only Chat apps"]
        #[doc = " can create card messages.)"]
        pub async fn create_message(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMessageRequest>,
        ) -> Result<tonic::Response<super::Message>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/CreateMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists messages in a space that the caller is a member of, including"]
        #[doc = " messages from blocked members and spaces. For an example, see"]
        #[doc = " [List messages](/chat/api/guides/v1/messages/list)."]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn list_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMessagesRequest>,
        ) -> Result<tonic::Response<super::ListMessagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/ListMessages");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists memberships in a space. For an example, see [List users and Google"]
        #[doc = " Chat apps in a"]
        #[doc = " space](https://developers.google.com/workspace/chat/list-members). Listing"]
        #[doc = " memberships with [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " lists memberships in spaces that the Chat app has"]
        #[doc = " access to, but excludes Chat app memberships,"]
        #[doc = " including its own. Listing memberships with"]
        #[doc = " [User"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)"]
        #[doc = " lists memberships in spaces that the authenticated user has access to."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn list_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListMembershipsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/ListMemberships");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns details about a membership. For an example, see"]
        #[doc = " [Get details about a user's or Google Chat app's"]
        #[doc = " membership](https://developers.google.com/workspace/chat/get-members)."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn get_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMembershipRequest>,
        ) -> Result<tonic::Response<super::Membership>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/GetMembership");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns details about a message."]
        #[doc = " For an example, see [Get details about a"]
        #[doc = " message](https://developers.google.com/workspace/chat/get-messages)."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        #[doc = ""]
        #[doc = " Note: Might return a message from a blocked member or space."]
        pub async fn get_message(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMessageRequest>,
        ) -> Result<tonic::Response<super::Message>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/GetMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a message. There's a difference between the `patch` and `update`"]
        #[doc = " methods. The `patch`"]
        #[doc = " method uses a `patch` request while the `update` method uses a `put`"]
        #[doc = " request. We recommend using the `patch` method. For an example, see"]
        #[doc = " [Update a"]
        #[doc = " message](https://developers.google.com/workspace/chat/update-messages)."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        #[doc = " When using app authentication, requests can only update messages"]
        #[doc = " created by the calling Chat app."]
        pub async fn update_message(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMessageRequest>,
        ) -> Result<tonic::Response<super::Message>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/UpdateMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a message."]
        #[doc = " For an example, see [Delete a"]
        #[doc = " message](https://developers.google.com/workspace/chat/delete-messages)."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        #[doc = " When using app authentication, requests can only delete messages"]
        #[doc = " created by the calling Chat app."]
        pub async fn delete_message(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMessageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/DeleteMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the metadata of a message attachment. The attachment data is fetched"]
        #[doc = " using the [media"]
        #[doc = " API](https://developers.google.com/workspace/chat/api/reference/rest/v1/media/download)."]
        #[doc = " For an example, see"]
        #[doc = " [Get metadata about a message"]
        #[doc = " attachment](https://developers.google.com/workspace/chat/get-media-attachments)."]
        #[doc = " Requires [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)."]
        pub async fn get_attachment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAttachmentRequest>,
        ) -> Result<tonic::Response<super::Attachment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/GetAttachment");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Uploads an attachment. For an example, see"]
        #[doc = " [Upload media as a file"]
        #[doc = " attachment](https://developers.google.com/workspace/chat/upload-media-attachments)."]
        #[doc = " Requires user"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        #[doc = ""]
        #[doc = " You can upload attachments up to 200 MB. Certain file types aren't"]
        #[doc = " supported. For details, see [File types blocked by Google"]
        #[doc = " Chat](https://support.google.com/chat/answer/7651457?&co=GENIE.Platform%3DDesktop#File%20types%20blocked%20in%20Google%20Chat)."]
        pub async fn upload_attachment(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadAttachmentRequest>,
        ) -> Result<tonic::Response<super::UploadAttachmentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/UploadAttachment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists spaces the caller is a member of. Group chats and DMs aren't listed"]
        #[doc = " until the first message is sent. For an example, see"]
        #[doc = " [List"]
        #[doc = " spaces](https://developers.google.com/workspace/chat/list-spaces)."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        #[doc = ""]
        #[doc = " Lists spaces visible to the caller or authenticated user. Group chats"]
        #[doc = " and DMs aren't listed until the first message is sent."]
        #[doc = ""]
        #[doc = " To list all named spaces by Google Workspace organization, use the"]
        #[doc = " [`spaces.search()`](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/search)"]
        #[doc = " method using Workspace administrator privileges instead."]
        pub async fn list_spaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSpacesRequest>,
        ) -> Result<tonic::Response<super::ListSpacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/ListSpaces");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns details about a space. For an example, see"]
        #[doc = " [Get details about a"]
        #[doc = " space](https://developers.google.com/workspace/chat/get-spaces)."]
        #[doc = ""]
        #[doc = " Requires"]
        #[doc = " [authentication](https://developers.google.com/workspace/chat/authenticate-authorize)."]
        #[doc = " Supports"]
        #[doc = " [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)"]
        #[doc = " and [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn get_space(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpaceRequest>,
        ) -> Result<tonic::Response<super::Space>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/GetSpace");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a named space. Spaces grouped by topics aren't supported. For an"]
        #[doc = " example, see [Create a"]
        #[doc = " space](https://developers.google.com/workspace/chat/create-spaces)."]
        #[doc = ""]
        #[doc = "  If you receive the error message `ALREADY_EXISTS` when creating"]
        #[doc = "  a space, try a different `displayName`. An existing space within"]
        #[doc = "  the Google Workspace organization might already use this display name."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn create_space(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpaceRequest>,
        ) -> Result<tonic::Response<super::Space>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/CreateSpace");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a space and adds specified users to it. The calling user is"]
        #[doc = " automatically added to the space, and shouldn't be specified as a"]
        #[doc = " membership in the request. For an example, see"]
        #[doc = " [Set up a space with initial"]
        #[doc = " members](https://developers.google.com/workspace/chat/set-up-spaces)."]
        #[doc = ""]
        #[doc = " To specify the human members to add, add memberships with the appropriate"]
        #[doc = " `membership.member.name`. To add a human user, use `users/{user}`, where"]
        #[doc = " `{user}` can be the email address for the user. For users in the same"]
        #[doc = " Workspace organization `{user}` can also be the `id` for the person from"]
        #[doc = " the People API, or the `id` for the user in the Directory API. For example,"]
        #[doc = " if the People API Person profile ID for `user@example.com` is `123456789`,"]
        #[doc = " you can add the user to the space by setting the `membership.member.name`"]
        #[doc = " to `users/user@example.com` or `users/123456789`."]
        #[doc = ""]
        #[doc = " To specify the Google groups to add, add memberships with the"]
        #[doc = " appropriate `membership.group_member.name`. To add or invite a Google"]
        #[doc = " group, use `groups/{group}`, where `{group}` is the `id` for the group from"]
        #[doc = " the Cloud Identity Groups API. For example, you can use [Cloud Identity"]
        #[doc = " Groups lookup"]
        #[doc = " API](https://cloud.google.com/identity/docs/reference/rest/v1/groups/lookup)"]
        #[doc = " to retrieve the ID `123456789` for group email `group@example.com`, then"]
        #[doc = " you can add the group to the space by setting the"]
        #[doc = " `membership.group_member.name` to `groups/123456789`. Group email is not"]
        #[doc = " supported, and Google groups can only be added as members in named spaces."]
        #[doc = ""]
        #[doc = " For a named space or group chat, if the caller blocks, or is blocked"]
        #[doc = " by some members, or doesn't have permission to add some members, then"]
        #[doc = " those members aren't added to the created space."]
        #[doc = ""]
        #[doc = " To create a direct message (DM) between the calling user and another human"]
        #[doc = " user, specify exactly one membership to represent the human user. If"]
        #[doc = " one user blocks the other, the request fails and the DM isn't created."]
        #[doc = ""]
        #[doc = " To create a DM between the calling user and the calling app, set"]
        #[doc = " `Space.singleUserBotDm` to `true` and don't specify any memberships. You"]
        #[doc = " can only use this method to set up a DM with the calling app. To add the"]
        #[doc = " calling app as a member of a space or an existing DM between two human"]
        #[doc = " users, see"]
        #[doc = " [Invite or add a user or app to a"]
        #[doc = " space](https://developers.google.com/workspace/chat/create-members)."]
        #[doc = ""]
        #[doc = " If a DM already exists between two users, even when one user blocks the"]
        #[doc = " other at the time a request is made, then the existing DM is returned."]
        #[doc = ""]
        #[doc = " Spaces with threaded replies aren't supported. If you receive the error"]
        #[doc = " message `ALREADY_EXISTS` when setting up a space, try a different"]
        #[doc = " `displayName`. An existing space within the Google Workspace organization"]
        #[doc = " might already use this display name."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn set_up_space(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUpSpaceRequest>,
        ) -> Result<tonic::Response<super::Space>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/SetUpSpace");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a space. For an example, see"]
        #[doc = " [Update a"]
        #[doc = " space](https://developers.google.com/workspace/chat/update-spaces)."]
        #[doc = ""]
        #[doc = " If you're updating the `displayName` field and receive the error message"]
        #[doc = " `ALREADY_EXISTS`, try a different display name.. An existing space within"]
        #[doc = " the Google Workspace organization might already use this display name."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn update_space(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpaceRequest>,
        ) -> Result<tonic::Response<super::Space>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/UpdateSpace");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a named space. Always performs a cascading delete, which means"]
        #[doc = " that the space's child resources—like messages posted in the space and"]
        #[doc = " memberships in the space—are also deleted. For an example, see"]
        #[doc = " [Delete a"]
        #[doc = " space](https://developers.google.com/workspace/chat/delete-spaces)."]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)"]
        #[doc = " from a user who has permission to delete the space."]
        pub async fn delete_space(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSpaceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/DeleteSpace");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Completes the"]
        #[doc = " [import process](https://developers.google.com/workspace/chat/import-data)"]
        #[doc = " for the specified space and makes it visible to users."]
        #[doc = " Requires app authentication and domain-wide delegation. For more"]
        #[doc = " information, see [Authorize Google Chat apps to import"]
        #[doc = " data](https://developers.google.com/workspace/chat/authorize-import)."]
        pub async fn complete_import_space(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteImportSpaceRequest>,
        ) -> Result<tonic::Response<super::CompleteImportSpaceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/CompleteImportSpace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the existing direct message with the specified user. If no direct"]
        #[doc = " message space is found, returns a `404 NOT_FOUND` error. For an example,"]
        #[doc = " see"]
        #[doc = " [Find a direct message](/chat/api/guides/v1/spaces/find-direct-message)."]
        #[doc = ""]
        #[doc = " With [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user),"]
        #[doc = " returns the direct message space between the specified user and the"]
        #[doc = " authenticated user."]
        #[doc = ""]
        #[doc = " With [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app),"]
        #[doc = " returns the direct message space between the specified user and the calling"]
        #[doc = " Chat app."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)"]
        #[doc = " or [app"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app)."]
        pub async fn find_direct_message(
            &mut self,
            request: impl tonic::IntoRequest<super::FindDirectMessageRequest>,
        ) -> Result<tonic::Response<super::Space>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/FindDirectMessage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a human membership or app membership for the calling app. Creating"]
        #[doc = " memberships for other apps isn't supported. For an example, see"]
        #[doc = " [Invite or add a user or a Google Chat app to a"]
        #[doc = " space](https://developers.google.com/workspace/chat/create-members)."]
        #[doc = " When creating a membership, if the specified member has their auto-accept"]
        #[doc = " policy turned off, then they're invited, and must accept the space"]
        #[doc = " invitation before joining. Otherwise, creating a membership adds the member"]
        #[doc = " directly to the specified space. Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        #[doc = ""]
        #[doc = " To specify the member to add, set the `membership.member.name` for the"]
        #[doc = " human or app member, or set the `membership.group_member.name` for the"]
        #[doc = " group member."]
        #[doc = ""]
        #[doc = " - To add the calling app to a space or a direct message between two human"]
        #[doc = "   users, use `users/app`. Unable to add other"]
        #[doc = "   apps to the space."]
        #[doc = ""]
        #[doc = " - To add a human user, use `users/{user}`, where `{user}` can be the email"]
        #[doc = " address for the user. For users in the same Workspace organization `{user}`"]
        #[doc = " can also be the `id` for the person from the People API, or the `id` for"]
        #[doc = " the user in the Directory API. For example, if the People API Person"]
        #[doc = " profile ID for `user@example.com` is `123456789`, you can add the user to"]
        #[doc = " the space by setting the `membership.member.name` to"]
        #[doc = " `users/user@example.com` or `users/123456789`."]
        #[doc = ""]
        #[doc = " - To add or invite a Google group in a named space, use"]
        #[doc = " `groups/{group}`, where `{group}` is the `id` for the group from the Cloud"]
        #[doc = " Identity Groups API. For example, you can use [Cloud Identity Groups lookup"]
        #[doc = " API](https://cloud.google.com/identity/docs/reference/rest/v1/groups/lookup)"]
        #[doc = " to retrieve the ID `123456789` for group email `group@example.com`, then"]
        #[doc = " you can add or invite the group to a named space by setting the"]
        #[doc = " `membership.group_member.name` to `groups/123456789`. Group email is not"]
        #[doc = " supported, and Google groups can only be added as members in named spaces."]
        pub async fn create_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMembershipRequest>,
        ) -> Result<tonic::Response<super::Membership>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/CreateMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a membership. For an example, see [Update a user's membership in"]
        #[doc = " a space](https://developers.google.com/workspace/chat/update-members)."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn update_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMembershipRequest>,
        ) -> Result<tonic::Response<super::Membership>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/UpdateMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a membership. For an example, see"]
        #[doc = " [Remove a user or a Google Chat app from a"]
        #[doc = " space](https://developers.google.com/workspace/chat/delete-members)."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn delete_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMembershipRequest>,
        ) -> Result<tonic::Response<super::Membership>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/DeleteMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a reaction and adds it to a message. Only unicode emojis are"]
        #[doc = " supported. For an example, see"]
        #[doc = " [Add a reaction to a"]
        #[doc = " message](https://developers.google.com/workspace/chat/create-reactions)."]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn create_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReactionRequest>,
        ) -> Result<tonic::Response<super::Reaction>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/CreateReaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists reactions to a message. For an example, see"]
        #[doc = " [List reactions for a"]
        #[doc = " message](https://developers.google.com/workspace/chat/list-reactions)."]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn list_reactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReactionsRequest>,
        ) -> Result<tonic::Response<super::ListReactionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/ListReactions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a reaction to a message. Only unicode emojis are supported."]
        #[doc = " For an example, see"]
        #[doc = " [Delete a"]
        #[doc = " reaction](https://developers.google.com/workspace/chat/delete-reactions)."]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn delete_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReactionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.chat.v1.ChatService/DeleteReaction");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns details about a user's read state within a space, used to identify"]
        #[doc = " read and unread messages. For an example, see [Get details about a user's"]
        #[doc = " space read"]
        #[doc = " state](https://developers.google.com/workspace/chat/get-space-read-state)."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn get_space_read_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpaceReadStateRequest>,
        ) -> Result<tonic::Response<super::SpaceReadState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/GetSpaceReadState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a user's read state within a space, used to identify read and"]
        #[doc = " unread messages. For an example, see [Update a user's space read"]
        #[doc = " state](https://developers.google.com/workspace/chat/update-space-read-state)."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn update_space_read_state(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpaceReadStateRequest>,
        ) -> Result<tonic::Response<super::SpaceReadState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/UpdateSpaceReadState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns details about a user's read state within a thread, used to identify"]
        #[doc = " read and unread messages. For an example, see [Get details about a user's"]
        #[doc = " thread read"]
        #[doc = " state](https://developers.google.com/workspace/chat/get-thread-read-state)."]
        #[doc = ""]
        #[doc = " Requires [user"]
        #[doc = " authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user)."]
        pub async fn get_thread_read_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetThreadReadStateRequest>,
        ) -> Result<tonic::Response<super::ThreadReadState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chat.v1.ChatService/GetThreadReadState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
