-- Modify "tenants" table
ALTER TABLE "tenants" ADD COLUMN "slug" text NOT NULL;
-- Create index "idx_tenants_slug" to table: "tenants"
CREATE UNIQUE INDEX "idx_tenants_slug" ON "tenants" ("slug") WHERE (deleted_at IS NULL);
