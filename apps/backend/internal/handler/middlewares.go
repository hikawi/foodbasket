package handler

import (
	"net/http"
	"net/url"
	"strings"
	"time"

	"github.com/labstack/echo/v5"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/services"
)

// CORSMiddleware provides a dynamic CORS endpoint for browsers.
func CORSMiddleware(valkey services.IValkeyService) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			origin := c.Request().Header.Get(echo.HeaderOrigin)
			if origin == "" {
				return next(c) // Not a browser's request.
			}

			u, err := url.Parse(origin)
			if err != nil {
				return c.NoContent(http.StatusBadRequest)
			}

			host := u.Hostname()

			// Check if there is a tenant with that hostname
			//  TODO: Implement caching for tenant hosts to reduce round trips to Valkey
			if ok, err := valkey.Sismember(c.Request().Context(), constants.ValkeyRegistryTenantsKey, host); err == nil || !ok {
				return next(c) // Not a host, blocking the browser
			}

			res := c.Response()
			res.Header().Set(echo.HeaderAccessControlAllowOrigin, origin)
			res.Header().Set(echo.HeaderAccessControlAllowMethods, "GET,POST,PUT,PATCH,DELETE,OPTIONS")
			res.Header().Set(echo.HeaderAccessControlAllowHeaders, "Content-Type, Authorization")
			res.Header().Set(echo.HeaderAccessControlAllowCredentials, "true")

			// Preflight OPTIONS
			if c.Request().Method == http.MethodOptions {
				return c.NoContent(http.StatusNoContent)
			}

			return next(c)
		}
	}
}

// HostHydrate populates the echo's context with the current host, just to make sure that
// when SessionHydrate starts to look, permissions can be read from Valkey easily using tenant_id
// using a key like foodbasket:auth:perms:tenant_id:user_id?
func HostHydrate() echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			host := c.Request().Host
			if host == "" {
				return echo.ErrBadRequest // Rare, but protect
			}

			forwarded := c.Request().Header.Get(echo.HeaderXForwardedFor)
			if forwarded != "" {
				host = forwarded
			}

			// Parse subdomain (assume format: [pos.]tenant.foodbasket.app)
			parts := strings.Split(host, ".")
			if len(parts) < 3 { // Adjust based on your TLD
				c.Set("tenant_id", "main") // Fallback hard-coded main's tenant.
				return next(c)
			}

			subdomain := parts[0]

			// TODO:
			// I don't know what to do with domain spoofing in host yet.
			// But I hope I can get it done somewhere soon. But seems OOS for this right now.

			var tenantID string
			isPOS := false

			if subdomain == "pos" && len(parts) >= 4 {
				tenantID = parts[1]
				isPOS = true
			} else {
				tenantID = subdomain
			}

			c.Set("tenant_id", tenantID)
			c.Set("is_pos", isPOS)
			return next(c)
		}
	}
}

// SessionHydrate populates the echo's context with the current request's cookies,
// involving authentication, what they have already set, and where they're coming from.
func SessionHydrate(sess services.ISessionService) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			ctx := c.Request().Context()
			cookie, err := c.Cookie(constants.CookieNameSessionID)

			var session *services.SessionData

			if err == nil {
				session, _ = sess.GetSession(ctx, cookie.Value)
			}

			// Some pre-check before defaulting back to guests.
			// Check if the session is currently directing somewhere else.
			if session != nil {
				if redirectId, ok := session.Metadata["rotated_to"]; ok {
					session, _ = sess.GetSession(ctx, redirectId.(string))
				}
			}

			// Init guest if no session found.
			if session == nil {
				// We don't save to Valkey yet to prevent "Bot Bloat"
				// Thanks Gemini for this warning.
				session = &services.SessionData{
					Metadata:  make(map[string]any),
					IsGuest:   true,
					CreatedAt: time.Now(),
				}
			}

			c.Set("session", *session)
			return next(c)
		}
	}
}

// TenantHydrate hydrates the echo's context with the current tenant's permission,
// provided the tenant's host ID, the user ID, and the session ID has been considered
// valid by previous middlewares.
func TenantHydrate(valkey services.IValkeyService) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			return next(c)
		}
	}
}
