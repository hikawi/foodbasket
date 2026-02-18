package services_test

import (
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/mocks"
	"luny.dev/foodbasket/internal/services"
)

func TestHashPassword_FailedToGenerateSalt(t *testing.T) {
	randomSvc := mocks.NewMockIRandomService(t)

	randomSvc.EXPECT().GenerateRandomBytes(uint(16)).Return(nil, errors.New("test error")).Once()

	passwordSvc := services.NewPasswordService(randomSvc)
	password, err := passwordSvc.HashPassword("testPassword123")

	assert.Equal(t, "", password)
	assert.Error(t, err)
}

func TestHashPassword_SuccessEncode(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	password, err := passwordSvc.HashPassword("testPassword123")

	assert.Contains(t, password, "$argon2id")
	assert.NoError(t, err)
}

func TestVerifyPassword_InvalidHashFormat(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	hardcodedHash := "bad$hash"
	passwordToTest := "testPassword123"

	_, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)
	assert.Error(t, err)
}

func TestVerifyPassword_UnsupportedAlgorithm(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	hardcodedHash := "$scrypt$v=19$m=65536,t=1,p=4$salt$hash"
	passwordToTest := "testPassword123"

	_, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)
	assert.Error(t, err)
}

func TestVerifyPassword_InvalidVersion(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	hardcodedHash := "$argon2id$v=ABC$m=65536,t=1,p=4$salt$hash"
	passwordToTest := "testPassword123"

	_, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)
	assert.Error(t, err)
}

func TestVerifyPassword_InvalidParameters(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	hardcodedHash := "$argon2id$v=19$m=-W,t=ABC,p=4.5$salt$hash"
	passwordToTest := "testPassword123"

	_, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)
	assert.Error(t, err)
}

func TestVerifyPassword_InvalidSalt(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	hardcodedHash := "$argon2id$v=19$m=65536,t=1,p=4$SGV!@$hash"
	passwordToTest := "testPassword123"

	_, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)
	assert.Error(t, err)
}

func TestVerifyPassword_InvalidHash(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	hardcodedHash := "$argon2id$v=19$m=65536,t=1,p=4$salt$invalidHash@!"
	passwordToTest := "testPassword123"

	_, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)
	assert.Error(t, err)
}

func TestVerifyPassword_Success(t *testing.T) {
	randomSvc := services.NewRandomService()
	passwordSvc := services.NewPasswordService(randomSvc)

	// Hard-coded argon2id by Gemini
	// This hash represents the password "testPassword123"
	// using: m=65536, t=1, p=4
	// salt: "staticSalt123456" (base64: c3RhdGljU2FsdDEyMzQ1Ng)
	hardcodedHash := "$argon2id$v=19$m=65536,t=1,p=4$c3RhdGljU2FsdDEyMzQ1Ng$sRrvMCtJB1YlhsiozFFm7eXC08JR1gb99rV/puUL9bw"
	passwordToTest := "testPassword123"

	match, err := passwordSvc.VerifyPassword(hardcodedHash, passwordToTest)

	assert.NoError(t, err)
	assert.True(t, match)
}
