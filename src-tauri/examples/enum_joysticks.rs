//! Step 1 of the input layer: enumerate connected joysticks via the SDL raw
//! joystick API and print their identity and capabilities. Run headless with:
//!
//!     cargo run --example enum_joysticks
//!
//! Thin wrapper around `bindsight_lib::input::enumerate` — the same code path
//! the Tauri `list_devices` command's device list is built from (minus the
//! synthetic keyboard the app appends). The SC Product GUID printed
//! here is derived from the SDL GUID and should line up with SC's
//! `options/@Product` value.

fn main() -> Result<(), String> {
    let devices = bindsight_lib::input::enumerate()?;
    if std::env::args().any(|a| a == "--json") {
        println!("{}", serde_json::to_string_pretty(&devices).map_err(|e| e.to_string())?);
        return Ok(());
    }
    println!("{} joystick(s) connected\n", devices.len());

    for d in &devices {
        println!("[{}] {}", d.index, d.sc_name.as_deref().unwrap_or("<no HID name>"));
        println!("    SDL name:   {}", d.sdl_name);
        println!("    kind:       {:?} (SDL type {})", d.kind, d.sdl_type);
        if let Some(name) = &d.controller_name {
            println!("    controller: {name}, slot {:?}", d.gamepad_slot);
        } else if d.wine_gamepad {
            println!("    controller: (Wine rule, mapped like Wine), slot {:?}", d.gamepad_slot);
        }
        println!("    instance:   {}", d.sdl_instance_id);
        println!("    path:       {:?}", d.sdl_path);
        println!("    serial:     {:?}", d.sdl_serial);
        println!("    SDL GUID:   {}", d.sdl_guid);
        println!(
            "    SC Product: {}",
            d.sc_product_guid.as_deref().unwrap_or("<unparseable>")
        );
        println!("    buttons:    {}", d.num_buttons);
        println!("    axes:       {}", d.num_axes);
        println!("    hats:       {}", d.num_hats);
        match &d.axes_error {
            Some(e) => println!("    SC axes:    unavailable ({e})"),
            None => println!("    SC axes:    {}", d.axes.join(" ")),
        }
    }

    Ok(())
}
