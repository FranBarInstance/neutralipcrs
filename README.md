
Neutral TS Rust IPC Client
==========================

Neutral TS is a **safe, modular, language-agnostic template engine** built in Rust. It works as a **native Rust library** or via **IPC** for other languages like Python and PHP. With Neutral TS you can reuse the **same template across multiple languages** with consistent results.

Examples for [Rust](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/rust), [Python](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python), [PHP](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/php), [Node.js](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/node) and [Go](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/go) here: [download](https://github.com/FranBarInstance/neutralts-docs/releases). All PWA [examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples) use the same template: [Neutral templates](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/neutral).

The documentation of the **web template** engine is here: [template engine doc](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/) and **Rust** documentation here: [Rust doc](https://docs.rs/neutralts/latest/neutralts/).

Rust IPC use
------------

```
use neutralipcrs::NeutralIpcTemplate;
use serde_json::json;

let schema = json!({
    "data": {
        "hello": "Hello World"
    }
});

let template = Template::from_file_value("file.ntpl", schema).unwrap();
let contents = template.render().unwrap();

// e.g.: 200
let status_code: &str = template.get_status_code();

// e.g.: OK
let status_text: &str = template.get_status_text();

// empty if no error
let status_param: &str = template.get_status_param();

// act accordingly at this point according to your framework
```

- Requires the IPC server: [Neutral TS IPC Server](https://github.com/FranBarInstance/neutral-ipc/releases)
- Requires the Rust IPC client: [Neutral TS Rust IPC Client](https://crates.io/crates/neutralipcrs)


Neutral TS template engine
--------------------------

- [Rust docs](https://docs.rs/neutralts/latest/neutralts/)
- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [IPC server](https://github.com/FranBarInstance/neutral-ipc/releases)
- [IPC clients](https://github.com/FranBarInstance/neutral-ipc/tree/master/clients)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [IPC Client Crate](https://crates.io/crates/neutralipcrs)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
