SHELL := /bin/bash

.PHONY: doc readme test

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

readme:
	@echo "Generating README.md from template..."
	@> README.md # Clear README.md content before starting
	@while IFS= read -r line || [[ -n "$$line" ]]; do \
		if [[ $$line == *"<CODE>"* && $$line == *"</CODE>"* ]]; then \
			example_path=$$(echo $$line | sed -n 's/.*<CODE>\(.*\)<\/CODE>.*/\1/p'); \
			if [ -f $$example_path ]; then \
				echo '```rs' >> README.md; \
				cat $$example_path >> README.md; \
				echo '```' >> README.md; \
			else \
				echo "Error: Example $$example_path not found." >&2; \
			fi; \
		else \
			echo "$$line" >> README.md; \
		fi; \
	done < README.template.md
	@echo "README.md has been generated."

release-patch:
	$(call RELEASE_TEMPLATE,patch)
release-minor:
	$(call RELEASE_TEMPLATE,minor)
release-major:
	$(call RELEASE_TEMPLATE,major)

test:
	@$(source_env_if_not_ci)
	cargo test --no-fail-fast
test-cover:
	@$(source_env_if_not_ci)
	cargo llvm-cov
test-doc:
	@$(source_env_if_not_ci)
	cargo test --doc --no-fail-fast
test-examples:
	@$(source_env_if_not_ci)
	@for example in $$(ls examples/*.rs | sed 's/examples\/\(.*\)\.rs/\1/'); do \
		echo "Running $$example"; \
		cargo run --example $$example; \
	done
test-watch:
	@source ./.env
	cargo watch -x "test -- --nocapture"
