use crate::elevation::request::{locations::Locations, Request};
use crate::types::LatLng;

// =============================================================================

impl<'a> Request<'a> {

    // -------------------------------------------------------------------------
    //
    /// Adds the _positional request_ parameter to the Elevation API query.
    ///
    /// ## Arguments:
    ///
    /// * `location` ‧ Defines the location on the earth from which to
    /// return elevation data. This parameter takes a single `LatLng`
    /// coordinate.
    ///
    /// ## Example:
    ///
    /// ```rust
    /// // Denver, Colorado, the "Mile High City"
    /// .for_positional_request(LatLng::try_from_dec(dec!(39.7391536), dec!(-104.9847034))?)
    /// ```

    pub fn for_positional_request(
        &'a mut self,
        location: LatLng,
    ) -> &'a mut Request {
        // Set the path in Request struct.
        self.locations = Some(Locations::LatLngs(vec![location]));
        // Return modified Request struct to caller.
        self
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Adds a single _positional request_ parameter to the Elevation API query.
    ///
    /// ## Arguments:
    ///
    /// * `locations` ‧ Defines the location(s) on the earth from which to
    /// return elevation data. This parameter takes either a single location,
    /// as a latitude/longitude pair, multiple latitude/longitude pairs, or an
    /// encoded polyline. For more information, see [Specifying
    /// Locations](https://developers.google.com/maps/documentation/elevation/intro#Locations).
    ///
    /// ## Example:
    ///
    /// ```rust
    /// .for_positional_requests(ElevationLocations::LatLngs(vec![
    ///     // Denver, Colorado, the "Mile High City"
    ///     LatLng::try_from_dec(dec!(39.7391536), dec!(-104.9847034))?,
    ///     // Death Valley
    ///     LatLng::try_from_dec(dec!(36.23998), dec!(-116.83171))?,
    /// ]))
    /// ```

    pub fn for_positional_requests(
        &'a mut self,
        locations: Locations,
    ) -> &'a mut Request {
        // Set the path in Request struct.
        self.locations = Some(locations);
        // Return modified Request struct to caller.
        self
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Adds the _positional request_ parameter to the Elevation API query.
    ///
    /// This function is the same as `for_positional_request` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [Coord](https://docs.rs/geo/latest/geo/geometry/struct.Coord.html) type.
    ///
    /// ## Arguments:
    ///
    /// * `coordinate` ‧ Defines the location on the earth from which to
    /// return elevation data. This parameter takes a single `Coord`.

    #[cfg(feature = "geo")]
    pub fn for_coordinate_request(
        &'a mut self,
        coordinate: &geo_types::Coord,
    ) -> Result<&'a mut Request, crate::error::Error> {
        // Set the path in Request struct.
        self.locations = Some(Locations::LatLngs(vec![LatLng::try_from(coordinate)?]));
        // Return modified Request struct to caller.
        Ok(self)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Adds the _positional request_ parameter to the Elevation API query.
    ///
    /// This function is the same as `for_positional_request` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [Point](https://docs.rs/geo/latest/geo/geometry/struct.Point.html) type.
    ///
    /// ## Arguments:
    ///
    /// * `point` ‧ Defines the location on the earth from which to return
    /// elevation data. This parameter takes a single `Point`.

    #[cfg(feature = "geo")]
    pub fn try_point_request(
        &'a mut self,
        point: &geo_types::Point,
    ) -> Result<&'a mut Request, crate::error::Error> {
        // Set the path in Request struct.
        self.locations = Some(Locations::LatLngs(vec![LatLng::try_from(point)?]));
        // Return modified Request struct to caller.
        Ok(self)
    } // fn

} // impl