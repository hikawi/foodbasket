package handler

import (
	"net/http"

	"github.com/labstack/echo/v5"
	"luny.dev/foodbasket/internal/dto"
	"luny.dev/foodbasket/internal/logging"
)

type HealthHandler struct{}

func NewHealthHandler() *HealthHandler {
	return new(HealthHandler)
}

func (h *HealthHandler) SetupRoutes(e *echo.Group) {
	e.GET("/health", h.GetHealth)
}

func (h *HealthHandler) GetHealth(c *echo.Context) error {
	logging.Info(c, "success")
	return c.JSON(http.StatusOK, dto.MessageResponse{Message: "healthy"})
}
