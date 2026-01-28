// Package app provides type-safe app-wide constants
package app

import "luny.dev/foodbasket/pkg/env"

type AppConfig struct {
	PostgresURL string
}

func LoadConfig() AppConfig {
	return AppConfig{
		PostgresURL: env.GetenvFatal("POSTGRES_URL"),
	}
}
