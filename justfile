set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]
set dotenv-load
crates := 'beet beet_core beet_ecs beet_net'

default:
	just --list --unsorted


publish crate *args:
	cargo publish -p {{crate}} --allow-dirty --no-verify {{args}}
	sleep 2

publish-all *args:
	just publish beetmash {{args}}					|| true
	just publish beetmash_template {{args}}	|| true


patch:
	cargo set-version --bump patch