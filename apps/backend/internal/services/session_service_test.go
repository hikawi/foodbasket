package services_test

import (
	"encoding/json"
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/valkey-io/valkey-go"
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

func TestCreateSession_FailedToGenerateID(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	sessData := services.SessionData{}
	mockRandom.EXPECT().GenerateSecretToken(uint(32)).Return("", errors.New("test"))

	sess, err := sessionSvc.CreateSession(ctx, sessData)

	assert.Error(t, err)
	assert.Equal(t, "", sess)
}

func TestCreateSession_FailedToSaveSession(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	sessData := services.SessionData{}
	jsonData, _ := json.Marshal(sessData)
	mockRandom.EXPECT().GenerateSecretToken(uint(32)).Return("test", nil)
	mockValkey.EXPECT().SetNx(ctx, constants.SessionKey("test"), string(jsonData), constants.ValkeySessionTTL).Return(errors.New("not valkey nil"))

	sess, err := sessionSvc.CreateSession(ctx, sessData)

	assert.Error(t, err)
	assert.Equal(t, "", sess)
}

func TestCreateSession_FailedToCreateUniqueID(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	sessData := services.SessionData{}
	jsonData, _ := json.Marshal(sessData)
	mockRandom.EXPECT().GenerateSecretToken(uint(32)).Return("test", nil)
	mockValkey.EXPECT().SetNx(ctx, constants.SessionKey("test"), string(jsonData), constants.ValkeySessionTTL).Return(valkey.Nil)

	sess, err := sessionSvc.CreateSession(ctx, sessData)

	assert.Error(t, err)
	assert.Equal(t, "", sess)
}

func TestCreateSession_Success(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	userID := "testuser"
	sessData := services.SessionData{UserID: &userID}
	jsonData, _ := json.Marshal(sessData)
	mockRandom.EXPECT().GenerateSecretToken(uint(32)).Return("test", nil)
	mockValkey.EXPECT().SetNx(ctx, constants.SessionKey("test"), string(jsonData), constants.ValkeySessionTTL).Return(nil)
	mockValkey.EXPECT().Sadd(ctx, constants.SessionIndexKey(userID), []string{"test"}).Return(nil)

	sess, err := sessionSvc.CreateSession(ctx, sessData)

	assert.NoError(t, err)
	assert.Equal(t, "test", sess)
}

func TestDeleteSession_NoSessionsFound(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	mockValkey.EXPECT().Get(ctx, constants.SessionKey("test")).Return("", errors.New("error"))

	err := sessionSvc.DeleteSession(ctx, "test")

	assert.NoError(t, err)
}

func TestDeleteSession_NoUserID(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	sessData := `{"user_id":null}`
	mockValkey.EXPECT().Get(ctx, constants.SessionKey("test")).Return(sessData, nil)
	mockValkey.EXPECT().Del(ctx, []string{constants.SessionKey("test")}).Return(nil)

	err := sessionSvc.DeleteSession(ctx, "test")

	assert.NoError(t, err)
}

func TestDeleteSession_WithUserID(t *testing.T) {
	mockValkey := mocks.NewMockIValkeyService(t)
	mockRandom := mocks.NewMockIRandomService(t)
	ctx := t.Context()

	sessionSvc := services.NewSessionService(mockValkey, mockRandom)
	sessData := `{"user_id":"123"}`
	mockValkey.EXPECT().Get(ctx, constants.SessionKey("test")).Return(sessData, nil)
	mockValkey.EXPECT().Del(ctx, []string{constants.SessionKey("test")}).Return(nil)
	mockValkey.EXPECT().Sremove(ctx, constants.SessionIndexKey("123"), "test").Return(nil)

	err := sessionSvc.DeleteSession(ctx, "test")

	assert.NoError(t, err)
}
