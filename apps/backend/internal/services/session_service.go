package services

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/valkey-io/valkey-go"
	"luny.dev/foodbasket/internal/constants"
)

type ISessionService interface {
	// GetSession attempts to retrieve a session inside the cache, and returns it if the session is found.
	// Otherwise an error is raised.
	GetSession(ctx context.Context, sessionID string) (*SessionData, error)

	// CreateSession attempts to create a new session with the session data, ensuring no previously used IDs.
	// It tries a total of 3 times to try for a unique ID, but the odds are so low you shouldn't worry about it.
	CreateSession(ctx context.Context, data SessionData) (sessionID string, err error)

	// DeleteSession attempts to delete a session by that ID.
	DeleteSession(ctx context.Context, sessionID string) error

	// RotateSession rotates the existing session to a new one while copying all data, if necessary.
	// This marks the old session with a grace period of 30 seconds with a rotated_to in metadata.
	RotateSession(ctx context.Context, sessionID string) (newSessionID string, err error)
}

type SessionService struct {
	valkeyService IValkeyService
	randomService IRandomService
}

type SessionData struct {
	UserID    *string        `json:"user_id"`
	Email     *string        `json:"email"`
	Metadata  map[string]any `json:"meta"`
	IsGuest   bool           `json:"is_guest"`
	CreatedAt time.Time      `json:"created_at"`
}

func NewSessionService(valkeyService IValkeyService, randomService IRandomService) ISessionService {
	return &SessionService{valkeyService: valkeyService, randomService: randomService}
}

func (s *SessionService) GetSession(ctx context.Context, sessionID string) (*SessionData, error) {
	key := constants.ValkeySessionPrefix + sessionID

	val, err := s.valkeyService.Get(ctx, key)
	if err != nil {
		return nil, err // No session found.
	}

	var data SessionData
	if err := json.Unmarshal([]byte(val), &data); err != nil {
		return nil, err
	}

	// Slide the expiration up
	go s.valkeyService.SetXx(ctx, key, val, constants.ValkeySessionTTL)
	return &data, nil
}

func (s *SessionService) CreateSession(ctx context.Context, data SessionData) (string, error) {
	jsonData, err := json.Marshal(data)
	if err != nil {
		return "", fmt.Errorf("failed to marshal session: %w", err)
	}

	var sessionID string

	for range 3 {
		newID, err := s.randomService.GenerateSecretToken(32)
		if err != nil {
			return "", err
		}

		key := constants.ValkeySessionPrefix + newID
		err = s.valkeyService.SetNx(ctx, key, string(jsonData), constants.ValkeySessionTTL)

		if err == nil {
			sessionID = newID
			break
		}

		if !valkey.IsValkeyNil(err) {
			return "", err
		}
	}

	if sessionID == "" {
		return "", fmt.Errorf("failed to generate unique session after multiple attempts")
	}

	// Session created successful. Now we just log it inside a user index for "Logout all devices"
	idxKey := constants.ValkeyUserIndexPrefix + *data.UserID
	_ = s.valkeyService.Sadd(ctx, idxKey, sessionID)
	return sessionID, nil
}

func (s *SessionService) DeleteSession(ctx context.Context, sessionID string) error {
	key := constants.ValkeySessionPrefix + sessionID
	sessString, err := s.valkeyService.Get(ctx, key)
	var userID string

	if err == nil {
		var sess SessionData
		err = json.Unmarshal([]byte(sessString), &sess)
		if err == nil && sess.UserID != nil {
			// Okay we successfully decoded it, we can now do this to get the user ID.
			userID = *sess.UserID
		}
	}

	// Delete the session. I don't really mind if it failed here.
	_ = s.valkeyService.Del(ctx, key)

	// Remove from the user index.
	idxKey := constants.ValkeyUserIndexPrefix + userID
	_ = s.valkeyService.Sremove(ctx, idxKey, sessionID)
	return nil
}

func (s *SessionService) RotateSession(ctx context.Context, sessionID string) (string, error) {
	sess, err := s.GetSession(ctx, sessionID)
	if err != nil || sess == nil {
		return "", nil // Well, that session doesn't exist to rotate.
	}

	// Create a new session.
	newID, err := s.CreateSession(ctx, *sess)
	if err != nil {
		return "", err
	}

	// Add a grace period for old ID.
	rotatedSession := SessionData{
		Metadata: map[string]any{
			"rotated_to": newID,
		},
	}
	jsonData, _ := json.Marshal(rotatedSession)

	// Overwrite ID with new thing and short TTL.
	oldKey := constants.ValkeySessionPrefix + sessionID
	_ = s.valkeyService.Set(ctx, oldKey, string(jsonData), 30*time.Second)

	// Return the new thing
	return newID, nil
}
