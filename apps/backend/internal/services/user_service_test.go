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
