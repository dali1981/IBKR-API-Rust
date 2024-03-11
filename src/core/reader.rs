//! Reads and processes messages from the TCP socket
use std::convert::TryInto;
use tokio::io::{self, AsyncReadExt};
use std::net::Shutdown;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use byteorder::{NetworkEndian, ReadBytesExt};
// use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;

use tracing::*;

use tokio::net::TcpStream;
use crate::core::errors::IBKRApiLibError;
use crate::core::messages::{read_msg, read_one_msg};

//==================================================================================================
pub struct Reader {
    stream: OwnedReadHalf,
    messages: Sender<String>,
    disconnect_requested: Arc<AtomicBool>,
    is_connected: bool,
}

impl Reader {

    const MAX_MSG_LEN: usize = 0xFFFFFF; // 16Mb - 1 byte

    pub fn new(
        stream: OwnedReadHalf,
        messages: Sender<String>,
        disconnect_requested: Arc<AtomicBool>,
    ) -> Self {
        Reader {
            stream,
            messages,
            disconnect_requested,
            is_connected: true,
        }
    }

    //----------------------------------------------------------------------------------------------

    // pub async fn recv_msg(&mut self) -> Result<Vec<u8>, IBKRApiLibError> {
    //     // Attempt to read the size of the incoming message
    //
    //     let msg_size = match self.stream.read().await {
    //         Ok(size) => size as usize,
    //         Err(e) => {
    //             error!("{}", e);
    //             return Err(e.into());
    //         },
    //     };
    //
    //     // Convert from network byte order to host byte order and validate the size
    //     if msg_size <= 0 || msg_size > Self::MAX_MSG_LEN {
    //         warn!(msg_size, "Invalid message size.");
    //         return Ok(vec![]);
    //         // todo: return an error here
    //     }
    //
    //     // Allocate a buffer of the appropriate size
    //     let mut buf = vec![0u8; msg_size];
    //
    //     // Read the message into the buffer
    //     self.stream.read_exact(&mut buf)?;
    //
    //     Ok(buf)
    // }


    //----------------------------------------------------------------------------------------------
    // async fn process_reader_msgs(&mut self) -> Result<(), IBKRApiLibError> {
    //     // grab a packet of messages from the socket
    //     let mut message:Vec<u8> = self.recv_msg().expect("failed to read message");
    //     // debug!(" recvd size {}", message_packet.len());
    //     // debug!(" recvd  {:?}", message_packet);
    //     // debug!(?message);
    //
    //     if message.len() == 0 { return Ok(());}
    //     // let size = i32::from_be_bytes(message[0..4].try_into().unwrap()) as usize;
    //     // let size = i32::from_ne_bytes(message[0..4].try_into().unwrap()) as usize;
    //     info!(?message);
    //
    //     let mut str = String::new();
    //     // Iterate over the first 3 bytes and convert each to a char, constructing a string.
    //     for &byte in &message[0..3] {
    //         str.push(byte as char);
    //     }

// Parse the constructed string into an integer.
        // Parse the constructed string into an integer.
        // let size = str.parse::<i32>().unwrap();
        // info!(size);
        // // let text = String::from_utf8(message[4..4 + size].to_vec()).unwrap();
        //
        // // for &byte in &message[4..4 + size as usize] {
        // //     str.push(byte as char);
        // // }
        // let (text, remaining) = Self::decode_field(&message[4..]).unwrap();
        // info!("text : {}", text.to_string());
        // info!(?remaining);
        // self.messages.send(text.to_string()).expect("READER CANNOT SEND MESSAGE");

        // Read messages from the packet until there are no more.
        // When this loop ends, break into the outer loop and grab another packet.
        // Repeat until the connection is closed
        //
        // let _msg = String::new();
        // while message_packet.len() > 0 {
        //     // Read a message from the packet then add it to the message queue below.
        //     let (_size, msg, remaining_messages) = read_msg(message_packet.as_slice())?;
        //
        //     debug!(_size, msg, ?remaining_messages);
        //
        //     // clear the Vec that holds the bytes from the packet
        //     // and reload with the bytes that haven't been read.
        //     // The variable remaining_messages only holds the unread messagesleft in the packet
        //     message_packet.clear();
        //     message_packet.extend_from_slice(remaining_messages.as_slice());
        //
        //     if msg.as_str() != "" {
        //         self.messages.send(msg).expect("READER CANNOT SEND MESSAGE");
        //     } else {
        //         //Break to the outer loop in run and get another packet of messages.
        //
        //         debug!("more incoming packet(s) are needed ");
        //         break;
        //     }
        // }
    //     Ok(())
    // }
    //----------------------------------------------------------------------------------------------
    // pub async fn run(&mut self) {
    //     debug!("starting reader loop");
    //     self.stream.read()
    //         if self.disconnect_requested.load(Ordering::Acquire) || !self.is_connected {
    //             return;
    //         }
    //         let result = self.process_reader_msgs();
    //         if !result.is_err() {
    //             continue;
    //         }
    //         error!("{:?}", result);
    //     }
    // }


}


async fn decode_field(input: &[u8]) -> Option<(String, &[u8])> {
    // Attempt to find the end of the field. Assuming a null byte as a delimiter for simplicity.
    if let Some(end_index) = input.iter().position(|&x| x == 0) {
        // Convert the field bytes up to the delimiter into a string.
        let field = match std::str::from_utf8(&input[..end_index]) {
            Ok(s) => s.to_string(),
            Err(_) => return None,
        };
        // Return the decoded string and the rest of the input, skipping the delimiter.
        Some((field, &input[end_index + 1..]))
    } else {
        // If no delimiter is found, the whole slice is considered a single field.
        match std::str::from_utf8(input) {
            Ok(s) => Some((s.to_string(), &[][..])),
            Err(_) => None,
        }
    }
}

pub async fn read_message_loop(mut stream: OwnedReadHalf,
                           mut tx: Sender<String>, disconnect_requested: Arc<AtomicBool>
// ) -> io::Result<()>
) -> Result<(), IBKRApiLibError>
{
    info!("started reader");
    let mut len_buf = [0u8; 4]; // Buffer for length prefix

    while let Ok(_) = stream.read_exact(&mut len_buf).await {
        // debug!(?len_buf, "length buffer");
        let len = u32::from_be_bytes(len_buf) as usize;
        // info!(?len, "length");

        if len > Reader::MAX_MSG_LEN {
            error!(len, "Message length too large");
            continue; // Skip this message (or you might want to handle this differently)
        }

        let mut buf = vec![0u8; len];
        if let Ok(_) = stream.read_exact(&mut buf).await {
            // debug!(?buf);
            let text = read_one_msg(&buf).await?;
            debug!(text);

            // Send the message buffer to processing queue/channel
            if tx.send(text).await.is_err() {
                eprintln!("Failed to send message for processing.");
                break; // Break if we cannot send to the processing channel
            }
        }
    }

    Ok(())
}
