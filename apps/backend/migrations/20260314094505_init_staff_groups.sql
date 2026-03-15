-- Modify "branches" table
ALTER TABLE "public"."branches" ADD CONSTRAINT "uidx_branches_tenant_id_id" UNIQUE ("tenant_id", "id");
-- Create "staff_groups" table
CREATE TABLE "public"."staff_groups" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "tenant_id" uuid NOT NULL,
  "branch_id" uuid NULL,
  "name" text NOT NULL,
  "description" text NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  "deleted_at" timestamptz NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "fk_branch_tenant_consistency" FOREIGN KEY ("tenant_id", "branch_id") REFERENCES "public"."branches" ("tenant_id", "id") ON UPDATE NO ACTION ON DELETE NO ACTION,
  CONSTRAINT "staff_groups_branch_id_fkey" FOREIGN KEY ("branch_id") REFERENCES "public"."branches" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "staff_groups_tenant_id_fkey" FOREIGN KEY ("tenant_id") REFERENCES "public"."tenants" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- Create index "idx_staff_groups_tenant_id" to table: "staff_groups"
CREATE INDEX "idx_staff_groups_tenant_id" ON "public"."staff_groups" ("tenant_id");
-- Create "staff_profiles_branches" table
CREATE TABLE "public"."staff_profiles_branches" (
  "staff_profile_id" uuid NOT NULL,
  "branch_id" uuid NOT NULL,
  "is_primary" boolean NULL DEFAULT false,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("staff_profile_id", "branch_id"),
  CONSTRAINT "staff_profiles_branches_branch_id_fkey" FOREIGN KEY ("branch_id") REFERENCES "public"."branches" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "staff_profiles_branches_staff_profile_id_fkey" FOREIGN KEY ("staff_profile_id") REFERENCES "public"."staff_profiles" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- Create index "idx_staff_branches_branch_id" to table: "staff_profiles_branches"
CREATE INDEX "idx_staff_branches_branch_id" ON "public"."staff_profiles_branches" ("branch_id");
-- Create "staff_profiles_staff_groups" table
CREATE TABLE "public"."staff_profiles_staff_groups" (
  "staff_profile_id" uuid NOT NULL,
  "staff_group_id" uuid NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  CONSTRAINT "pk_staff_profiles_staff_groups" PRIMARY KEY ("staff_profile_id", "staff_group_id"),
  CONSTRAINT "staff_profiles_staff_groups_staff_group_id_fkey" FOREIGN KEY ("staff_group_id") REFERENCES "public"."staff_groups" ("id") ON UPDATE NO ACTION ON DELETE CASCADE,
  CONSTRAINT "staff_profiles_staff_groups_staff_profile_id_fkey" FOREIGN KEY ("staff_profile_id") REFERENCES "public"."staff_profiles" ("id") ON UPDATE NO ACTION ON DELETE CASCADE
);
-- Create index "uidx_staff_profiles_staff_groups_reverse" to table: "staff_profiles_staff_groups"
CREATE UNIQUE INDEX "uidx_staff_profiles_staff_groups_reverse" ON "public"."staff_profiles_staff_groups" ("staff_group_id", "staff_profile_id");
