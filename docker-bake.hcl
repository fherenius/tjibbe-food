group "default" {
  targets = ["app"]
}

target "app" {
  context = "."
  dockerfile = "Dockerfile"
  tags = [
    "nutrient-calculator:latest",
  ]
  platforms = ["linux/amd64"]
  args = {
    RUST_VERSION = "1.82"
  }
}