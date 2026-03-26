# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [0.4.0] - 2026-03-18

### Release Highlights

- Ariel OS's MSRV is now 1.94. This version stabilizes Cargo's config `include` key, allowing Ariel OS's configuration to be discovered automatically by Cargo, without the need for either providing `--config` or using the nightly version of Cargo, including in out-of-tree applications. ([#1865](https://github.com/ariel-os/ariel-os/pull/1865))
- (Xtensa) The toolchain for Xtensa targets has been bumped to 1.94.0.2. ([#1911](https://github.com/ariel-os/ariel-os/pull/1911))
- The minimum required laze version is now 0.1.39. This internally allows to use Cargo's output directly without copying to a file with the `.elf` extension. ([#1896](https://github.com/ariel-os/ariel-os/pull/1896))
- (ESP32) BLE is now supported on ESP32 MCUs, and exposed through the [existing BLE API](https://ariel-os.github.io/ariel-os/0.4.0/docs/book/bluetooth.html). ([#1869](https://github.com/ariel-os/ariel-os/pull/1869))

### Breaking Changes

- The variant of the `IntoPeripheral` trait that is used only for documentation is now sealed. It should never be implemented on anything outside of Ariel OS. ([#1910](https://github.com/ariel-os/ariel-os/pull/1910))
- (ESP32-C3) Support for the undocumented `ai-c3` board has been removed, as it was unclear what board this was. ([#1912](https://github.com/ariel-os/ariel-os/pull/1912))
- Timing functionality is now not indirectly enabled by enabling networking anymore. Applications that were relying on this simply need to enable the `time` Cargo feature. ([#1897](https://github.com/ariel-os/ariel-os/pull/1897))
- `embassy-net` has been updated to v0.8.0: the [network stack](https://ariel-os.github.io/ariel-os/0.4.0/docs/api/ariel_os/net/type.NetworkStack.html) is now provided by that version, which is a breaking change. The crate is re-exported from `ariel_os::reexports`. ([#1653](https://github.com/ariel-os/ariel-os/pull/1653))
- The `embassy-time` time driver is now *only* provided by HALs when the already-existing `time` Cargo feature is enabled. Existing applications that relied on it being always enabled will need to select that Cargo feature. ([#1809](https://github.com/ariel-os/ariel-os/pull/1809))
- (nRF) The protection circuit on NFC pins has been re-enabled, disabling the GPIO functionality on them. If needed, GPIO functionality can be re-enabled out of tree by enabling `embassy-nrf`'s `nfc-pins-as-gpio` Cargo feature. ([#1857](https://github.com/ariel-os/ariel-os/pull/1857))
- (STM32U083C-DK) All three LEDs are now supported and ordered according to the on-board labeling. Notably, `led0` changed from the blue LD4 LED to the green LD3 LED. ([#1856](https://github.com/ariel-os/ariel-os/pull/1856))
- The `network-config-override` Cargo feature has been removed from the documentation, as it was not possible to actually use it, and has been replaced by the laze module of the same name, which must be used instead. ([#1612](https://github.com/ariel-os/ariel-os/pull/1612))
- A lifetime generic has been introduced on `ariel_os::i2c::controller::I2cDevice`. This avoids the need for storing it in a global `OnceCell` (or similar) inside applications. It is a breaking change but should not require changes to many existing applications in practice. ([#1793](https://github.com/ariel-os/ariel-os/pull/1793))
- A lifetime generic has been introduced on `ariel_os::spi::main::SpiDevice`. This avoids the need for storing it in a global `OnceCell` (or similar) inside applications. It is a breaking change but should not require changes to many existing applications in practice. ([#1790](https://github.com/ariel-os/ariel-os/pull/1790))
- `ariel_os::reexports::ble` has been removed. This export was misplaced and was not adding functionality. ([#1788](https://github.com/ariel-os/ariel-os/pull/1788))

### Fixed

- The internal runtime loop is now not busy-looping anymore when using the `executor-interrupt` flavor. This may bring significant power consumption improvements when using that executor flavor. ([#1808](https://github.com/ariel-os/ariel-os/pull/1808))
- Dependency on `embedded-nal-coap` has been pinned due to its alpha status and known breakage in its version `0.1.0-alpha.6`. ([#1812](https://github.com/ariel-os/ariel-os/pull/1812))
- It has been clarified in the documentation that `network-config-*` laze modules must come before the `network` laze module. ([#1796](https://github.com/ariel-os/ariel-os/pull/1796))

### Added

- The book now states that `defmt` is the default logger when available for the target. ([#1916](https://github.com/ariel-os/ariel-os/pull/1916))
- Facilities for measuring stack usage, using stack painting, have been added. ([#1538](https://github.com/ariel-os/ariel-os/pull/1538))
- The `network` laze module is now automatically enabled by network-link laze modules (e.g., `usb-ethernet`, `wifi-esp`). ([#1895](https://github.com/ariel-os/ariel-os/pull/1895))
- (ESP32) USB CDC-NCM (Ethernet over USB) is now supported on ESP32-S2 and ESP32-S3. ([#1785](https://github.com/ariel-os/ariel-os/pull/1785))
- A sensor extension has been added, `GnssTimeExt`, that allows to read the time and date from a GNSS sensor. ([#1683](https://github.com/ariel-os/ariel-os/pull/1683))
- CI workflows are now checked for security issues with [zizmor](https://docs.zizmor.sh/), a static analysis tool for Github Actions. ([#1738](https://github.com/ariel-os/ariel-os/pull/1738))
- (nRF91) The nRF91xx MCUs can now be initialized in DECT-2020 NR+ mode. ([#1819](https://github.com/ariel-os/ariel-os/pull/1819))
- The porting guide has been updated to mention that new laze builders also need to be added to the support information when adding new boards to it. ([#1813](https://github.com/ariel-os/ariel-os/pull/1813))
- (STM32) It is now possible to provide an RCC configuration from out-of-tree applications, allowing in particular to adjust the clock configuration to the board and the application. ([#1389](https://github.com/ariel-os/ariel-os/pull/1389))
- The book's glossary has been corrected and expanded. ([#1745](https://github.com/ariel-os/ariel-os/pull/1745))
- The documentation about adding a new MCU has been updated to include the file `ariel-chips.yaml`. ([#1772](https://github.com/ariel-os/ariel-os/pull/1772))

### Changed

- Timer-related log statements have been demoted from `debug` to `trace` log level. ([#1906](https://github.com/ariel-os/ariel-os/pull/1906))
- The `featurecomb` dependency has been updated to v0.2.0. It has been relicensed to MIT OR Apache-2.0 (from MPL-2.0). ([#1873](https://github.com/ariel-os/ariel-os/pull/1873))

### New Supported Hardware

- The STM32WBA65RI MCU and the ST NUCLEO-WBA65RI board are now supported. ([#1771](https://github.com/ariel-os/ariel-os/pull/1771))
- The Seeed Studio XIAO nRF52840 Plus board is now supported. ([#1817](https://github.com/ariel-os/ariel-os/pull/1817))
- The STM32F303CB and STM32F303RE MCUs and the ST NUCLEO-F303RE board are now supported. ([#1781](https://github.com/ariel-os/ariel-os/pull/1781))

## [0.3.0] - 2026-02-02

### Release Highlights

- Ariel OS's MSRV is now 1.91. `laze build install-toolchain` can be used to update the toolchains. ([#1720](https://github.com/ariel-os/ariel-os/pull/1720))
- The Embassy and `esp-hal`-related crates have been updated to the following versions: ([#1328](https://github.com/ariel-os/ariel-os/pull/1328))

  - `embassy-executor@0.9.1`
  - `embassy-net@0.7.1`
  - `embassy-nrf@0.8.0`
  - `embassy-rp@0.8.0`
  - `embassy-stm32@0.4.0`
  - `embassy-sync@0.7.2`
  - `embassy-time@0.5.0`
  - `embassy-usb@0.5.1`
  - `embassy-usb-driver@0.2.0`
  - `esp-hal@1.0.0`

  This comes with some potentially breaking changes:

  - The GPIO, I2C, and SPI drivers have been adjusted to take instances of `IntoPeripheral`s following the upstream removal of the unsound `Peripheral` type, which has been replaced with different solutions in Embassy and `esp-hal`. The `IntoPeripheral` trait unifies this in Ariel OS and should most often require no changes to applications.
  - Some of these crates were re-exported in `ariel_os::reexports`, upstream breaking changes are therefore exposed this way.
  - The `executor-single-thread` async executor flavor has been removed.
  - The network stack returned by `ariel_os::net::network_stack()` now comes from `embassy-net@0.7.1`.
  - The module-level types from `ariel_os::async` now come from `embassy-executor@0.9.1`.
  - The `Mutex` type required by `ariel_os::i2c::controller::I2cDevice::new()` and `ariel_os::spi::main::SpiDevice::new()` now comes from `embassy-sync@0.7.2`.
  - The `ariel_os::usb::UsbBuilder` type now comes from `embassy-usb@0.5.1` and the `ariel_os::usb::UsbDriver` type from `embassy-usb-driver@0.2.0`.
  - (ESP32-S3) Multicore support on ESP32-S3 had to be disabled as a temporary workaround following changes in `esp-hal` requiring that peripheral instances be pinned to the core they have been initialized on.
  - (ESP32) The GPIO peripheral types have been revisited: there is now a dedicated type per GPIO, instead of type aliases.
  - (STM32F767ZI) The single-bank flash setup is now enabled on this MCU.
- The hardware support documentation has been revamped: chips and boards now have dedicated pages, and board pages list laze builders that can be used. ([#1574](https://github.com/ariel-os/ariel-os/pull/1574))
- Support for IPv6 has been added, which can be used alongside IPv4 or in place of it. Only static configuration is currently supported. See [the networking documentation](https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html) for details. ([#1377](https://github.com/ariel-os/ariel-os/pull/1377))
- A native target has been introduced: it allows running an application as a Linux process. This is especially useful for experimenting without a physical board, testing applications, and for simulation purposes. A subset of features is currently supported, and will be expanded in the future. ([#1617](https://github.com/ariel-os/ariel-os/pull/1617))
- The value of Cargo's `include` unstable configuration key has been updated to not use the now-unsupported string-only value. Existing applications need to update the value in their `.cargo/config.toml` configuration file to use the array or table types instead. ([#1572](https://github.com/ariel-os/ariel-os/pull/1572))
- Bluetooth Low Energy (BLE) is now supported on the nRF52 and the nRF53 chip families and on the Raspberry Pi Pico W and Pico 2 W boards using the onboard CYW43 chip. Two examples are available for testing: `ble-advertiser` and `ble-scanner`. See [the documentation](https://ariel-os.github.io/ariel-os/dev/docs/book/bluetooth.html) for details. ([#1560](https://github.com/ariel-os/ariel-os/pull/1560))
- A custom sensor abstraction has been introduced: sensor drivers can be written against it, and sensor driver instances can be registered in a sensor registry inside an application. The registry then allows to query sensor driver instances and fetch their readings asynchronously. The `sensors-debug` example is available for testing. See the documentation of `ariel_os::sensors` for details. ([#1313](https://github.com/ariel-os/ariel-os/pull/1313))
- A UART abstraction has been introduced, similar to the I2C and SPI abstractions. Drivers are provided for each currently supported HAL: ESP32, nRF, RP, and STM32. ([#1365](https://github.com/ariel-os/ariel-os/pull/1365))
- Board pin information is now read from declarative files and processed by [`sbd-gen`](https://github.com/ariel-os/sbd). This makes adding support for new boards easier, allows moving pin information out of applications, and these SBD files should be re-usable by other projects. ([#1397](https://github.com/ariel-os/ariel-os/pull/1397))
- The `network-config-dhcp` and `network-config-static` laze modules have been renamed to `network-config-ipv4-dhcp` and `network-config-ipv4-static` respectively. The old names are now deprecated. ([#1348](https://github.com/ariel-os/ariel-os/pull/1348))

### Breaking Changes

- The laze contexts and builders targeting the application core of the nRF5340 chip have been renamed. The chip laze context is now named `nrf5340-app` and the laze builder of the nRF5340-DK board targeting the application core is now `nrf5340dk-app`. This is a breaking change for applications relying on the laze context for feature-gating, and when targeting this specific development kit. ([#1699](https://github.com/ariel-os/ariel-os/pull/1699))
- The `ariel_os::asynch::blocker::block_on()` function has been moved into `ariel_os::thread` and is now `ariel_os::thread::block_on()`. ([#1567](https://github.com/ariel-os/ariel-os/pull/1567))
- (nRF5340, nRF9151, nRF9160) The `SERIAL3` peripheral is now dedicated to the UART drivers instead of the SPI drivers. ([#1507](https://github.com/ariel-os/ariel-os/pull/1507))
- (ST NUCLEO-WB55) The SWI has been switched from `LPUART1` to `USART1` to free up the interrupt for UART. ([#1457](https://github.com/ariel-os/ariel-os/pull/1457))
- (STM32U083C-DK) The SWI has been switched from `USART2_LPUART2` to `USART4_LPUART3` to free up the interrupt for UART. ([#1456](https://github.com/ariel-os/ariel-os/pull/1456))
- New laze contexts have been introduced for ESP32 chips with in-package flash. No existing `esp32*` chip laze modules have been deleted, but board laze builders have been adjusted to use the new ones when appropriate, which can be breaking if applications were relying on these for feature-gating. ([#1433](https://github.com/ariel-os/ariel-os/pull/1433))
- The documentation of the `ariel_os::time` module has been clarified: its items must only be used in combination with other items from that module, and not be passed as arguments to other crates. ([#1321](https://github.com/ariel-os/ariel-os/pull/1321))
- (ST NUCLEO-WBA55) The SWI has been switched from `LPUART1` to `USART2` to free up the interrupt for UART. ([#1203](https://github.com/ariel-os/ariel-os/pull/1203))
- The `network-config-static` Cargo feature has been removed from the documentation. It should not be used directly. ([#1090](https://github.com/ariel-os/ariel-os/pull/1090))
- The `ariel_os::debug::log::print!()` macro has been removed in favor of `println!()` to reduce RAM usage. Providing `print!()` required keeping a dedicated RTT channel when using `defmt`. ([#1052](https://github.com/ariel-os/ariel-os/pull/1052))

### Fixed

- The custom panic handler is now only provided on embedded architectures. This fixes potential issues when running host tests or generating documentation. ([#1614](https://github.com/ariel-os/ariel-os/pull/1614))
- (ESP32-S3) Using GPIO26 to GPIO48 is now supported on this MCU. ([#1210](https://github.com/ariel-os/ariel-os/pull/1210))
- Log statements are now properly filtered based on their log level when using the `log` logger. ([#1152](https://github.com/ariel-os/ariel-os/pull/1152))
- (RP235x) The `ariel_os::random` module is now seeding its RNGs from the TRNG (which is not available on the RP2040), instead of relying on the `RoscRng`. ([#1077](https://github.com/ariel-os/ariel-os/pull/1077))

### Added

- (ESP32-S2) USB is now marked as supported on this MCU. ([#1767](https://github.com/ariel-os/ariel-os/pull/1767))
- (ESP32-S2) I2C is now marked as supported on this MCU. ([#1766](https://github.com/ariel-os/ariel-os/pull/1766))
- (ESP32-S3) I2C is now marked as supported on this MCU. ([#1765](https://github.com/ariel-os/ariel-os/pull/1765))
- A `tcp-client` example is now available: it makes it easy to test Internet connectivity without requiring a HWRNG. ([#1690](https://github.com/ariel-os/ariel-os/pull/1690))
- (nRF5340) Both the application core and the network core of this MCU are now supported and their usage is documented. ([#1658](https://github.com/ariel-os/ariel-os/pull/1658))
- The concept of laze builder is now explained in the documentation. ([#1619](https://github.com/ariel-os/ariel-os/pull/1619))
- A thermometer example is now available: it demonstrates usage of the new sensor API and, on the STM32U083C-DK, displays the reading on the onboard LCD. ([#1530](https://github.com/ariel-os/ariel-os/pull/1530))
- Using `embassy-time` types (e.g., `Timer`) within threads is now supported. A generic timer queue is used, whose size can be configured using the `generic-timer-queue-*` Cargo features. ([#1555](https://github.com/ariel-os/ariel-os/pull/1555))
- (ESP32-S3) USB is now supported on this MCU. ([#1561](https://github.com/ariel-os/ariel-os/pull/1561))
- The `defmt` Cargo feature is now propagated to `embedded-hal`, `embedded-hal-async`, `embedded-io`, and `embedded-io-async`. ([#1535](https://github.com/ariel-os/ariel-os/pull/1535))
- The `Debug2Format` and `Display2Format` decorators are now provided for the `log` logging facade as well (on top of `defmt`'s), improving the portability of log statements. ([#1485](https://github.com/ariel-os/ariel-os/pull/1485))
- (ESP32-C3, ESP32-C6) Wi-Fi and multithreading can now be used at the same time. ([#1455](https://github.com/ariel-os/ariel-os/pull/1455))
- Support for the `getrandom` crate has been added. Applications can now use it directly or through transitive dependencies and the CSPRNG will automatically be seeded appropriately. See [the documentation](https://ariel-os.github.io/ariel-os/dev/docs/book/randomness.html) for details. ([#1416](https://github.com/ariel-os/ariel-os/pull/1416))
- (STM32U083MC) Networking is now supported on this MCU. It was previously disabled because of lack of RAM. ([#1177](https://github.com/ariel-os/ariel-os/pull/1177))
- A `multicast` Cargo feature is now exposed, that enables multicast on the network stack. ([#1336](https://github.com/ariel-os/ariel-os/pull/1336))
- The `ariel_os::time` module now provides a `with_timeout()` function. ([#1329](https://github.com/ariel-os/ariel-os/pull/1329))
- (STM32F4) The flash cache is now enabled. This increases performance on supported STM32F4 MCUs. ([#1201](https://github.com/ariel-os/ariel-os/pull/1201))
- The `#[thread]` attribute macro now allows pinning threads to specific cores using the `affinity` parameter. ([#1134](https://github.com/ariel-os/ariel-os/pull/1134))
- (Cortex-M) Hard floats are now supported on this architecture: applications are now compiled with the `eabihf` variant, and floating point registers are now saved and restored by the preemptive scheduler as necessary. ([#1097](https://github.com/ariel-os/ariel-os/pull/1097))
- A laze task has been added to configure Visual Studio Code and derivatives to work well in an Ariel OS project, you can read about it [in the book](https://ariel-os.github.io/ariel-os/dev/docs/book/vscode-configuration.html). ([#1049](https://github.com/ariel-os/ariel-os/pull/1049))
- (nRF5340) The HWRNG is now supported on the network core, allowing to use the `ariel_os::random` module. ([#1102](https://github.com/ariel-os/ariel-os/pull/1102))
- An `i2c-scanner` example is now available: it allows finding connected I2C devices. ([#1071](https://github.com/ariel-os/ariel-os/pull/1071))

### Changed

- `defmt-rtt` is now used in place of `rtt-target` when using `defmt`. ([#1328](https://github.com/ariel-os/ariel-os/pull/1328))
- The `executor-interrupt` flavor is now using the lowest interrupt priority on STM32 and nRF MCUs. This allows using BLE in combination with that executor flavor on nRF. ([#1168](https://github.com/ariel-os/ariel-os/pull/1168))
- DHCPv4 is now only enabled when the `network-config-ipv4-dhcp` laze module (formerly `network-config-dhcp`) is enabled, instead of always being enabled. This may reduce the size of applications not using DHCP. ([#1378](https://github.com/ariel-os/ariel-os/pull/1378))

### New Supported Hardware

- The Adafruit Feather nRF52840 Express and Sense boards are now supported. ([#1622](https://github.com/ariel-os/ariel-os/pull/1622))
- The Seeed Studio XIAO ESP32C6 board is now supported. ([#1479](https://github.com/ariel-os/ariel-os/pull/1479))
- The STM32H753ZI MCU and the ST NUCLEO-H753ZI board are now supported. ([#1494](https://github.com/ariel-os/ariel-os/pull/1494))
- The Espressif ESP32-C3-DevKit-RUST-1 board is now supported. ([#1466](https://github.com/ariel-os/ariel-os/pull/1466))
- The nRF9151-DK board is now supported. ([#1463](https://github.com/ariel-os/ariel-os/pull/1463))
- The ESP32-S2, ESP32-S2Fx2, ESP32-S2Fx4, ESP32-S2Fx4R2 MCUs, the ESP32-S2-SOLO-2 hardware module, and the Espressif ESP32-S2-DevKitC-1 board are now supported. ([#1465](https://github.com/ariel-os/ariel-os/pull/1465))
- The STM32U073KC MCU is now supported. ([#1183](https://github.com/ariel-os/ariel-os/pull/1183))
- The Seeed Studio LoRa-E5 mini board is now supported. ([#1125](https://github.com/ariel-os/ariel-os/pull/1125))
- The Heltec WiFi LoRa 32 V3 board is now supported. ([#1199](https://github.com/ariel-os/ariel-os/pull/1199))
- The STM32U585AI MCU and the ST STEVAL-MKBOXPRO board are now supported. ([#1117](https://github.com/ariel-os/ariel-os/pull/1117))
- The BBC micro:bit V1 board is now supported. ([#1068](https://github.com/ariel-os/ariel-os/pull/1068))
- The nRF52-DK board is now supported. ([#1066](https://github.com/ariel-os/ariel-os/pull/1066))
- The STM32WBA55CG MCU and the ST NUCLEO-WBA55CG board are now supported. ([#1064](https://github.com/ariel-os/ariel-os/pull/1064))
- The STM32F042K6 MCU and the ST NUCLEO-F042K6 board are now supported. ([#1050](https://github.com/ariel-os/ariel-os/pull/1050))
- The STM32L475VG MCU and the ST B-L475E-IOT01A board are now supported. ([#1034](https://github.com/ariel-os/ariel-os/pull/1034))

### New Sensor Drivers

- A sensor driver for the LPS22DF, compatible with the newly-introduced sensor API, is now available. ([#1418](https://github.com/ariel-os/ariel-os/pull/1418))
- A sensor driver for the LIS2DU12, compatible with the newly-introduced sensor API, is now available. ([#1431](https://github.com/ariel-os/ariel-os/pull/1431))
- A sensor driver for the STTS22H, compatible with the newly-introduced sensor API, is now available. ([#1363](https://github.com/ariel-os/ariel-os/pull/1363))

## [0.2.1] - 2025-06-24

### Fixed

- fix(deps): bump `static_cell` as it fixed a soundness issue ([#1107](https://github.com/ariel-os/ariel-os/pull/1107))
- fix(deps): disable static cell nightly feature ([#1106](https://github.com/ariel-os/ariel-os/pull/1106))

## [0.2.0] - 2025-05-07

This release allows Ariel OS to be built on stable Rust, and updates
all crates to edition 2024.
Apart from that, it adds support for a couple of new boards. And a lot of
internal polish that is not mentioned here.

### Added

- feat(build): default to `stable` build ([#987](https://github.com/ariel-os/ariel-os/pull/987))
- feat(boards): add support for the ST NUCLEO-F411RE ([#1002](https://github.com/ariel-os/ariel-os/pull/1002))
- feat: Add power management crate & implement reboot function ([#910](https://github.com/ariel-os/ariel-os/pull/910))
- feat(rt): more flexible stacksize configuration ([#786](https://github.com/ariel-os/ariel-os/pull/786))
- feat(stm32): allow the interrupt executor on STM32 ([#871](https://github.com/ariel-os/ariel-os/pull/871))
- feat(network): seed `embassy_net` from the device ID when no RNG ([#873](https://github.com/ariel-os/ariel-os/pull/873))
- feat(coap): support stored security configurations ([#814](https://github.com/ariel-os/ariel-os/pull/814))
- feat(network): Add ethernet from nucleo-144 board family ([#993](https://github.com/ariel-os/ariel-os/pull/993))
- feat(boards): add support for the SMT32U083C-DK ([#986](https://github.com/ariel-os/ariel-os/pull/986))
- feat(boards): add support for the FireBeetle 2 ESP32-C6 ([#983](https://github.com/ariel-os/ariel-os/pull/983))
- feat(boards): add initial support for Espressif ESP32-C3-LCDkit ([#477](https://github.com/ariel-os/ariel-os/pull/477))
- feat(boards): add support for the Nordic Thingy:91 X ([#974](https://github.com/ariel-os/ariel-os/pull/974))
- feat(boards): add support for the Raspberry Pi Pico 2 W ([#943](https://github.com/ariel-os/ariel-os/pull/943))
- feat(nrf): add basic support for nRF9160 ([#926](https://github.com/ariel-os/ariel-os/pull/926))
- feat(board): add support for the ST-NUCLEO-C031C6 board  ([#838](https://github.com/ariel-os/ariel-os/pull/838))

### Changed

- refactor(stm32)!: remove unneeded info from laze context names  ([#961](https://github.com/ariel-os/ariel-os/pull/961))
- chore(build): re-enable sccache ([#970](https://github.com/ariel-os/ariel-os/pull/970))
- fix(task-macro): avoid the need for importing `UsbBuilderHook` ([#918](https://github.com/ariel-os/ariel-os/pull/918))
- perf(storage): block on storage init to spare RAM ([#931](https://github.com/ariel-os/ariel-os/pull/931))
- build: enable Rust edition 2024 ([#584](https://github.com/ariel-os/ariel-os/pull/584))

### Fixed

- fix(log): add support for `log` on architectures without atomics ([#990](https://github.com/ariel-os/ariel-os/pull/990))

## [0.1.0] - 2025-02-25

<!-- next-url -->
[Unreleased]: https://github.com/ariel-os/ariel-os/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/ariel-os/ariel-os/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ariel-os/ariel-os/compare/v0.2.0...v0.3.0
[0.2.1]: https://github.com/ariel-os/ariel-os/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ariel-os/ariel-os/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ariel-os/ariel-os/releases/tag/v0.1.0
