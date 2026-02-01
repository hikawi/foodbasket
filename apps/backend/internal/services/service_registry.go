// Package services provides a set of injectable interfaces
// for decoupling logic and easier mocking
package services

type ServiceRegistry struct {
	RandomService     IRandomService
	UserService       IUserService
	PasswordService   IPasswordService
	ValkeyService     IValkeyService
	SessionService    ISessionService
	PermissionService IPermissionService
}
