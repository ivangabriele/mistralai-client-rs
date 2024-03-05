SHELL := /bin/bash

.PHONY: test

define source_env_if_not_ci
	@if [ -z "$${CI}" ]; then \
		if [ -f ./.env ]; then \
			source ./.env; \
		else \
			echo "No .env file found"; \
			exit 1; \
		fi \
	fi
endef

define RELEASE_TEMPLATE
	conventional-changelog -p conventionalcommits -i ./CHANGELOG.md -s
	git add .
	git commit -m "docs(changelog): update"
	git push origin HEAD
	cargo release $(1) --execute
	git push origin HEAD --tags
endef

doc:
	cargo doc
	open ./target/doc/mistralai_client/index.html

release-patch:
	$(call RELEASE_TEMPLATE,patch)

release-minor:
	$(call RELEASE_TEMPLATE,minor)

release-major:
	$(call RELEASE_TEMPLATE,major)

test:
	@$(source_env_if_not_ci) && cargo test --no-fail-fast
test-cover:
	@$(source_env_if_not_ci) && cargo llvm-cov
test-doc:
	@$(source_env_if_not_ci) && cargo test --doc --no-fail-fast
test-watch:
	@source ./.env && cargo watch -x "test -- --nocapture"
