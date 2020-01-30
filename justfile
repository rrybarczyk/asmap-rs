test: 
    cargo watch --clear --exec test

check: 
    cargo watch --clear --exec check

test-print:
    cargo test -- --nocapture

build: 
	cargo build --release

r:
    cargo watch --clear --exec run

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
