package services

import (
	"crypto/subtle"
	"encoding/base64"
	"errors"
	"fmt"
	"strings"

	"golang.org/x/crypto/argon2"
)

type IPasswordService interface {
	HashPassword(password string) (string, error)
	VerifyPassword(hash, password string) (bool, error)
}

type PasswordService struct {
	randomService IRandomService
}

type argon2Config struct {
	hashRaw    []byte
	salt       []byte
	timeCost   uint32
	memoryCost uint32
	threads    uint8
	keyLength  uint32
}

func NewPasswordService(randomService IRandomService) IPasswordService {
	return &PasswordService{randomService}
}

// HashPassword uses Argon2ID to hash a password, and returns a saved hash that combines all information
// for later verification.
func (s *PasswordService) HashPassword(password string) (string, error) {
	config := &argon2Config{
		timeCost:   2,
		memoryCost: 64 * 1024,
		threads:    4,
		keyLength:  32,
	}

	salt, err := s.randomService.GenerateRandomBytes(16)
	if err != nil {
		return "", fmt.Errorf("password hashing failed: %w", err)
	}
	config.salt = salt

	// Execute Argon2id hashing algorithm
	config.hashRaw = argon2.IDKey(
		[]byte(password),
		config.salt,
		config.timeCost,
		config.memoryCost,
		config.threads,
		config.keyLength,
	)

	// Generate standardized hash format
	encodedHash := fmt.Sprintf(
		"$argon2id$v=%d$m=%d,t=%d,p=%d$%s$%s",
		argon2.Version,
		config.memoryCost,
		config.timeCost,
		config.threads,
		base64.RawStdEncoding.EncodeToString(config.salt),
		base64.RawStdEncoding.EncodeToString(config.hashRaw),
	)

	return encodedHash, nil
}

func (s *PasswordService) parseArgon2Config(encodedHash string) (*argon2Config, error) {
	components := strings.Split(encodedHash, "$")
	if len(components) != 6 {
		return nil, errors.New("invalid hash format structure")
	}

	// Validate algorithm identifier
	if !strings.HasPrefix(components[1], "argon2id") {
		return nil, errors.New("unsupported algorithm variant")
	}

	// Extract version information
	var version int
	_, err := fmt.Sscanf(components[2], "v=%d", &version)
	if err != nil {
		return nil, err
	}

	// Parse configuration parameters
	config := &argon2Config{}
	_, err = fmt.Sscanf(components[3], "m=%d,t=%d,p=%d",
		&config.memoryCost, &config.timeCost, &config.threads)
	if err != nil {
		return nil, err
	}

	// Decode salt component
	salt, err := base64.RawStdEncoding.DecodeString(components[4])
	if err != nil {
		return nil, fmt.Errorf("salt decoding failed: %w", err)
	}
	config.salt = salt

	// Decode hash component
	hash, err := base64.RawStdEncoding.DecodeString(components[5])
	if err != nil {
		return nil, fmt.Errorf("hash decoding failed: %w", err)
	}
	config.hashRaw = hash
	config.keyLength = uint32(len(hash))

	return config, nil
}

// VerifyPassword verifies a password against a hash.
func (s *PasswordService) VerifyPassword(storedHash string, providedPassword string) (bool, error) {
	// Parse stored hash parameters
	config, err := s.parseArgon2Config(storedHash)
	if err != nil {
		return false, fmt.Errorf("hash parsing failed: %w", err)
	}

	// Generate hash using identical parameters
	computedHash := argon2.IDKey(
		[]byte(providedPassword),
		config.salt,
		config.timeCost,
		config.memoryCost,
		config.threads,
		config.keyLength,
	)

	// Perform constant-time comparison to prevent timing attacks
	match := subtle.ConstantTimeCompare(config.hashRaw, computedHash) == 1
	return match, nil
}
