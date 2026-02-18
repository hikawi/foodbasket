-- name: GetTenantByID :one
SELECT * FROM tenants
WHERE deleted_at IS NULL AND id = $1
LIMIT 1;

-- name: GetTenantBySlug :one
SELECT * FROM tenants
WHERE deleted_at IS NULL AND LOWER(slug) = LOWER($1)
LIMIT 1;

-- name: GetAllTenants :many
SELECT * FROM tenants
WHERE deleted_at IS NULL
ORDER BY id;

-- name: GetTenants :many
SELECT * FROM tenants
WHERE deleted_at IS NULL
ORDER BY id
OFFSET $1 LIMIT $2;

