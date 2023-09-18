# LLM WEB API SERVER

> Note: Before reading the following content, please make sure that you are working in an environment of Ubuntu 20.04/22.04 and have installed the following necessary dependencies:
>
> * Rust-stable (>= 1.69.0)
> * Add `wasm32-wasi` target to Rust toolchain by running `rustup target add wasm32-wasi` in the terminal
> * WasmEdge 0.13.4 ([Installation](https://wasmedge.org/docs/start/install#generic-linux-and-macos))
> * WasmEdge TLS plugin ([Installation](https://wasmedge.org/docs/start/install#tls-plug-in))
> 

## How to build and run?

- First, build the `llm-web-api-server` wasm app:

    ```bash
    git clone https://github.com/apepkuss/llm-web-api-server.git

    cd llm-web-api-server

    // build the wasm app
    cargo build --target wasm32-wasi --release
    ```

    If the commands are successful, you should find the wasm app in `target/wasm32-wasi/release/llm-web-api-server.wasm`.

- Second, to maximize the performance of the wasm app, use `WasmEdge AOT Compiler` to compile the wasm app to native code:

    ```bash
    wasmedge compile target/wasm32-wasi/release/llm-web-api-server.wasm llm-web-api-server.so
    ```

    If the command is successful, you should find `llm-web-api-server.so` in the root directory.

- Finally, run the wasm app, namely starting the web API server:

    ```bash
    wasmedge run --dir .:. llm-web-api-server.so
    ```

    if the command is successful, you should see the following output in the terminal:

    ```bash
    Listening on http://0.0.0.0:8080
    ```

## Test the web API server

`llm-web-api-server` provides a POST API `/echo` for testing. You can use `curl` to test it:

```bash
curl -X POST http://localhost:8080/echo
```

If the command is successful, you should see the following output in the terminal:

```bash
echo test
```
