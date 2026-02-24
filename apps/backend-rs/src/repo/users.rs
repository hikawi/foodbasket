/*

-- name: CreateUser :one
INSERT INTO users (name, email, password) VALUES ($1, $2, $3)
RETURNING *;

-- name: GetUserPermissions :many
SELECT permissions.id FROM permissions
INNER JOIN roles_permissions ON permissions.id = roles_permissions.permission_id
INNER JOIN roles ON roles_permissions.role_id = roles.id
INNER JOIN users_roles ON users_roles.role_id = roles.id
WHERE users_roles.user_id = $1 AND roles.tenant_id = $2 AND roles.deleted_at IS NULL;
 */

use sqlx::PgPool;

use crate::models::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL"#,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
