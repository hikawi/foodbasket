package services_test

import (
	"errors"
	"testing"

	"github.com/jackc/pgx/v5/pgtype"
	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/mocks"
	"luny.dev/foodbasket/internal/postgres"
	"luny.dev/foodbasket/internal/services"
)

func TestCheckUserCredentials_NoUserByEmail(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockPassword := mocks.NewMockIPasswordService(t)
	ctx := t.Context()

	testEmail := "testuser@foodbasket.app"
	testPassword := "password"

	userSvc := services.NewUserService(mockQ, mockPassword)
	mockQ.EXPECT().GetUserByEmail(ctx, testEmail).Return(postgres.User{}, errors.New("error"))

	_, err := userSvc.CheckUserCredentials(ctx, testEmail, testPassword)

	assert.Error(t, err)
	assert.ErrorIs(t, err, services.ErrUserNotFound)

	mockPassword.AssertNotCalled(t, "VerifyPassword")
}

func TestCheckUserCredentials_NoPassword(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockPassword := mocks.NewMockIPasswordService(t)
	ctx := t.Context()

	testEmail := "testuser@foodbasket.app"
	testPassword := "password"

	userSvc := services.NewUserService(mockQ, mockPassword)
	mockQ.EXPECT().GetUserByEmail(ctx, testEmail).Return(postgres.User{Password: pgtype.Text{
		Valid: false,
	}}, nil)

	_, err := userSvc.CheckUserCredentials(ctx, testEmail, testPassword)

	assert.Error(t, err)
	assert.ErrorIs(t, err, services.ErrUserDoesNotUsePassword)

	mockPassword.AssertNotCalled(t, "VerifyPassword")
}

func TestCheckUserCredentials_CanVerifyPassword(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockPassword := mocks.NewMockIPasswordService(t)
	ctx := t.Context()

	testEmail := "testuser@foodbasket.app"
	testPasswordHash := "password_hash"
	testPassword := "password"

	userSvc := services.NewUserService(mockQ, mockPassword)
	mockQ.EXPECT().GetUserByEmail(ctx, testEmail).Return(postgres.User{Password: pgtype.Text{
		String: testPasswordHash,
		Valid:  true,
	}}, nil)
	mockPassword.EXPECT().VerifyPassword(testPasswordHash, testPassword).Return(true, nil)

	_, err := userSvc.CheckUserCredentials(ctx, testEmail, testPassword)

	assert.NoError(t, err)
}

func TestRegisterUser_FailedToHashPassword(t *testing.T) {
	q := mocks.NewMockQuerier(t)
	passwordSvc := mocks.NewMockIPasswordService(t)

	userSvc := services.NewUserService(q, passwordSvc)
	passwordSvc.EXPECT().HashPassword("password").Return("", errors.New("test error")).Once()

	_, err := userSvc.RegisterUser(t.Context(), "testuser", "testuser@foodbasket.app", "password")
	assert.Error(t, err)
}

func TestRegisterUser_SuccessRegistration(t *testing.T) {
	q := mocks.NewMockQuerier(t)
	passwordSvc := mocks.NewMockIPasswordService(t)

	userSvc := services.NewUserService(q, passwordSvc)
	passwordSvc.EXPECT().HashPassword("password").Return("hashedPassword", nil).Once()
	q.EXPECT().CreateUser(t.Context(), postgres.CreateUserParams{
		Name:  "testuser",
		Email: "testuser@foodbasket.app",
		Password: pgtype.Text{
			String: "hashedPassword",
			Valid:  true,
		},
	}).Return(postgres.User{}, nil)

	_, err := userSvc.RegisterUser(t.Context(), "testuser", "testuser@foodbasket.app", "password")
	assert.NoError(t, err)
}
