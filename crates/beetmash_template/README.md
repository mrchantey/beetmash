# Beetmash Template

- [docs](https://beetmash.com/docs/beetmash)
- [github](https://github.com/mrchantey/beetmash)

A demonstration of the workflow for publishing Bevy apps and scenes to [beetmash](https://beetmash.com).


### Optimization table

Based on Bevy's internal optimizations, commands used with the `base_app` example, measured on 13/09/24 with `bevy@0.14.2` and `beetmash@0.0.6-rc.4`.

| Command                                              | size       | Notes                    |
| ---------------------------------------------------- | ---------- | ------------------------ |
| `cargo build --release`                              | `46.65 MB` |                          |
| `cargo build --release && wasm-opt -Oz`              | `26.11 MB` | About half               |
| `cargo build --profile wasm-release`                 | `46.65 MB` | No measurable difference |
| `cargo build --profile wasm-release && wasm-opt -Oz` | `46.65 MB` | No measurable difference |

## Getting started

Running `cargo run` won't do much, the app is a blank canvas 🖌️

1. Export scenes: `cargo run --bin export_scenes`
2. Run the app: `cargo run scenes/my_base_scene.json scenes/my_beautiful_scene.json`


## Deploying Apps

Beetmash currently does not host apps, the simplest approach is to deploy to Github Pages and link to that. Because Bevy apps can be several megabytes at least, I recommend creating a new repo just for releases so it doesn't clog up your codebase repo.

For cross-repo deployment you will need to generate a Personal Access Token and place it in your github secrets.
- https://github.com/settings/tokens?type=beta
- 

```sh
# setup repo
gh auth login
gh repo create my-releases --public --confirm
gh repo clone my-releases
cd my-releases
# initial commit
echo "# My GitHub Pages Site" > README.md
echo "<div>hello world</div>" > index.html
git add .
git commit -m "Initial commit with README"
git push origin main
```
