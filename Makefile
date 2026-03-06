target/debug/git-contributors: src
	cargo build

target/release/git-contributors: src
	cargo build --release

# NOTE: you will need to add these custom paths to your shell PATH
# for this to work. See manpath(1).
install: target/release/git-contributors
	cp target/release/git-contributors ~/bin
	cp doc/git-contributors.1 ~/man/man1

test:
	set -e
	cargo test
	tests/test_simple_git_repo.sh
