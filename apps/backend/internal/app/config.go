// Package app provides type-safe app-wide constants
package app

import (
	"strings"

	"luny.dev/foodbasket/pkg/env"
)

var Test = 1

type AppConfig struct {
	PostgresURL  string
	ValkeyURLs   []string
	CookieDomain string
	CookieSecure bool
}

func LoadConfig() AppConfig {
	return AppConfig{
		PostgresURL:  env.GetenvFatal("POSTGRES_URL"),
		ValkeyURLs:   strings.Split(env.GetenvFatal("VALKEY_URLS"), " "),
		CookieDomain: env.GetenvFatal("COOKIE_DOMAIN"),
		CookieSecure: env.GetenvFatalBool("COOKIE_SECURE"),
	}
}
