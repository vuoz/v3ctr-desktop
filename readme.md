## Run

1. Open 2 terminal windows
2. In the first run: ( this builds the systems )
```bash
cargo watch -w v3ctr -w components -x "build -p v3ctr --features dynamic"
``` 
3. In the second run: ( this build the main.rs file and the hot reload system )
```bash
 cargo run --features reload,dynamic --target-dir "target-bin"
```
