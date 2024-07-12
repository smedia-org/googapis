/// The weight represented as the value in string and the unit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Weight {
    /// Required. The weight represented as a number in micros (1 million micros is
    /// an equivalent to one's currency standard unit, for example, 1 kg = 1000000
    /// micros).
    /// This field can also be set as infinity by setting to -1.
    /// This field only support -1 and positive value.
    #[prost(int64, optional, tag = "1")]
    pub amount_micros: ::core::option::Option<i64>,
    /// Required. The weight unit.
    /// Acceptable values are: kg and lb
    #[prost(enumeration = "weight::WeightUnit", tag = "2")]
    pub unit: i32,
}
/// Nested message and enum types in `Weight`.
pub mod weight {
    /// The weight unit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WeightUnit {
        /// unit unspecified
        Unspecified = 0,
        /// lb unit.
        Pound = 1,
        /// kg unit.
        Kilogram = 2,
    }
}
/// The price represented as a number and currency.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The price represented as a number in micros (1 million micros is an
    /// equivalent to one's currency standard unit, for example, 1 USD = 1000000
    /// micros).
    #[prost(int64, optional, tag = "1")]
    pub amount_micros: ::core::option::Option<i64>,
    /// The currency of the price using three-letter acronyms according to [ISO
    /// 4217](<http://en.wikipedia.org/wiki/ISO_4217>).
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message that represents custom attributes. Exactly one of `value` or
/// `group_values` must not be empty.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The name of the attribute.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The value of the attribute. If `value` is not empty, `group_values` must be
    /// empty.
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Subattributes within this attribute group.  If
    /// `group_values` is not empty, `value` must be empty.
    #[prost(message, repeated, tag = "3")]
    pub group_values: ::prost::alloc::vec::Vec<CustomAttribute>,
}
/// Destinations available for a product.
///
/// Destinations are used in Merchant Center to allow you to control where the
/// products from your data feed should be displayed.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {}
/// Nested message and enum types in `Destination`.
pub mod destination {
    /// Destination values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DestinationEnum {
        /// Not specified.
        Unspecified = 0,
        /// [Shopping ads](<https://support.google.com/google-ads/answer/2454022>).
        ShoppingAds = 1,
        /// [Display ads](<https://support.google.com/merchants/answer/6069387>).
        DisplayAds = 2,
        /// [Local inventory
        /// ads](<https://support.google.com/merchants/answer/3057972>).
        LocalInventoryAds = 3,
        /// [Free listings](<https://support.google.com/merchants/answer/9199328>).
        FreeListings = 4,
        /// [Free local product
        /// listings](<https://support.google.com/merchants/answer/9825611>).
        FreeLocalListings = 5,
        /// [YouTube Shopping](<https://support.google.com/merchants/answer/12362804>).
        YoutubeShopping = 6,
    }
}
/// Reporting contexts that your account and product issues apply to.
///
/// Reporting contexts are groups of surfaces and formats for product results on
/// Google. They can represent the entire destination (for example, [Shopping
/// ads](<https://support.google.com/merchants/answer/6149970>)) or a subset of
/// formats within a destination (for example, [Demand Gen
/// ads](<https://support.google.com/merchants/answer/13389785>)).
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportingContext {}
/// Nested message and enum types in `ReportingContext`.
pub mod reporting_context {
    /// Reporting context values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReportingContextEnum {
        /// Not specified.
        Unspecified = 0,
        /// [Shopping ads](<https://support.google.com/merchants/answer/6149970>).
        ShoppingAds = 1,
        /// Deprecated:  Use `DEMAND_GEN_ADS` instead.
        /// [Discovery and Demand Gen
        /// ads](<https://support.google.com/merchants/answer/13389785>).
        DiscoveryAds = 2,
        /// [Demand Gen ads](<https://support.google.com/merchants/answer/13389785>).
        DemandGenAds = 13,
        /// [Demand Gen ads on Discover
        /// surface](<https://support.google.com/merchants/answer/13389785>).
        DemandGenAdsDiscoverSurface = 14,
        /// [Video ads](<https://support.google.com/google-ads/answer/6340491>).
        VideoAds = 3,
        /// [Display ads](<https://support.google.com/merchants/answer/6069387>).
        DisplayAds = 4,
        /// [Local inventory
        /// ads](<https://support.google.com/merchants/answer/3271956>).
        LocalInventoryAds = 5,
        /// [Vehicle inventory
        /// ads](<https://support.google.com/merchants/answer/11544533>).
        VehicleInventoryAds = 6,
        /// [Free product
        /// listings](<https://support.google.com/merchants/answer/9199328>).
        FreeListings = 7,
        /// [Free local product
        /// listings](<https://support.google.com/merchants/answer/9825611>).
        FreeLocalListings = 8,
        /// [Free local vehicle
        /// listings](<https://support.google.com/merchants/answer/11544533>).
        FreeLocalVehicleListings = 9,
        /// [YouTube
        /// Shopping](<https://support.google.com/merchants/answer/13478370>).
        YoutubeShopping = 10,
        /// [Cloud retail](<https://cloud.google.com/solutions/retail>).
        CloudRetail = 11,
        /// [Local cloud retail](<https://cloud.google.com/solutions/retail>).
        LocalCloudRetail = 12,
    }
}
/// \[Channel\](<https://support.google.com/merchants/answer/7361332>) of a product.
///
/// Channel is used to distinguish between online and local products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {}
/// Nested message and enum types in `Channel`.
pub mod channel {
    /// Channel values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChannelEnum {
        /// Not specified.
        Unspecified = 0,
        /// Online product.
        Online = 1,
        /// Local product.
        Local = 2,
    }
}
