# Glossary

> [!NOTE]
> This glossary does not intend to provide complete definitions of each term.
> Instead, it aims to clarify and emphasize the differences between related terms.
> Links are provided when relevant to learn more about each concept.

- Ariel OS HALs: currently the following crates:
  [`ariel-os-esp`][ariel-os-esp-rustdoc],
  [`ariel-os-nrf`][ariel-os-nrf-rustdoc],
  [`ariel-os-rp`][ariel-os-rp-rustdoc], and
  [`ariel-os-stm32`][ariel-os-stm32-rustdoc].

- **[ARM (or Arm)][arm-arch-wikipedia]:** a family of instruction set architectures.

- **[Arm (Arm Holdings)][arm-holdings]:** the company behind the ARM architecture family.

- **[Chip][integrated-circuit-wikipedia]:** an integrated circuit (IC).

- **[Cortex-M][cortex-m-wikipedia]:** a family of 32-bit processor implementations from Arm.

- **[ESP32][esp32-wikipedia]:** a family of 32-bit microcontrollers from Espressif.
  Its older microcontrollers are based on the Xtensa architecture, while newer ones use RISC-V.
  Can also specifically refer to the eponymous ESP32 microcontroller.

- **[Embassy][embassy-homepage]:** a software project developing HALs for multiple microcontroller families (e.g., [`embassy-nrf`][embassy-nrf-embassydev], [`embassy-rp`][embassy-rp-embassydev], [`embassy-stm32`][embassy-stm32-embassydev]), along with other components (e.g., [`embassy-executor`][embassy-executor-docsrs], [`embassy-time`][embassy-time-docsrs]).

- Embassy-style HALs: currently the following crates:
  [`embassy-nrf`][embassy-nrf-embassydev],
  [`embassy-rp`][embassy-rp-embassydev],
  [`embassy-stm32`][embassy-stm32-embassydev], and
  [`esp-hal`][esp-hal-homepage].
  In particular, these HALs feature [peripheral ZSTs][peripheral-zsts] modeling compile-time
  exclusive access, that drivers require for instantiation.

- **[esp-hal][esp-hal-homepage]:** a HAL for ESP32 microcontrollers, developed by their manufacturer Espressif.

- **HAL (Hardware Abstraction Layer):** a software layer that makes specific pieces of hardware (e.g., microcontroller peripherals) easier to use, by hiding some of their details.

- **[MCU][microcontroller-wikipedia]:** a microcontroller.
  Contains a processor, memory, and peripherals to interact with the outside world.

- **nRF:** a family of 32-bit microcontrollers developed by Nordic Semiconductor.

- **Raspberry Pi:** can refer to the [family of single-board computers][raspberry-pi-sbc-wikipedia] or to [Raspberry Pi Holdings][raspberry-pi-holdings-wikipedia], the company that manufactures them as well as RP microcontrollers.
  Sometimes abbreviated to "RPi."

- **[RISC-V][risc-v-wikipedia]:** an open, royalty-free instruction set architecture.

- **RP:** a family of 32-bit microcontrollers designed by Raspberry Pi, which includes the RP2040 and RP2350 microcontrollers.

- **[SoC (System on Chip)][system-on-chip-wikipedia]:** often synonymous with microcontroller in practice, at least in the embedded domain.
  Sometimes used to emphasize the presence of a sizable analog component (usually for radio).

- **[STM32][stm32-wikipedia]:** a family of 32-bit microcontrollers developed by STMicroelectronics.

- **[UART (Universal Asynchronous Receiver-Transmitter)][uart-wikipedia]:** a (microcontroller) peripheral.
  By extension, can also refer to the link-layer protocol it implements.
  Sometimes incorrectly used to refer to serial communication over USB (e.g., instead of [USB CDC-ACM][usb-cdc-acm-wikipedia]).

- **[USB CDC-ACM][usb-cdc-acm-wikipedia]:** a standard protocol implementing serial communication over USB.

- **[USB CDC-NCM][usb-cdc-ncm-wikipedia]:** a standard protocol implementing Ethernet over USB.

- **[Xtensa][xtensa-wikipedia]:** a family of instruction set architectures, used in some ESP32 microcontrollers.

[ariel-os-esp-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_esp/index.html
[ariel-os-nrf-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_nrf/index.html
[ariel-os-rp-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_rp/index.html
[ariel-os-stm32-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os_stm32/index.html
[arm-arch-wikipedia]: https://en.wikipedia.org/wiki/ARM_architecture_family
[arm-holdings]: https://en.wikipedia.org/wiki/Arm_Holdings
[integrated-circuit-wikipedia]: https://en.wikipedia.org/wiki/Integrated_circuit
[cortex-m-wikipedia]: https://en.wikipedia.org/wiki/ARM_Cortex-M
[embassy-homepage]: https://embassy.dev/
[embassy-executor-docsrs]: https://docs.rs/embassy-executor/latest/embassy_executor/
[embassy-nrf-embassydev]: https://docs.embassy.dev/embassy-nrf/
[embassy-rp-embassydev]: https://docs.embassy.dev/embassy-rp/
[embassy-stm32-embassydev]: https://docs.embassy.dev/embassy-stm32/
[embassy-time-docsrs]: https://docs.rs/embassy-time/latest/embassy_time/
[esp32-wikipedia]: https://en.wikipedia.org/wiki/ESP32
[esp-hal-homepage]: https://docs.espressif.com/projects/rust/esp-hal/latest/
[microcontroller-wikipedia]: https://en.wikipedia.org/wiki/Microcontroller
[peripheral-zsts]: ./application.md#obtaining-peripheral-access
[raspberry-pi-sbc-wikipedia]: https://en.wikipedia.org/wiki/Raspberry_Pi
[raspberry-pi-holdings-wikipedia]: https://en.wikipedia.org/wiki/Raspberry_Pi_Holdings
[risc-v-wikipedia]: https://en.wikipedia.org/wiki/RISC-V
[stm32-wikipedia]: https://en.wikipedia.org/wiki/STM32
[system-on-chip-wikipedia]: https://en.wikipedia.org/wiki/System_on_a_chip
[uart-wikipedia]: https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter
[usb-cdc-acm-wikipedia]: https://en.wikipedia.org/wiki/USB_communications_device_class#Abstract_Control_Model
[usb-cdc-ncm-wikipedia]: https://en.wikipedia.org/wiki/Ethernet_over_USB#Protocols
[xtensa-wikipedia]: https://en.wikipedia.org/wiki/Tensilica#Xtensa_configurable_cores
