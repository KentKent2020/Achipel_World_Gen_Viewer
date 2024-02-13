use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use log::*;
use log::LevelFilter::Debug;
use msg::msg_chunkend::MsgOutChunkEndData;
use serde_json::{from_str};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error as Err, WebSocketStream};
use tungstenite::Result as Res;


use crate::handler::handler_echo::handle_echo;
use crate::handler::handler_chunk::handle_chunk;
use crate::msg::msg_in::MsgIn;
use crate::msg::msg_out::MsgOut;
use crate::util::{get_msg_text, ToMessage};
use crate::msg::msg_chunk::MsgInChunkData;


mod msg;
mod args;
mod handler;
mod util;
mod worldgen;
    

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Err::ConnectionClosed | Err::Protocol(_) | Err::Utf8 => (),
            err => error!("Error processing connection: {:?}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Res<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;



        // Handle msg!
        if let Some(text) = get_msg_text(&msg) {
            let event: MsgIn = from_str(text).expect("Invalid input data");
            handle_event(&mut ws_stream, event).await?;
       
        }
    }

    Ok(())
}

async fn send_chunk(stream: &mut WebSocketStream<TcpStream>, data: MsgInChunkData) -> Result<(), Err> {

    

    for x in 18..21 {
        for z  in 18..21 {
            stream.send(MsgOut::Chunk(handle_chunk(&data, x, z)).to_msg()).await?;
        }
    }

    stream.send(MsgOut::ChunkEnd(MsgOutChunkEndData{endmessage: String::from("End")}).to_msg()).await?;

    Ok(())
}

async fn handle_event(stream: &mut WebSocketStream<TcpStream>, event: MsgIn) -> Result<(), Err> {
    match event {
        MsgIn::Echo(data) => {
            stream.send(MsgOut::Echo(handle_echo(&data)).to_msg()).await?;
        }
        MsgIn::Chunk(data) => {
            send_chunk(stream, data).await?;
        }
    };

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    env_logger::builder()
        .filter_level(Debug)
        .init();

    // Parse args
    // let args: Args = Args::parse();
    let addr = format!("127.0.0.1:{}", 8069);

    // Start server
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    // Handle connections
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }

    Ok(())
}
