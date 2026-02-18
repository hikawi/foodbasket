package handler_test

import (
	"io"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/labstack/echo/v5"
	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/handler"
)

func TestHealthHandler_GetHealth(t *testing.T) {
	e := echo.New()
	e.Logger = slog.New(slog.NewTextHandler(io.Discard, nil))

	h := handler.NewHealthHandler()
	h.SetupRoutes(e.Group(""))

	req := httptest.NewRequest(http.MethodGet, "/health", nil)
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)
	if assert.NoError(t, h.GetHealth(c)) {
		assert.Equal(t, http.StatusOK, rec.Code)
		assert.Contains(t, rec.Body.String(), `{"message":"healthy"}`)
	}
}
