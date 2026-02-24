// Package constants provides a set of reusable keys and static values across the app
// for uniformity.
package constants

import "time"

const (
	CookieNameSessionID = "foodbasket_sessionid"
	CookieSessionTTL    = 60 * time.Minute

	ApplicationRootDomain = "foodbasket.app"
	DefaultTenantID       = "default"

	ValkeySessionPrefix     = "foodbasket:auth:sess:"        // foodbasket:auth:sess:<session_id>
	ValkeyUserIndexPrefix   = "foodbasket:auth:uidx:"        // foodbasket:auth:uidx:<user_id>
	ValkeyTenantExists      = "foodbasket:registry:tenants:" // foodbasket:registry:tenants:<tenant_id>
	ValkeyTenantUUID        = "foodbasket:tenants:uuid:"     // foodbasket:tenants:uuid
	ValkeyPermissionsPrefix = "foodbasket:auth:perms:"       // foodbasket:auth:perms:<tenant_id>:<user_id>

	ValkeyNilValue   = "nil"
	ValkeyCacheTTL   = 5 * time.Minute
	ValkeySessionTTL = 30 * time.Minute
	ValkeyBoolYes    = "true"
	ValkeyBoolNo     = "false"
)

const (
	valkeySessionKey      = "foodbasket:auth:sess:"
	valkeySessionIndexKey = "foodbasket:auth:uidx:"
	valkeyTenantUUIDKey   = "foodbasket:tenants:uuid:"
	valkeyTenantSlugKey   = "foodbasket:tenants:slug:"
)

// Builds a key in Valkey that is used to map a tenant UUID -> hash set of variables,
// such as state or more business stuff that might be of use.
//
// foodbasket:tenants:uuid:<uuid>
func TenantUUIDKey(uuid string) string {
	return valkeyTenantUUIDKey + uuid
}

// Builds a key in Valkey that is used to map a tenant slug -> tenant UUID.
// This serves as a redirect to TenantUUIDKey.
//
// foodbasket:tenants:slug:<slug>
func TenantSlugKey(slug string) string {
	return valkeyTenantSlugKey + slug
}

// Builds a key in Valkey that is used to map a sessionID -> session data.
//
// foodbasket:auth:sess:<session_id>
func SessionKey(sessionID string) string {
	return valkeySessionKey + sessionID
}

// Builds a key in Valkey that is used to map a userID -> active sessions.
//
// foodbasket:auth:uidx:<user_id>
func SessionIndexKey(userID string) string {
	return valkeySessionIndexKey + userID
}
