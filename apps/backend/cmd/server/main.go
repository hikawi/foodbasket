package main

import (
	"context"
	"log"

	"github.com/jackc/pgx/v5/pgxpool"
	"luny.dev/foodbasket/internal/app"
	"luny.dev/foodbasket/internal/postgres"
	"luny.dev/foodbasket/internal/router"
	"luny.dev/foodbasket/internal/services"
)

func main() {
	appConfig := app.LoadConfig()
	ctx := context.Background()

	// Setup the application.
	pool, err := pgxpool.New(ctx, appConfig.PostgresURL)
	if err != nil {
		log.Fatalln("failed to open connection to database")
	}

	q := postgres.New(pool)

	// Setup services.
	randomService := services.NewRandomService()
	passwordService := services.NewPasswordService(randomService)
	userService := services.NewUserService(q, passwordService)
	serviceRegistry := services.ServiceRegistry{
		RandomService:   randomService,
		UserService:     userService,
		PasswordService: passwordService,
	}

	e := router.SetupServer()
	router.SetupRoutes(e, appConfig, serviceRegistry)

	// Start to initialize a config

	if err := e.Start(":8080"); err != nil {
		e.Logger.Error("failed to start server", "error", err)
	}
}
