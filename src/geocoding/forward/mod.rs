//! **Look in this module for documentation on building your _Geocoding API_
//! forward query**. In particular, look at the _ForwardRequest_ struct for
//! examples of the builder pattern. This module contains the tools (enums,
//! structs, methods) for building your Google Maps Platform request. Forward
//! geocoding converts a street address to latitude & longitude coordinates.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod new;
mod query_url;
mod validate;
mod with_address;
mod with_place_id;
mod with_bounds;
mod with_components;
mod with_language;
mod with_region;
pub mod component;

// -----------------------------------------------------------------------------

use crate::client::GoogleMapsClient;
use crate::geocoding::forward::component::Component;
use crate::types::{Bounds, Language, Region};

// -----------------------------------------------------------------------------

/// **Look at this `Request` struct for documentation on how to build your
/// _Geocoding API_ query**. The methods implemented for this struct are what's
/// used to build your request. Forward geocoding looks up a longitude &
/// latitude coordinates from a street address.

#[derive(Debug)]
pub struct ForwardRequest<'a> {

    // Required parameters:
    // --------------------

    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a GoogleMapsClient,

    // Optional parameters:
    // --------------------

    /// The street address that you want to geocode, in the format used by the
    /// national postal service of the country concerned. Additional address
    //elements such as business names and unit, suite or floor numbers should be
    /// avoided. Please refer to [the
    /// FAQ](https://developers.google.com/maps/faq#geocoder_queryformat) for
    /// additional guidance.
    address: Option<String>,

    /// The place ID of the place for which you wish to obtain the
    /// human-readable address. The place ID is a unique identifier that can
    /// be used with other Google APIs. For example, you can use the placeID
    /// returned by the [Roads
    /// API](https://developers.google.com/maps/documentation/roads/snap) to
    /// get the address for a snapped point.
    /// For more information about place IDs, see the place [ID
    /// overview](https://developers.google.com/maps/documentation/places/web-service/place-id)..
    place_id: Option<String>,

    /// The bounding box of the viewport within which to bias geocode results
    /// more prominently. This parameter will only influence, not fully
    /// restrict, results from the geocoder. (For more information see [Viewport
    /// Biasing](https://developers.google.com/maps/documentation/geocoding/intro#Viewports).)
    bounds: Option<Bounds>,

    /// A components filter with elements. The components filter is also
    /// accepted as an optional parameter if an `address` is provided. Each
    /// element in the components filter fully restricts the results from the
    /// geocoder. See more information about [component
    /// filtering](https://developers.google.com/maps/documentation/geocoding/intro#ComponentFiltering).
    components: Option<Vec<Component>>,

    /// The language in which to return results.
    language: Option<Language>,

    /// The region code, specified as a ccTLD ("top-level domain") two-character
    /// value. This parameter will only influence, not fully restrict, results
    /// from the geocoder. (For more information see [Region
    /// Biasing](https://developers.google.com/maps/documentation/geocoding/intro#RegionCodes)
    /// below.)
    region: Option<Region>,

    // Internal use only:
    // ------------------

    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,

    /// Has the request been validated?
    validated: bool,

} // struct