group "default" {
  targets = ["catix", "catix-distroless"]
}

target "catix" {
  dockerfile = "dev-support/containers/alpine/Containerfile"
  platforms  = ["linux/amd64"]
  target     = "catix"
  contexts = {
    sccache = "docker-image://ghcr.io/thxnet/ci-containers/sccache:0.5.4"
    rust    = "docker-image://docker.io/library/rust:1.74.0-alpine3.18"
    alpine  = "docker-image://docker.io/library/alpine:3.18"
  }
  args = {
    RUSTC_WRAPPER         = "/usr/bin/sccache"
    AWS_ACCESS_KEY_ID     = null
    AWS_SECRET_ACCESS_KEY = null
    SCCACHE_BUCKET        = null
    SCCACHE_ENDPOINT      = null
    SCCACHE_S3_USE_SSL    = null
  }
  labels = {
    "description"                     = "Container image for Catix"
    "image.type"                      = "final"
    "image.authors"                   = "46590321+xrelkd@users.noreply.github.com"
    "image.vendor"                    = "xrelkd"
    "image.description"               = "Catix - Nix binary cache proxy service"
    "org.opencontainers.image.source" = "https://github.com/xrelkd/catix"
  }
}

target "catix-distroless" {
  dockerfile = "dev-support/containers/distroless/Containerfile"
  platforms  = ["linux/amd64"]
  target     = "catix"
  contexts = {
    sccache    = "docker-image://ghcr.io/thxnet/ci-containers/sccache:0.5.4"
    rust       = "docker-image://docker.io/library/rust:1.74-slim-buster"
    distroless = "docker-image://gcr.io/distroless/cc-debian11:latest"
  }
  args = {
    RUSTC_WRAPPER         = "/usr/bin/sccache"
    AWS_ACCESS_KEY_ID     = null
    AWS_SECRET_ACCESS_KEY = null
    SCCACHE_BUCKET        = null
    SCCACHE_ENDPOINT      = null
    SCCACHE_S3_USE_SSL    = null
  }
  labels = {
    "description"                     = "Container image for Catix"
    "image.type"                      = "final"
    "image.authors"                   = "46590321+xrelkd@users.noreply.github.com"
    "image.vendor"                    = "xrelkd"
    "image.description"               = "Catix - Nix binary cache proxy service"
    "org.opencontainers.image.source" = "https://github.com/xrelkd/catix"
  }
}
