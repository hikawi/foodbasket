env "local" {
  migration {
    dir = "file://migrations"
    revisions_schema = "atlas_schema_revisions"
  }

  schemas = ["public"]
  dev = "docker://postgres/18/dev?search_path=public"
  url = "postgres://postgres:postgres@localhost:5432/foodbasket?sslmode=disable&search_path=public"

  format {
    migrate {
      diff = "{{ sql . \"  \" }}"
    }
  }
}
