/// Upload reference for media files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRef {
    /// Required.
    #[prost(oneof = "upload_ref::FileSource", tags = "1")]
    pub file_source: ::core::option::Option<upload_ref::FileSource>,
}
/// Nested message and enum types in `UploadRef`.
pub mod upload_ref {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FileSource {
        /// An upload reference should be unique for each user. It follows
        /// the form:
        /// "<https://streetviewpublish.googleapis.com/media/user/{account_id}/photo/{upload_reference}">
        #[prost(string, tag = "1")]
        UploadUrl(::prost::alloc::string::String),
    }
}
/// Identifier for a \[Photo][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoId {
    /// A unique identifier for a photo.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Level information containing level number and its corresponding name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// Optional. Floor number, used for ordering. 0 indicates the ground level, 1 indicates
    /// the first level above ground level, -1 indicates the first level under
    /// ground level. Non-integer values are OK.
    #[prost(double, tag = "1")]
    pub number: f64,
    /// Required. A name assigned to this Level, restricted to 3 characters.
    /// Consider how the elevator buttons would be labeled for this level if there
    /// was an elevator.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Raw pose measurement for an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pose {
    /// Latitude and longitude pair of the pose, as explained here:
    /// <https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/LatLng>
    /// When creating a \[Photo][google.streetview.publish.v1.Photo\], if the
    /// latitude and longitude pair are not provided, the geolocation from the
    /// exif header is used. A latitude and longitude pair not provided in the
    /// photo or exif header causes the photo process to fail.
    #[prost(message, optional, tag = "1")]
    pub lat_lng_pair: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Altitude of the pose in meters above WGS84 ellipsoid.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "2")]
    pub altitude: f64,
    /// The following pose parameters pertain to the center of the photo. They
    /// match <https://developers.google.com/streetview/spherical-metadata.>
    /// Compass heading, measured at the center of the photo in degrees clockwise
    /// from North. Value must be >=0 and <360. NaN indicates an unmeasured
    /// quantity.
    #[prost(double, tag = "3")]
    pub heading: f64,
    /// Pitch, measured at the center of the photo in degrees. Value must be >=-90
    /// and <= 90. A value of -90 means looking directly down, and a value of 90
    /// means looking directly up.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "4")]
    pub pitch: f64,
    /// Roll, measured in degrees. Value must be >= 0 and <360. A value of 0
    /// means level with the horizon.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "5")]
    pub roll: f64,
    /// Time of the GPS record since UTC epoch.
    #[prost(message, optional, tag = "6")]
    pub gps_record_timestamp_unix_epoch: ::core::option::Option<::prost_types::Timestamp>,
    /// Level (the floor in a building) used to configure vertical navigation.
    #[prost(message, optional, tag = "7")]
    pub level: ::core::option::Option<Level>,
    /// The estimated horizontal accuracy of this pose in meters with 68%
    /// confidence (one standard deviation). For example, on Android, this value is
    /// available from this method:
    /// <https://developer.android.com/reference/android/location/Location#getAccuracy(>).
    /// Other platforms have different methods of obtaining similar accuracy
    /// estimations.
    #[prost(float, tag = "9")]
    pub accuracy_meters: f32,
}
/// IMU data from the device sensors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Imu {
    /// The accelerometer measurements in meters/sec^2 with increasing timestamps
    /// from devices.
    #[prost(message, repeated, tag = "1")]
    pub accel_mpsps: ::prost::alloc::vec::Vec<imu::Measurement3d>,
    /// The gyroscope measurements in radians/sec with increasing timestamps from
    /// devices.
    #[prost(message, repeated, tag = "2")]
    pub gyro_rps: ::prost::alloc::vec::Vec<imu::Measurement3d>,
    /// The magnetometer measurements of the magnetic field in microtesla (uT) with
    /// increasing timestamps from devices.
    #[prost(message, repeated, tag = "3")]
    pub mag_ut: ::prost::alloc::vec::Vec<imu::Measurement3d>,
}
/// Nested message and enum types in `Imu`.
pub mod imu {
    /// A Generic 3d measurement sample.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Measurement3d {
        /// The timestamp of the IMU measurement.
        #[prost(message, optional, tag = "1")]
        pub capture_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The sensor measurement in the x axis.
        #[prost(float, tag = "2")]
        pub x: f32,
        /// The sensor measurement in the y axis.
        #[prost(float, tag = "3")]
        pub y: f32,
        /// The sensor measurement in the z axis.
        #[prost(float, tag = "4")]
        pub z: f32,
    }
}
/// Place metadata for an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Place {
    /// Place identifier, as described in
    /// <https://developers.google.com/places/place-id.>
    #[prost(string, tag = "1")]
    pub place_id: ::prost::alloc::string::String,
    /// Output only. The name of the place, localized to the language_code.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The language_code that the name is localized with. This should
    /// be the language_code specified in the request, but may be a fallback.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// A connection is the link from a source photo to a destination photo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Required. The destination of the connection from the containing photo to
    /// another photo.
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<PhotoId>,
}
/// Photo is used to store 360 photos along with photo metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Photo {
    /// Required. Output only. Required when updating a photo. Output only when creating a photo.
    /// Identifier for the photo, which is unique among all photos in
    /// Google.
    #[prost(message, optional, tag = "1")]
    pub photo_id: ::core::option::Option<PhotoId>,
    /// Input only. Required when creating a photo. Input only. The resource URL where the
    /// photo bytes are uploaded to.
    #[prost(message, optional, tag = "2")]
    pub upload_reference: ::core::option::Option<UploadRef>,
    /// Output only. The download URL for the photo bytes. This field is set only
    /// when
    /// \[GetPhotoRequest.view][google.streetview.publish.v1.GetPhotoRequest.view\]
    /// is set to
    /// \[PhotoView.INCLUDE_DOWNLOAD_URL][google.streetview.publish.v1.PhotoView.INCLUDE_DOWNLOAD_URL\].
    #[prost(string, tag = "3")]
    pub download_url: ::prost::alloc::string::String,
    /// Output only. The thumbnail URL for showing a preview of the given photo.
    #[prost(string, tag = "9")]
    pub thumbnail_url: ::prost::alloc::string::String,
    /// Output only. The share link for the photo.
    #[prost(string, tag = "11")]
    pub share_link: ::prost::alloc::string::String,
    /// Optional. Pose of the photo.
    #[prost(message, optional, tag = "4")]
    pub pose: ::core::option::Option<Pose>,
    /// Optional. Connections to other photos. A connection represents the link from this
    /// photo to another photo.
    #[prost(message, repeated, tag = "5")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// Optional. Absolute time when the photo was captured.
    /// When the photo has no exif timestamp, this is used to set a timestamp in
    /// the photo metadata.
    #[prost(message, optional, tag = "6")]
    pub capture_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the image was uploaded.
    #[prost(message, optional, tag = "14")]
    pub upload_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Places where this photo belongs.
    #[prost(message, repeated, tag = "7")]
    pub places: ::prost::alloc::vec::Vec<Place>,
    /// Output only. View count of the photo.
    #[prost(int64, tag = "10")]
    pub view_count: i64,
    /// Output only. Status of rights transfer on this photo.
    #[prost(enumeration = "photo::TransferStatus", tag = "12")]
    pub transfer_status: i32,
    /// Output only. Status in Google Maps, whether this photo was published or rejected.
    #[prost(enumeration = "photo::MapsPublishStatus", tag = "13")]
    pub maps_publish_status: i32,
}
/// Nested message and enum types in `Photo`.
pub mod photo {
    /// Status of rights transfer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransferStatus {
        /// The status of this transfer is unspecified.
        Unknown = 0,
        /// This photo has never been in a transfer.
        NeverTransferred = 1,
        /// This photo transfer has been initiated, but the receiver has not yet
        /// responded.
        Pending = 2,
        /// The photo transfer has been completed, and this photo has been
        /// transferred to the recipient.
        Completed = 3,
        /// The recipient rejected this photo transfer.
        Rejected = 4,
        /// The photo transfer expired before the recipient took any action.
        Expired = 5,
        /// The sender cancelled this photo transfer.
        Cancelled = 6,
        /// The recipient owns this photo due to a rights transfer.
        ReceivedViaTransfer = 7,
    }
    /// Publication status of the photo in Google Maps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MapsPublishStatus {
        /// The status of the photo is unknown.
        UnspecifiedMapsPublishStatus = 0,
        /// The photo is published to the public through Google Maps.
        Published = 1,
        /// The photo has been rejected for an unknown reason.
        RejectedUnknown = 2,
    }
}
/// A sequence of 360 photos along with metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoSequence {
    /// Output only. Unique identifier for the photo sequence.
    /// This also acts as a long running operation ID if uploading is performed
    /// asynchronously.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Photos with increasing timestamps.
    #[prost(message, repeated, tag = "2")]
    pub photos: ::prost::alloc::vec::Vec<Photo>,
    /// Input only. Required when creating photo sequence. The resource name
    /// where the bytes of the photo sequence (in the form of video) are uploaded.
    #[prost(message, optional, tag = "3")]
    pub upload_reference: ::core::option::Option<UploadRef>,
    /// Optional. Absolute time when the photo sequence starts to be captured.
    /// If the photo sequence is a video, this is the start time of the video.
    /// If this field is populated in input, it overrides the capture time in the
    /// video or XDM file.
    #[prost(message, optional, tag = "4")]
    pub capture_time_override: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this photo sequence was created in uSV Store service.
    #[prost(message, optional, tag = "18")]
    pub upload_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Raw GPS measurements with increasing timestamps from the device that
    /// aren't time synced with each photo.
    /// These raw measurements will be used to infer the pose of each frame.
    /// Required in input when InputType is VIDEO and raw GPS measurements are not
    /// in Camera Motion Metadata Track (CAMM).
    /// User can indicate which takes precedence using gps_source if raw GPS
    /// measurements are provided in both raw_gps_timeline and
    /// Camera Motion Metadata Track (CAMM).
    #[prost(message, repeated, tag = "7")]
    pub raw_gps_timeline: ::prost::alloc::vec::Vec<Pose>,
    /// Input only. If both raw_gps_timeline and
    /// the Camera Motion Metadata Track (CAMM) contain GPS measurements,
    /// indicate which takes precedence.
    #[prost(enumeration = "photo_sequence::GpsSource", tag = "8")]
    pub gps_source: i32,
    /// Input only. Three axis IMU data for the collection.
    /// If this data is too large to put in the request, then it should be put in
    /// the CAMM track for the video. This data always takes precedence over the
    /// equivalent CAMM data, if it exists.
    #[prost(message, optional, tag = "11")]
    pub imu: ::core::option::Option<Imu>,
    /// Output only. The processing state of this sequence.
    #[prost(enumeration = "ProcessingState", tag = "12")]
    pub processing_state: i32,
    /// Output only. If this sequence has processing_state = FAILED, this will contain the
    /// reason why it failed. If the processing_state is any other value, this
    /// field will be unset.
    #[prost(enumeration = "ProcessingFailureReason", tag = "13")]
    pub failure_reason: i32,
    /// Output only. If this sequence has `failure_reason` set, this may contain additional
    /// details about the failure.
    #[prost(message, optional, tag = "23")]
    pub failure_details: ::core::option::Option<ProcessingFailureDetails>,
    /// Output only. The computed distance of the photo sequence in meters.
    #[prost(double, tag = "16")]
    pub distance_meters: f64,
    /// Output only. A rectangular box that encapsulates every image in this photo sequence.
    #[prost(message, optional, tag = "20")]
    pub sequence_bounds: ::core::option::Option<LatLngBounds>,
    /// Output only. The total number of views that all the published images in this
    /// PhotoSequence have received.
    #[prost(int64, tag = "21")]
    pub view_count: i64,
    /// Output only. The filename of the upload. Does not include the directory path. Only
    /// available if the sequence was uploaded on a platform that provides the
    /// filename.
    #[prost(string, tag = "22")]
    pub filename: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PhotoSequence`.
pub mod photo_sequence {
    /// Primary source of GPS measurements.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GpsSource {
        /// GPS in raw_gps_timeline takes precedence if it exists.
        PhotoSequence = 0,
        /// GPS in Camera Motion Metadata Track (CAMM) takes precedence if it exists.
        CameraMotionMetadataTrack = 1,
    }
}
/// A rectangle in geographical coordinates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLngBounds {
    /// The southwest corner of these bounds.
    #[prost(message, optional, tag = "1")]
    pub southwest: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The northeast corner of these bounds.
    #[prost(message, optional, tag = "2")]
    pub northeast: ::core::option::Option<super::super::super::r#type::LatLng>,
}
/// Additional details to accompany the ProcessingFailureReason enum.
/// This message is always expected to be used in conjunction with
/// ProcessingFailureReason, and the oneof value set in this message should match
/// the FailureReason.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingFailureDetails {
    /// Only one set of details will be set, and must match the corresponding enum
    /// in ProcessingFailureReason.
    #[prost(oneof = "processing_failure_details::Details", tags = "1, 2, 3, 4")]
    pub details: ::core::option::Option<processing_failure_details::Details>,
}
/// Nested message and enum types in `ProcessingFailureDetails`.
pub mod processing_failure_details {
    /// Only one set of details will be set, and must match the corresponding enum
    /// in ProcessingFailureReason.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// See InsufficientGpsFailureDetails.
        #[prost(message, tag = "1")]
        InsufficientGpsDetails(super::InsufficientGpsFailureDetails),
        /// See GpsDataGapFailureDetails.
        #[prost(message, tag = "2")]
        GpsDataGapDetails(super::GpsDataGapFailureDetails),
        /// See ImuDataGapFailureDetails.
        #[prost(message, tag = "3")]
        ImuDataGapDetails(super::ImuDataGapFailureDetails),
        /// See NotOutdoorsFailureDetails.
        #[prost(message, tag = "4")]
        NotOutdoorsDetails(super::NotOutdoorsFailureDetails),
    }
}
/// Details related to ProcessingFailureReason#INSUFFICIENT_GPS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsufficientGpsFailureDetails {
    /// The number of GPS points that were found in the video.
    #[prost(int32, optional, tag = "1")]
    pub gps_points_found: ::core::option::Option<i32>,
}
/// Details related to ProcessingFailureReason#GPS_DATA_GAP.
/// If there are multiple GPS data gaps, only the one with the largest duration
/// is reported here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsDataGapFailureDetails {
    /// The duration of the gap in GPS data that was found.
    #[prost(message, optional, tag = "1")]
    pub gap_duration: ::core::option::Option<::prost_types::Duration>,
    /// Relative time (from the start of the video stream) when the gap started.
    #[prost(message, optional, tag = "2")]
    pub gap_start_time: ::core::option::Option<::prost_types::Duration>,
}
/// Details related to ProcessingFailureReason#IMU_DATA_GAP.
/// If there are multiple IMU data gaps, only the one with the largest duration
/// is reported here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImuDataGapFailureDetails {
    /// The duration of the gap in IMU data that was found.
    #[prost(message, optional, tag = "1")]
    pub gap_duration: ::core::option::Option<::prost_types::Duration>,
    /// Relative time (from the start of the video stream) when the gap started.
    #[prost(message, optional, tag = "2")]
    pub gap_start_time: ::core::option::Option<::prost_types::Duration>,
}
/// Details related to ProcessingFailureReason#NOT_OUTDOORS.
/// If there are multiple indoor frames found, the first frame is recorded here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotOutdoorsFailureDetails {
    /// Relative time (from the start of the video stream) when an indoor frame was
    /// found.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Duration>,
}
/// The processing state of the sequence. The states move as follows:
///
/// ```
///      +-------------------------+
///      |                         |
///  +---v---+  +----------+  +----+----+
///  |PENDING+-->PROCESSING+-->PROCESSED|
///  +---+---+  +----+-----+  +----+----+
///      |           |             |
///      |        +--v---+         |
///      +-------->FAILED<---------+
///               +------+
/// ```
///
/// The sequence may move to FAILED from any state. Additionally, a processed
/// sequence may be re-processed at any time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProcessingState {
    /// The state is unspecified, this is the default value.
    Unspecified = 0,
    /// The sequence has not yet started processing.
    Pending = 1,
    /// The sequence is currently in processing.
    Processing = 2,
    /// The sequence has finished processing including refining position.
    Processed = 3,
    /// The sequence failed processing. See FailureReason for more details.
    Failed = 4,
}
/// The possible reasons this \[PhotoSequence\]
/// \[google.streetview.publish.v1.PhotoSequence\] failed to process.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProcessingFailureReason {
    /// The failure reason is unspecified, this is the default value.
    Unspecified = 0,
    /// Video frame's resolution is too small.
    LowResolution = 1,
    /// This video has been uploaded before.
    Duplicate = 2,
    /// Too few GPS points.
    InsufficientGps = 3,
    /// No overlap between the time frame of GPS track and the time frame of
    /// video.
    NoOverlapGps = 4,
    /// GPS is invalid (e.x. all GPS points are at (0,0))
    InvalidGps = 5,
    /// The sequence of photos could not be accurately located in the world.
    FailedToRefinePositions = 6,
    /// The sequence was taken down for policy reasons.
    Takedown = 7,
    /// The video file was corrupt or could not be decoded.
    CorruptVideo = 8,
    /// A permanent failure in the underlying system occurred.
    Internal = 9,
    /// The video format is invalid or unsupported.
    InvalidVideoFormat = 10,
    /// Invalid image aspect ratio found.
    InvalidVideoDimensions = 11,
    /// Invalid capture time. Timestamps were from the future.
    InvalidCaptureTime = 12,
    /// GPS data contains a gap greater than 5 seconds in duration.
    GpsDataGap = 13,
    /// GPS data is too erratic to be processed.
    JumpyGps = 14,
    /// IMU (Accelerometer, Gyroscope, etc.) data are not valid. They may be
    /// missing required fields (x, y, z or time), may not be formatted correctly,
    /// or any other issue that prevents our systems from parsing it.
    InvalidImu = 15,
    /// Too few IMU points.
    InsufficientImu = 21,
    /// Insufficient overlap in the time frame between GPS, IMU, and other time
    /// series data.
    InsufficientOverlapTimeSeries = 22,
    /// IMU (Accelerometer, Gyroscope, etc.) data contain gaps greater than 0.1
    /// seconds in duration.
    ImuDataGap = 16,
    /// The camera is not supported.
    UnsupportedCamera = 17,
    /// Some frames were indoors, which is unsupported.
    NotOutdoors = 18,
    /// Not enough video frames.
    InsufficientVideoFrames = 19,
}
/// Request to create a \[Photo][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhotoRequest {
    /// Required. Photo to create.
    #[prost(message, optional, tag = "1")]
    pub photo: ::core::option::Option<Photo>,
}
/// Request to get a \[Photo][google.streetview.publish.v1.Photo\].
///
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhotoRequest {
    /// Required. ID of the \[Photo][google.streetview.publish.v1.Photo\].
    #[prost(string, tag = "1")]
    pub photo_id: ::prost::alloc::string::String,
    /// Required. Specifies if a download URL for the photo bytes should be returned in the
    /// \[Photo][google.streetview.publish.v1.Photo\] response.
    #[prost(enumeration = "PhotoView", tag = "2")]
    pub view: i32,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request to get one or more \[Photos][google.streetview.publish.v1.Photo\].
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetPhotosRequest {
    /// Required. IDs of the \[Photos][google.streetview.publish.v1.Photo\]. For HTTP
    /// GET requests, the URL query parameter should be
    /// `photoIds=<id1>&photoIds=<id2>&...`.
    #[prost(string, repeated, tag = "1")]
    pub photo_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Specifies if a download URL for the photo bytes should be returned in the
    /// Photo response.
    #[prost(enumeration = "PhotoView", tag = "2")]
    pub view: i32,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response to batch get of \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetPhotosResponse {
    /// List of results for each individual
    /// \[Photo][google.streetview.publish.v1.Photo\] requested, in the same order as
    /// the requests in
    /// \[BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos\].
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<PhotoResponse>,
}
/// Response payload for a single
/// \[Photo][google.streetview.publish.v1.Photo\]
/// in batch operations including
/// \[BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos\]
/// and
/// \[BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoResponse {
    /// The status for the operation to get or update a single photo in the batch
    /// request.
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// The \[Photo][google.streetview.publish.v1.Photo\] resource, if the request
    /// was successful.
    #[prost(message, optional, tag = "2")]
    pub photo: ::core::option::Option<Photo>,
}
/// Request to list all photos that belong to the user sending the request.
///
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
/// * `pageSize` determines the maximum number of photos to return.
/// * `pageToken` is the next page token value returned from a previous
/// \[ListPhotos][google.streetview.publish.v1.StreetViewPublishService.ListPhotos\]
///     request, if any.
/// * `filter` allows filtering by a given parameter. 'placeId' is the only
/// parameter supported at the moment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotosRequest {
    /// Required. Specifies if a download URL for the photos bytes should be returned in the
    /// Photos response.
    #[prost(enumeration = "PhotoView", tag = "1")]
    pub view: i32,
    /// Optional. The maximum number of photos to return.
    /// `pageSize` must be non-negative. If `pageSize` is zero or is not provided,
    /// the default page size of 100 is used.
    /// The number of photos returned in the response may be less than `pageSize`
    /// if the number of photos that belong to the user is less than `pageSize`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The
    /// \[nextPageToken][google.streetview.publish.v1.ListPhotosResponse.next_page_token\]
    /// value returned from a previous
    /// \[ListPhotos][google.streetview.publish.v1.StreetViewPublishService.ListPhotos\]
    /// request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter expression. For example: `placeId=ChIJj61dQgK6j4AR4GeTYWZsKWw`.
    ///
    /// The filters supported are: `placeId`, `min_latitude`, `max_latitude`,
    /// `min_longitude`, and `max_longitude`. See <https://google.aip.dev/160> for
    /// more information.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response to list all photos that belong to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotosResponse {
    /// List of photos. The
    /// \[pageSize][google.streetview.publish.v1.ListPhotosRequest.page_size\] field
    /// in the request determines the number of items returned.
    #[prost(message, repeated, tag = "1")]
    pub photos: ::prost::alloc::vec::Vec<Photo>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to update the metadata of a
/// \[Photo][google.streetview.publish.v1.Photo\]. Updating the pixels of a photo
/// is not supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhotoRequest {
    /// Required. \[Photo][google.streetview.publish.v1.Photo\] object containing the
    /// new metadata.
    #[prost(message, optional, tag = "1")]
    pub photo: ::core::option::Option<Photo>,
    /// Required. Mask that identifies fields on the photo metadata to update.
    /// If not present, the old \[Photo][google.streetview.publish.v1.Photo\]
    /// metadata is entirely replaced with the
    /// new \[Photo][google.streetview.publish.v1.Photo\] metadata in this request.
    /// The update fails if invalid fields are specified. Multiple fields can be
    /// specified in a comma-delimited list.
    ///
    /// The following fields are valid:
    ///
    /// * `pose.heading`
    /// * `pose.latLngPair`
    /// * `pose.pitch`
    /// * `pose.roll`
    /// * `pose.level`
    /// * `pose.altitude`
    /// * `connections`
    /// * `places`
    ///
    ///
    /// > Note: When
    /// \[updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask\]
    /// contains repeated fields, the entire set of repeated values get replaced
    /// with the new contents. For example, if
    /// \[updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask\]
    /// contains `connections` and `UpdatePhotoRequest.photo.connections` is empty,
    /// all connections are removed.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to update the metadata of photos.
/// Updating the pixels of photos is not supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdatePhotosRequest {
    /// Required. List of
    /// \[UpdatePhotoRequests][google.streetview.publish.v1.UpdatePhotoRequest\].
    #[prost(message, repeated, tag = "1")]
    pub update_photo_requests: ::prost::alloc::vec::Vec<UpdatePhotoRequest>,
}
/// Response to batch update of metadata of one or more
/// \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdatePhotosResponse {
    /// List of results for each individual
    /// \[Photo][google.streetview.publish.v1.Photo\] updated, in the same order as
    /// the request.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<PhotoResponse>,
}
/// Request to delete a \[Photo][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhotoRequest {
    /// Required. ID of the \[Photo][google.streetview.publish.v1.Photo\].
    #[prost(string, tag = "1")]
    pub photo_id: ::prost::alloc::string::String,
}
/// Request to delete multiple \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeletePhotosRequest {
    /// Required. IDs of the \[Photos][google.streetview.publish.v1.Photo\]. HTTP
    /// GET requests require the following syntax for the URL query parameter:
    /// `photoIds=<id1>&photoIds=<id2>&...`.
    #[prost(string, repeated, tag = "1")]
    pub photo_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request to create a
/// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\] from a video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhotoSequenceRequest {
    /// Required. \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\] to
    /// create.
    #[prost(message, optional, tag = "1")]
    pub photo_sequence: ::core::option::Option<PhotoSequence>,
    /// Required. The input form of
    /// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\].
    #[prost(enumeration = "create_photo_sequence_request::InputType", tag = "2")]
    pub input_type: i32,
}
/// Nested message and enum types in `CreatePhotoSequenceRequest`.
pub mod create_photo_sequence_request {
    /// Input forms of \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\].
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InputType {
        /// Not specified. Server will return \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\].
        Unspecified = 0,
        /// 360 Video.
        Video = 1,
        /// Extensible Device Metadata, <http://www.xdm.org>
        Xdm = 2,
    }
}
/// Request to get a \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\].
///
/// By default
///
/// * does not return the download URL for the
/// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\].
///
/// Parameters:
///
/// * `view` controls if the download URL for the
/// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\] is
///   returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhotoSequenceRequest {
    /// Required. ID of the photo sequence.
    #[prost(string, tag = "1")]
    pub sequence_id: ::prost::alloc::string::String,
    /// Specifies if a download URL for the photo sequence should be returned in
    /// `download_url` of individual photos in the
    /// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\] response.
    /// > Note: Currently not implemented.
    #[deprecated]
    #[prost(enumeration = "PhotoView", tag = "2")]
    pub view: i32,
    /// Optional. The filter expression. For example: `published_status=PUBLISHED`.
    ///
    /// The filters supported are: `published_status`.  See
    /// <https://google.aip.dev/160> for more information.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// Request to delete a
/// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhotoSequenceRequest {
    /// Required. ID of the
    /// \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\].
    #[prost(string, tag = "1")]
    pub sequence_id: ::prost::alloc::string::String,
}
/// Response to batch delete of one or more
/// \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeletePhotosResponse {
    /// The status for the operation to delete a single
    /// \[Photo][google.streetview.publish.v1.Photo\] in the batch request.
    #[prost(message, repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Request to list all photo sequences that belong to the user sending the
/// request.
///
/// Parameters:
///
/// * `pageSize` determines the maximum number of photo sequences to return.
/// * `pageToken` is the next page token value returned from a previous
/// \[ListPhotoSequences][google.streetview.publish.v1.StreetViewPublishService.ListPhotoSequences\]
///   request, if any.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotoSequencesRequest {
    /// Optional. The maximum number of photo sequences to return.
    /// `pageSize` must be non-negative. If `pageSize` is zero or is not
    /// provided, the default page size of 100 is used.
    /// The number of photo sequences returned in the response may be less than
    /// `pageSize` if the number of matches is less than `pageSize`.
    /// This is currently unimplemented but is in process.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. The
    /// \[nextPageToken][google.streetview.publish.v1.ListPhotosResponse.next_page_token\]
    /// value returned from a previous
    /// \[ListPhotoSequences][google.streetview.publish.v1.StreetViewPublishService.ListPhotoSequences\]
    /// request, if any.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter expression. For example: `imagery_type=SPHERICAL`.
    ///
    /// The filters supported are: `imagery_type`, `processing_state`,
    /// `min_latitude`, `max_latitude`, `min_longitude`, `max_longitude`, and
    /// `filename_query`. See <https://google.aip.dev/160> for more information.
    /// Filename queries should sent as a Phrase in order to support multple words
    /// and special characters by adding escaped quotes. Ex:
    /// filename_query="example of a phrase.mp4"
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response to list all photo sequences that belong to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotoSequencesResponse {
    /// List of photo sequences via \[Operation][google.longrunning.Operation\]
    /// interface.
    ///
    /// The maximum number of items returned is based on the
    /// \[pageSize][google.streetview.publish.v1.ListPhotoSequencesRequest.page_size\]
    /// field in the request.
    ///
    /// Each item in the list can have three possible states,
    ///
    /// * `Operation.done` = false, if the processing of
    ///   \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\] is not
    ///   finished yet.
    /// * `Operation.done` = true and `Operation.error` is populated, if there was
    ///   an error in processing.
    /// * `Operation.done` = true and `Operation.response` contains a
    ///   \[PhotoSequence][google.streetview.publish.v1.PhotoSequence\] message,
    ///   In each sequence, only
    ///   \[Id][google.streetview.publish.v1.PhotoSequence.id\] is populated.
    #[prost(message, repeated, tag = "1")]
    pub photo_sequences: ::prost::alloc::vec::Vec<super::super::super::longrunning::Operation>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Specifies which view of the \[Photo][google.streetview.publish.v1.Photo\]
/// to include in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhotoView {
    /// Server responses do not include the download URL for the photo bytes.
    /// The default value.
    Basic = 0,
    /// Server responses include the download URL for the photo bytes.
    IncludeDownloadUrl = 1,
}
#[doc = r" Generated client implementations."]
pub mod street_view_publish_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Publishes and connects user-contributed photos on Street View."]
    #[derive(Debug, Clone)]
    pub struct StreetViewPublishServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StreetViewPublishServiceClient<T>
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
        ) -> StreetViewPublishServiceClient<InterceptedService<T, F>>
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
            StreetViewPublishServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates an upload session to start uploading photo bytes.  The method uses"]
        #[doc = " the upload URL of the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] to upload the bytes for"]
        #[doc = " the [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = ""]
        #[doc = " In addition to the photo requirements shown in"]
        #[doc = " https://support.google.com/maps/answer/7012050?ref_topic=6275604,"]
        #[doc = " the photo must meet the following requirements:"]
        #[doc = ""]
        #[doc = " * Photo Sphere XMP metadata must be included in the photo metadata. See"]
        #[doc = " https://developers.google.com/streetview/spherical-metadata for the"]
        #[doc = " required fields."]
        #[doc = " * The pixel size of the photo must meet the size requirements listed in"]
        #[doc = " https://support.google.com/maps/answer/7012050?ref_topic=6275604, and"]
        #[doc = " the photo must be a full 360 horizontally."]
        #[doc = ""]
        #[doc = " After the upload completes, the method uses"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] with"]
        #[doc = " [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]"]
        #[doc = " to create the [Photo][google.streetview.publish.v1.Photo] object entry."]
        pub async fn start_upload(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::UploadRef>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/StartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " After the client finishes uploading the photo with the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef],"]
        #[doc = " [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]"]
        #[doc = " publishes the uploaded [Photo][google.streetview.publish.v1.Photo] to"]
        #[doc = " Street View on Google Maps."]
        #[doc = ""]
        #[doc = " Currently, the only way to set heading, pitch, and roll in CreatePhoto is"]
        #[doc = " through the [Photo Sphere XMP"]
        #[doc = " metadata](https://developers.google.com/streetview/spherical-metadata) in"]
        #[doc = " the photo bytes. CreatePhoto ignores the  `pose.heading`, `pose.pitch`,"]
        #[doc = " `pose.roll`, `pose.altitude`, and `pose.level` fields in Pose."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if the request is malformed or if"]
        #[doc = " the uploaded photo is not a 360 photo."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the upload reference does not exist."]
        #[doc = " * [google.rpc.Code.RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED] if the account has reached the"]
        #[doc = " storage limit."]
        pub async fn create_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/CreatePhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not"]
        #[doc = " create the requested [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] does not exist."]
        #[doc = " * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] is still being indexed."]
        pub async fn get_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/GetPhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] batch."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchGetPhotosResponse.results][google.streetview.publish.v1.BatchGetPhotosResponse.results]."]
        #[doc = " See"]
        #[doc = " [GetPhoto][google.streetview.publish.v1.StreetViewPublishService.GetPhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        pub async fn batch_get_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetPhotosRequest>,
        ) -> Result<tonic::Response<super::BatchGetPhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/BatchGetPhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the [Photos][google.streetview.publish.v1.Photo] that belong to"]
        #[doc = " the user."]
        #[doc = ""]
        #[doc = " > Note: Recently created photos that are still"]
        #[doc = " being indexed are not returned in the response."]
        pub async fn list_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhotosRequest>,
        ) -> Result<tonic::Response<super::ListPhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/ListPhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the metadata of a [Photo][google.streetview.publish.v1.Photo], such"]
        #[doc = " as pose, place association, connections, etc. Changing the pixels of a"]
        #[doc = " photo is not supported."]
        #[doc = ""]
        #[doc = " Only the fields specified in the"]
        #[doc = " [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]"]
        #[doc = " field are used. If `updateMask` is not present, the update applies to all"]
        #[doc = " fields."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not"]
        #[doc = " create the requested photo."]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if the request is malformed."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested photo does not exist."]
        #[doc = " * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] is still being indexed."]
        pub async fn update_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/UpdatePhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the metadata of [Photos][google.streetview.publish.v1.Photo], such"]
        #[doc = " as pose, place association, connections, etc. Changing the pixels of photos"]
        #[doc = " is not supported."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchUpdatePhotosResponse.results][google.streetview.publish.v1.BatchUpdatePhotosResponse.results]."]
        #[doc = " See"]
        #[doc = " [UpdatePhoto][google.streetview.publish.v1.StreetViewPublishService.UpdatePhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        #[doc = ""]
        #[doc = " Only the fields specified in"]
        #[doc = " [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]"]
        #[doc = " field are used. If `updateMask` is not present, the update applies to all"]
        #[doc = " fields."]
        #[doc = ""]
        #[doc = " The number of"]
        #[doc = " [UpdatePhotoRequest][google.streetview.publish.v1.UpdatePhotoRequest]"]
        #[doc = " messages in a"]
        #[doc = " [BatchUpdatePhotosRequest][google.streetview.publish.v1.BatchUpdatePhotosRequest]"]
        #[doc = " must not exceed 20."]
        #[doc = ""]
        #[doc = " > Note: To update"]
        #[doc = " [Pose.altitude][google.streetview.publish.v1.Pose.altitude],"]
        #[doc = " [Pose.latLngPair][google.streetview.publish.v1.Pose.lat_lng_pair] has to be"]
        #[doc = " filled as well. Otherwise, the request will fail."]
        pub async fn batch_update_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdatePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchUpdatePhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/BatchUpdatePhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [Photo][google.streetview.publish.v1.Photo] and its metadata."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not"]
        #[doc = " create the requested photo."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the photo ID does not exist."]
        pub async fn delete_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhotoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/DeletePhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a list of [Photos][google.streetview.publish.v1.Photo] and their"]
        #[doc = " metadata."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchDeletePhotosResponse.results][google.streetview.publish.v1.BatchDeletePhotosResponse.status]."]
        #[doc = " See"]
        #[doc = " [DeletePhoto][google.streetview.publish.v1.StreetViewPublishService.DeletePhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        pub async fn batch_delete_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeletePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchDeletePhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/BatchDeletePhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an upload session to start uploading photo sequence data."]
        #[doc = " The upload URL of the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] is used to upload the"]
        #[doc = " data for the `photoSequence`."]
        #[doc = ""]
        #[doc = " After the upload is complete, the"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] is used with"]
        #[doc = " [CreatePhotoSequence][google.streetview.publish.v1.StreetViewPublishService.CreatePhotoSequence]"]
        #[doc = " to create the [PhotoSequence][google.streetview.publish.v1.PhotoSequence]"]
        #[doc = " object entry."]
        pub async fn start_photo_sequence_upload(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::UploadRef>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/StartPhotoSequenceUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " After the client finishes uploading the"]
        #[doc = " [PhotoSequence][google.streetview.publish.v1.PhotoSequence] with the"]
        #[doc = " returned [UploadRef][google.streetview.publish.v1.UploadRef],"]
        #[doc = " [CreatePhotoSequence][google.streetview.publish.v1.StreetViewPublishService.CreatePhotoSequence]"]
        #[doc = " extracts a sequence of 360 photos from a video or Extensible Device"]
        #[doc = " Metadata (XDM, http://www.xdm.org/) to be published to Street View on"]
        #[doc = " Google Maps."]
        #[doc = ""]
        #[doc = " `CreatePhotoSequence` returns an [Operation][google.longrunning.Operation],"]
        #[doc = " with the [PhotoSequence][google.streetview.publish.v1.PhotoSequence] Id set"]
        #[doc = " in the `Operation.name` field."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if the request is malformed."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the upload reference does not exist."]
        pub async fn create_photo_sequence(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhotoSequenceRequest>,
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
                "/google.streetview.publish.v1.StreetViewPublishService/CreatePhotoSequence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [PhotoSequence][google.streetview.publish.v1.PhotoSequence] via the"]
        #[doc = " [Operation][google.longrunning.Operation] interface."]
        #[doc = ""]
        #[doc = " This method returns the following three types of responses:"]
        #[doc = ""]
        #[doc = " * `Operation.done` = false, if the processing of"]
        #[doc = "   [PhotoSequence][google.streetview.publish.v1.PhotoSequence] is not"]
        #[doc = "   finished yet."]
        #[doc = " * `Operation.done` = true and `Operation.error` is populated, if there was"]
        #[doc = "   an error in processing."]
        #[doc = " * `Operation.done` = true and `Operation.response` is poulated, which"]
        #[doc = "   contains a [PhotoSequence][google.streetview.publish.v1.PhotoSequence]"]
        #[doc = "   message."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not"]
        #[doc = " create the requested"]
        #[doc = " [PhotoSequence][google.streetview.publish.v1.PhotoSequence]."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested"]
        #[doc = " [PhotoSequence][google.streetview.publish.v1.PhotoSequence] does not exist."]
        pub async fn get_photo_sequence(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhotoSequenceRequest>,
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
                "/google.streetview.publish.v1.StreetViewPublishService/GetPhotoSequence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the [PhotoSequences][google.streetview.publish.v1.PhotoSequence]"]
        #[doc = " that belong to the user, in descending CreatePhotoSequence timestamp order."]
        pub async fn list_photo_sequences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhotoSequencesRequest>,
        ) -> Result<tonic::Response<super::ListPhotoSequencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/ListPhotoSequences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [PhotoSequence][google.streetview.publish.v1.PhotoSequence] and"]
        #[doc = " its metadata."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not"]
        #[doc = " create the requested photo sequence."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the photo sequence ID does not exist."]
        #[doc = " * [google.rpc.Code.FAILED_PRECONDITION][google.rpc.Code.FAILED_PRECONDITION] if the photo sequence ID is not"]
        #[doc = " yet finished processing."]
        pub async fn delete_photo_sequence(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhotoSequenceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/DeletePhotoSequence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
