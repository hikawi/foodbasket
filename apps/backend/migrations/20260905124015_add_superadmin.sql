-- Modify "branches" table
ALTER TABLE "public"."branches" ADD COLUMN "description" text NULL, ADD COLUMN "avatar_url" text NULL, ADD COLUMN "banner_url" text NULL, ADD COLUMN "location_primary" text NULL, ADD COLUMN "location_secondary" text NULL;
-- Modify "staff_profiles" table
ALTER TABLE "public"."staff_profiles" ADD COLUMN "superadmin" boolean NOT NULL DEFAULT false;
-- Modify "system_profiles" table
ALTER TABLE "public"."system_profiles" ADD COLUMN "superadmin" boolean NOT NULL DEFAULT false;
-- Modify "tenants" table
ALTER TABLE "public"."tenants" ADD COLUMN "description" text NULL, ADD COLUMN "avatar_url" text NULL, ADD COLUMN "banner_url" text NULL;
