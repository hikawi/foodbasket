package services

import (
	"crypto/rand"
	"encoding/base64"
	"fmt"
)

type IRandomService interface {
	GenerateRandomBytes(keySize uint) ([]byte, error)
	GenerateSecretToken(tokenSize uint) (string, error)
}

type RandomService struct{}

func NewRandomService() IRandomService {
	return &RandomService{}
}

// GenerateRandomBytes generates a cryptographically random string
// of a certain size that fills keySize number of bytes.
func (s *RandomService) GenerateRandomBytes(keySize uint) ([]byte, error) {
	key := make([]byte, keySize)
	_, err := rand.Read(key)
	if err != nil {
		return nil, fmt.Errorf("salt generation failed: %w", err)
	}
	return key, nil
}

func (s *RandomService) GenerateSecretToken(tokenSize uint) (string, error) {
	bytes, err := s.GenerateRandomBytes(tokenSize)
	if err != nil {
		return "", err
	}
	return base64.RawURLEncoding.EncodeToString(bytes), nil
}
