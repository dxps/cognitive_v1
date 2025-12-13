#[cfg(feature = "server")]
use std::{borrow::Cow, net::SocketAddr, ops::ControlFlow};

#[cfg(feature = "server")]
use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::IntoResponse,
};
#[cfg(feature = "server")]
use axum_extra::{headers::UserAgent, TypedHeader};

//allows to split the websocket stream into separate TX and RX branches
#[cfg(feature = "server")]
use futures::{sink::SinkExt, stream::StreamExt};

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
#[cfg(feature = "server")]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    //
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::debug!("WebSocket client user agent '{user_agent}' at address '{addr}' connected.");
    // Finalize the upgrade process by returning upgrade callback.
    // We can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection).
#[cfg(feature = "server")]
async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    // Send a ping (unsupported by some browsers) just to kick things off and get a response.
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        log::debug!("Pinged {who} ...");
    } else {
        log::debug!("Could not send ping {who}!");
        // No error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // Receive a single message from a client (we can either receive or send with socket).
    // This will likely be the Pong for our Ping or a hello message from client.
    // Waiting for message from a client will block this task, but will not block other client's connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            log::debug!("Client {who} abruptly disconnected.");
            return;
        }
    }

    // Since each client gets individual statemachine, we can pause handling
    // when necessary to wait for some external event (in this case illustrated by sleeping).
    // Waiting for this client to finish getting its greetings does not prevent other clients
    // from connecting to server and receiving their greetings.
    for i in 1..5 {
        if socket.send(Message::Text(format!("Hi {i} times!"))).await.is_err() {
            log::debug!("WebSocket client '{who}' abruptly disconnected.");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        let n_msg = 10;
        for i in 0..n_msg {
            // In case of any websocket error, we exit.
            if sender.send(Message::Text(format!("Server message {i} ..."))).await.is_err() {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        log::debug!("Sending close to client '{who}' ...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Bye!"),
            })))
            .await
        {
            log::debug!("Could not send Close due to {e}, probably it is ok?");
        }
        n_msg
    });

    // This second task will receive messages from client and print them on server console.
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => log::debug!("{a} messages sent to {who}"),
                Err(a) => log::debug!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => log::debug!("Received {b} messages"),
                Err(b) => log::debug!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // Returning from the handler closes the Websocket connection.
    log::debug!("Websocket context {who} destroyed");
}

/// A helper to print contents of messages to stdout. Has special treatment for Close.
#[cfg(feature = "server")]
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            log::debug!("Websocket client '{who}' sent str: {t:?}");
        }
        Message::Binary(d) => {
            log::debug!("Websocket client '{}' sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                log::debug!(
                    "Websocket client '{}' sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                log::debug!("Websocket client '{who}' somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            log::debug!("Websocket client '{who}' sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            log::debug!("Websocket client '{who}' sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
