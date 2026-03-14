cargo build --release;

./target/release/avatahex -- $(echo -n "hello" | sha1sum | cut -d' ' -f1) ./hello.svg