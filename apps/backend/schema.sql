-- Multi-Tenancy portion.
-- Our restaurants are here.

CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE UNIQUE INDEX uidx_tenants_slug ON tenants(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_tenants_deleted_at ON tenants(deleted_at);

CREATE TABLE branches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE INDEX idx_branches_tenant_id ON branches(tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_branches_deleted_at ON branches(deleted_at);

-- User Identities portion.
-- How do we check who is doing what?

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE UNIQUE INDEX uidx_users_email ON users(LOWER(email)) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_deleted_at ON users(deleted_at);

CREATE TABLE customer_profiles ( -- Explicitly ONLY for customers ordering. Kinda like swapping accounts in Discord/Slack/Atlassian.
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE UNIQUE INDEX uidx_customer_profiles_user_id_tenant_id ON customer_profiles(user_id, tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_customer_profiles_deleted_at ON customer_profiles(deleted_at);

CREATE TABLE staff_profiles ( -- Staff's specific profiles for RBAC at tenant level.
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    pin_code TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE UNIQUE INDEX uidx_staff_profiles_user_id_tenant_id ON staff_profiles(user_id, tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_staff_profiles_deleted_at ON staff_profiles(deleted_at);

-- Role-based Access Control or Attribute-based Access Control
-- How do we know who can do what?

CREATE TABLE permissions ( -- This thing doesn't have deleted_at because it should be hard-coded.
    id TEXT PRIMARY KEY, -- Basically stuff like pos:orders:create
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE policies ( -- It's basically ROLES, but better formed.
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id), -- NULL means platform-wide
    branch_id UUID REFERENCES branches(id), -- NULL means tenant-wide
    name TEXT NOT NULL,
    statements JSONB NOT NULL, -- Stuff like AWS [{"action": "pos:menus:read", "effect": "allow/deny"}]
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE INDEX idx_policies_tenant_id_branch_id ON policies(tenant_id, branch_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_policies_deleted_at ON policies(deleted_at);

CREATE TYPE scope_type AS ENUM ('system', 'tenant', 'branch');

CREATE TABLE assignments ( -- Who gets what policies. Is this necessary if we have tenant_id already? Should we add branch_id NULLABLE into policies?
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    staff_profile_id UUID NOT NULL REFERENCES staff_profiles(id),
    policy_id UUID NOT NULL REFERENCES policies(id),
    scope_type scope_type NOT NULL,
    scope_id UUID NOT NULL, -- if scope_type is branch, this is a branch_id, or tenant then tenant_id. If system then this is Zero UUID.
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX uidx_assignments_staff_profile_id_policy_id_scope_id ON assignments(staff_profile_id, policy_id, scope_id);

