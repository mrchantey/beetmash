set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load

default:
	just --list --unsorted

run example *args:
	cargo run --example {{example}} {{args}}

cli *args:
	cargo run -p beetmash-cli -- {{args}}

export-scenes *args:
	cargo run --example export_scenes {{args}}

export-type-registry *args:
	cargo run --example export_type_registry
	cp ./target/type_registries/replication_registry.json \
	../beetmash-site/packages/comp-solid/src/demo/default-replication-registry.json

export-typescript *args:
	cargo run --example export_typescript
	rm -rf ../beetmash-site/packages/editor/src/serdeTypes || true
	mkdir -p ../beetmash-site/packages/editor/src/serdeTypes
	cp -r target/typescript/* ../beetmash-site/packages/editor/src/serdeTypes

install-cli *args:
	cargo install --path ./crates/cli {{args}}

build-wasm *args:
	just export-scenes
	beetmash build \
	--example app \
	--release \
	--copy-local ../beetmash-apps \
	--copy-scenes scenes
	beetmash build \
	-p beetmash_template --example app \
	--release \
	--copy-local ../beetmash-apps \
	--copy-scenes crates/beetmash_template/scenes \
	{{args}}

build-wasm-test *args:
	just cli build \
	-p beetmash_template --example app \
	--release	\
	--copy-local ../beetmash-apps \
	--copy-scenes crates/beetmash_template/scenes \
	{{args}}

export-test-scene:
	cargo run -p beetmash_scene --example export_test_scene

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


test *args:
	just watch 'cargo test --workspace --lib -- {{args}}'

test-core *args:
	just watch 'cargo test -p beetmash_core --lib -- {{args}}'
test-net *args:
	just watch 'cargo test -p beetmash_net --lib -- {{args}}'





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