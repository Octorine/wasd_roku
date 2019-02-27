use cursive::view::Identifiable;
use std::net::UdpSocket;
use std::str;
use std::rc::Rc;
use regex::Regex;

fn discover() -> Option<String> {
    const MULTICAST_ADDRESS: &'static str = "239.255.255.250:1900";

    const MULTICAST_REQUEST: &str = r#"M-SEARCH * HTTP/1.1
Host: 239.255.255.250:1900
Man: "ssdp:discover"
ST: roku:ecp
 "#;
    println!("Discovering...");
    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket
        .send_to(MULTICAST_REQUEST.as_bytes(), MULTICAST_ADDRESS)
        .expect("couldn't send data");

    let mut buf = [0; 1024];
    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data...");
        let filled_buf = &mut buf[..amt];
        let s = str::from_utf8(filled_buf).unwrap();
        let location_regex = Regex::new("LOCATION: ([htp:/.0-9]*)").unwrap();
        return location_regex
            .captures(&s)
            .and_then(|c| c.at(1).map(String::from));
    }
}

fn send_key(c: &mut cursive::Cursive, location: &String, msg: String, view: String) {
    let url = format!("{}keypress/{}", location, msg);
    let client = reqwest::Client::new().unwrap();
    client
        .post(&url)
        .unwrap()
        .send()
        .unwrap();
    c.call_on_id(&view, |view: &mut cursive::views::TextView| {
        view.set_content(msg.clone())
    });
}

fn main() {
    let location = discover().expect("Couldn't find Roku");
    let mut siv = cursive::Cursive::ncurses();

    let label = cursive::views::TextView::new("Hit Key to begin").with_id("label");
    siv.add_layer(label);
    siv.add_global_callback('q', |s| s.quit());
    let location_ref = Rc::new(location);
    let loc = location_ref.clone();
    siv.add_global_callback('w', move |s| {
        send_key(s, &loc, "Up".to_string(), "label".to_string())
    });
    let loc = location_ref.clone();
    siv.add_global_callback('a', move |s| {
        send_key(s, &loc, "Left".to_string(), "label".to_string())
    });
    let loc = location_ref.clone();
    siv.add_global_callback('s', move |s| {
        send_key(s, &loc, "Down".to_string(), "label".to_string())
    });
    let loc = location_ref.clone();
    siv.add_global_callback('d', move |s| {
        send_key(s, &loc, "Right".to_string(), "label".to_string())
    });
    let loc = location_ref.clone();
    siv.add_global_callback(' ', move |s| {
        send_key(s, &loc, "Select".to_string(), "label".to_string())
    });
    let loc = location_ref.clone();
    siv.add_global_callback('h', move |s| {
        send_key(s, &loc, "Home".to_string(), "label".to_string())
    });
    let loc = location_ref.clone();
    siv.add_global_callback(cursive::event::Key::Backspace, move |s| {
        send_key(s, &loc, "Back".to_string(), "label".to_string())
    });
    siv.run();
}
