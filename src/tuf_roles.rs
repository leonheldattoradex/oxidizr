use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoleEnum {
    Root = 0,
    Snapshot = 1,
    Targets = 2,
    Timestamp = 3,
    Delegation = 4,
    OfflineSnapshot = 5,
    OfflineUpdates = 6,
    InvalidRole = -1,
}

#[derive(Debug, Clone, Eq)]
pub struct Role {
    role: RoleEnum,
    name: String,
}

impl Role {
    pub const ROOT: &'static str = "root";
    pub const SNAPSHOT: &'static str = "snapshot";
    pub const TARGETS: &'static str = "targets";
    pub const TIMESTAMP: &'static str = "timestamp";
    pub const OFFLINESNAPSHOT: &'static str = "offlinesnapshot";
    pub const OFFLINEUPDATES: &'static str = "offlineupdates";

    // Standard role constructors
    pub fn root() -> Self {
        Role::new(RoleEnum::Root)
    }

    pub fn snapshot() -> Self {
        Role::new(RoleEnum::Snapshot)
    }

    pub fn targets() -> Self {
        Role::new(RoleEnum::Targets)
    }

    pub fn timestamp() -> Self {
        Role::new(RoleEnum::Timestamp)
    }

    pub fn offline_snapshot() -> Self {
        Role::new(RoleEnum::OfflineSnapshot)
    }

    pub fn offline_updates() -> Self {
        Role::new(RoleEnum::OfflineUpdates)
    }

    pub fn invalid_role() -> Self {
        Role::new(RoleEnum::InvalidRole)
    }

    // Delegation role
    pub fn delegation(name: &str) -> Self {
        Role {
            role: RoleEnum::Delegation,
            name: name.to_string(),
        }
    }

    // Create a role from RoleEnum
    pub fn new(role_enum: RoleEnum) -> Self {
        let name = match role_enum {
            RoleEnum::Root => Role::ROOT.to_string(),
            RoleEnum::Snapshot => Role::SNAPSHOT.to_string(),
            RoleEnum::Targets => Role::TARGETS.to_string(),
            RoleEnum::Timestamp => Role::TIMESTAMP.to_string(),
            RoleEnum::OfflineSnapshot => Role::OFFLINESNAPSHOT.to_string(),
            RoleEnum::OfflineUpdates => Role::OFFLINEUPDATES.to_string(),
            _ => "invalidrole".to_string(),
        };

        Role {
            role: role_enum,
            name,
        }
    }

    // Return all standard roles
    pub fn roles() -> Vec<Role> {
        vec![
            Role::root(),
            Role::snapshot(),
            Role::targets(),
            Role::timestamp(),
            Role::offline_snapshot(),
            Role::offline_updates(),
        ]
    }

    // Check if a role is reserved
    pub fn is_reserved(name: &str) -> bool {
        matches!(
            name,
            Role::ROOT
                | Role::SNAPSHOT
                | Role::TARGETS
                | Role::TIMESTAMP
                | Role::OFFLINESNAPSHOT
                | Role::OFFLINEUPDATES
        )
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }

    pub fn to_int(&self) -> i32 {
        self.role as i32
    }

    pub fn is_delegation(&self) -> bool {
        self.role == RoleEnum::Delegation
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl std::cmp::PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
