package main

import (
	"context"
	"log"
	"net/http"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/labstack/echo/v5"
	httpSwagger "github.com/swaggo/http-swagger"
	"github.com/valkey-io/valkey-go"
	_ "luny.dev/foodbasket/docs"
	"luny.dev/foodbasket/internal/app"
	"luny.dev/foodbasket/internal/postgres"
	"luny.dev/foodbasket/internal/router"
	"luny.dev/foodbasket/internal/services"
)

func setupPostgres(ctx context.Context, url string) *postgres.Queries {
	pool, err := pgxpool.New(ctx, url)
	if err != nil {
		log.Fatalf("failed to open connection to database: %v\n", err)
	}
	return postgres.New(pool)
}

func setupServices(q postgres.Querier, valkeyClient valkey.Client) services.ServiceRegistry {
	randomService := services.NewRandomService()
	passwordService := services.NewPasswordService(randomService)
	userService := services.NewUserService(q, passwordService)
	valkeyService := services.NewValkeyService(valkeyClient)
	sessionService := services.NewSessionService(valkeyService, randomService)
	permissionService := services.NewPermissionService(q, valkeyService)
	tenantService := services.NewTenantService(q, valkeyService)
	timeService := services.NewTimeService()

	return services.ServiceRegistry{
		RandomService:     randomService,
		UserService:       userService,
		PasswordService:   passwordService,
		ValkeyService:     valkeyService,
		SessionService:    sessionService,
		PermissionService: permissionService,
		TenantService:     tenantService,
		TimeService:       timeService,
	}
}

// @title			FoodBasket
// @version		1.0
// @description	Backend documentation for FoodBasket platform.
// @contact.name	Nguyệt Ánh
// @contact.url	https://www.luny.dev
// @contact.email	hello@luny.dev
// @license.name	Apache 2.0
// @license.url	http://www.apache.org/licenses/LICENSE-2.0.html
// @BasePath		/v1
func main() {
	appConfig := app.LoadConfig()
	ctx := context.Background()

	// Setup the application.
	q := setupPostgres(ctx, appConfig.PostgresURL)

	// Setup the valkey connection
	valkeyClient, err := valkey.NewClient(valkey.ClientOption{InitAddress: appConfig.ValkeyURLs})
	if err != nil {
		log.Fatalf("failed to create a valkey client: %v\n", err)
	}
	defer valkeyClient.Close()

	// Setup services.
	serviceRegistry := setupServices(q, valkeyClient)

	e := router.SetupServer(serviceRegistry)
	versionedRouter := e.Group("/v1")
	router.SetupRoutes(versionedRouter, appConfig, serviceRegistry)

	// Setup swagger
	e.GET("/swagger", func(c *echo.Context) error {
		return c.Redirect(http.StatusMovedPermanently, "/swagger/index.html")
	})
	e.GET("/swagger/*", func(c *echo.Context) error {
		handler := httpSwagger.WrapHandler
		handler(c.Response(), c.Request())
		return nil
	})

	// Start to initialize a config
	if err := e.Start(":8080"); err != nil {
		e.Logger.Error("failed to start server", "error", err)
	}
}
