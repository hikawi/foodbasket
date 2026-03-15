-- Modify "customer_profiles" table
ALTER TABLE "public"."customer_profiles" ADD COLUMN "name" text NOT NULL;
-- Modify "staff_profiles" table
ALTER TABLE "public"."staff_profiles" ADD COLUMN "name" text NOT NULL;
-- Modify "system_profiles" table
ALTER TABLE "public"."system_profiles" ADD COLUMN "name" text NOT NULL;
