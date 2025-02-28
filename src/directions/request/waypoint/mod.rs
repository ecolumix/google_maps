//! Contains the `Waypoint` enum and its associated traits. It is used to
//! specify intermediate locations in the form of a text address,
//! a latitude & longitude pair, a Google Place ID, or as an Encoded Polyline.

#[cfg(feature = "geo")]
mod geo_conversions;

// -----------------------------------------------------------------------------

use crate::types::LatLng;

// -----------------------------------------------------------------------------
//
/// Used to specify pass throughs or stopovers at intermediate locations.

#[cfg(not(feature = "geo"))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum Waypoint {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to latitude & longitude coordinates to calculate directions.
    /// This coordinate may be different from that returned by the Geocoding
    /// API, for example a building entrance rather than its center.
    Address(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions.
    LatLng(LatLng),
    /// The place ID may only be specified if the request includes an API key or
    /// a Google Maps Platform Premium Plan client ID. You can retrieve place
    /// IDs from the Geocoding API and the Places API (including Place
    /// Autocomplete). For an example using place IDs from Place Autocomplete,
    /// see [Place Autocomplete and
    /// Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID
    /// overview](https://developers.google.com/places/place-id).
    PlaceId(String),
    /// Alternatively, you can supply an encoded set of points using the
    /// [Encoded Polyline
    /// Algorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    /// You will find an encoded set is useful for a large number of waypoints,
    /// because the URL is significantly shorter. All web services have a URL
    /// limit of 8192 characters.
    Polyline(String),
} // enum

// -----------------------------------------------------------------------------

#[cfg(not(feature = "geo"))]
impl std::convert::From<&Waypoint> for String {
    /// Converts a `Waypoint` enum to a `String` that contains a
    /// [waypoint](https://developers.google.com/maps/documentation/directions/intro#Waypoints)
    /// value.
    fn from(waypoint: &Waypoint) -> String {
        match waypoint {
            Waypoint::Address(address) => address.clone(),
            Waypoint::LatLng(latlng) => String::from(latlng),
            Waypoint::PlaceId(place_id) => format!("place_id:{place_id}"),
            Waypoint::Polyline(polyline) => format!("enc:{polyline}:"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------
//
/// Used to specify pass throughs or stopovers at intermediate locations.

#[cfg(feature = "geo")]
#[derive(Clone, Debug, PartialEq)]
pub enum Waypoint {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to latitude & longitude coordinates to calculate directions.
    /// This coordinate may be different from that returned by the Geocoding
    /// API, for example a building entrance rather than its center.
    Address(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions.
    LatLng(LatLng),
    /// The place ID may only be specified if the request includes an API key or
    /// a Google Maps Platform Premium Plan client ID. You can retrieve place
    /// IDs from the Geocoding API and the Places API (including Place
    /// Autocomplete). For an example using place IDs from Place Autocomplete,
    /// see [Place Autocomplete and
    /// Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID
    /// overview](https://developers.google.com/places/place-id).
    PlaceId(String),
    /// Alternatively, you can supply an encoded set of points using the
    /// [Encoded Polyline
    /// Algorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    /// You will find an encoded set is useful for a large number of waypoints,
    /// because the URL is significantly shorter. All web services have a URL
    /// limit of 8192 characters.
    Polyline(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions. This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [Coord](https://docs.rs/geo/latest/geo/geometry/struct.Coord.html) type.
    Coord(geo_types::geometry::Coord),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions. This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [Point](https://docs.rs/geo/latest/geo/geometry/struct.Point.html) type.
    Point(geo_types::geometry::Point),
} // enum

// -----------------------------------------------------------------------------

#[cfg(feature = "geo")]
impl std::convert::From<&Waypoint> for String {
    /// Converts a `Waypoint` enum to a `String` that contains a
    /// [waypoint](https://developers.google.com/maps/documentation/directions/intro#Waypoints)
    /// value.
    fn from(waypoint: &Waypoint) -> String {
        match waypoint {

            Waypoint::Address(address) =>
                address.clone(),

            Waypoint::LatLng(latlng) =>
                String::from(latlng),

            Waypoint::PlaceId(place_id) =>
                format!("place_id:{place_id}"),

            Waypoint::Polyline(polyline) =>
                format!("enc:{polyline}:"),

            Waypoint::Coord(coordinate) =>
                format!("{latitude},{longitude}", latitude=coordinate.y, longitude=coordinate.x),

            Waypoint::Point(point) =>
                format!("{latitude},{longitude}", latitude=point.y(), longitude=point.x()),

        } // match
    } // fn
} // impl