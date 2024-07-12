/// Information related to how and why a fallback result was used. If this field
/// is set, then it means the server used a different routing mode from your
/// preferred mode as fallback.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FallbackInfo {
    /// Routing mode used for the response. If fallback was triggered, the mode
    /// may be different from routing preference set in the original client
    /// request.
    #[prost(enumeration = "FallbackRoutingMode", tag = "1")]
    pub routing_mode: i32,
    /// The reason why fallback response was used instead of the original response.
    /// This field is only populated when the fallback mode is triggered and the
    /// fallback response is returned.
    #[prost(enumeration = "FallbackReason", tag = "2")]
    pub reason: i32,
}
/// Reasons for using fallback response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FallbackReason {
    /// No fallback reason specified.
    Unspecified = 0,
    /// A server error happened while calculating routes with your preferred
    /// routing mode, but we were able to return a result calculated by an
    /// alternative mode.
    ServerError = 1,
    /// We were not able to finish the calculation with your preferred routing mode
    /// on time, but we were able to return a result calculated by an alternative
    /// mode.
    LatencyExceeded = 2,
}
/// Actual routing mode used for returned fallback response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FallbackRoutingMode {
    /// Not used.
    Unspecified = 0,
    /// Indicates the `TRAFFIC_UNAWARE`
    /// \[`RoutingPreference`][google.maps.routing.v2.RoutingPreference\] was used to
    /// compute the response.
    FallbackTrafficUnaware = 1,
    /// Indicates the `TRAFFIC_AWARE`
    /// \[`RoutingPreference`][google.maps.routing.v2.RoutingPreference\] was used to
    /// compute the response.
    FallbackTrafficAware = 2,
}
/// Contains \[`GeocodedWaypoints`][google.maps.routing.v2.GeocodedWaypoint\] for
/// origin, destination and intermediate waypoints. Only populated for address
/// waypoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeocodingResults {
    /// Origin geocoded waypoint.
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<GeocodedWaypoint>,
    /// Destination geocoded waypoint.
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<GeocodedWaypoint>,
    /// A list of intermediate geocoded waypoints each containing an index field
    /// that corresponds to the zero-based position of the waypoint in the order
    /// they were specified in the request.
    #[prost(message, repeated, tag = "3")]
    pub intermediates: ::prost::alloc::vec::Vec<GeocodedWaypoint>,
}
/// Details about the locations used as waypoints. Only populated for address
/// waypoints. Includes details about the geocoding results for the purposes of
/// determining what the address was geocoded to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeocodedWaypoint {
    /// Indicates the status code resulting from the geocoding operation.
    #[prost(message, optional, tag = "1")]
    pub geocoder_status: ::core::option::Option<super::super::super::rpc::Status>,
    /// The index of the corresponding intermediate waypoint in the request.
    /// Only populated if the corresponding waypoint is an intermediate
    /// waypoint.
    #[prost(int32, optional, tag = "2")]
    pub intermediate_waypoint_request_index: ::core::option::Option<i32>,
    /// The type(s) of the result, in the form of zero or more type tags.
    /// Supported types: [Address types and address component
    /// types](<https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types>).
    #[prost(string, repeated, tag = "3")]
    pub r#type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indicates that the geocoder did not return an exact match for the original
    /// request, though it was able to match part of the requested address. You may
    /// wish to examine the original request for misspellings and/or an incomplete
    /// address.
    #[prost(bool, tag = "4")]
    pub partial_match: bool,
    /// The place ID for this result.
    #[prost(string, tag = "5")]
    pub place_id: ::prost::alloc::string::String,
}
/// Localized description of time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedTime {
    /// The time specified as a string in a given time zone.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<super::super::super::r#type::LocalizedText>,
    /// Contains the time zone. The value is the name of the time zone as defined
    /// in the [IANA Time Zone Database](<http://www.iana.org/time-zones>), e.g.
    /// "America/New_York".
    #[prost(string, tag = "2")]
    pub time_zone: ::prost::alloc::string::String,
}
/// Encapsulates a location (a geographic point, and an optional heading).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The waypoint's geographic coordinates.
    #[prost(message, optional, tag = "1")]
    pub lat_lng: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The compass heading associated with the direction of the flow of traffic.
    /// This value specifies the side of the road for pickup and drop-off. Heading
    /// values can be from 0 to 360, where 0 specifies a heading of due North, 90
    /// specifies a heading of due East, and so on. You can use this field only for
    /// `DRIVE` and `TWO_WHEELER`
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(message, optional, tag = "2")]
    pub heading: ::core::option::Option<i32>,
}
/// A set of values that specify the navigation action to take for the current
/// step (for example, turn left, merge, or straight).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Maneuver {
    /// Not used.
    Unspecified = 0,
    /// Turn slightly to the left.
    TurnSlightLeft = 1,
    /// Turn sharply to the left.
    TurnSharpLeft = 2,
    /// Make a left u-turn.
    UturnLeft = 3,
    /// Turn left.
    TurnLeft = 4,
    /// Turn slightly to the right.
    TurnSlightRight = 5,
    /// Turn sharply to the right.
    TurnSharpRight = 6,
    /// Make a right u-turn.
    UturnRight = 7,
    /// Turn right.
    TurnRight = 8,
    /// Go straight.
    Straight = 9,
    /// Take the left ramp.
    RampLeft = 10,
    /// Take the right ramp.
    RampRight = 11,
    /// Merge into traffic.
    Merge = 12,
    /// Take the left fork.
    ForkLeft = 13,
    /// Take the right fork.
    ForkRight = 14,
    /// Take the ferry.
    Ferry = 15,
    /// Take the train leading onto the ferry.
    FerryTrain = 16,
    /// Turn left at the roundabout.
    RoundaboutLeft = 17,
    /// Turn right at the roundabout.
    RoundaboutRight = 18,
    /// Initial maneuver.
    Depart = 19,
    /// Used to indicate a street name change.
    NameChange = 20,
}
/// Encapsulates navigation instructions for a
/// \[`RouteLegStep`][google.maps.routing.v2.RouteLegStep\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationInstruction {
    /// Encapsulates the navigation instructions for the current step (for example,
    /// turn left, merge, or straight). This field determines which icon to
    /// display.
    #[prost(enumeration = "Maneuver", tag = "1")]
    pub maneuver: i32,
    /// Instructions for navigating this step.
    #[prost(string, tag = "2")]
    pub instructions: ::prost::alloc::string::String,
}
/// Encapsulates an encoded polyline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polyline {
    /// Encapsulates the type of polyline. Defaults to encoded_polyline.
    #[prost(oneof = "polyline::PolylineType", tags = "1, 2")]
    pub polyline_type: ::core::option::Option<polyline::PolylineType>,
}
/// Nested message and enum types in `Polyline`.
pub mod polyline {
    /// Encapsulates the type of polyline. Defaults to encoded_polyline.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolylineType {
        /// The string encoding of the polyline using the [polyline encoding
        /// algorithm](<https://developers.google.com/maps/documentation/utilities/polylinealgorithm>)
        #[prost(string, tag = "1")]
        EncodedPolyline(::prost::alloc::string::String),
        /// Specifies a polyline using the [GeoJSON LineString
        /// format](<https://tools.ietf.org/html/rfc7946#section-3.1.4>).
        #[prost(message, tag = "2")]
        GeoJsonLinestring(::prost_types::Struct),
    }
}
/// A set of values that specify the quality of the polyline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineQuality {
    /// No polyline quality preference specified. Defaults to `OVERVIEW`.
    Unspecified = 0,
    /// Specifies a high-quality polyline - which is composed using more points
    /// than `OVERVIEW`, at the cost of increased response size. Use this value
    /// when you need more precision.
    HighQuality = 1,
    /// Specifies an overview polyline - which is composed using a small number of
    /// points. Use this value when displaying an overview of the route. Using this
    /// option has a lower request latency compared to using the
    /// `HIGH_QUALITY` option.
    Overview = 2,
}
/// Specifies the preferred type of polyline to be returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineEncoding {
    /// No polyline type preference specified. Defaults to `ENCODED_POLYLINE`.
    Unspecified = 0,
    /// Specifies a polyline encoded using the [polyline encoding
    /// algorithm](/maps/documentation/utilities/polylinealgorithm).
    EncodedPolyline = 1,
    /// Specifies a polyline using the [GeoJSON LineString
    /// format](<https://tools.ietf.org/html/rfc7946#section-3.1.4>)
    GeoJsonLinestring = 2,
}
/// Labels for the \[`Route`][google.maps.routing.v2.Route\] that are useful to
/// identify specific properties of the route to compare against others.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteLabel {
    /// Default - not used.
    Unspecified = 0,
    /// The default "best" route returned for the route computation.
    DefaultRoute = 1,
    /// An alternative to the default "best" route. Routes like this will be
    /// returned when
    /// \[`compute_alternative_routes`][google.maps.routing.v2.ComputeRoutesRequest.compute_alternative_routes\]
    /// is specified.
    DefaultRouteAlternate = 2,
    /// Fuel efficient route. Routes labeled with this value are determined to be
    /// optimized for Eco parameters such as fuel consumption.
    FuelEfficient = 3,
}
/// A set of values used to specify the mode of travel.
/// NOTE: `WALK`, `BICYCLE`, and `TWO_WHEELER` routes are in beta and might
/// sometimes be missing clear sidewalks, pedestrian paths, or bicycling paths.
/// You must display this warning to the user for all walking, bicycling, and
/// two-wheel routes that you display in your app.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteTravelMode {
    /// No travel mode specified. Defaults to `DRIVE`.
    TravelModeUnspecified = 0,
    /// Travel by passenger car.
    Drive = 1,
    /// Travel by bicycle.
    Bicycle = 2,
    /// Travel by walking.
    Walk = 3,
    /// Two-wheeled, motorized vehicle. For example, motorcycle. Note that this
    /// differs from the `BICYCLE` travel mode which covers human-powered mode.
    TwoWheeler = 4,
    /// Travel by public transit routes, where available.
    Transit = 7,
}
/// Traffic density indicator on a contiguous segment of a polyline or path.
/// Given a path with points P_0, P_1, ... , P_N (zero-based index), the
/// `SpeedReadingInterval` defines an interval and describes its traffic using
/// the following categories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedReadingInterval {
    /// The starting index of this interval in the polyline.
    #[prost(int32, optional, tag = "1")]
    pub start_polyline_point_index: ::core::option::Option<i32>,
    /// The ending index of this interval in the polyline.
    #[prost(int32, optional, tag = "2")]
    pub end_polyline_point_index: ::core::option::Option<i32>,
    #[prost(oneof = "speed_reading_interval::SpeedType", tags = "3")]
    pub speed_type: ::core::option::Option<speed_reading_interval::SpeedType>,
}
/// Nested message and enum types in `SpeedReadingInterval`.
pub mod speed_reading_interval {
    /// The classification of polyline speed based on traffic data.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Speed {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Normal speed, no slowdown is detected.
        Normal = 1,
        /// Slowdown detected, but no traffic jam formed.
        Slow = 2,
        /// Traffic jam detected.
        TrafficJam = 3,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SpeedType {
        /// Traffic speed in this interval.
        #[prost(enumeration = "Speed", tag = "3")]
        Speed(i32),
    }
}
/// Encapsulates toll information on a \[`Route`][google.maps.routing.v2.Route\] or
/// on a \[`RouteLeg`][google.maps.routing.v2.RouteLeg\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TollInfo {
    /// The monetary amount of tolls for the corresponding
    /// \[`Route`][google.maps.routing.v2.Route\] or
    /// \[`RouteLeg`][google.maps.routing.v2.RouteLeg\]. This list contains a money
    /// amount for each currency that is expected to be charged by the toll
    /// stations. Typically this list will contain only one item for routes with
    /// tolls in one currency. For international trips, this list may contain
    /// multiple items to reflect tolls in different currencies.
    #[prost(message, repeated, tag = "1")]
    pub estimated_price: ::prost::alloc::vec::Vec<super::super::super::r#type::Money>,
}
/// A transit agency that operates a transit line.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitAgency {
    /// The name of this transit agency.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The transit agency's locale-specific formatted phone number.
    #[prost(string, tag = "2")]
    pub phone_number: ::prost::alloc::string::String,
    /// The transit agency's URI.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
/// Contains information about the transit line used in this step.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitLine {
    /// The transit agency (or agencies) that operates this transit line.
    #[prost(message, repeated, tag = "1")]
    pub agencies: ::prost::alloc::vec::Vec<TransitAgency>,
    /// The full name of this transit line, For example, "8 Avenue Local".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// the URI for this transit line as provided by the transit agency.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// The color commonly used in signage for this line. Represented in
    /// hexadecimal.
    #[prost(string, tag = "4")]
    pub color: ::prost::alloc::string::String,
    /// The URI for the icon associated with this line.
    #[prost(string, tag = "5")]
    pub icon_uri: ::prost::alloc::string::String,
    /// The short name of this transit line. This name will normally be a line
    /// number, such as "M7" or "355".
    #[prost(string, tag = "6")]
    pub name_short: ::prost::alloc::string::String,
    /// The color commonly used in text on signage for this line. Represented in
    /// hexadecimal.
    #[prost(string, tag = "7")]
    pub text_color: ::prost::alloc::string::String,
    /// The type of vehicle that operates on this transit line.
    #[prost(message, optional, tag = "8")]
    pub vehicle: ::core::option::Option<TransitVehicle>,
}
/// Information about a transit stop.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitStop {
    /// The name of the transit stop.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The location of the stop expressed in latitude/longitude coordinates.
    #[prost(message, optional, tag = "2")]
    pub location: ::core::option::Option<Location>,
}
/// Information about a vehicle used in transit routes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitVehicle {
    /// The name of this vehicle, capitalized.
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<super::super::super::r#type::LocalizedText>,
    /// The type of vehicle used.
    #[prost(enumeration = "transit_vehicle::TransitVehicleType", tag = "2")]
    pub r#type: i32,
    /// The URI for an icon associated with this vehicle type.
    #[prost(string, tag = "3")]
    pub icon_uri: ::prost::alloc::string::String,
    /// The URI for the icon associated with this vehicle type, based on the local
    /// transport signage.
    #[prost(string, tag = "4")]
    pub local_icon_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransitVehicle`.
pub mod transit_vehicle {
    /// The type of vehicles for transit routes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransitVehicleType {
        /// Unused.
        Unspecified = 0,
        /// Bus.
        Bus = 1,
        /// A vehicle that operates on a cable, usually on the ground. Aerial cable
        /// cars may be of the type `GONDOLA_LIFT`.
        CableCar = 2,
        /// Commuter rail.
        CommuterTrain = 3,
        /// Ferry.
        Ferry = 4,
        /// A vehicle that is pulled up a steep incline by a cable. A Funicular
        /// typically consists of two cars, with each car acting as a counterweight
        /// for the other.
        Funicular = 5,
        /// An aerial cable car.
        GondolaLift = 6,
        /// Heavy rail.
        HeavyRail = 7,
        /// High speed train.
        HighSpeedTrain = 8,
        /// Intercity bus.
        IntercityBus = 9,
        /// Long distance train.
        LongDistanceTrain = 10,
        /// Light rail transit.
        MetroRail = 11,
        /// Monorail.
        Monorail = 12,
        /// All other vehicles.
        Other = 13,
        /// Rail.
        Rail = 14,
        /// Share taxi is a kind of bus with the ability to drop off and pick up
        /// passengers anywhere on its route.
        ShareTaxi = 15,
        /// Underground light rail.
        Subway = 16,
        /// Above ground light rail.
        Tram = 17,
        /// Trolleybus.
        Trolleybus = 18,
    }
}
/// Contains a route, which consists of a series of connected road segments
/// that join beginning, ending, and intermediate waypoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Labels for the `Route` that are useful to identify specific properties
    /// of the route to compare against others.
    #[prost(enumeration = "RouteLabel", repeated, tag = "13")]
    pub route_labels: ::prost::alloc::vec::Vec<i32>,
    /// A collection of legs (path segments between waypoints) that make up the
    /// route. Each leg corresponds to the trip between two non-`via`
    /// \[`Waypoints`][google.maps.routing.v2.Waypoint\]. For example, a route with
    /// no intermediate waypoints has only one leg. A route that includes one
    /// non-`via` intermediate waypoint has two legs. A route that includes one
    /// `via` intermediate waypoint has one leg. The order of the legs matches the
    /// order of waypoints from `origin` to `intermediates` to `destination`.
    #[prost(message, repeated, tag = "1")]
    pub legs: ::prost::alloc::vec::Vec<RouteLeg>,
    /// The travel distance of the route, in meters.
    #[prost(int32, tag = "2")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the route. If you set the
    /// `routing_preference` to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If you set the `routing_preference` to either
    /// `TRAFFIC_AWARE` or `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated
    /// taking traffic conditions into account.
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of travel through the route without taking traffic
    /// conditions into consideration.
    #[prost(message, optional, tag = "4")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The overall route polyline. This polyline is the combined polyline of
    /// all `legs`.
    #[prost(message, optional, tag = "5")]
    pub polyline: ::core::option::Option<Polyline>,
    /// A description of the route.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// An array of warnings to show when displaying the route.
    #[prost(string, repeated, tag = "7")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The viewport bounding box of the polyline.
    #[prost(message, optional, tag = "8")]
    pub viewport: ::core::option::Option<super::super::super::geo::r#type::Viewport>,
    /// Additional information about the route.
    #[prost(message, optional, tag = "9")]
    pub travel_advisory: ::core::option::Option<RouteTravelAdvisory>,
    /// If you set
    /// \[`optimize_waypoint_order`][google.maps.routing.v2.ComputeRoutesRequest.optimize_waypoint_order\]
    /// to true, this field contains the optimized ordering of intermediate
    /// waypoints. Otherwise, this field is empty.
    /// For example, if you give an input of Origin: LA; Intermediate waypoints:
    /// Dallas, Bangor, Phoenix; Destination: New York; and the optimized
    /// intermediate waypoint order is Phoenix, Dallas, Bangor, then this field
    /// contains the values [2, 0, 1]. The index starts with 0 for the first
    /// intermediate waypoint provided in the input.
    #[prost(int32, repeated, tag = "10")]
    pub optimized_intermediate_waypoint_index: ::prost::alloc::vec::Vec<i32>,
    /// Text representations of properties of the `Route`.
    #[prost(message, optional, tag = "11")]
    pub localized_values: ::core::option::Option<route::RouteLocalizedValues>,
    /// A web-safe, base64-encoded route token that can be passed to the Navigation
    /// SDK, that allows the Navigation SDK to reconstruct the route during
    /// navigation, and, in the event of rerouting, honor the original intention
    /// when you created the route by calling ComputeRoutes. Customers should treat
    /// this token as an opaque blob. It is not meant for reading or mutating.
    /// NOTE: `Route.route_token` is only available for requests that have set
    /// `ComputeRoutesRequest.routing_preference` to `TRAFFIC_AWARE` or
    /// `TRAFFIC_AWARE_OPTIMAL`. `Route.route_token` is not supported for requests
    /// that have Via waypoints.
    #[prost(string, tag = "12")]
    pub route_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Route`.
pub mod route {
    /// Text representations of certain properties.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteLocalizedValues {
        /// Travel distance represented in text form.
        #[prost(message, optional, tag = "1")]
        pub distance: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration taking traffic conditions into consideration, represented in
        /// text form. Note: If you did not request traffic information, this value
        /// will be the same value as `static_duration`.
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration without taking traffic conditions into consideration,
        /// represented in text form.
        #[prost(message, optional, tag = "3")]
        pub static_duration:
            ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Transit fare represented in text form.
        #[prost(message, optional, tag = "4")]
        pub transit_fare: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
    }
}
/// Contains the additional information that the user should be informed
/// about, such as possible traffic zone restrictions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTravelAdvisory {
    /// Contains information about tolls on the route. This field is only populated
    /// if tolls are expected on the route. If this field is set, but the
    /// `estimatedPrice` subfield is not populated, then the route contains tolls,
    /// but the estimated price is unknown. If this field is not set, then there
    /// are no tolls expected on the route.
    #[prost(message, optional, tag = "2")]
    pub toll_info: ::core::option::Option<TollInfo>,
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the route without overlap.
    /// The start point of a specified interval is the same as the end point of the
    /// preceding interval.
    ///
    /// Example:
    ///
    ///     polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///     speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag = "3")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
    /// The predicted fuel consumption in microliters.
    #[prost(int64, tag = "5")]
    pub fuel_consumption_microliters: i64,
    /// Returned route may have restrictions that are not suitable for requested
    /// travel mode or route modifiers.
    #[prost(bool, tag = "6")]
    pub route_restrictions_partially_ignored: bool,
    /// If present, contains the total fare or ticket costs on this route
    /// This property is only returned for `TRANSIT` requests and only
    /// for routes where fare information is available for all transit steps.
    #[prost(message, optional, tag = "7")]
    pub transit_fare: ::core::option::Option<super::super::super::r#type::Money>,
}
/// Contains the additional information that the user should be informed
/// about on a leg step, such as possible traffic zone restrictions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegTravelAdvisory {
    /// Contains information about tolls on the specific `RouteLeg`.
    /// This field is only populated if we expect there are tolls on the
    /// `RouteLeg`. If this field is set but the estimated_price subfield is not
    /// populated, we expect that road contains tolls but we do not know an
    /// estimated price. If this field does not exist, then there is no toll on the
    /// `RouteLeg`.
    #[prost(message, optional, tag = "1")]
    pub toll_info: ::core::option::Option<TollInfo>,
    /// Speed reading intervals detailing traffic density. Applicable in case of
    /// `TRAFFIC_AWARE` and `TRAFFIC_AWARE_OPTIMAL` routing preferences.
    /// The intervals cover the entire polyline of the `RouteLeg` without overlap.
    /// The start point of a specified interval is the same as the end point of the
    /// preceding interval.
    ///
    /// Example:
    ///
    ///     polyline: A ---- B ---- C ---- D ---- E ---- F ---- G
    ///     speed_reading_intervals: [A,C), [C,D), [D,G).
    #[prost(message, repeated, tag = "2")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Contains the additional information that the user should be informed
/// about, such as possible traffic zone restrictions on a leg step.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStepTravelAdvisory {
    /// NOTE: This field is not currently populated.
    #[prost(message, repeated, tag = "1")]
    pub speed_reading_intervals: ::prost::alloc::vec::Vec<SpeedReadingInterval>,
}
/// Contains a segment between non-`via` waypoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLeg {
    /// The travel distance of the route leg, in meters.
    #[prost(int32, tag = "1")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the leg. If the `route_preference`
    /// is set to `TRAFFIC_UNAWARE`, then this value is the same as
    /// `static_duration`. If the `route_preference` is either `TRAFFIC_AWARE` or
    /// `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated taking traffic
    /// conditions into account.
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of travel through the leg, calculated without taking
    /// traffic conditions into consideration.
    #[prost(message, optional, tag = "3")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The overall polyline for this leg that includes each `step`'s
    /// polyline.
    #[prost(message, optional, tag = "4")]
    pub polyline: ::core::option::Option<Polyline>,
    /// The start location of this leg. This location might be different from the
    /// provided `origin`. For example, when the provided `origin` is not near a
    /// road, this is a point on the road.
    #[prost(message, optional, tag = "5")]
    pub start_location: ::core::option::Option<Location>,
    /// The end location of this leg. This location might be different from the
    /// provided `destination`. For example, when the provided `destination` is not
    /// near a road, this is a point on the road.
    #[prost(message, optional, tag = "6")]
    pub end_location: ::core::option::Option<Location>,
    /// An array of steps denoting segments within this leg. Each step represents
    /// one navigation instruction.
    #[prost(message, repeated, tag = "7")]
    pub steps: ::prost::alloc::vec::Vec<RouteLegStep>,
    /// Contains the additional information that the user should be informed
    /// about, such as possible traffic zone restrictions, on a route leg.
    #[prost(message, optional, tag = "8")]
    pub travel_advisory: ::core::option::Option<RouteLegTravelAdvisory>,
    /// Text representations of properties of the `RouteLeg`.
    #[prost(message, optional, tag = "9")]
    pub localized_values: ::core::option::Option<route_leg::RouteLegLocalizedValues>,
    /// Overview information about the steps in this `RouteLeg`. This field is only
    /// populated for TRANSIT routes.
    #[prost(message, optional, tag = "10")]
    pub steps_overview: ::core::option::Option<route_leg::StepsOverview>,
}
/// Nested message and enum types in `RouteLeg`.
pub mod route_leg {
    /// Text representations of certain properties.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteLegLocalizedValues {
        /// Travel distance represented in text form.
        #[prost(message, optional, tag = "1")]
        pub distance: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration taking traffic conditions into consideration represented in text
        /// form. Note: If you did not request traffic information, this value will
        /// be the same value as static_duration.
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration without taking traffic conditions into
        /// consideration, represented in text form.
        #[prost(message, optional, tag = "3")]
        pub static_duration:
            ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
    }
    /// Provides overview information about a list of `RouteLegStep`s.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StepsOverview {
        /// Summarized information about different multi-modal segments of
        /// the `RouteLeg.steps`. This field is not populated if the `RouteLeg` does
        /// not contain any multi-modal segments in the steps.
        #[prost(message, repeated, tag = "1")]
        pub multi_modal_segments: ::prost::alloc::vec::Vec<steps_overview::MultiModalSegment>,
    }
    /// Nested message and enum types in `StepsOverview`.
    pub mod steps_overview {
        /// Provides summarized information about different multi-modal segments of
        /// the `RouteLeg.steps`. A multi-modal segment is defined as one or more
        /// contiguous `RouteLegStep` that have the same `RouteTravelMode`.
        /// This field is not populated if the `RouteLeg` does not contain any
        /// multi-modal segments in the steps.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MultiModalSegment {
            /// The corresponding `RouteLegStep` index that is the start of a
            /// multi-modal segment.
            #[prost(int32, optional, tag = "1")]
            pub step_start_index: ::core::option::Option<i32>,
            /// The corresponding `RouteLegStep` index that is the end of a
            /// multi-modal segment.
            #[prost(int32, optional, tag = "2")]
            pub step_end_index: ::core::option::Option<i32>,
            /// NavigationInstruction for the multi-modal segment.
            #[prost(message, optional, tag = "3")]
            pub navigation_instruction: ::core::option::Option<super::super::NavigationInstruction>,
            /// The travel mode of the multi-modal segment.
            #[prost(enumeration = "super::super::RouteTravelMode", tag = "4")]
            pub travel_mode: i32,
        }
    }
}
/// Contains a segment of a \[`RouteLeg`][google.maps.routing.v2.RouteLeg\]. A
/// step corresponds to a single navigation instruction. Route legs are made up
/// of steps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStep {
    /// The travel distance of this step, in meters. In some circumstances, this
    /// field might not have a value.
    #[prost(int32, tag = "1")]
    pub distance_meters: i32,
    /// The duration of travel through this step without taking traffic conditions
    /// into consideration. In some circumstances, this field might not have a
    /// value.
    #[prost(message, optional, tag = "2")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// The polyline associated with this step.
    #[prost(message, optional, tag = "3")]
    pub polyline: ::core::option::Option<Polyline>,
    /// The start location of this step.
    #[prost(message, optional, tag = "4")]
    pub start_location: ::core::option::Option<Location>,
    /// The end location of this step.
    #[prost(message, optional, tag = "5")]
    pub end_location: ::core::option::Option<Location>,
    /// Navigation instructions.
    #[prost(message, optional, tag = "6")]
    pub navigation_instruction: ::core::option::Option<NavigationInstruction>,
    /// Contains the additional information that the user should be informed
    /// about, such as possible traffic zone restrictions, on a leg step.
    #[prost(message, optional, tag = "7")]
    pub travel_advisory: ::core::option::Option<RouteLegStepTravelAdvisory>,
    /// Text representations of properties of the `RouteLegStep`.
    #[prost(message, optional, tag = "8")]
    pub localized_values: ::core::option::Option<route_leg_step::RouteLegStepLocalizedValues>,
    /// Details pertaining to this step if the travel mode is `TRANSIT`.
    #[prost(message, optional, tag = "9")]
    pub transit_details: ::core::option::Option<RouteLegStepTransitDetails>,
    /// The travel mode used for this step.
    #[prost(enumeration = "RouteTravelMode", tag = "10")]
    pub travel_mode: i32,
}
/// Nested message and enum types in `RouteLegStep`.
pub mod route_leg_step {
    /// Text representations of certain properties.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteLegStepLocalizedValues {
        /// Travel distance represented in text form.
        #[prost(message, optional, tag = "1")]
        pub distance: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration without taking traffic conditions into
        /// consideration, represented in text form.
        #[prost(message, optional, tag = "3")]
        pub static_duration:
            ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
    }
}
/// Additional information for the `RouteLegStep` related to `TRANSIT` routes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteLegStepTransitDetails {
    /// Information about the arrival and departure stops for the step.
    #[prost(message, optional, tag = "1")]
    pub stop_details: ::core::option::Option<route_leg_step_transit_details::TransitStopDetails>,
    /// Text representations of properties of the `RouteLegStepTransitDetails`.
    #[prost(message, optional, tag = "2")]
    pub localized_values:
        ::core::option::Option<route_leg_step_transit_details::TransitDetailsLocalizedValues>,
    /// Specifies the direction in which to travel on this line as marked on
    /// the vehicle or at the departure stop. The direction is often the terminus
    /// station.
    #[prost(string, tag = "3")]
    pub headsign: ::prost::alloc::string::String,
    /// Specifies the expected time as a duration between departures from the same
    /// stop at this time. For example, with a headway seconds value of 600, you
    /// would expect a ten minute wait if you should miss your bus.
    #[prost(message, optional, tag = "4")]
    pub headway: ::core::option::Option<::prost_types::Duration>,
    /// Information about the transit line used in this step.
    #[prost(message, optional, tag = "5")]
    pub transit_line: ::core::option::Option<TransitLine>,
    /// The number of stops from the departure to the arrival stop. This count
    /// includes the arrival stop, but excludes the departure stop. For example, if
    /// your route leaves from Stop A, passes through stops B and C, and arrives at
    /// stop D, stop_count will return 3.
    #[prost(int32, tag = "6")]
    pub stop_count: i32,
    /// The text that appears in schedules and sign boards to identify a transit
    /// trip to passengers. The text should uniquely identify a trip within a
    /// service day. For example, "538" is the `trip_short_text` of the Amtrak
    /// train that leaves San Jose, CA at 15:10 on weekdays to Sacramento, CA.
    #[prost(string, tag = "7")]
    pub trip_short_text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RouteLegStepTransitDetails`.
pub mod route_leg_step_transit_details {
    /// Details about the transit stops for the `RouteLegStep`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransitStopDetails {
        /// Information about the arrival stop for the step.
        #[prost(message, optional, tag = "1")]
        pub arrival_stop: ::core::option::Option<super::TransitStop>,
        /// The estimated time of arrival for the step.
        #[prost(message, optional, tag = "2")]
        pub arrival_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Information about the departure stop for the step.
        #[prost(message, optional, tag = "3")]
        pub departure_stop: ::core::option::Option<super::TransitStop>,
        /// The estimated time of departure for the step.
        #[prost(message, optional, tag = "4")]
        pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Localized descriptions of values for `RouteTransitDetails`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransitDetailsLocalizedValues {
        /// Time in its formatted text representation with a corresponding time zone.
        #[prost(message, optional, tag = "1")]
        pub arrival_time: ::core::option::Option<super::LocalizedTime>,
        /// Time in its formatted text representation with a corresponding time zone.
        #[prost(message, optional, tag = "2")]
        pub departure_time: ::core::option::Option<super::LocalizedTime>,
    }
}
/// List of toll passes around the world that we support.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TollPass {
    /// Not used. If this value is used, then the request fails.
    Unspecified = 0,
    /// Sydney toll pass. See additional details at
    /// <https://www.myetoll.com.au.>
    AuEtollTag = 82,
    /// Sydney toll pass. See additional details at <https://www.tollpay.com.au.>
    AuEwayTag = 83,
    /// Australia-wide toll pass.
    /// See additional details at <https://www.linkt.com.au/.>
    AuLinkt = 2,
    /// Argentina toll pass. See additional details at <https://telepase.com.ar>
    ArTelepase = 3,
    /// Brazil toll pass. See additional details at <https://www.autoexpreso.com>
    BrAutoExpreso = 81,
    /// Brazil toll pass. See additional details at <https://conectcar.com.>
    BrConectcar = 7,
    /// Brazil toll pass. See additional details at <https://movemais.com.>
    BrMoveMais = 8,
    /// Brazil toll pass. See additional details at <https://pasorapido.gob.do/>
    BrPassaRapido = 88,
    /// Brazil toll pass. See additional details at <https://www.semparar.com.br.>
    BrSemParar = 9,
    /// Brazil toll pass. See additional details at <https://taggy.com.br.>
    BrTaggy = 10,
    /// Brazil toll pass. See additional details at
    /// <https://veloe.com.br/site/onde-usar.>
    BrVeloe = 11,
    /// Canada to United States border crossing.
    CaUsAkwasasneSeawayCorporateCard = 84,
    /// Canada to United States border crossing.
    CaUsAkwasasneSeawayTransitCard = 85,
    /// Ontario, Canada to Michigan, United States border crossing.
    CaUsBlueWaterEdgePass = 18,
    /// Ontario, Canada to Michigan, United States border crossing.
    CaUsConnexion = 19,
    /// Canada to United States border crossing.
    CaUsNexusCard = 20,
    /// Indonesia.
    /// E-card provided by multiple banks used to pay for tolls. All e-cards
    /// via banks are charged the same so only one enum value is needed. E.g.
    /// - Bank Mandiri <https://www.bankmandiri.co.id/e-money>
    /// - BCA <https://www.bca.co.id/flazz>
    /// - BNI <https://www.bni.co.id/id-id/ebanking/tapcash>
    IdEToll = 16,
    /// India.
    InFastag = 78,
    /// India, HP state plate exemption.
    InLocalHpPlateExempt = 79,
    /// Japan
    /// ETC. Electronic wireless system to collect tolls.
    /// <https://www.go-etc.jp/>
    JpEtc = 98,
    /// Japan
    /// ETC2.0. New version of ETC with further discount and bidirectional
    /// communication between devices on vehicles and antennas on the road.
    /// <https://www.go-etc.jp/etc2/index.html>
    JpEtc2 = 99,
    /// Mexico toll pass.
    /// <https://iave.capufe.gob.mx/#/>
    MxIave = 90,
    /// Mexico
    /// <https://www.pase.com.mx>
    MxPase = 91,
    /// Mexico
    ///  <https://operadoravial.com/quick-pass/>
    MxQuickpass = 93,
    /// <http://appsh.chihuahua.gob.mx/transparencia/?doc=/ingresos/TelepeajeFormato4.pdf>
    MxSistemaTelepeajeChihuahua = 89,
    /// Mexico
    MxTagIave = 12,
    /// Mexico toll pass company. One of many operating in Mexico City. See
    /// additional details at <https://www.televia.com.mx.>
    MxTagTelevia = 13,
    /// Mexico toll pass company. One of many operating in Mexico City.
    /// <https://www.televia.com.mx>
    MxTelevia = 92,
    /// Mexico toll pass. See additional details at
    /// <https://www.viapass.com.mx/viapass/web_home.aspx.>
    MxViapass = 14,
    /// AL, USA.
    UsAlFreedomPass = 21,
    /// AK, USA.
    UsAkAntonAndersonTunnelBookOf10Tickets = 22,
    /// CA, USA.
    UsCaFastrak = 4,
    /// Indicates driver has any FasTrak pass in addition to the DMV issued Clean
    /// Air Vehicle (CAV) sticker.
    /// <https://www.bayareafastrak.org/en/guide/doINeedFlex.shtml>
    UsCaFastrakCavSticker = 86,
    /// CO, USA.
    UsCoExpresstoll = 23,
    /// CO, USA.
    UsCoGoPass = 24,
    /// DE, USA.
    UsDeEzpassde = 25,
    /// FL, USA.
    UsFlBobSikesTollBridgePass = 65,
    /// FL, USA.
    UsFlDunesCommunityDevelopmentDistrictExpresscard = 66,
    /// FL, USA.
    UsFlEpass = 67,
    /// FL, USA.
    UsFlGibaTollPass = 68,
    /// FL, USA.
    UsFlLeeway = 69,
    /// FL, USA.
    UsFlSunpass = 70,
    /// FL, USA.
    UsFlSunpassPro = 71,
    /// IL, USA.
    UsIlEzpassil = 73,
    /// IL, USA.
    UsIlIpass = 72,
    /// IN, USA.
    UsInEzpassin = 26,
    /// KS, USA.
    UsKsBestpassHorizon = 27,
    /// KS, USA.
    UsKsKtag = 28,
    /// KS, USA.
    UsKsNationalpass = 29,
    /// KS, USA.
    UsKsPrepassElitepass = 30,
    /// KY, USA.
    UsKyRiverlink = 31,
    /// LA, USA.
    UsLaGeauxpass = 32,
    /// LA, USA.
    UsLaTollTag = 33,
    /// MA, USA.
    UsMaEzpassma = 6,
    /// MD, USA.
    UsMdEzpassmd = 34,
    /// ME, USA.
    UsMeEzpassme = 35,
    /// MI, USA.
    UsMiAmbassadorBridgePremierCommuterCard = 36,
    /// MI, USA.
    UsMiBcpass = 94,
    /// MI, USA.
    UsMiGrosseIleTollBridgePassTag = 37,
    /// MI, USA.
    /// Deprecated as this pass type no longer exists.
    UsMiIqProxCard = 38,
    /// MI, USA.
    UsMiIqTag = 95,
    /// MI, USA.
    UsMiMackinacBridgeMacPass = 39,
    /// MI, USA.
    UsMiNexpressToll = 40,
    /// MN, USA.
    UsMnEzpassmn = 41,
    /// NC, USA.
    UsNcEzpassnc = 42,
    /// NC, USA.
    UsNcPeachPass = 87,
    /// NC, USA.
    UsNcQuickPass = 43,
    /// NH, USA.
    UsNhEzpassnh = 80,
    /// NJ, USA.
    UsNjDownbeachExpressPass = 75,
    /// NJ, USA.
    UsNjEzpassnj = 74,
    /// NY, USA.
    UsNyExpresspass = 76,
    /// NY, USA.
    UsNyEzpassny = 77,
    /// OH, USA.
    UsOhEzpassoh = 44,
    /// PA, USA.
    UsPaEzpasspa = 45,
    /// RI, USA.
    UsRiEzpassri = 46,
    /// SC, USA.
    UsScPalpass = 47,
    /// TX, USA.
    UsTxAviTag = 97,
    /// TX, USA.
    UsTxBancpass = 48,
    /// TX, USA.
    UsTxDelRioPass = 49,
    /// TX, USA.
    UsTxEfastPass = 50,
    /// TX, USA.
    UsTxEaglePassExpressCard = 51,
    /// TX, USA.
    UsTxEptoll = 52,
    /// TX, USA.
    UsTxEzCross = 53,
    /// TX, USA.
    UsTxEztag = 54,
    /// TX, USA.
    UsTxFuegoTag = 96,
    /// TX, USA.
    UsTxLaredoTradeTag = 55,
    /// TX, USA.
    UsTxPluspass = 56,
    /// TX, USA.
    UsTxTolltag = 57,
    /// TX, USA.
    UsTxTxtag = 58,
    /// TX, USA.
    UsTxXpressCard = 59,
    /// UT, USA.
    UsUtAdamsAveParkwayExpresscard = 60,
    /// VA, USA.
    UsVaEzpassva = 61,
    /// WA, USA.
    UsWaBreezeby = 17,
    /// WA, USA.
    UsWaGoodToGo = 1,
    /// WV, USA.
    UsWvEzpasswv = 62,
    /// WV, USA.
    UsWvMemorialBridgeTickets = 63,
    /// WV, USA
    UsWvMovPass = 100,
    /// WV, USA.
    UsWvNewellTollBridgeTicket = 64,
}
/// A set of values describing the vehicle's emission type.
/// Applies only to the `DRIVE`
/// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleEmissionType {
    /// No emission type specified. Default to `GASOLINE`.
    Unspecified = 0,
    /// Gasoline/petrol fueled vehicle.
    Gasoline = 1,
    /// Electricity powered vehicle.
    Electric = 2,
    /// Hybrid fuel (such as gasoline + electric) vehicle.
    Hybrid = 3,
    /// Diesel fueled vehicle.
    Diesel = 4,
}
/// Contains the vehicle information, such as the vehicle emission type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleInfo {
    /// Describes the vehicle's emission type.
    /// Applies only to the `DRIVE`
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(enumeration = "VehicleEmissionType", tag = "2")]
    pub emission_type: i32,
}
/// Encapsulates a set of optional conditions to satisfy when calculating the
/// routes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteModifiers {
    /// When set to true, avoids toll roads where reasonable, giving preference to
    /// routes not containing toll roads. Applies only to the `DRIVE` and
    /// `TWO_WHEELER` \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(bool, tag = "1")]
    pub avoid_tolls: bool,
    /// When set to true, avoids highways where reasonable, giving preference to
    /// routes not containing highways. Applies only to the `DRIVE` and
    /// `TWO_WHEELER` \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(bool, tag = "2")]
    pub avoid_highways: bool,
    /// When set to true, avoids ferries where reasonable, giving preference to
    /// routes not containing ferries. Applies only to the `DRIVE` and`TWO_WHEELER`
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(bool, tag = "3")]
    pub avoid_ferries: bool,
    /// When set to true, avoids navigating indoors where reasonable, giving
    /// preference to routes not containing indoor navigation. Applies only to the
    /// `WALK` \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(bool, tag = "4")]
    pub avoid_indoor: bool,
    /// Specifies the vehicle information.
    #[prost(message, optional, tag = "5")]
    pub vehicle_info: ::core::option::Option<VehicleInfo>,
    /// Encapsulates information about toll passes.
    /// If toll passes are provided, the API tries to return the pass price. If
    /// toll passes are not provided, the API treats the toll pass as unknown and
    /// tries to return the cash price.
    /// Applies only to the `DRIVE` and `TWO_WHEELER`
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(enumeration = "TollPass", repeated, tag = "6")]
    pub toll_passes: ::prost::alloc::vec::Vec<i32>,
}
/// A set of values that specify factors to take into consideration when
/// calculating the route.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoutingPreference {
    /// No routing preference specified. Default to `TRAFFIC_UNAWARE`.
    Unspecified = 0,
    /// Computes routes without taking live traffic conditions into consideration.
    /// Suitable when traffic conditions don't matter or are not applicable.
    /// Using this value produces the lowest latency.
    /// Note: For \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\]
    /// `DRIVE` and `TWO_WHEELER`, the route and duration chosen are based on road
    /// network and average time-independent traffic conditions, not current road
    /// conditions. Consequently, routes may include roads that are temporarily
    /// closed. Results for a given
    /// request may vary over time due to changes in the road network, updated
    /// average traffic conditions, and the distributed nature of the service.
    /// Results may also vary between nearly-equivalent routes at any time or
    /// frequency.
    TrafficUnaware = 1,
    /// Calculates routes taking live traffic conditions into consideration.
    /// In contrast to `TRAFFIC_AWARE_OPTIMAL`, some optimizations are applied to
    /// significantly reduce latency.
    TrafficAware = 2,
    /// Calculates the routes taking live traffic conditions into consideration,
    /// without applying most performance optimizations. Using this value produces
    /// the highest latency.
    TrafficAwareOptimal = 3,
}
/// Specifies the assumptions to use when calculating time in traffic. This
/// setting affects the value returned in the `duration` field in the
/// response, which contains the predicted time in traffic based on historical
/// averages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficModel {
    /// Unused. If specified, will default to `BEST_GUESS`.
    Unspecified = 0,
    /// Indicates that the returned `duration` should be the best
    /// estimate of travel time given what is known about both historical traffic
    /// conditions and live traffic. Live traffic becomes more important the closer
    /// the `departure_time` is to now.
    BestGuess = 1,
    /// Indicates that the returned duration should be longer than the
    /// actual travel time on most days, though occasional days with particularly
    /// bad traffic conditions may exceed this value.
    Pessimistic = 2,
    /// Indicates that the returned duration should be shorter than the actual
    /// travel time on most days, though occasional days with particularly good
    /// traffic conditions may be faster than this value.
    Optimistic = 3,
}
/// Preferences for `TRANSIT` based routes that influence the route that is
/// returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransitPreferences {
    /// A set of travel modes to use when getting a `TRANSIT` route. Defaults to
    /// all supported modes of travel.
    #[prost(
        enumeration = "transit_preferences::TransitTravelMode",
        repeated,
        tag = "1"
    )]
    pub allowed_travel_modes: ::prost::alloc::vec::Vec<i32>,
    /// A routing preference that, when specified, influences the `TRANSIT` route
    /// returned.
    #[prost(
        enumeration = "transit_preferences::TransitRoutingPreference",
        tag = "2"
    )]
    pub routing_preference: i32,
}
/// Nested message and enum types in `TransitPreferences`.
pub mod transit_preferences {
    /// A set of values used to specify the mode of transit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransitTravelMode {
        /// No transit travel mode specified.
        Unspecified = 0,
        /// Travel by bus.
        Bus = 1,
        /// Travel by subway.
        Subway = 2,
        /// Travel by train.
        Train = 3,
        /// Travel by light rail or tram.
        LightRail = 4,
        /// Travel by rail. This is equivalent to a combination of `SUBWAY`, `TRAIN`,
        /// and `LIGHT_RAIL`.
        Rail = 5,
    }
    /// Specifies routing preferences for transit routes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransitRoutingPreference {
        /// No preference specified.
        Unspecified = 0,
        /// Indicates that the calculated route should prefer limited amounts of
        /// walking.
        LessWalking = 1,
        /// Indicates that the calculated route should prefer a limited number of
        /// transfers.
        FewerTransfers = 2,
    }
}
/// A set of values that specify the unit of measure used in the display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Units {
    /// Units of measure not specified. Defaults to the unit of measure inferred
    /// from the request.
    Unspecified = 0,
    /// Metric units of measure.
    Metric = 1,
    /// Imperial (English) units of measure.
    Imperial = 2,
}
/// Encapsulates a waypoint. Waypoints mark both the beginning and end of a
/// route, and include intermediate stops along the route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// Marks this waypoint as a milestone rather a stopping point. For
    /// each non-via waypoint in the request, the response appends an entry to the
    /// \[`legs`][google.maps.routing.v2.Route.legs\]
    /// array to provide the details for stopovers on that leg of the trip. Set
    /// this value to true when you want the route to pass through this waypoint
    /// without stopping over. Via waypoints don't cause an entry to be added to
    /// the `legs` array, but they do route the journey through the waypoint. You
    /// can only set this value on waypoints that are intermediates. The request
    /// fails if you set this field on terminal waypoints. If
    /// `ComputeRoutesRequest.optimize_waypoint_order` is set to true then this
    /// field cannot be set to true; otherwise, the request fails.
    #[prost(bool, tag = "3")]
    pub via: bool,
    /// Indicates that the waypoint is meant for vehicles to stop at, where the
    /// intention is to either pickup or drop-off. When you set this value, the
    /// calculated route won't include non-`via` waypoints on roads that are
    /// unsuitable for pickup and drop-off. This option works only for `DRIVE` and
    /// `TWO_WHEELER` travel modes, and when the `location_type` is
    /// \[`Location`][google.maps.routing.v2.Location\].
    #[prost(bool, tag = "4")]
    pub vehicle_stopover: bool,
    /// Indicates that the location of this waypoint is meant to have a preference
    /// for the vehicle to stop at a particular side of road. When you set this
    /// value, the route will pass through the location so that the vehicle can
    /// stop at the side of road that the location is biased towards from the
    /// center of the road. This option works only for `DRIVE` and `TWO_WHEELER`
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\].
    #[prost(bool, tag = "5")]
    pub side_of_road: bool,
    /// Different ways to represent a location.
    #[prost(oneof = "waypoint::LocationType", tags = "1, 2, 7")]
    pub location_type: ::core::option::Option<waypoint::LocationType>,
}
/// Nested message and enum types in `Waypoint`.
pub mod waypoint {
    /// Different ways to represent a location.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LocationType {
        /// A point specified using geographic coordinates, including an optional
        /// heading.
        #[prost(message, tag = "1")]
        Location(super::Location),
        /// The POI Place ID associated with the waypoint.
        #[prost(string, tag = "2")]
        PlaceId(::prost::alloc::string::String),
        /// Human readable address or a plus code.
        /// See <https://plus.codes> for details.
        #[prost(string, tag = "7")]
        Address(::prost::alloc::string::String),
    }
}
/// ComputeRoutes request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRoutesRequest {
    /// Required. Origin waypoint.
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<Waypoint>,
    /// Required. Destination waypoint.
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<Waypoint>,
    /// Optional. A set of waypoints along the route (excluding terminal points),
    /// for either stopping at or passing by. Up to 25 intermediate waypoints are
    /// supported.
    #[prost(message, repeated, tag = "3")]
    pub intermediates: ::prost::alloc::vec::Vec<Waypoint>,
    /// Optional. Specifies the mode of transportation.
    #[prost(enumeration = "RouteTravelMode", tag = "4")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server
    /// attempts to use the selected routing preference to compute the route. If
    ///  the routing preference results in an error or an extra long latency, then
    /// an error is returned. You can specify this option only when the
    /// `travel_mode` is `DRIVE` or `TWO_WHEELER`, otherwise the request fails.
    #[prost(enumeration = "RoutingPreference", tag = "5")]
    pub routing_preference: i32,
    /// Optional. Specifies your preference for the quality of the polyline.
    #[prost(enumeration = "PolylineQuality", tag = "6")]
    pub polyline_quality: i32,
    /// Optional. Specifies the preferred encoding for the polyline.
    #[prost(enumeration = "PolylineEncoding", tag = "12")]
    pub polyline_encoding: i32,
    /// Optional. The departure time. If you don't set this value, then this value
    /// defaults to the time that you made the request.
    /// NOTE: You can only specify a `departure_time` in the past when
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\] is set to
    /// `TRANSIT`. Transit trips are available for up to 7 days in the past or 100
    /// days in the future.
    #[prost(message, optional, tag = "7")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The arrival time.
    /// NOTE: Can only be set when
    /// \[RouteTravelMode][google.maps.routing.v2.RouteTravelMode\] is set to
    /// `TRANSIT`. You can specify either `departure_time` or `arrival_time`, but
    /// not both. Transit trips are available for up to 7 days in the past or 100
    /// days in the future.
    #[prost(message, optional, tag = "19")]
    pub arrival_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Specifies whether to calculate alternate routes in addition to
    /// the route. No alternative routes are returned for requests that have
    /// intermediate waypoints.
    #[prost(bool, tag = "8")]
    pub compute_alternative_routes: bool,
    /// Optional. A set of conditions to satisfy that affect the way routes are
    /// calculated.
    #[prost(message, optional, tag = "9")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see [Unicode Locale
    /// Identifier](<http://www.unicode.org/reports/tr35/#Unicode_locale_identifier>).
    /// See [Language
    /// Support](<https://developers.google.com/maps/faq#languagesupport>)
    /// for the list of supported languages. When you don't provide this value, the
    /// display language is inferred from the location of the route request.
    #[prost(string, tag = "10")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The region code, specified as a ccTLD ("top-level domain")
    /// two-character value. For more information see [Country code top-level
    /// domains](<https://en.wikipedia.org/wiki/List_of_Internet_top-level_domains#Country_code_top-level_domains>).
    #[prost(string, tag = "16")]
    pub region_code: ::prost::alloc::string::String,
    /// Optional. Specifies the units of measure for the display fields. These
    /// fields include the `instruction` field in
    /// \[`NavigationInstruction`][google.maps.routing.v2.NavigationInstruction\].
    /// The units of measure used for the route, leg, step distance, and duration
    /// are not affected by this value. If you don't provide this value, then the
    /// display units are inferred from the location of the first origin.
    #[prost(enumeration = "Units", tag = "11")]
    pub units: i32,
    /// Optional. If set to true, the service attempts to minimize the overall cost
    /// of the route by re-ordering the specified intermediate waypoints. The
    /// request fails if any of the intermediate waypoints is a `via` waypoint. Use
    /// `ComputeRoutesResponse.Routes.optimized_intermediate_waypoint_index` to
    /// find the new ordering.
    /// If `ComputeRoutesResponseroutes.optimized_intermediate_waypoint_index` is
    /// not requested in the `X-Goog-FieldMask` header, the request fails.
    /// If `optimize_waypoint_order` is set to false,
    /// `ComputeRoutesResponse.optimized_intermediate_waypoint_index` will be
    /// empty.
    #[prost(bool, tag = "13")]
    pub optimize_waypoint_order: bool,
    /// Optional. Specifies what reference routes to calculate as part of the
    /// request in addition to the default route. A reference route is a route with
    /// a different route calculation objective than the default route. For example
    /// a `FUEL_EFFICIENT` reference route calculation takes into account various
    /// parameters that would generate an optimal fuel efficient route.
    #[prost(
        enumeration = "compute_routes_request::ReferenceRoute",
        repeated,
        packed = "false",
        tag = "14"
    )]
    pub requested_reference_routes: ::prost::alloc::vec::Vec<i32>,
    /// Optional. A list of extra computations which may be used to complete the
    /// request. Note: These extra computations may return extra fields on the
    /// response. These extra fields must also be specified in the field mask to be
    /// returned in the response.
    #[prost(
        enumeration = "compute_routes_request::ExtraComputation",
        repeated,
        packed = "false",
        tag = "15"
    )]
    pub extra_computations: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Specifies the assumptions to use when calculating time in
    /// traffic. This setting affects the value returned in the duration field in
    /// the
    /// \[`Route`][google.maps.routing.v2.Route\] and
    /// \[`RouteLeg`][google.maps.routing.v2.RouteLeg\] which contains the predicted
    /// time in traffic based on historical averages.
    /// `TrafficModel` is only available for requests that have set
    /// \[`RoutingPreference`][google.maps.routing.v2.RoutingPreference\] to
    /// `TRAFFIC_AWARE_OPTIMAL` and
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\] to `DRIVE`.
    /// Defaults to `BEST_GUESS` if traffic is requested and `TrafficModel` is not
    /// specified.
    #[prost(enumeration = "TrafficModel", tag = "18")]
    pub traffic_model: i32,
    /// Optional. Specifies preferences that influence the route returned for
    /// `TRANSIT` routes. NOTE: You can only specify a `transit_preferences` when
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\] is set to
    /// `TRANSIT`.
    #[prost(message, optional, tag = "20")]
    pub transit_preferences: ::core::option::Option<TransitPreferences>,
}
/// Nested message and enum types in `ComputeRoutesRequest`.
pub mod compute_routes_request {
    /// A supported reference route on the ComputeRoutesRequest.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReferenceRoute {
        /// Not used. Requests containing this value fail.
        Unspecified = 0,
        /// Fuel efficient route. Routes labeled with this value are determined to be
        /// optimized for parameters such as fuel consumption.
        FuelEfficient = 1,
    }
    /// Extra computations to perform while completing the request.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtraComputation {
        /// Not used. Requests containing this value will fail.
        Unspecified = 0,
        /// Toll information for the route(s).
        Tolls = 1,
        /// Estimated fuel consumption for the route(s).
        FuelConsumption = 2,
        /// Traffic aware polylines for the route(s).
        TrafficOnPolyline = 3,
        /// \[`NavigationInstructions`\](google.maps.routing.v2.NavigationInstructions.instructions)
        /// presented as a formatted HTML text string. This content
        /// is meant to be read as-is. This content is for display only.
        /// Do not programmatically parse it.
        HtmlFormattedNavigationInstructions = 4,
    }
}
/// ComputeRoutes the response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRoutesResponse {
    /// Contains an array of computed routes (up to three) when you specify
    /// `compute_alternatives_routes`, and contains just one route when you don't.
    /// When this array contains multiple entries, the first one is the most
    /// recommended route. If the array is empty, then it means no route could be
    /// found.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// In some cases when the server is not able to compute the route results with
    /// all of the input preferences, it may fallback to using a different way of
    /// computation. When fallback mode is used, this field contains detailed info
    /// about the fallback response. Otherwise this field is unset.
    #[prost(message, optional, tag = "2")]
    pub fallback_info: ::core::option::Option<FallbackInfo>,
    /// Contains geocoding response info for waypoints specified as addresses.
    #[prost(message, optional, tag = "3")]
    pub geocoding_results: ::core::option::Option<GeocodingResults>,
}
/// ComputeRouteMatrix request message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeRouteMatrixRequest {
    /// Required. Array of origins, which determines the rows of the response
    /// matrix. Several size restrictions apply to the cardinality of origins and
    /// destinations:
    ///
    /// * The sum of the number of origins + the number of destinations specified
    /// as either `place_id` or `address` must be no greater than 50.
    /// * The product of number of origins × number of destinations must be no
    /// greater than 625 in any case.
    /// * The product of the number of origins × number of destinations must be no
    /// greater than 100 if routing_preference is set to `TRAFFIC_AWARE_OPTIMAL`.
    /// * The product of the number of origins × number of destinations must be no
    /// greater than 100 if travel_mode is set to `TRANSIT`.
    #[prost(message, repeated, tag = "1")]
    pub origins: ::prost::alloc::vec::Vec<RouteMatrixOrigin>,
    /// Required. Array of destinations, which determines the columns of the
    /// response matrix.
    #[prost(message, repeated, tag = "2")]
    pub destinations: ::prost::alloc::vec::Vec<RouteMatrixDestination>,
    /// Optional. Specifies the mode of transportation.
    #[prost(enumeration = "RouteTravelMode", tag = "3")]
    pub travel_mode: i32,
    /// Optional. Specifies how to compute the route. The server attempts to use
    /// the selected routing preference to compute the route. If the routing
    /// preference results in an error or an extra long latency, an error is
    /// returned. You can specify this option only when the `travel_mode` is
    /// `DRIVE` or `TWO_WHEELER`, otherwise the request fails.
    #[prost(enumeration = "RoutingPreference", tag = "4")]
    pub routing_preference: i32,
    /// Optional. The departure time. If you don't set this value, then this value
    /// defaults to the time that you made the request.
    /// NOTE: You can only specify a `departure_time` in the past when
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\] is set to
    /// `TRANSIT`.
    #[prost(message, optional, tag = "5")]
    pub departure_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The arrival time.
    /// NOTE: Can only be set when
    /// \[`RouteTravelMode`][google.maps.routing.v2.RouteTravelMode\] is set to
    /// `TRANSIT`. You can specify either `departure_time` or `arrival_time`, but
    /// not both.
    #[prost(message, optional, tag = "11")]
    pub arrival_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see [Unicode Locale
    /// Identifier](<http://www.unicode.org/reports/tr35/#Unicode_locale_identifier>).
    /// See [Language
    /// Support](<https://developers.google.com/maps/faq#languagesupport>)
    /// for the list of supported languages. When you don't provide this value, the
    /// display language is inferred from the location of the first origin.
    #[prost(string, tag = "6")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. The region code, specified as a ccTLD ("top-level domain")
    /// two-character value. For more information see [Country code top-level
    /// domains](<https://en.wikipedia.org/wiki/List_of_Internet_top-level_domains#Country_code_top-level_domains>).
    #[prost(string, tag = "9")]
    pub region_code: ::prost::alloc::string::String,
    /// Optional. Specifies the units of measure for the display fields.
    #[prost(enumeration = "Units", tag = "7")]
    pub units: i32,
    /// Optional. A list of extra computations which may be used to complete the
    /// request. Note: These extra computations may return extra fields on the
    /// response. These extra fields must also be specified in the field mask to be
    /// returned in the response.
    #[prost(
        enumeration = "compute_route_matrix_request::ExtraComputation",
        repeated,
        packed = "false",
        tag = "8"
    )]
    pub extra_computations: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Specifies the assumptions to use when calculating time in
    /// traffic. This setting affects the value returned in the duration field in
    /// the \[RouteMatrixElement][google.maps.routing.v2.RouteMatrixElement\] which
    /// contains the predicted time in traffic based on historical averages.
    /// \[RoutingPreference][google.maps.routing.v2.RoutingPreference\] to
    /// `TRAFFIC_AWARE_OPTIMAL` and
    /// \[RouteTravelMode][google.maps.routing.v2.RouteTravelMode\] to `DRIVE`.
    /// Defaults to `BEST_GUESS` if traffic is requested and `TrafficModel` is not
    /// specified.
    #[prost(enumeration = "TrafficModel", tag = "10")]
    pub traffic_model: i32,
    /// Optional. Specifies preferences that influence the route returned for
    /// `TRANSIT` routes. NOTE: You can only specify a `transit_preferences` when
    /// \[RouteTravelMode][google.maps.routing.v2.RouteTravelMode\] is set to
    /// `TRANSIT`.
    #[prost(message, optional, tag = "12")]
    pub transit_preferences: ::core::option::Option<TransitPreferences>,
}
/// Nested message and enum types in `ComputeRouteMatrixRequest`.
pub mod compute_route_matrix_request {
    /// Extra computations to perform while completing the request.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtraComputation {
        /// Not used. Requests containing this value will fail.
        Unspecified = 0,
        /// Toll information for the matrix element(s).
        Tolls = 1,
    }
}
/// A single origin for ComputeRouteMatrixRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixOrigin {
    /// Required. Origin waypoint
    #[prost(message, optional, tag = "1")]
    pub waypoint: ::core::option::Option<Waypoint>,
    /// Optional. Modifiers for every route that takes this as the origin
    #[prost(message, optional, tag = "2")]
    pub route_modifiers: ::core::option::Option<RouteModifiers>,
}
/// A single destination for ComputeRouteMatrixRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixDestination {
    /// Required. Destination waypoint
    #[prost(message, optional, tag = "1")]
    pub waypoint: ::core::option::Option<Waypoint>,
}
/// Contains route information computed for an origin/destination pair in the
/// ComputeRouteMatrix API. This proto can be streamed to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatrixElement {
    /// Zero-based index of the origin in the request.
    #[prost(int32, optional, tag = "1")]
    pub origin_index: ::core::option::Option<i32>,
    /// Zero-based index of the destination in the request.
    #[prost(int32, optional, tag = "2")]
    pub destination_index: ::core::option::Option<i32>,
    /// Error status code for this element.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Indicates whether the route was found or not. Independent of status.
    #[prost(enumeration = "RouteMatrixElementCondition", tag = "9")]
    pub condition: i32,
    /// The travel distance of the route, in meters.
    #[prost(int32, tag = "4")]
    pub distance_meters: i32,
    /// The length of time needed to navigate the route. If you set the
    /// \[routing_preference][google.maps.routing.v2.ComputeRouteMatrixRequest.routing_preference\]
    /// to `TRAFFIC_UNAWARE`, then this value is the same as `static_duration`. If
    /// you set the `routing_preference` to either `TRAFFIC_AWARE` or
    /// `TRAFFIC_AWARE_OPTIMAL`, then this value is calculated taking traffic
    /// conditions into account.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// The duration of traveling through the route without taking traffic
    /// conditions into consideration.
    #[prost(message, optional, tag = "6")]
    pub static_duration: ::core::option::Option<::prost_types::Duration>,
    /// Additional information about the route. For example: restriction
    /// information and toll information
    #[prost(message, optional, tag = "7")]
    pub travel_advisory: ::core::option::Option<RouteTravelAdvisory>,
    /// In some cases when the server is not able to compute the route with the
    /// given preferences for this particular origin/destination pair, it may
    /// fall back to using a different mode of computation. When fallback mode is
    /// used, this field contains detailed information about the fallback response.
    /// Otherwise this field is unset.
    #[prost(message, optional, tag = "8")]
    pub fallback_info: ::core::option::Option<FallbackInfo>,
    /// Text representations of properties of the `RouteMatrixElement`.
    #[prost(message, optional, tag = "10")]
    pub localized_values: ::core::option::Option<route_matrix_element::LocalizedValues>,
}
/// Nested message and enum types in `RouteMatrixElement`.
pub mod route_matrix_element {
    /// Text representations of certain properties.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalizedValues {
        /// Travel distance represented in text form.
        #[prost(message, optional, tag = "1")]
        pub distance: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration represented in text form taking traffic conditions into
        /// consideration. Note: If traffic information was not requested, this value
        /// is the same value as static_duration.
        #[prost(message, optional, tag = "2")]
        pub duration: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Duration represented in text form without taking traffic conditions into
        /// consideration.
        #[prost(message, optional, tag = "3")]
        pub static_duration:
            ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
        /// Transit fare represented in text form.
        #[prost(message, optional, tag = "4")]
        pub transit_fare: ::core::option::Option<super::super::super::super::r#type::LocalizedText>,
    }
}
/// The condition of the route being returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteMatrixElementCondition {
    /// Only used when the `status` of the element is not OK.
    Unspecified = 0,
    /// A route was found, and the corresponding information was filled out for the
    /// element.
    RouteExists = 1,
    /// No route could be found. Fields containing route information, such as
    /// `distance_meters` or `duration`, will not be filled out in the element.
    RouteNotFound = 2,
}
#[doc = r" Generated client implementations."]
pub mod routes_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Routes API."]
    #[derive(Debug, Clone)]
    pub struct RoutesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoutesClient<T>
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
        ) -> RoutesClient<InterceptedService<T, F>>
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
            RoutesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the primary route along with optional alternate routes, given a set"]
        #[doc = " of terminal and intermediate waypoints."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using URL parameter"]
        #[doc = " `$fields` or `fields`, or by using an HTTP/gRPC header `X-Goog-FieldMask`"]
        #[doc = " (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters)). The value"]
        #[doc = " is a comma separated list of field paths. See detailed documentation about"]
        #[doc = " [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of Route-level duration, distance, and polyline (an example"]
        #[doc = " production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`"]
        #[doc = ""]
        #[doc = " Google discourage the use of the wildcard (`*`) response field mask, or"]
        #[doc = " specifying the field mask at the top level (`routes`), because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need"]
        #[doc = " in your production job ensures stable latency performance. We might add"]
        #[doc = " more response fields in the future, and those new fields might require"]
        #[doc = " extra computation time. If you select all fields, or if you select all"]
        #[doc = " fields at the top level, then you might experience performance degradation"]
        #[doc = " because any new field we add will be automatically included in the"]
        #[doc = " response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        pub async fn compute_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeRoutesRequest>,
        ) -> Result<tonic::Response<super::ComputeRoutesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routing.v2.Routes/ComputeRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Takes in a list of origins and destinations and returns a stream containing"]
        #[doc = " route information for each combination of origin and destination."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters))."]
        #[doc = " The value is a comma separated list of field paths. See this detailed"]
        #[doc = " documentation about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of route durations, distances, element status, condition, and"]
        #[doc = "   element indices (an example production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   originIndex,destinationIndex,status,condition,distanceMeters,duration`"]
        #[doc = ""]
        #[doc = " It is critical that you include `status` in your field mask as otherwise"]
        #[doc = " all messages will appear to be OK. Google discourages the use of the"]
        #[doc = " wildcard (`*`) response field mask, because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        pub async fn compute_route_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeRouteMatrixRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RouteMatrixElement>>,
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
                "/google.maps.routing.v2.Routes/ComputeRouteMatrix",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
