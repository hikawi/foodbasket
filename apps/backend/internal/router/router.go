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
	e.Use(handler.CORSMiddleware(serviceRegistry.ValkeyService))
	e.Use(handler.HostHydrate())
	e.Use(handler.SessionHydrate(serviceRegistry.SessionService))
	e.Validator = &CustomValidator{validator: validator.New()}

	return e
}

func SetupRoutes(e *echo.Group, cfg app.AppConfig, serviceRegistry services.ServiceRegistry) {
	authHandler := handler.NewAuthHandler(serviceRegistry.UserService, serviceRegistry.SessionService)
	authHandler.SetupRoutes(e)

	healthHandler := handler.NewHealthHandler()
	healthHandler.SetupRoutes(e)
}
