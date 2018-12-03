# rust_blog

rust博客系统，欢迎朋友们一起加入贡献！欢迎加qq群：945017397

Once the container is running, open http://0.0.0.0:8001/ in your web browser.

## Developing
Clone this repo, the dependent post repo, then build and link.

```sh
git clone git@github.com:shirdonliao/rust_blog.git && cd rust_blog
git clone git@github.com:shirdonliao/rust_blog/posts.git
rustup override set $(cat .rustup)
cargo build
cargo run
```

Iterate and verify:

```sh
cargo run
cargo fmt
cargo test
cargo doc
```
