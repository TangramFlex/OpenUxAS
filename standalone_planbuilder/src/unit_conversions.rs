// WGS-84 parameters
const RADIUS_EQUATORIAL_M: f64 = 6378135.0;
const _FLATTENING: f64 = 3.352810664724998e-003;
const ECCENTRICITY_SQUARED: f64 = 6.694379990096503e-003;

pub struct LatLongDeg(pub f64, pub f64);
pub struct NorthEastM(pub f64, pub f64);

pub struct UnitConverter {
    latitude_initial_rad: f64,
    longitude_initial_rad: f64,
    radius_meridional_m: f64,
    radius_small_circle_latitude_m: f64
}

impl UnitConverter {
    pub fn new(latitude_init_rad: f64, longitude_init_rad: f64) -> UnitConverter {
        let denominator_meridional =
            (1.0 - (ECCENTRICITY_SQUARED * latitude_init_rad.sin().powf(2.0))).powf(3.0 / 2.0);
        assert!(denominator_meridional > 0.0);

        let radius_meridional_m =
            if denominator_meridional <= 0.0 {
                0.0
            } else {
                (RADIUS_EQUATORIAL_M * (1.0 - ECCENTRICITY_SQUARED)) / denominator_meridional
            };

        let denominator_transverse = (1.0 - (ECCENTRICITY_SQUARED * latitude_init_rad.sin().powf(2.0))).powf(0.5);
        assert!(denominator_transverse > 0.0);

        let radius_transverse_m =
            if denominator_transverse <= 0.0 {
                0.0
            } else {
                RADIUS_EQUATORIAL_M / denominator_transverse
            };

        let radius_small_circle_latitude_m = radius_transverse_m * latitude_init_rad.cos();

        UnitConverter {
            latitude_initial_rad: latitude_init_rad,
            longitude_initial_rad: longitude_init_rad,
            radius_meridional_m: radius_meridional_m,
            radius_small_circle_latitude_m: radius_small_circle_latitude_m,
        }
    }
}

impl Default for UnitConverter {
    fn default() -> UnitConverter {
        UnitConverter::new(0.0, 0.0)
    }
}

impl LatLongDeg {
    pub fn to_north_east_m(&self, uc: &UnitConverter) -> NorthEastM {
        let &LatLongDeg(lat_deg, long_deg) = self;
        let lat_rad = lat_deg.to_radians();
        let long_rad = long_deg.to_radians();

        let north_m =
            uc.radius_meridional_m * (lat_rad - uc.latitude_initial_rad);
        let east_m =
            uc.radius_small_circle_latitude_m * (long_rad - uc.longitude_initial_rad);

        NorthEastM(north_m, east_m)
    }
}

impl NorthEastM {
    pub fn to_lat_long_deg(&self, uc: &UnitConverter) -> LatLongDeg {
        let &NorthEastM(north_m, east_m) = self;
        assert!(uc.radius_meridional_m > 0.0);
        assert!(uc.radius_small_circle_latitude_m > 0.0);

        let lat_deg =
            if uc.radius_meridional_m <= 0.0 {
                0.0
            } else {
                ((north_m / uc.radius_meridional_m) + uc.latitude_initial_rad).to_degrees()
            };

        let long_deg =
            if uc.radius_small_circle_latitude_m <= 0.0 {
                0.0
            } else {
                ((east_m / uc.radius_small_circle_latitude_m) + uc.longitude_initial_rad).to_degrees()
            };

        LatLongDeg(lat_deg, long_deg)
    }
}
