package services

import (
	"crypto/rand"
	"fmt"
)

type IRandomService interface {
	GenerateRandomBytes(keySize uint) ([]byte, error)
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
