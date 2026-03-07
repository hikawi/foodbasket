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
    avatar_url TEXT DEFAULT NULL,
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
    avatar_url TEXT DEFAULT NULL,
    pin_code TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE UNIQUE INDEX uidx_staff_profiles_user_id_tenant_id ON staff_profiles(user_id, tenant_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_staff_profiles_deleted_at ON staff_profiles(deleted_at);

CREATE TABLE system_profiles ( -- System-wide profiles
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    avatar_url TEXT DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL
);
CREATE UNIQUE INDEX uidx_system_profiles_user_id ON system_profiles(user_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_system_profiles_deleted_at ON system_profiles(deleted_at);

-- Role-based Access Control or Attribute-based Access Control
-- How do we know who can do what?

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

CREATE TABLE assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- Exactly ONE of these must be filled.
    staff_profile_id    UUID REFERENCES staff_profiles(id) ON DELETE CASCADE,
    customer_profile_id UUID REFERENCES customer_profiles(id) ON DELETE CASCADE,
    system_profile_id   UUID REFERENCES system_profiles(id) ON DELETE CASCADE,
    policy_id           UUID NOT NULL REFERENCES policies(id),
    scope_type          scope_type NOT NULL,
    scope_id            UUID NOT NULL, 
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT one_profile_only CHECK (
        (staff_profile_id IS NOT NULL)::int + 
        (customer_profile_id IS NOT NULL)::int + 
        (system_profile_id IS NOT NULL)::int = 1
    )
);
CREATE UNIQUE INDEX uidx_assignments_staff_profile_id ON assignments(staff_profile_id, policy_id, scope_id) WHERE staff_profile_id IS NOT NULL;
CREATE UNIQUE INDEX uidx_assignments_customer_profile_id ON assignments(customer_profile_id, policy_id, scope_id) WHERE customer_profile_id IS NOT NULL;
CREATE UNIQUE INDEX uidx_assignments_system_profile_id ON assignments(system_profile_id, policy_id, scope_id) WHERE system_profile_id IS NOT NULL;
