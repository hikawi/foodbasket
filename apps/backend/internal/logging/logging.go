// Package logging provides some simple wrappers over the Echo's context.
package logging

import (
	"log/slog"
	"strings"

	"github.com/labstack/echo/v5"
)

// Log logs certain info about the context, before you can inject your own logging data for each endpoint.
// Written by Google Gemini.
func Log(c *echo.Context, lvl slog.Level, msg string, data ...any) {
	req := c.Request()

	// 1. Prepare headers for logging
	headers := make(map[string]string)
	for k, v := range req.Header {
		if strings.EqualFold(k, "Cookie") || strings.EqualFold(k, "Authorization") {
			headers[k] = "[REDACTED]"
		} else {
			headers[k] = strings.Join(v, ", ")
		}
	}

	// 2. Build base attributes
	args := []any{
		"method", req.Method,
		"uri", req.RequestURI,
		"ip", c.RealIP(),
		"ua", req.UserAgent(),
		"headers", headers,
	}

	// 3. Append user-provided data
	args = append(args, data...)
	c.Logger().Log(req.Context(), lvl, msg, args...)
}

// Info logs at LevelInfo
func Info(c *echo.Context, msg string, data ...any) {
	Log(c, slog.LevelInfo, msg, data...)
}

// Error logs at LevelError
func Error(c *echo.Context, msg string, data ...any) {
	Log(c, slog.LevelError, msg, data...)
}

// Debug logs at LevelDebug
func Debug(c *echo.Context, msg string, data ...any) {
	Log(c, slog.LevelDebug, msg, data...)
}

// Warn logs at LevelWarn
func Warn(c *echo.Context, msg string, data ...any) {
	Log(c, slog.LevelWarn, msg, data...)
}
