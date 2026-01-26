-- Create "tenants" table
CREATE TABLE "tenants" (
  "id" uuid NOT NULL,
  "name" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id")
);
-- Create index "idx_tenants_deleted_at" to table: "tenants"
CREATE INDEX "idx_tenants_deleted_at" ON "tenants" ("deleted_at");
-- Create "roles" table
CREATE TABLE "roles" (
  "id" uuid NOT NULL,
  "name" text NOT NULL,
  "tenant_id" uuid NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "roles_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_roles_tenant_id" to table: "roles"
CREATE INDEX "idx_roles_tenant_id" ON "roles" ("tenant_id");
-- Create "permissions" table
CREATE TABLE "permissions" (
  "id" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id")
);
-- Create "roles_permissions" table
CREATE TABLE "roles_permissions" (
  "role_id" uuid NOT NULL,
  "permission_id" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("role_id", "permission_id"),
  CONSTRAINT "roles_permissions_permission_id_fkey" FOREIGN KEY ("permission_id") REFERENCES "permissions" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "roles_permissions_role_id_fkey" FOREIGN KEY ("role_id") REFERENCES "roles" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_roles_permissions_permission_id" to table: "roles_permissions"
CREATE INDEX "idx_roles_permissions_permission_id" ON "roles_permissions" ("permission_id");
-- Create "users" table
CREATE TABLE "users" (
  "id" uuid NOT NULL,
  "name" text NOT NULL,
  "email" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id")
);
-- Create index "idx_users_deleted_at" to table: "users"
CREATE INDEX "idx_users_deleted_at" ON "users" ("deleted_at");
-- Create index "idx_users_email_active" to table: "users"
CREATE UNIQUE INDEX "idx_users_email_active" ON "users" ("email") WHERE (deleted_at IS NULL);
-- Create "users_roles" table
CREATE TABLE "users_roles" (
  "user_id" uuid NOT NULL,
  "role_id" uuid NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("user_id", "role_id"),
  CONSTRAINT "users_roles_role_id_fkey" FOREIGN KEY ("role_id") REFERENCES "roles" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "users_roles_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_users_roles_role_id" to table: "users_roles"
CREATE INDEX "idx_users_roles_role_id" ON "users_roles" ("role_id");
