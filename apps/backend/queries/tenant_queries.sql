-- name: GetAllTenants :many
SELECT * FROM tenants
WHERE deleted_at IS NULL
ORDER BY id;

-- name: GetTenants :many
SELECT * FROM tenants
WHERE deleted_at IS NULL
ORDER BY id
OFFSET $1 LIMIT $2;

