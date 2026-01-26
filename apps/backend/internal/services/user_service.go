package services

type UserService struct {
}

func (s *UserService) CheckUserCredentials(email, password string) bool {
	return true
}
