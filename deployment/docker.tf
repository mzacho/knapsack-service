resource "docker_image" "api" {
  name = "api"
  build {
    context = "../services/api"
    dockerfile = "../services/rust-1.79-Dockerfile"
    build_arg = {
      APP_NAME : "api"
      DEPENDENCIES : "libpq-dev"
    }
  }
}

resource "docker_image" "optimizer" {
  name = "api"
  build {
    context = "../services/optimizer"
    dockerfile = "../services/rust-1.79-Dockerfile"
    build_arg = {
      APP_NAME : "optimizer"
      DEPENDENCIES : "libpq-dev"
    }
  }
}

resource "docker_image" "postgres" {
  name = "postgres"
  build {
    context = "../db/postgres"
  }
}

resource "docker_image" "rabbitmq" {
  name = "rabbitmq"
  build {
    context = "../db/rabbitmq"
  }
}
