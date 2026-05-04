use std::{env, path::PathBuf};

const KIBIBYTES: u32 = 1024;
const MIBIBYTES: u32 = 1024 * KIBIBYTES;

fn main() {
    // NOTE(hal): values of `flash_page_size` from the datasheets, confirmed by HAL's constants.
    // Important: only homogeneous flash organizations are currently supported.
    // Trying to restrict the storage size to the subset of homogeneous flash would not work as it
    // could be pushed out of it by a large enough binary.

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if is_in_current_contexts(&["esp32c6"]) {
        // PoC assumptions:
        // - 4 MiB flash
        // - default factory app partition begins at 0x10000
        // - reserve the last 64 KiB for storage
        const FLASH_TOTAL: u32 = 4 * MIBIBYTES;
        const APP_PARTITION_OFFSET: u32 = 0x0001_0000;
        const STORAGE_OFFSET: u32 = 0x003F_0000; // start of last 64 KiB
        const STORAGE_SIZE: u32 = 64 * KIBIBYTES;
        const FLASH_PAGE_SIZE: u32 = 4 * KIBIBYTES;
        const MMAP_BASE: u32 = 0x4200_0000;

        assert_eq!(STORAGE_OFFSET % FLASH_PAGE_SIZE, 0);
        assert_eq!(STORAGE_SIZE % FLASH_PAGE_SIZE, 0);
        const { assert!(STORAGE_OFFSET >= APP_PARTITION_OFFSET) };
        const { assert!(STORAGE_OFFSET + STORAGE_SIZE <= FLASH_TOTAL) };

        // Note: we cannot use ".storage > FLASH INSERT AFTER .rodata"
        // section for ESP32-C6, because that would introduce another mapped
        // flash segment and trips the bootloader assertion.
        //
        // Instead we define absolute linker symbols only.
        let vaddr_start = MMAP_BASE + (STORAGE_OFFSET - APP_PARTITION_OFFSET);
        let vaddr_end = vaddr_start + STORAGE_SIZE;

        let storage_x = format!(
            "\
PROVIDE(__storage_start = 0x{vaddr_start:08x});
PROVIDE(__storage_end   = 0x{vaddr_end:08x});
"
        );

        std::fs::write(out.join("storage.x"), storage_x).unwrap();

        println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
        println!("cargo:rustc-link-search={}", out.display());
        return;
    }

    let (storage_size_total, flash_page_size) = if is_in_current_contexts(&[
        "stm32f303cb",
        "stm32f303re",
        "stm32u073kc",
        "stm32u083mc",
        "stm32l475vg",
        "nrf5340-net",
        "stm32wle5jc",
    ]) {
        (4 * KIBIBYTES, 2 * KIBIBYTES)
    } else if is_in_current_contexts(&["nrf52", "nrf5340-app", "nrf91", "rp", "stm32wb55rg"]) {
        (8 * KIBIBYTES, 4 * KIBIBYTES)
    } else if is_in_current_contexts(&["stm32u585ai", "stm32wba65ri"]) {
        (16 * KIBIBYTES, 8 * KIBIBYTES)
    } else if is_in_current_contexts(&["stm32h755zi", "stm32h753zi"]) {
        (256 * KIBIBYTES, 128 * KIBIBYTES)
    } else if !is_in_current_contexts(&["ariel-os"]) {
        // Dummy value for platform-independent tooling.
        (8 * KIBIBYTES, 4 * KIBIBYTES)
    } else {
        panic!("MCU not supported");
    };

    // `sequential-storage` needs at least two flash pages.
    assert!(storage_size_total / flash_page_size >= 2);

    let mut storage_template = std::fs::read_to_string("storage.ld.in").unwrap();
    storage_template = storage_template.replace("${ALIGNMENT}", &format!("{flash_page_size}"));
    storage_template = storage_template.replace("${SIZE}", &format!("{storage_size_total}"));

    std::fs::write(out.join("storage.x"), &storage_template).unwrap();

    println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
    println!("cargo:rerun-if-changed=storage.ld.in");
    println!("cargo:rustc-link-search={}", out.display());
}

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| contexts.contains(&c))
}
