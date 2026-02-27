package handler

import (
	"fmt"
	"net/http"
	"net/url"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/labstack/echo/v5"
	"github.com/samber/lo"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/dto"
	"luny.dev/foodbasket/internal/logging"
	"luny.dev/foodbasket/internal/services"
)

// CORSMiddleware provides a dynamic CORS endpoint for browsers.
func CORSMiddleware(tenantSvc services.ITenantService) echo.MiddlewareFunc {
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

			ctx := c.Request().Context()
			host := u.Hostname()

			if uuid, err := tenantSvc.GetTenantID(ctx, host); err != nil || uuid == nil {

			}

			// Check if there is a tenant with that hostname
			//  TODO: Implement caching for tenant hosts to reduce round trips to Valkey
			// if ok, err := valkey.Sismember(c.Request().Context(), constants.ValkeyRegistryTenantsKey, host); err == nil || !ok {
			// 	return next(c) // Not a host, blocking the browser
			// }

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
// when SessionHydrate starts to look, permissions can be read from Valkey easily using tenant_id.
func HostHydrate(tenantSvc services.ITenantService) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			host := c.Request().Host
			ctx := c.Request().Context()

			if host == "" {
				return echo.ErrBadRequest // Rare, but protect
			}

			forwarded := c.Request().Header.Get(echo.HeaderXForwardedFor)
			if forwarded != "" {
				host = forwarded
			}

			// Parse subdomain (assume format: tenant.[pos.]foodbasket.app)
			parts := strings.Split(host, ".")
			c.Set("tenant_id", constants.DefaultTenantID)
			if len(parts) < 3 {
				// Not matched any of pos. or tenant.
				return next(c)
			}

			// Handle for cases of tenant.foodbasket.app.
			// pos.foodbasket.app is special case, handle separately.
			if len(parts) == 3 {
				if strings.ToLower(parts[0]) == "pos" {
					// TODO: Show a separate landing page for POS.
					// Redirect to foodbasket.app
					return c.JSON(http.StatusServiceUnavailable, dto.MessageResponse{Message: "pos landing page not available yet"})
				}

				// It's a tenant's slug. We grab the UUID.
				uuid, err := tenantSvc.GetTenantID(ctx, strings.ToLower(parts[0]))
				if err != nil {
					// Somehow there's an error
					return c.JSON(http.StatusInternalServerError, dto.MessageResponse{Message: "unable to fetch tenant id"})
				}

				// Valid tenant ID, set the context
				if uuid != nil {
					c.Set("tenant_id", uuid.String())
				}

				return next(c)
			}

			// Handle for cases of tenant.pos.foodbasket.app
			// len(parts) should be > 3 here.
			if parts[1] != "pos" {
				// We don't know tenant.whatever.foodbasket.app as valid. Ignore.
				return next(c)
			}

			// Okay, now parts[0] should be the tenant id.
			uuid, err := tenantSvc.GetTenantID(ctx, strings.ToLower(parts[0]))
			if err != nil {
				// Somehow there's an error
				return c.JSON(http.StatusInternalServerError, dto.MessageResponse{Message: "unable to fetch tenant id"})
			}

			// Valid tenant ID, set the context
			if uuid != nil {
				c.Set("tenant_id", uuid.String())
			}

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
				if redirectID, ok := session.Metadata["rotated_to"]; ok {
					session, _ = sess.GetSession(ctx, redirectID.(string))
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

// PermissionHydrate hydrates the echo's context with the current tenant's permission,
// provided the tenant's host ID, the user ID, and the session ID has been considered
// valid by previous middlewares.
func PermissionHydrate(permissionSvc services.IPermissionService) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			session, ok := c.Get("session").(services.SessionData)
			if !ok || session.UserID == nil {
				return next(c) // no session to populate, or guest can't populate.
			}

			tenantID, ok := c.Get("tenant_id").(string)
			if !ok || tenantID == "" {
				return next(c) // no tenant to populate
			}

			perms, err := permissionSvc.GetUserPermissions(c.Request().Context(), uuid.MustParse(*session.UserID), uuid.MustParse(tenantID))
			if err != nil {
				logging.Error(c, "failed to populate permissions for session", "session", session, "tenant_id", tenantID)
				return next(c)
			}

			c.Set("permissions", perms)
			return next(c)
		}
	}
}

func HasPermission(permission string) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c *echo.Context) error {
			permissions, ok := c.Get("permissions").([]string)
			if ok && lo.Contains(permissions, permission) {
				return next(c) // allowed through
			}

			return echo.ErrForbidden.Wrap(fmt.Errorf("failed to meet permission %s", permission))
		}
	}
}
