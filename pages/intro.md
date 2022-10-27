# WebAssembly

WebAssembly is a low-level platform that can be run in modern web browsers with near-native performance. It is primarily a compilation target for languages like C, C++, or Rust.

WebAssembly modules can be imported into a web or Node.js environment, exposing a JavaScript interface to call high-performance functions. This allows JavaScript frameworks to use WebAssembly to gain massive performance advantages while making functionality easily available to web developers.

Historically, the browser's virtual machine was only able to load JavaScript. This posed an issue for high-performance use cases like 3D games, Virtual / Augmented Reality, computer vision, image/video editing, and other domains that demand native performance.

The cost of downloading, parsing, and compiling large JavaScript applications also became prohibitive. WebAssembly provides a more compact format to deliver content.

WebAssembly is primarily meant to be a low-level assembly-like language with a compact binary format. Currently, WebAssembly supports languages with low-level memory models but it has the high-level goal of supporting other languages with garbage-collected memory models in the future.

## Key concepts

- **Module**: Represents a WebAssembly binary that has been compiled by the browser into executable machine code. A Module is **stateless** and thus can be explicitly shared between windows and worders. A Module declares imports and exports just like an ES module.
- **Memory**: A resizable `ArrayBuffer` that contains the linear array of bytes read and written by WebAssembly's low-level memory access instructions.
- **Table**: A resizable typed array of references (e.g. to functions) that could not otherwise be stored as raw bytes in Memory (for safety and portability reasons).
- **Instance**: A Module paired with all the state it uses at runtime including a Memory, Table, and a set of imported values. An Instance is like an ES module that has been loaded into a particular global with a particular set of imports.

## JavaScript API

The JavaScript API provides develoeprs with the ability to create modules, memories, tables, and instances.

Given a WebAssembly instance, JavaScript code can synchronously call its exports, which are exposed as normal JavaScript functions.

Arbitrary JavaScript functions can also be synchronously called by WebAssembly code by passing in those JavaScript functions as the imports to a WebAssembly instance.
