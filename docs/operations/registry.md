# Container Registry

## GHCR Publishing

- GitHub Actions builds and pushes on `main` and tags.
- Images are tagged `ghcr.io/<owner>/<repo>:latest` and `ghcr.io/<owner>/<repo>:sha-<short>`.

## Usage

- Pull: `docker pull ghcr.io/<owner>/<repo>:latest`.
- Deploy: update compose `image` if you want to run prebuilt images.
