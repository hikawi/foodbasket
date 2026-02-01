package services

import (
	"context"
	"fmt"

	"github.com/jackc/pgx/v5/pgtype"
	"luny.dev/foodbasket/internal/postgres"
)

type IUserService interface {
	// CheckUserCredentials checks an account associated with the email if it has a password.
	// The password passed in is not hashed, but will be checked against a hash.
	CheckUserCredentials(ctx context.Context, email, password string) (bool, error)

	// RegisterUser registers a new user in the database.
	// The password passed in is hashed inside this function.
	RegisterUser(ctx context.Context, name, email, password string) (postgres.User, error)
}

type UserService struct {
	q               postgres.Querier
	passwordService IPasswordService
}

func NewUserService(q postgres.Querier, passwordService IPasswordService) IUserService {
	return &UserService{q: q, passwordService: passwordService}
}

func (s *UserService) CheckUserCredentials(ctx context.Context, email, password string) (bool, error) {
	user, err := s.q.GetUserByEmail(ctx, email)
	if err != nil {
		return false, err
	}
	if !user.Password.Valid {
		return false, fmt.Errorf("user %s does not have a password", email)
	}

	return s.passwordService.VerifyPassword(user.Password.String, password)
}

func (s *UserService) RegisterUser(ctx context.Context, name, email, password string) (postgres.User, error) {
	hashedPassword, err := s.passwordService.HashPassword(password)
	if err != nil {
		return postgres.User{}, err
	}

	return s.q.CreateUser(ctx, postgres.CreateUserParams{
		Name:  name,
		Email: email,
		Password: pgtype.Text{
			String: hashedPassword,
			Valid:  true,
		},
	})
}
