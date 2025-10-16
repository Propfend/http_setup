RUST_LOG ?= debug

# -----------------------------------------------------------------------------
# Real targets
# -----------------------------------------------------------------------------

package-lock.json: package.json
	npm install --package-lock-only

node_modules: package-lock.json
	npm install --from-lockfile
	touch node_modules

# -----------------------------------------------------------------------------
# Phony targets
# -----------------------------------------------------------------------------

.PHONY: clean
clean:
	rm -rf esbuild-meta.json
	rm -rf node_modules
	rm -rf static/*
	rm -rf target

esbuild: node_modules
	node esbuild.mjs

watch: node_modules
	./jarmuz-watch.mjs

release: node_modules
	./jarmuz-release.mjs
	