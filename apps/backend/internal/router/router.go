// Package router provides a set of routing.
package router

import (
	"github.com/go-playground/validator/v10"
	"github.com/labstack/echo/v5"
	"github.com/labstack/echo/v5/middleware"
	"luny.dev/foodbasket/internal/app"
	"luny.dev/foodbasket/internal/handler"
	"luny.dev/foodbasket/internal/services"
)

type CustomValidator struct {
	validator *validator.Validate
}

func (cv *CustomValidator) Validate(i any) error {
	if err := cv.validator.Struct(i); err != nil {
		return err
	}
	return nil
}

func SetupServer(serviceRegistry services.ServiceRegistry) *echo.Echo {
	e := echo.New()
	e.Use(middleware.RemoveTrailingSlash(), middleware.Recover())
	e.Use(handler.CORSMiddleware(serviceRegistry.TenantService))
	e.Use(handler.HostHydrate(serviceRegistry.TenantService))
	e.Use(handler.SessionHydrate(serviceRegistry.SessionService))
	e.Use(handler.PermissionHydrate(serviceRegistry.PermissionService))
	e.Validator = &CustomValidator{validator: validator.New()}

	return e
}

func SetupRoutes(e *echo.Group, cfg app.AppConfig, serviceRegistry services.ServiceRegistry) {
	authHandler := handler.NewAuthHandler(
		serviceRegistry.UserService,
		serviceRegistry.SessionService,
		cfg.CookieDomain,
		cfg.CookieSecure,
	)
	authHandler.SetupRoutes(e)

	healthHandler := handler.NewHealthHandler()
	healthHandler.SetupRoutes(e)
}
