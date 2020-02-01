alias b := build
alias br := build-release
alias c := check
alias t := test
alias tp := test-print

build:
    cargo watch --clear --exec run

build-release: 
	cargo build --release

check: 
    cargo watch --clear --exec check

test: 
    cargo watch --clear --exec test

test-print:
    cargo test -- --nocapture

# clean up feature branch BRANCH
done BRANCH:
    git checkout master
    git diff --no-ext-diff --quiet --exit-code
    git pull --rebase origin master
    git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
    git branch -D {{BRANCH}}


publish:
	cargo build
	cargo publish


