release:
	cargo release --no-dev-version --skip-publish minor
release-patch:
	cargo release --no-dev-version --skip-publish patch
release-major:
	cargo release --no-dev-version --skip-publish major
