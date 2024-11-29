set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load

default:
	just --list --unsorted


app *scenes:
	cargo run --example app -- {{scenes}}


app-terminal:
	just app \
	scenes/camera-2d.json \
	scenes/ui-terminal-input.json \
app-space:
	just app \
	scenes/camera-2d.json \
	scenes/space-scene.json	\


run example *args:
	cargo run --example {{example}} {{args}}

cli *args:
	cargo run -p beetmash-cli -- {{args}}

export-scenes *args:
	cargo run --example export_scenes {{args}}
	cd crates/beetmash_template && cargo run --example export_scenes {{args}}

ts-dst:= '../beetmash-site/packages/editor/src/serdeTypes/generated'

export-typescript *args:
	cargo run --example export_typescript
	rm -rf {{ts-dst}} || true
	mkdir -p {{ts-dst}}
	cp -r target/typescript/* {{ts-dst}}

install-cli *args:
	cargo install --path ./crates/cli {{args}}

build-wasm *args:
	@echo "exporting beetmash"
	just export-scenes
	beetmash build \
	--example app \
	--release \
	--copy-local ../beetmash-apps \
	--copy-scenes scenes \
	--copy-registries target/registries {{args}}
	@echo "exporting beet"
	cd crates/beetmash_template && just export-scenes
	beetmash build \
	-p beetmash_template --example app \
	--release \
	--copy-local ../beetmash-apps \
	--copy-scenes crates/beetmash_template/scenes \
	--copy-registries crates/beetmash_template/target/registries {{args}}

build-wasm-test *args:
	just cli build \
	-p beetmash_template --example app \
	--release	\
	--copy-local ../beetmash-apps \
	--copy-scenes crates/beetmash_template/scenes \
	--copy-registries target/registries \
	{{args}}

export-test-scene:
	cargo run -p beetmash_scene --example export_test_scene


test *args:
	just watch 'cargo test --workspace --lib -- {{args}}'

test-core *args:
	just watch 'cargo test -p beetmash_core --lib -- {{args}}'
test-net *args:
	just watch 'cargo test -p beetmash_net --lib -- {{args}}'
test-scene *args:
	just watch 'cargo test -p beetmash_scene --lib -- {{args}}'





publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

publish-all *args:
	just publish beetmash_scene 	 {{args}} || true
	just publish beetmash_net 		 {{args}} || true
	just publish beetmash_core 		 {{args}} || true
	just publish beetmash 				 {{args}} || true
	just publish beetmash_template {{args}}	|| true
# just publish beetmash_server 	 {{args}} || true
# just publish beetmash-cli 		 {{args}}	|| true


patch:
	cargo set-version --bump patch

watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-- {{command}}