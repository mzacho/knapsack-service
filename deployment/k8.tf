resource "kubernetes_namespace" "example" {
  metadata {
    name = "example"
  }
}

resource "kubernetes_deployment" "api" {
  metadata {
    name = "api"
    namespace = kubernetes_namespace.example.metadata.0.name
  }
  spec {
    replicas = 1
    selector {
      match_labels {
        app = "knapsack-api"
      }
    }
    template {
      metadata {
        labels {
          app = "knapsack-api"
        }
      }
      spec {
        container = {
          image = docker_image.api.
        }
      }
    }
  }
}
