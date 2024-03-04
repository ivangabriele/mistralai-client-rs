SHELL := /bin/bash

.PHONY: test

define RELEASE_TEMPLATE
	conventional-changelog -p conventionalcommits -i ./CHANGELOG.md -s
	git add .
	git commit -m "docs(changelog): update"
	git push origin HEAD
	cargo release $(1) --execute
	git push origin HEAD --tags
endef

release-patch:
	$(call RELEASE_TEMPLATE,patch)

release-minor:
	$(call RELEASE_TEMPLATE,minor)

release-major:
	$(call RELEASE_TEMPLATE,major)

test:
	@source ./.env && cargo test --all-targets --no-fail-fast
test-cover:
	cargo tarpaulin --all-targets --frozen --no-fail-fast --out Xml --skip-clean
test-watch:
	cargo watch -x "test -- --all-targets --nocapture"
