package services_test

import (
	"errors"
	"testing"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/mocks"
	"luny.dev/foodbasket/internal/postgres"
	"luny.dev/foodbasket/internal/services"
)

func TestGetTenantID_SlugDoesntExist(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	mockValkey.EXPECT().Get(ctx, constants.TenantSlugKey("test")).Return(constants.ValkeyNilValue, nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	uuid, err := tenantSvc.GetTenantID(ctx, "test")

	assert.NoError(t, err)
	assert.Nil(t, uuid)

	mockQ.AssertNotCalled(t, "GetTenantBySlug", ctx, "test")
}

func TestGetTenantID_CacheHit(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	// randomly chosen by neovim
	id := uuid.MustParse("b5f55b7d-c58c-4825-8b48-3e5c0cda531e")
	mockValkey.EXPECT().Get(ctx, constants.TenantSlugKey("test")).Return(id.String(), nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	uuid, err := tenantSvc.GetTenantID(ctx, "test")

	assert.NoError(t, err)
	assert.NotNil(t, uuid)
	assert.Equal(t, *uuid, id)

	mockQ.AssertNotCalled(t, "GetTenantBySlug", ctx, "test")
}

func TestGetTenantID_CacheMissNoRows(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	mockValkey.EXPECT().Get(ctx, constants.TenantSlugKey("test")).Return("", errors.New("test"))
	mockQ.EXPECT().GetTenantBySlug(ctx, "test").Return(postgres.Tenant{}, pgx.ErrNoRows)
	mockValkey.EXPECT().Set(ctx, constants.TenantSlugKey("test"), constants.ValkeyNilValue, constants.ValkeyCacheTTL).Return(nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	uuid, err := tenantSvc.GetTenantID(ctx, "test")

	assert.NoError(t, err)
	assert.Nil(t, uuid)
}

func TestGetTenantID_CacheMissBadConnection(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	mockValkey.EXPECT().Get(ctx, constants.TenantSlugKey("test")).Return("", errors.New("test"))
	mockQ.EXPECT().GetTenantBySlug(ctx, "test").Return(postgres.Tenant{}, pgx.ErrTxClosed)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	uuid, err := tenantSvc.GetTenantID(ctx, "test")

	assert.Error(t, err)
	assert.Nil(t, uuid)
}

func TestGetTenantID_CacheMissWithRows(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	// randomly chosen by neovim
	id := uuid.MustParse("b5f55b7d-c58c-4825-8b48-3e5c0cda531e")
	mockValkey.EXPECT().Get(ctx, constants.TenantSlugKey("test")).Return("", errors.New("test"))
	mockQ.EXPECT().GetTenantBySlug(ctx, "test").Return(postgres.Tenant{ID: id}, nil)
	mockValkey.EXPECT().Set(ctx, constants.TenantSlugKey("test"), id.String(), constants.ValkeyCacheTTL).Return(nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	uuid, err := tenantSvc.GetTenantID(ctx, "test")

	assert.NoError(t, err)
	assert.NotNil(t, uuid)
	assert.Equal(t, *uuid, id)
}

func TestIsTenant_CacheHit(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	// randomly chosen by neovim
	id := uuid.MustParse("b5f55b7d-c58c-4825-8b48-3e5c0cda531e")
	mockValkey.EXPECT().Get(ctx, constants.TenantUUIDKey(id.String())).Return(constants.ValkeyBoolYes, nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	ok, err := tenantSvc.IsTenant(ctx, id)

	assert.NoError(t, err)
	assert.Equal(t, ok, true)
}

func TestIsTenant_CacheMissNotATenant(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	// randomly chosen by neovim
	id := uuid.MustParse("b5f55b7d-c58c-4825-8b48-3e5c0cda531e")
	mockValkey.EXPECT().Get(ctx, constants.TenantUUIDKey(id.String())).Return("", errors.New("error"))
	mockQ.EXPECT().GetTenantByID(ctx, id).Return(postgres.Tenant{}, pgx.ErrNoRows)
	mockValkey.EXPECT().Set(ctx, constants.TenantUUIDKey(id.String()), constants.ValkeyBoolNo, constants.ValkeyCacheTTL).Return(nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	ok, err := tenantSvc.IsTenant(ctx, id)

	assert.NoError(t, err)
	assert.False(t, ok)
}

func TestIsTenant_CacheMissIsATenant(t *testing.T) {
	mockQ := mocks.NewMockQuerier(t)
	mockValkey := mocks.NewMockIValkeyService(t)
	ctx := t.Context()

	// randomly chosen by neovim
	id := uuid.MustParse("b5f55b7d-c58c-4825-8b48-3e5c0cda531e")
	mockValkey.EXPECT().Get(ctx, constants.TenantUUIDKey(id.String())).Return("", errors.New("error"))
	mockQ.EXPECT().GetTenantByID(ctx, id).Return(postgres.Tenant{ID: id}, nil)
	mockValkey.EXPECT().Set(ctx, constants.TenantUUIDKey(id.String()), constants.ValkeyBoolYes, constants.ValkeyCacheTTL).Return(nil)

	tenantSvc := services.NewTenantService(mockQ, mockValkey)
	ok, err := tenantSvc.IsTenant(ctx, id)

	assert.NoError(t, err)
	assert.True(t, ok)
}
