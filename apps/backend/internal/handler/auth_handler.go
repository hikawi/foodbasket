// Package handler provides controllers and handlers for REST endpoints.
package handler

import (
	"net/http"
	"time"

	"github.com/labstack/echo/v5"
	"luny.dev/foodbasket/internal/dto"
	"luny.dev/foodbasket/internal/logging"
	"luny.dev/foodbasket/internal/services"
)

type AuthHandler struct {
	userService    services.IUserService
	sessionService services.ISessionService
}

func NewAuthHandler(
	userService services.IUserService,
	sessionService services.ISessionService,
) *AuthHandler {
	return &AuthHandler{
		userService,
		sessionService,
	}
}

func (h *AuthHandler) SetupRoutes(e *echo.Group) {
	r := e.Group("/auth")

	r.POST("/login", h.postLogin)
	r.POST("/register", h.postRegister)
}

// @summary		Logins to an existing account.
// @description	Logins to an existing account if there is an account with that specific credentials.
// @tags			authentication
// @accept			json
// @router			/auth/login [post]
func (h *AuthHandler) postLogin(c *echo.Context) error {
	var body dto.PostLoginBody

	if err := c.Bind(&body); err != nil {
		logging.Error(c, "failed to bind body", "error", err.Error())
		return echo.ErrBadRequest.Wrap(err)
	}

	// Redact the password stuff
	loggableBody := body
	loggableBody.Password = "[REDACTED]"

	if err := c.Validate(&body); err != nil {
		logging.Error(c, "failed to validate body", "error", err.Error(), "body", loggableBody)
		return echo.ErrBadRequest.Wrap(err)
	}

	return c.JSON(200, map[string]any{"hello": "world"})
}

// @summary		Registers an account.
// @description	Creates a user account within the system. This registration does not create a new tenant.
// @tags			authentication
// @accept			json
// @param			body	body	dto.PostRegisterBody	true	"Body credentials"
// @router			/auth/register [post]
// @failure		400	"Bad request"
// @failure		500 "Couldn't register a new user"
func (h *AuthHandler) postRegister(c *echo.Context) error {
	var body dto.PostRegisterBody
	ctx := c.Request().Context()

	if err := c.Bind(&body); err != nil {
		logging.Error(c, "failed to bind body", "error", err.Error())
		return echo.ErrBadRequest.Wrap(err)
	}

	// Redact the password stuff
	loggableBody := body
	loggableBody.Password = "[REDACTED]"

	if err := c.Validate(&body); err != nil {
		logging.Error(c, "failed to validate body", "error", err.Error(), "body", loggableBody)
		return echo.ErrBadRequest.Wrap(err)
	}

	user, err := h.userService.RegisterUser(ctx, body.Name, body.Email, body.Password)
	if err != nil {
		return echo.ErrInternalServerError.Wrap(err)
	}

	// Create a new session.
	userID := user.ID.String()
	userName := user.Name

	sessionId, err := h.sessionService.Create(ctx, user.ID.String(), services.SessionData{
		UserID:    &userID,
		Email:     &userName,
		IsGuest:   false,
		CreatedAt: time.Now(),
	})

	c.SetCookie(&http.Cookie{
		Name: "",
	})

	return c.JSON(http.StatusOK, map[string]any{"hello": "world"})
}
