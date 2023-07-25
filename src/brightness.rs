use notify_rust::Notification;
use notify_rust::Hint;
use std::fs;


const BRIGHTNESS_DEVICE: &str = "/sys/class/backlight/acpi_video0";


pub fn send_notification(brightness: i32) {
    let brightness = if brightness < 0 { 0 } else { to_percent(brightness) };
    let icon_path: &str = match brightness {
        x if x <= 100 => "/usr/share/icons/Adwaita/96x96/status/display-brightness-symbolic.symbolic.png",
        _ => "/usr/share/icons/Adwaita/96x96/status/dialog-warning-symbolic.symbolic.png",
    };

    match Notification::new()
            .summary("Brightness")
            .body(&(brightness.to_string() + "%"))
            .id(69)
            .hint(Hint::CustomInt(String::from("value"), brightness as i32))
            .icon(icon_path)
            .timeout(1500)
            .show() {
        Ok(it) => it,
        Err(_) => return (),
    };
}

pub fn get_current() -> i32 {
    let brighness: String = fs::read_to_string(BRIGHTNESS_DEVICE.to_string() + "/brightness").unwrap();
    brighness.trim().parse::<i32>().expect("Failed to read file")
}

pub fn get_max() -> i32 {
    let brighness: String = fs::read_to_string(BRIGHTNESS_DEVICE.to_string() + "/max_brightness").unwrap();
    brighness.trim().parse::<i32>().expect("Failed to read file")
}

pub fn set(brightness: i32) {
    let brightness = if brightness < 0 { 0 } else { brightness };
    fs::write(BRIGHTNESS_DEVICE.to_string() + "/brightness", brightness.to_string()).unwrap();
}

fn to_percent(num: i32) -> i32 {
    ((num as f32 / get_max() as f32) * 100_f32).ceil() as i32
}

