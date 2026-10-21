use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use super::*;
use crate::Config;

fn send(address: &str, token: &str, method: &str, args: Value, origin: bool) -> TcpStream {
    let mut stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    let mut command = json!({"method":method});
    command["args"] = args;
    let body = command.to_string();
    write!(stream,
        "POST / HTTP/1.1\r\nHost: {address}\r\nAuthorization: Bearer {token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{}\r\n{body}",
        body.len(), if origin { "Origin: https://example.com\r\n" } else { "" }).unwrap();
    stream
}

fn receive(mut stream: TcpStream) -> (u16, Value) {
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let (headers, body) = response.split_once("\r\n\r\n").unwrap();
    let status = headers.split_whitespace().nth(1).unwrap().parse().unwrap();
    assert!(headers.to_lowercase().contains("cache-control: no-store"));
    (status, serde_json::from_str(body).unwrap())
}

#[test]
fn loopback_waits_allow_other_requests_and_close_releases_pending_clients() {
    let host = Host::with_seed(
        Config {
            format: "old-school-93-94".into(),
            p1_deck: "Sligh".into(),
            p2_deck: "The Deck".into(),
            match_mode: "one-conclusion".into(),
        },
        1,
    )
    .unwrap();
    let organizer = host.organizer.clone();
    let seats = host.seats.clone();
    let revision = host.revision.clone();
    let index = host.session.discover(penta::PlayerId::One)["legalActions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|action| action["type"] == "KeepHand")
        .unwrap()["index"]
        .clone();
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let address = listener.local_addr().unwrap().to_string();
    let serving = thread::spawn(move || serve_loop(host, listener).unwrap());
    let request =
        |token: &str, method: &str, args| receive(send(&address, token, method, args, false));
    assert_eq!(request("invalid", "observe", json!({})).0, 403);
    assert_eq!(
        receive(send(&address, &seats[0], "observe", json!({}), true)).0,
        403
    );
    assert_eq!(request(&seats[0], "export", json!({})).0, 403);
    assert_eq!(
        request(&seats[1], "wait", json!({"waitMs":1})).1["status"],
        "waiting"
    );

    let waiting_stream = send(
        &address,
        &seats[1],
        "wait",
        json!({"waitMs":25_000,"afterEvent":0}),
        false,
    );
    let (completed, completion) = mpsc::channel();
    let waiting = thread::spawn(move || completed.send(receive(waiting_stream)).unwrap());
    assert_eq!(request(&organizer, "status", json!({})).0, 200);
    let choice = json!({"ticket":format!("{revision}:a{index}"),"requestId":"keep"});
    assert_eq!(
        request(&seats[0], "choose", choice.clone()).1["receipt"]["accepted"],
        1
    );
    let delivered = completion.recv_timeout(Duration::from_secs(10)).unwrap().1;
    assert_eq!(delivered["status"], "ready");
    assert_eq!(delivered["events"]["after"], 0);
    let observed = request(&seats[1], "observe", json!({"afterEvent":0})).1;
    assert_eq!(delivered["events"], observed["events"]);
    waiting.join().unwrap();
    assert_eq!(
        request(&seats[0], "choose", choice).1["receipt"]["accepted"],
        1
    );

    let waiting_stream = send(&address, &seats[0], "wait", json!({"waitMs":25_000}), false);
    assert_eq!(
        request(&organizer, "export", json!({})).1["commands"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(request(&organizer, "close", json!({})).0, 200);
    assert_eq!(receive(waiting_stream).0, 410);
    serving.join().unwrap();
}

#[test]
fn incomplete_and_oversized_bodies_cannot_block_match_authority() {
    let host = Host::with_seed(
        Config {
            format: "old-school-93-94".into(),
            p1_deck: "Sligh".into(),
            p2_deck: "The Deck".into(),
            match_mode: "one-conclusion".into(),
        },
        1,
    )
    .unwrap();
    let organizer = host.organizer.clone();
    let seat = host.seats[0].clone();
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let address = listener.local_addr().unwrap().to_string();
    let serving = thread::spawn(move || serve_loop(host, listener).unwrap());

    let partial = |token: &str, length: u64| {
        let mut stream = TcpStream::connect(&address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(10)))
            .unwrap();
        write!(stream,
            "POST / HTTP/1.1\r\nHost: {address}\r\nAuthorization: Bearer {token}\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n{{"
        ).unwrap();
        stream
    };
    // No body follows the huge declared size. Reject without draining or allocating it.
    assert_eq!(receive(partial("invalid", 1_u64 << 40)).0, 403);
    assert_eq!(receive(partial(&seat, 1_u64 << 40)).0, 413);
    let stalled = partial(&seat, 4096);
    let start = std::time::Instant::now();
    assert_eq!(
        receive(send(&address, &organizer, "status", json!({}), false)).0,
        200
    );
    assert_eq!(
        receive(send(&address, &organizer, "export", json!({}), false)).0,
        200
    );
    assert!(
        start.elapsed() < BODY_TIMEOUT,
        "socket reads must not serialize authority"
    );
    assert_eq!(receive(stalled).0, 408);
    assert_eq!(
        receive(send(&address, &organizer, "close", json!({}), false)).0,
        200
    );
    serving.join().unwrap();
}
