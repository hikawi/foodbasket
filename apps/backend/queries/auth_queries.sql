-- name: GetUserByEmail :one
SELECT * FROM users
WHERE email = $1 AND deleted_at IS NULL LIMIT 1;

-- name: CreateUser :one
INSERT INTO users (name, email, password) VALUES ($1, $2, $3)
RETURNING *;

-- name: GetUserPermissions :many
SELECT permissions.id FROM permissions
INNER JOIN roles_permissions ON permissions.id = roles_permissions.permission_id
INNER JOIN roles ON roles_permissions.role_id = roles.id
INNER JOIN users_roles ON users_roles.role_id = roles.id
WHERE users_roles.user_id = $1 AND roles.tenant_id = $2 AND roles.deleted_at IS NULL;
