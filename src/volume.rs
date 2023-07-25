use notify_rust::Notification;
use notify_rust::Hint;
use pipewire::{MainLoop, Context};

// Sets volume to set number for id 57
// pw-cli set-param 57 Route '{"index":1,"device":1,"props":{"mute":false,"channelVolumes":[0.011538000000000001,0.011538000000000001]}}'


pub fn get_volume() -> Result<(), Box<dyn std::error::Error>> {
    let mainloop = MainLoop::new().expect("Failed to create main loop");
    let context = Context::new(&mainloop).expect("Failed to create context");
    let core = context.connect(None).expect("Failed to connect context");
    let registry = core.get_registry().expect("Failed to get core registry");
    let _listener = registry
        .add_listener_local()
        .global(|global| println!("New global: {:?}", global))
        .register();

    mainloop.run();

    Ok(())
}

fn _get_max_volume() -> usize {
    todo!();
}

fn _set_volume(_num: usize) -> usize {
    // has to be less than max volume
    todo!();
}

fn _send_volume_notification(cur_percent: u32) {
    let icon_path: &str = match cur_percent {
        x if x == 0 => "/usr/share/icons/Adwaita/96x96/status/audio-volume-muted-symbolic.symbolic.png",
        x if x <= 33 => "/usr/share/icons/Adwaita/96x96/status/audio-volume-low-symbolic.symbolic.png",
        x if x <= 66 => "/usr/share/icons/Adwaita/96x96/status/audio-volume-medium-symbolic.symbolic.png",
        x if x <= 100 => "/usr/share/icons/Adwaita/96x96/status/audio-volume-high-symbolic.symbolic.png",
        _ => "/usr/share/icons/Adwaita/96x96/status/dialog-warning-symbolic.symbolic.png",
    };

    match Notification::new()
            .summary("Volume")
            .id(69)
            .hint(Hint::CustomInt(String::from("value"), cur_percent as i32))
            .icon(icon_path)
            .timeout(1500)
            .show() {
        Ok(it) => it,
        Err(_) => return (),
    };
}

