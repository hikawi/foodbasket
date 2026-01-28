// Package env provides some simple utilities working with environment variables
package env

import (
	"log"
	"os"
	"strconv"
)

// GetenvFatal retrieves an environment variable, and fatals if it has not been initialized.
func GetenvFatal(key string) string {
	val, ok := os.LookupEnv(key)
	if !ok {
		log.Fatalf("failed to lookup env key %s\n", key)
	}
	return val
}

// GetenvFatalInt retrieves an environment variable, and fatals if it has not been initialized
// or it is not an integer.
func GetenvFatalInt(key string) int64 {
	val := GetenvFatal(key)
	num, err := strconv.ParseInt(val, 10, 0)
	if err != nil {
		log.Fatalf("failed to parse env key to int %s, %s\n", key, val)
	}
	return num
}

// GetenvFatalBool retrieves an environment variable, and fatals if it has not been initialized
// or it is not a boolean.
func GetenvFatalBool(key string) bool {
	val := GetenvFatal(key)
	b, err := strconv.ParseBool(val)
	if err != nil {
		log.Fatalf("failed to parse env key to int %s, %s\n", key, val)
	}
	return b
}
