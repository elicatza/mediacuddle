extern crate mpris;
use notify_rust::Notification;
use notify_rust::Hint;

use mpris::PlayerFinder;

pub fn send_notification() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    let metadata = player.get_metadata().expect("Could not get metadata");
    let icon_path = metadata.art_url().unwrap_or("/usr/share/icons/hicolor/scalable/apps/mpv.svg");
    let title = metadata.title().unwrap_or("No metadata");


    match Notification::new()
            .summary("Playback")
            .body(title)
            .id(69)
            .hint(Hint::ImagePath(icon_path.to_string()))
            .timeout(1500)
            .show() {
        Ok(it) => it,
        Err(_) => return (),
    };
}

pub fn next() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    player.next().expect("Could not skip to next track");
}

pub fn prev() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    player.previous().expect("Could not skip to previous track");
}

pub fn toggle() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    player.play_pause().expect("Could not toggle playback");
}
