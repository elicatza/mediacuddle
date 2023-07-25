use notify_rust::Notification;


pub fn send_notification(name: &str) {
    let icon_path: &str = "/usr/share/icons/Adwaita/96x96/devices/input-keyboard-symbolic.symbolic.png";

    match Notification::new()
            .summary("Keymap")
            .body(&("Layout set to: ".to_string() + name))
            .id(69)
            .icon(icon_path)
            .timeout(1500)
            .show() {
        Ok(it) => it,
        Err(_) => return (),
    };
}
