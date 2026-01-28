// Package handler provides controllers and handlers for REST endpoints.
package handler

import (
	"log/slog"
	"net/http"

	"github.com/labstack/echo/v5"
	"luny.dev/foodbasket/internal/dto"
	"luny.dev/foodbasket/internal/services"
)

type AuthHandler struct {
	userService services.IUserService
}

func NewAuthHandler(
	userService services.IUserService,
) *AuthHandler {
	return &AuthHandler{userService}
}

func (h *AuthHandler) SetupRoutes(e *echo.Echo) {
	r := e.Group("/auth")

	r.POST("/login", h.postLogin)
	r.POST("/register", h.postRegister)
}

func (h *AuthHandler) postLogin(c *echo.Context) error {
	var body dto.PostLoginBody
	ctx := c.Request().Context()

	if err := c.Bind(&body); err != nil {
		c.Logger().Log(ctx, slog.LevelError, "failed to bind body", "error", err.Error())
		return echo.ErrBadRequest.Wrap(err)
	}
	if err := c.Validate(&body); err != nil {
		c.Logger().Log(ctx, slog.LevelError, "failed to validate body", "error", err.Error())
		return echo.ErrBadRequest.Wrap(err)
	}

	return c.JSON(200, map[string]any{"hello": "world"})
}

func (h *AuthHandler) postRegister(c *echo.Context) error {
	var body dto.PostRegisterBody
	ctx := c.Request().Context()

	if err := c.Bind(&body); err != nil {
		c.Logger().Log(ctx, slog.LevelError, "failed to bind body", "error", err.Error())
		return echo.ErrBadRequest.Wrap(err)
	}
	if err := c.Validate(&body); err != nil {
		c.Logger().Log(ctx, slog.LevelError, "failed to validate body", "error", err.Error())
		return echo.ErrBadRequest.Wrap(err)
	}

	return c.JSON(http.StatusOK, map[string]any{"hello": "world"})
}
