/// A key in Redis that maps a session ID -> session object.
pub fn session(id: &str) -> String {
    format!("foodbasket:sess:{id}")
}

/// A key in Redis that maps a tenant's slug -> tenant's uuid / NF.
pub fn tenant_slug(slug: &str) -> String {
    format!("foodbasket:tenants:slug:{slug}")
}

/// A key in Redis that maps a tenant uuid -> "active or no".
pub fn tenant_uuid(uuid: &str) -> String {
    format!("foodbasket:tenants:uuid:{uuid}")
}

/// A key in Redis that maps a tenant UUID -> set of branches.
pub fn tenant_branches(uuid: &str) -> String {
    format!("foodbasket:tenants:branches:{uuid}")
}
