package services

import (
	"context"
	"errors"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/postgres"
)

type ITenantService interface {
	// GetTenantID retrieves a UUID mapped to a slug.
	GetTenantID(ctx context.Context, slug string) (*uuid.UUID, error)

	// Checks if the provided slug is a valid tenant.
	IsTenant(ctx context.Context, tenantID uuid.UUID) (bool, error)
}

type TenantService struct {
	q      postgres.Querier
	valkey IValkeyService
}

func NewTenantService(q postgres.Querier, valkey IValkeyService) ITenantService {
	return &TenantService{q, valkey}
}

func (s *TenantService) GetTenantID(ctx context.Context, slug string) (*uuid.UUID, error) {
	val, err := s.valkey.Get(ctx, constants.TenantSlugKey(slug))
	if err == nil {
		// Cache hit. Try parse.
		if val == constants.ValkeyNilValue {
			return nil, nil // Not an error, but that slug doesn't exist
		}

		id, err := uuid.Parse(val)
		return &id, err
	}

	// Cache miss
	tenant, err := s.q.GetTenantBySlug(ctx, slug)
	if err != nil {
		// Actually no rows
		if errors.Is(err, pgx.ErrNoRows) {
			_ = s.valkey.Set(ctx, constants.TenantSlugKey(slug), constants.ValkeyNilValue, constants.ValkeyCacheTTL)
			return nil, nil
		}

		// Bad connection
		return nil, err
	} else {
		// If there is, put in the ID as value.
		err = s.valkey.Set(ctx, constants.TenantSlugKey(slug), tenant.ID.String(), constants.ValkeyCacheTTL)
		return &tenant.ID, nil
	}
}

func (s *TenantService) IsTenant(ctx context.Context, tenantID uuid.UUID) (bool, error) {
	tenantIDStr := tenantID.String()

	val, err := s.valkey.Get(ctx, constants.TenantUUIDKey(tenantIDStr))
	if err == nil {
		return val == constants.ValkeyBoolYes, nil // cache hit
	}

	// Make a request to db if cache missed.
	_, err = s.q.GetTenantByID(ctx, tenantID)
	if err != nil {
		// Set non-existing
		_ = s.valkey.Set(ctx, constants.TenantUUIDKey(tenantIDStr), constants.ValkeyBoolNo, constants.ValkeyCacheTTL)
		return false, nil
	} else {
		_ = s.valkey.Set(ctx, constants.TenantUUIDKey(tenantIDStr), constants.ValkeyBoolYes, constants.ValkeyCacheTTL)
		return true, nil
	}
}
