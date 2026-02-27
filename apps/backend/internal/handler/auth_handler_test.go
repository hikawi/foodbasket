package handler_test

import (
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/labstack/echo/v5"
	"github.com/labstack/echo/v5/echotest"
	"github.com/stretchr/testify/assert"
	"luny.dev/foodbasket/internal/handler"
	"luny.dev/foodbasket/internal/mocks"
	"luny.dev/foodbasket/internal/postgres"
	"luny.dev/foodbasket/internal/router"
	"luny.dev/foodbasket/internal/services"
)

// For testing
const (
	cookieDomain = ".foodbasket.app"
	cookieSecure = false
)

func setupTestEcho() *echo.Echo {
	e := echo.New()
	e.Validator = router.NewValidator()

	return e
}

// For coverage.
func TestAuthHandler_SetupRoutes(t *testing.T) {
	e := echo.New()

	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)
	h.SetupRoutes(e.Group(""))
}

func TestAuthHandler_LoginFailedToBind(t *testing.T) {
	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)

	c, _ := echotest.ContextConfig{
		Headers: map[string][]string{
			echo.HeaderContentType: {echo.MIMEApplicationJSON},
		},
		JSONBody: []byte(`{"email":"lol","pass":"wahoo"`),
	}.ToContextRecorder(t)

	err := h.PostLogin(c)
	if assert.Error(t, err) {
		he, ok := err.(*echo.HTTPError)
		assert.True(t, ok)
		assert.Equal(t, http.StatusBadRequest, he.Code)
	}
}

func TestAuthHandler_LoginBadRequest(t *testing.T) {
	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)

	c, _ := echotest.ContextConfig{
		Headers: map[string][]string{
			echo.HeaderContentType: {echo.MIMEApplicationJSON},
		},
		JSONBody: []byte(`{"email":"lol","password":"wahoo"}`),
	}.ToContextRecorder(t)

	err := h.PostLogin(c)
	if assert.Error(t, err) {
		he, ok := err.(*echo.HTTPError)
		assert.True(t, ok)
		assert.Equal(t, http.StatusBadRequest, he.Code)
	}
}

func TestAuthHandler_LoginUserNotFound(t *testing.T) {
	e := setupTestEcho()

	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)
	testJSON := `{"email":"lol@example.com","password":"wahoo123"}`

	req := httptest.NewRequest(http.MethodPost, "/login", strings.NewReader(testJSON))
	req.Header.Add(echo.HeaderContentType, echo.MIMEApplicationJSON)

	rec := httptest.NewRecorder()
	c := e.NewContext(req, rec)

	mockUserSvc.EXPECT().CheckUserCredentials(c.Request().Context(), "lol@example.com", "wahoo123").Return(postgres.User{}, services.ErrUserNotFound)

	err := h.PostLogin(c)
	if assert.Error(t, err) {
		he, ok := err.(*echo.HTTPError)
		assert.True(t, ok)
		assert.Equal(t, http.StatusNotFound, he.Code)
	}
}

func TestAuthHandler_LoginUserDoesNotUsePassword(t *testing.T) {
	e := setupTestEcho()

	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)
	testJSON := `{"email":"lol@example.com","password":"wahoo123"}`

	req := httptest.NewRequest(http.MethodPost, "/login", strings.NewReader(testJSON))
	req.Header.Add(echo.HeaderContentType, echo.MIMEApplicationJSON)

	rec := httptest.NewRecorder()
	c := e.NewContext(req, rec)

	mockUserSvc.EXPECT().CheckUserCredentials(c.Request().Context(), "lol@example.com", "wahoo123").Return(postgres.User{}, services.ErrUserDoesNotUsePassword)

	err := h.PostLogin(c)
	if assert.Error(t, err) {
		he, ok := err.(*echo.HTTPError)
		assert.True(t, ok)
		assert.Equal(t, http.StatusUnprocessableEntity, he.Code)
	}
}

func TestAuthHandler_LoginServerError(t *testing.T) {
	e := setupTestEcho()

	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)
	testJSON := `{"email":"lol@example.com","password":"wahoo123"}`

	req := httptest.NewRequest(http.MethodPost, "/login", strings.NewReader(testJSON))
	req.Header.Add(echo.HeaderContentType, echo.MIMEApplicationJSON)

	rec := httptest.NewRecorder()
	c := e.NewContext(req, rec)

	mockUserSvc.EXPECT().CheckUserCredentials(c.Request().Context(), "lol@example.com", "wahoo123").Return(postgres.User{}, errors.New("test error"))

	err := h.PostLogin(c)
	if assert.Error(t, err) {
		he, ok := err.(*echo.HTTPError)
		assert.True(t, ok)
		assert.Equal(t, http.StatusInternalServerError, he.Code)
	}
}

func TestAuthHandler_LoginWrongPassword(t *testing.T) {
	e := setupTestEcho()

	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)
	testJSON := `{"email":"lol@example.com","password":"wahoo123"}`

	req := httptest.NewRequest(http.MethodPost, "/login", strings.NewReader(testJSON))
	req.Header.Add(echo.HeaderContentType, echo.MIMEApplicationJSON)

	rec := httptest.NewRecorder()
	c := e.NewContext(req, rec)

	mockUserSvc.EXPECT().CheckUserCredentials(c.Request().Context(), "lol@example.com", "wahoo123").Return(postgres.User{}, services.ErrUserWrongPassword)

	err := h.PostLogin(c)
	if assert.Error(t, err) {
		he, ok := err.(*echo.HTTPError)
		assert.True(t, ok)
		assert.Equal(t, http.StatusForbidden, he.Code)
	}
}

func TestAuthHandler_LoginSuccess(t *testing.T) {
	e := setupTestEcho()

	mockUserSvc := mocks.NewMockIUserService(t)
	mockSessSvc := mocks.NewMockISessionService(t)
	mockTimeSvc := mocks.NewMockITimeService(t)

	now := time.Now()
	h := handler.NewAuthHandler(mockUserSvc, mockSessSvc, mockTimeSvc, cookieDomain, cookieSecure)
	testJSON := `{"email":"lol@example.com","password":"wahoo123"}`

	req := httptest.NewRequest(http.MethodPost, "/login", strings.NewReader(testJSON))
	req.Header.Add(echo.HeaderContentType, echo.MIMEApplicationJSON)

	rec := httptest.NewRecorder()
	c := e.NewContext(req, rec)
	userID := uuid.New()
	userIDStr := userID.String()
	userName := "hello"

	mockUserSvc.EXPECT().CheckUserCredentials(c.Request().Context(), "lol@example.com", "wahoo123").Return(postgres.User{
		ID:   userID,
		Name: userName,
	}, nil)
	mockTimeSvc.EXPECT().Now().Return(now)
	mockSessSvc.EXPECT().CreateSession(c.Request().Context(), services.SessionData{
		UserID:    &userIDStr,
		Email:     &userName,
		IsGuest:   false,
		CreatedAt: now,
	}).Return("test", nil)

	err := h.PostLogin(c)
	if assert.NoError(t, err) {
		assert.Equal(t, http.StatusOK, rec.Code)
	}
}
