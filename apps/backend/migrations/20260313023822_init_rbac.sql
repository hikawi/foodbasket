-- Create enum type "scope_type"
CREATE TYPE "public"."scope_type" AS ENUM ('system', 'tenant', 'branch');
-- Create "tenants" table
CREATE TABLE "public"."tenants" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "name" text NOT NULL,
  "slug" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id")
);
-- Create index "idx_tenants_deleted_at" to table: "tenants"
CREATE INDEX "idx_tenants_deleted_at" ON "public"."tenants" ("deleted_at");
-- Create index "uidx_tenants_slug" to table: "tenants"
CREATE UNIQUE INDEX "uidx_tenants_slug" ON "public"."tenants" ("slug") WHERE (deleted_at IS NULL);
-- Create "users" table
CREATE TABLE "public"."users" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "email" text NOT NULL,
  "password" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id")
);
-- Create index "idx_users_deleted_at" to table: "users"
CREATE INDEX "idx_users_deleted_at" ON "public"."users" ("deleted_at");
-- Create index "uidx_users_email" to table: "users"
CREATE UNIQUE INDEX "uidx_users_email" ON "public"."users" ((lower(email))) WHERE (deleted_at IS NULL);
-- Create "customer_profiles" table
CREATE TABLE "public"."customer_profiles" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "tenant_id" uuid NOT NULL,
  "avatar_url" text NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "customer_profiles_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "public"."tenants" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "customer_profiles_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_customer_profiles_deleted_at" to table: "customer_profiles"
CREATE INDEX "idx_customer_profiles_deleted_at" ON "public"."customer_profiles" ("deleted_at");
-- Create index "uidx_customer_profiles_user_id_tenant_id" to table: "customer_profiles"
CREATE UNIQUE INDEX "uidx_customer_profiles_user_id_tenant_id" ON "public"."customer_profiles" ("user_id", "tenant_id") WHERE (deleted_at IS NULL);
-- Create "branches" table
CREATE TABLE "public"."branches" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "name" text NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "branches_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "public"."tenants" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_branches_deleted_at" to table: "branches"
CREATE INDEX "idx_branches_deleted_at" ON "public"."branches" ("deleted_at");
-- Create index "idx_branches_tenant_id" to table: "branches"
CREATE INDEX "idx_branches_tenant_id" ON "public"."branches" ("tenant_id") WHERE (deleted_at IS NULL);
-- Create "policies" table
CREATE TABLE "public"."policies" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "tenant_id" uuid NULL,
  "branch_id" uuid NULL,
  "name" text NOT NULL,
  "statements" jsonb NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "policies_branch_id_fkey" FOREIGN KEY ("branch_id") REFERENCES "public"."branches" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "policies_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "public"."tenants" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_policies_deleted_at" to table: "policies"
CREATE INDEX "idx_policies_deleted_at" ON "public"."policies" ("deleted_at");
-- Create index "idx_policies_tenant_id_branch_id" to table: "policies"
CREATE INDEX "idx_policies_tenant_id_branch_id" ON "public"."policies" ("tenant_id", "branch_id") WHERE (deleted_at IS NULL);
-- Create "staff_profiles" table
CREATE TABLE "public"."staff_profiles" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "tenant_id" uuid NOT NULL,
  "avatar_url" text NULL,
  "pin_code" text NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "staff_profiles_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "public"."tenants" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "staff_profiles_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_staff_profiles_deleted_at" to table: "staff_profiles"
CREATE INDEX "idx_staff_profiles_deleted_at" ON "public"."staff_profiles" ("deleted_at");
-- Create index "uidx_staff_profiles_user_id_tenant_id" to table: "staff_profiles"
CREATE UNIQUE INDEX "uidx_staff_profiles_user_id_tenant_id" ON "public"."staff_profiles" ("user_id", "tenant_id") WHERE (deleted_at IS NULL);
-- Create "system_profiles" table
CREATE TABLE "public"."system_profiles" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "user_id" uuid NOT NULL,
  "avatar_url" text NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "system_profiles_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "public"."users" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION
);
-- Create index "idx_system_profiles_deleted_at" to table: "system_profiles"
CREATE INDEX "idx_system_profiles_deleted_at" ON "public"."system_profiles" ("deleted_at");
-- Create index "uidx_system_profiles_user_id" to table: "system_profiles"
CREATE UNIQUE INDEX "uidx_system_profiles_user_id" ON "public"."system_profiles" ("user_id") WHERE (deleted_at IS NULL);
-- Create "assignments" table
CREATE TABLE "public"."assignments" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "staff_profile_id" uuid NULL,
  "customer_profile_id" uuid NULL,
  "system_profile_id" uuid NULL,
  "policy_id" uuid NOT NULL,
  "scope_type" "public"."scope_type" NOT NULL,
  "scope_id" uuid NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id"),
  CONSTRAINT "assignments_customer_profile_id_fkey" FOREIGN KEY ("customer_profile_id") REFERENCES "public"."customer_profiles" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "assignments_policy_id_fkey" FOREIGN KEY ("policy_id") REFERENCES "public"."policies" ("id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "assignments_staff_profile_id_fkey" FOREIGN KEY ("staff_profile_id") REFERENCES "public"."staff_profiles" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "assignments_system_profile_id_fkey" FOREIGN KEY ("system_profile_id") REFERENCES "public"."system_profiles" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "one_profile_only" CHECK (((((staff_profile_id IS NOT NULL))::integer + ((customer_profile_id IS NOT NULL))::integer) + ((system_profile_id IS NOT NULL))::integer) = 1)
);
-- Create index "uidx_assignments_customer_profile_id" to table: "assignments"
CREATE UNIQUE INDEX "uidx_assignments_customer_profile_id" ON "public"."assignments" ("customer_profile_id", "policy_id", "scope_id") WHERE (customer_profile_id IS NOT NULL);
-- Create index "uidx_assignments_staff_profile_id" to table: "assignments"
CREATE UNIQUE INDEX "uidx_assignments_staff_profile_id" ON "public"."assignments" ("staff_profile_id", "policy_id", "scope_id") WHERE (staff_profile_id IS NOT NULL);
-- Create index "uidx_assignments_system_profile_id" to table: "assignments"
CREATE UNIQUE INDEX "uidx_assignments_system_profile_id" ON "public"."assignments" ("system_profile_id", "policy_id", "scope_id") WHERE (system_profile_id IS NOT NULL);
