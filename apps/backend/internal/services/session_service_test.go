package services_test

import (
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/constants"
	"luny.dev/foodbasket/internal/mocks"
	"luny.dev/foodbasket/internal/services"
)

func TestGetSession_NoSessionsFound(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	mockValkey.EXPECT().Get(ctx, constants.SessionKey("test")).Return("", errors.New("error"))

	sess, err := sessionSvc.GetSession(ctx, "test")

	assert.Nil(t, sess)
	assert.Error(t, err)
}

func TestGetSession_UnmarshalFailed(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	mockValkey.EXPECT().Get(ctx, constants.SessionKey("test")).Return("{\"test\":\"not valid}", nil)

	sess, err := sessionSvc.GetSession(ctx, "test")

	assert.Nil(t, sess)
	assert.Error(t, err)
}

func TestGetSession_Success(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	val := `{"user_id":"abc","email":"test@foodbasket.app","meta":null,"is_guest":false,"created_at":"2026-02-18T22:57:53Z"}`

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	mockValkey.EXPECT().Get(ctx, constants.SessionKey("test")).Return(val, nil)
	mockValkey.EXPECT().SetXx(ctx, constants.SessionKey("test"), val, constants.ValkeySessionTTL).Return(nil)

	sess, err := sessionSvc.GetSession(ctx, "test")

	assert.NotNil(t, sess)
	assert.NoError(t, err)
}
