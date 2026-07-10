use zbus_systemd::zvariant::OwnedObjectPath;

#[derive(Debug)]
pub struct SystemdService {
    pub name: String,
    pub description: String,
    pub load_state: String,
    pub active_state: String,
    pub sub_state: String,
    pub object_path: OwnedObjectPath
}

type SystemdUnitTuple = (String, String, String, String, String, String, OwnedObjectPath, u32, String, OwnedObjectPath);

impl From<SystemdUnitTuple> for SystemdService {
    fn from(unit: SystemdUnitTuple) -> Self {
        SystemdService {
            name: unit.0,
            description: unit.1,
            load_state: unit.2,
            active_state: unit.3,
            sub_state: unit.4,
            object_path: unit.6
        }
    }
}

