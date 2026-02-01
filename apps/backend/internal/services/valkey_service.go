package services

import (
	"context"
	"time"

	"github.com/valkey-io/valkey-go"
)

type IValkeyService interface {
	Get(ctx context.Context, key string) (string, error)
	Set(ctx context.Context, key string, val string, ttl time.Duration) error
	SetNx(ctx context.Context, key string, val string, ttl time.Duration) error
	SetXx(ctx context.Context, key string, val string, ttl time.Duration) error
	Del(ctx context.Context, keys ...string) error
	Sadd(ctx context.Context, key string, members ...string) error
	Sremove(ctx context.Context, key string, val string) error
	Smembers(ctx context.Context, key string) ([]string, error)
	Sismember(ctx context.Context, key string, val string) (bool, error)

	// Higher-level functions for valkey service
	Sset(ctx context.Context, key string, members []string) error
}

type ValkeyService struct {
	client valkey.Client
}

func NewValkeyService(client valkey.Client) IValkeyService {
	return &ValkeyService{
		client: client,
	}
}

func (s *ValkeyService) Get(ctx context.Context, key string) (string, error) {
	res := s.client.Do(ctx, s.client.B().Get().Key(key).Build())
	return res.ToString()
}

func (s *ValkeyService) Set(ctx context.Context, key string, val string, ttl time.Duration) error {
	return s.client.Do(ctx, s.client.B().Set().Key(key).Value(val).Ex(ttl).Build()).Error()
}

func (s *ValkeyService) SetNx(ctx context.Context, key string, val string, ttl time.Duration) error {
	return s.client.Do(ctx, s.client.B().Set().Key(key).Value(val).Nx().Ex(ttl).Build()).Error()
}

func (s *ValkeyService) SetXx(ctx context.Context, key string, val string, ttl time.Duration) error {
	return s.client.Do(ctx, s.client.B().Set().Key(key).Value(val).Xx().Ex(ttl).Build()).Error()
}

func (s *ValkeyService) Del(ctx context.Context, keys ...string) error {
	return s.client.Do(ctx, s.client.B().Del().Key(keys...).Build()).Error()
}

func (s *ValkeyService) Sadd(ctx context.Context, key string, members ...string) error {
	return s.client.Do(ctx, s.client.B().Sadd().Key(key).Member(members...).Build()).Error()
}

func (s *ValkeyService) Sremove(ctx context.Context, key string, member string) error {
	return s.client.Do(ctx, s.client.B().Srem().Key(key).Member(member).Build()).Error()
}

func (s *ValkeyService) Smembers(ctx context.Context, key string) ([]string, error) {
	return s.client.Do(ctx, s.client.B().Smembers().Key(key).Build()).AsStrSlice()
}

func (s *ValkeyService) Sismember(ctx context.Context, key string, member string) (bool, error) {
	res := s.client.Do(ctx, s.client.B().Sismember().Key(key).Member(member).Build())
	if err := res.Error(); err != nil {
		return false, err
	}

	v, err := res.AsInt64()
	return v == 1, err
}

func (s *ValkeyService) Sset(ctx context.Context, key string, members []string) error {
	b := s.client.B()

	cmds := make([]valkey.Completed, 0, 3)
	cmds = append(cmds, b.Del().Key(key).Build())
	cmds = append(cmds, b.Sadd().Key(key).Member(members...).Build())
	cmds = append(cmds, b.Expire().Key(key).Seconds(3600).Build())

	for _, res := range s.client.DoMulti(ctx, cmds...) {
		if err := res.Error(); err != nil {
			return err
		}
	}
	return nil
}
