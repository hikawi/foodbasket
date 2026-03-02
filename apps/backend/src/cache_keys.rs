/// A key in Redis that maps a session ID -> session object.
pub fn session(id: &str) -> String {
    format!("foodbasket:sess:{id}")
}

/// A key in Redis that maps a tenant's slug -> tenant's uuid.
pub fn tenant_slug(slug: &str) -> String {
    format!("foodbasket:tenants:slug:{slug}")
}

/// A key in Redis that maps
pub fn tenant_uuid(uuid: &str) -> String {
    format!("foodbasket:tenants:uuid:{uuid}")
}
