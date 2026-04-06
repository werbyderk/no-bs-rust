#![allow(dead_code)]

enum Vehicle {
    Sedan {
        engine_displacement: f32,
    },
    Truck {
        engine_displacement: f32,
        four_wheel_drive: bool,
    },
    SemiTruck {
        engine_displacement: f32,
        cargo_loaded: bool,
    },
}

// ** START EDITS HERE **

fn is_4wd(vehicle: Vehicle) -> bool {
    // return whether the input vehicle is four wheel drive or not
    let Vehicle::Truck {
        engine_displacement: _engine_displacement,
        four_wheel_drive,
    } = vehicle
    else {
        return false;
    };

    four_wheel_drive

    // if .. let or match .. are also valid solutions
}

fn semi_displacement(semi: Vehicle) -> Option<f32> {
    // return the engine displacement of the input vehicle only if it is a semi truck
    // otherwise return None
    match semi {
        Vehicle::SemiTruck {
            engine_displacement,
            cargo_loaded: _cargo_loaded,
        } => Some(engine_displacement),
        _ => None,
    }

    // if .. let or let .. else are also valid solutions
}

// ** END EDITS HERE **

#[cfg(test)]
pub mod tests {
    use super::Vehicle;

    #[test]
    fn is_4wd() {
        let veh = Vehicle::Sedan {
            engine_displacement: 3.0,
        };
        assert!(!super::is_4wd(veh));

        let veh = Vehicle::Truck {
            engine_displacement: 2.7,
            four_wheel_drive: true,
        };
        assert!(super::is_4wd(veh));

        let veh = Vehicle::Truck {
            engine_displacement: 5.4,
            four_wheel_drive: false,
        };
        assert!(!super::is_4wd(veh));
    }

    #[test]
    fn semi_displacement() {
        let veh = Vehicle::Sedan {
            engine_displacement: 4.0,
        };
        let maybe_displacement = super::semi_displacement(veh);
        assert_eq!(maybe_displacement, None);

        let veh = Vehicle::Truck {
            engine_displacement: 3.7,
            four_wheel_drive: false,
        };
        let maybe_displacement = super::semi_displacement(veh);
        assert_eq!(maybe_displacement, None);

        let semi_displacement: f32 = 13.7;
        let veh = Vehicle::SemiTruck {
            engine_displacement: semi_displacement,
            cargo_loaded: false,
        };
        let maybe_displacement = super::semi_displacement(veh);
        assert_eq!(maybe_displacement, Some(semi_displacement));
    }
}
