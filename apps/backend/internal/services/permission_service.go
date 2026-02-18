package services

import (
	"context"

	"github.com/google/uuid"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/postgres"
)

type IPermissionService interface {
	// GetUserPermissions retrieves a list of user's permissions for
	// a user and a specified tenant.
	GetUserPermissions(ctx context.Context, userID uuid.UUID, tenantID uuid.UUID) ([]string, error)
}

type PermissionService struct {
	q      postgres.Querier
	valkey IValkeyService
}

func NewPermissionService(q postgres.Querier, valkey IValkeyService) IPermissionService {
	return &PermissionService{
		q:      q,
		valkey: valkey,
	}
}

func (s *PermissionService) GetUserPermissions(ctx context.Context, userID uuid.UUID, tenantID uuid.UUID) ([]string, error) {
	// Check cache first.
	key := constants.ValkeyPermissionsPrefix + tenantID.String() + ":" + userID.String()
	perms, err := s.valkey.Smembers(ctx, key)
	if err == nil {
		return perms, nil
	}

	// If not query the db
	perms, err = s.q.GetUserPermissions(ctx, postgres.GetUserPermissionsParams{
		UserID:   userID,
		TenantID: tenantID,
	})
	if err != nil {
		_ = s.valkey.Sset(ctx, key, []string{}) // no permissions found
		return nil, err
	}

	// Put in the cache.
	_ = s.valkey.Sset(ctx, key, perms)
	return perms, err
}
