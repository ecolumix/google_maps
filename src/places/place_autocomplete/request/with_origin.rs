use crate::types::LatLng;
use crate::places::place_autocomplete::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the origin parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments:
    ///
    /// * `origin` ‧ The origin point from which to calculate straight-line
    /// distance to the destination (returned as `distance_meters`). If this
    /// value is omitted, straight-line distance will not be returned.

    pub fn with_origin(&'a mut self, origin: LatLng) -> &'a mut Request {
        // Set origin in Request struct.
        self.origin = Some(origin);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl