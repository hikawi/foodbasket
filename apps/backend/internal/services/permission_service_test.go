package services_test

import (
	"errors"
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/mocks"
	"luny.dev/foodbasket/internal/postgres"
	"luny.dev/foodbasket/internal/services"
)

func TestGetUserPermissions_CacheHit(t *testing.T) {
	valkey := mocks.NewMockIValkeyService(t)
	querier := mocks.NewMockQuerier(t)

	userID, _ := uuid.NewUUID()
	tenantID, _ := uuid.NewUUID()

	key := constants.ValkeyPermissionsPrefix + tenantID.String() + ":" + userID.String()
	valkey.EXPECT().Smembers(t.Context(), key).Return([]string{"a", "b", "d"}, nil).Once()

	permSvc := services.NewPermissionService(querier, valkey)
	perms, err := permSvc.GetUserPermissions(t.Context(), userID, tenantID)

	assert.Len(t, perms, 3)
	assert.Nil(t, err)
	assert.Contains(t, perms, "d")
	assert.NotContains(t, perms, "c")
}

func TestGetUserPermissions_CacheMiss(t *testing.T) {
	valkey := mocks.NewMockIValkeyService(t)
	querier := mocks.NewMockQuerier(t)

	userID, _ := uuid.NewUUID()
	tenantID, _ := uuid.NewUUID()

	key := constants.ValkeyPermissionsPrefix + tenantID.String() + ":" + userID.String()
	valkey.EXPECT().Smembers(t.Context(), key).Return(nil, errors.New("test error")).Once()
	querier.EXPECT().GetUserPermissions(t.Context(), postgres.GetUserPermissionsParams{
		UserID:   userID,
		TenantID: tenantID,
	}).Return([]string{"a", "b", "c"}, nil).Once()
	valkey.EXPECT().Sset(t.Context(), key, []string{"a", "b", "c"}).Return(nil).Once()

	permSvc := services.NewPermissionService(querier, valkey)
	perms, err := permSvc.GetUserPermissions(t.Context(), userID, tenantID)

	assert.Len(t, perms, 3)
	assert.Nil(t, err)
	assert.NotContains(t, perms, "d")
	assert.Contains(t, perms, "c")
}
