# μFork

A pure-actor virtual machine with object-capabilities and memory-safety.
The **uFork** processor architecture features
instruction-level concurrency,
automatic memory-management,
and fine-grained resource limits.

![μFork logo](ufork-wasm/www/ufork_logo.svg)

The blog post
"[Memory Safety Simplifies Microprocessor Design](http://www.dalnefre.com/wp/2022/08/memory-safety-simplifies-microprocessor-design/)"
describes the high-level architecture,
and the rationale behind it.

![uFork Rust/WASM debugger](screenshot20230410.png)

## Implementations

The initial prototype is [implemented in **C**](ufork-c/README.md)
and hosts a Scheme REPL with actor extensions.

The [Rust/WASM version](ufork-wasm/README.md)
is intended to be a more-robust implementation.
Multiple instances of the WASM component
can be instantiated to simulate a multi-core
or distributed system.

An FPGA implementation is planned for the future.

### Technology Stacks

```
+-----+--------+-------+-------+---------+-------------+
| ASM | Scheme | Humus | Misty | ...     | Scheme REPL |
|-----+--------+-------+-------+---------|-------------|
| uFork-CRLF intermediate representation | uFork VM    |
|========================================|=============|
| Rust (no-std)                | Verilog | C           |
|------+-----+--------+--------|---------|-------------|
| WASM | ARM | RISC-V | Xtensa | FPGA    | Host O/S    |
+------+-----+--------+--------+---------+-------------+
```

The C proof-of-concept was built to run
in a traditional hosted O/S environment,
and does not track the evolving semantics
of the uFork IR (intermediate representation).

The Rust implementation (and eventually the Verilog)
conforms to a common uFork IR semantics.
The Rust code-base currently targets WASM,
but it should be possible to compile the core
for a variety of other Rust target platforms.
The future Verilog implementation
will target small FPGA devices
as a secure soft-core processor.

## Documentation

  * uFork [Project Summary](summary.md)
  * uFork [Virtual Machine](ufork.md) description
  * [Actor Protocol Design](proto.md) in uFork
  * [Object Capabilities](ocaps.md) in uFork
  * uFork [Design Notes](design.md)

## License

[Apache License](LICENSE), Version 2.0

## Project Support

<img src="NLnet_banner.png" alt="Logo NLnet: abstract logo of four people seen from above" width="128" height="48" style="padding: 1ex 1em; background: #FFF;" />
<img src="NGI0Entrust_tag.svg" alt="Logo NGI Zero: letterlogo shaped like a tag" width="128" height="48" style="padding: 1ex 1em; background: inherit;" />

[This project](https://nlnet.nl/project/uFork/)
was funded through the [NGI0 Entrust](https://nlnet.nl/entrust) Fund,
a fund established by [NLnet](https://nlnet.nl/) with financial support from
the European Commission's [Next Generation Internet](https://ngi.eu/) programme,
under the aegis of DG Communications Networks,
Content and Technology under grant agreement N<sup>o</sup> 101069594.
