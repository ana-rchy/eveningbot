from rust
workdir /

run mkdir src
run echo 'fn main(){println!("Hello world")}'>src/main.rs

copy Cargo.toml Cargo.toml
copy Cargo.lock Cargo.lock

run cargo build --release

copy src/ src/
run cargo build --release

entrypoint ["/target/release/eveningbot"]
