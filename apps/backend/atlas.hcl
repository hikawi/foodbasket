env "local" {
  migration {
    dir = "file://migrations"
  }
  dev = "docker://postgres/18/dev?search_path=public"

  format {
    migrate {
      diff = "{{ sql . \"  \" }}"
    }
  }
}
