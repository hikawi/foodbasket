package services

import "time"

type ITimeService interface {
	Now() time.Time
}

type TimeService struct{}

func NewTimeService() ITimeService {
	return new(TimeService)
}

func (s *TimeService) Now() time.Time {
	return time.Now()
}
