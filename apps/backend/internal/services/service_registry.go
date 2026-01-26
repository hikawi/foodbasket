package services

type IUserService interface {
	CheckUserCredentials(email, password string) bool
}

type ServiceRegistry struct {
	UserService *IUserService
}
