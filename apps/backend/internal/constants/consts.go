// Package constants provides a set of reusable keys and static values across the app
// for uniformity.
package constants

import "time"

const (
	CookieNameSessionID = "foodbasket_sessionid"
	CookieSessionTTL    = 60 * time.Minute

	ValkeySessionPrefix      = "foodbasket:auth:sess:" // foodbasket:auth:sess:<session_id>
	ValkeyUserIndexPrefix    = "foodbasket:auth:uidx:" // foodbasket:auth:uidx:<user_id>
	ValkeyRegistryTenantsKey = "foodbasket:registry:tenants"
	ValkeyPermissionsPrefix  = "foodbasket:auth:perms:" // foodbasket:auth:perms:<tenant_id>:<user_id>
	ValkeySessionTTL         = 30 * time.Minute
)
