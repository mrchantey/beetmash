set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load

default:
	just --list --unsorted



test-all *args:
	just watch 'cargo test --workspace --lib -- {{args}}'

test-core *args:
	just watch 'cargo test -p beetmash_core --lib -- {{args}}'
test-net *args:
	just watch 'cargo test -p beetmash_net --lib -- {{args}}'





publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

publish-all *args:
	just publish beetmash_core 		 {{args}} || true
	just publish beetmash_net 		 {{args}} || true
	just publish beetmash_server 	 {{args}} || true
	just publish beetmash 				 {{args}} || true
	just publish beetmash_template {{args}}	|| true
	just publish beetmash-cli 		 {{args}}	|| true

patch:
	cargo set-version --bump patch


watch *command:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**' \
	-i '**/mod.rs' \
	-- {{command}}