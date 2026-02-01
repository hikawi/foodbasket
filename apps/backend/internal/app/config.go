// Package app provides type-safe app-wide constants
package app

import (
	"strings"

	"luny.dev/foodbasket/pkg/env"
)

type AppConfig struct {
	PostgresURL string
	ValkeyURLs  []string
}

func LoadConfig() AppConfig {
	return AppConfig{
		PostgresURL: env.GetenvFatal("POSTGRES_URL"),
		ValkeyURLs:  strings.Split(env.GetenvFatal("VALKEY_URLS"), " "),
	}
}
