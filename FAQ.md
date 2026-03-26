# Frequently Asked Questions

This FAQ attempts to address common questions with regards to Ariel OS. In case of further questions [contact us](https://matrix.to/#/#ariel-os:matrix.org)!

## How is Ariel OS different from other operating systems?

The table below summarizes, at high-level, and based on our experience, a comparison with other operating systems we have tried out. For more details on how Ariel OS relates to Embassy and RIOT, see the next entries.

|                      | Ariel OS           | Embassy            | Tock               | RIOT               | Zephyr             | FreeRTOS           |
| -------------------- | ------------------ | ------------------ | ------------------ | ------------------ | ------------------ | ------------------ |
| Rust-based           | :white_check_mark: | :white_check_mark: | :white_check_mark: | &cross;            | &cross;            | &cross;            |
| Async                | :white_check_mark: | :white_check_mark: | &cross;            | &cross;            | &cross;            | &cross;            |
| Preemptive scheduler | :white_check_mark: | &cross;            | :white_check_mark: | :white_check_mark: | :white_check_mark: | :white_check_mark: |
| Multicore scheduler  | :white_check_mark: | &cross;            | &cross;            | &cross;            | :white_check_mark: | :white_check_mark: |
| App. portability     | :white_check_mark: | &cross;            | &cross;            | :white_check_mark: | &cross;            | &cross;            |


## What is the relationship between Embassy and Ariel OS?

We love Embassy the way it is. Ariel OS would not be possible without it. We'll keep building on top of Embassy: we track Embassy's development, and we work on upstreaming all changes that Embassy deems worthy.

Ariel OS has Embassy at its heart, using it for hardware abstraction, async executor, networking, timers etc. On top of Embassy, Ariel OS adds:

- a multi-core enabled preemptive scheduler
  - this allows mix-and-match of sync and async code within an application
- abstracted peripheral APIs, increasing portability
  - while Embassy's HALs implement the embedded-hal(-async) traits, the initialization API is slightly different for each MCU family (it fits perfectly for the corresponding hardware). Ariel OS abstracts that provides an API that is identical *across* MCU families.
- pre-integrated modules like networking, storage that are ready to use
  - where Embassy and the eco system provide all the building blocks (embassy-net, rand), setup and initialization needs per-MCU copy+pasting and combining from the examples and resources. Ariel OS improves on that, e.g., setting up the hwrng or network stack centrally, making it ready to use for applications.
- a modularized system initialization scheme
  - where Embassy leaves full control to the application developer by requiring all set up to be done in the application's `main()`, by the application developer, Ariel OS does a lot of initialization (e.g., network stack, rng, storage) internally, and hands a ready-to-use system to the developer
- a meta build system that handles supporting many different target devices
  - Ariel OS wraps Cargo in [laze](https://kaspar030.github.io/laze/dev/index.html) in order to manage all the build-system configuration that Cargo cannot handle itself, like `--target`, the used runner, probe-rs configuration, linker setting, that are usually hard-coded per board. laze simplifies building for multiple target boards.

Practically, Ariel OS moves some of the control of the classical main loop most Embassy applications have, and handles a lot of the "generic system" stuff itself. Actual applications contain less system bring-up, less boilerplate, and can concentrate on business logic.

## What is the relationship between Ariel OS and RIOT?

Ariel OS was started by RIOT maintainers as a rewrite of specific components of RIOT, in the context of a cybersecurity research project named [RIOT-fp](https://future-proof-iot.github.io/RIOT-fp/about). Initially the goal was to apply memory safety and modern language concepts to RIOT, while providing  identical C APIs. The project eventually shifted towards not providing identical C APIs, but still builds on concepts and experience from RIOT -- in some sense it is a rewrite of RIOT, but trying to translate meaning rather than words.

There is active collaboration between RIOT and Ariel OS. While applications can generally not be ported between Ariel OS and RIOT OS for the moment, there is already shared code used through Rust RIOT wrappers. We hope to enable some applications to be portable between both platforms in the future. In addition to shared code, we also share infrastructure, have common goals, and meet as part of the RIOT community.


## Does Ariel OS provide an SBOM (Software Bill of Materials)?

All code that goes into an Ariel OS build is gathered, compiled and linked through Rust's built-in cargo package manager, stemming from its default package repository [crates.io](https://crates.io/) or, during development, upstream git repositories. The crate descriptions provide metadata about versions, licenses, upstream web pages and much more. Some of that metadata is checked at CI time; for example, we use [`cargo deny`](https://docs.rs/cargo-deny/latest/cargo_deny/) to vet our licenses. Tools are available to extract information from there (like for any other Rust project), for example [`cargo sbom`](https://crates.io/crates/cargo-sbom).
Note that not all dependencies of Ariel OS are always built for all applications. If you want to evaluate properties of a particular build, tooling is being developed that simplifies calling custom Cargo commands on a particular narrow configuration.
