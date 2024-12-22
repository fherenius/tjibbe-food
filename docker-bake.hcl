variable "IMAGE_NAME" {
  default = "$IMAGE_NAME"
}

variable "REGISTRY" {
  default = "$REGISTRY"
}

variable "REPOSITORY" {
  default = "${REGISTRY}/${IMAGE_NAME}"
}

variable "COMMIT_SHA" {
  default = "$COMMIT_SHA"
}

group "default" {
  targets = ["app"]
}

target "app" {
  context = "."
  dockerfile = "Dockerfile"
  tags       = [
    "${REPOSITORY}:${COMMIT_SHA}",
    "${REPOSITORY}:latest"
  ]
  platforms = ["linux/amd64"]
  args = {
    RUST_VERSION = "1.82"
  }
  cache-to = [
    "type=gha,mode=max"
  ]
  cache-from = [
    "type=gha"
  ]
}