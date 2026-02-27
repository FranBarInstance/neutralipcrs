Neutral TS Rust IPC Client
==========================

**Neutral IPC Client** is a Rust library that provides a client for the [Neutral](https://github.com/FranBarInstance/neutralts) template engine via Inter-Process Communication (IPC). It allows you to render templates using a language-agnostic approach, communicating with a Neutral IPC server to process templates with JSON or MsgPack data schemas.

## Features

- **IPC Communication**: Connect to a Neutral template server via TCP
- **Flexible Schema Support**: Use JSON or MsgPack data formats
- **Template Sources**: Load templates from files or inline strings
- **Schema Merging**: Incrementally build complex schemas with deep merge support
- **Status Handling**: Retrieve status codes, messages, and parameters from rendered templates
- **Configurable**: Customize connection settings (host, port, timeout, buffer size)
- **Safe & Reliable**: Built with Rust's type safety and comprehensive error handling

Rust IPC use
------------

```rust
use neutralipcrs::NeutralIpcTemplate;
use serde_json::json;

let schema = json!({
    "data": {
        "hello": "Hello World"
    }
});

let mut template = NeutralIpcTemplate::from_file_value("file.ntpl", schema).unwrap();
let contents = template.render().unwrap();

// e.g.: 200
let status_code: &str = template.get_status_code();

// e.g.: OK
let status_text: &str = template.get_status_text();

// empty if no error
let status_param: &str = template.get_status_param();

// act accordingly at this point according to your framework
```

Rust IPC with MsgPack schema
----------------------------

```rust
use neutralipcrs::NeutralIpcTemplate;
use serde_json::json;

let schema = json!({
    "data": {
        "hello": "Hello World"
    }
});
let schema_msgpack = rmp_serde::to_vec(&schema).unwrap();

let mut template = NeutralIpcTemplate::from_src_msgpack("Message: {:;hello:}", &schema_msgpack).unwrap();
let contents = template.render().unwrap();
assert_eq!(contents, "Message: Hello World");
```

- Requires the IPC server: [Neutral TS IPC Server](https://github.com/FranBarInstance/neutral-ipc/releases)
- Requires the Rust IPC client: [Neutral TS Rust IPC Client](https://crates.io/crates/neutralipcrs)


Neutral TS template engine
--------------------------

- [Rust docs](https://docs.rs/neutralipcrs/latest)
- [Rust IPC Client Crate](https://crates.io/crates/neutralipcrs)
- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [IPC server](https://github.com/FranBarInstance/neutral-ipc/releases)
- [IPC clients](https://github.com/FranBarInstance/neutral-ipc/tree/master/clients)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
