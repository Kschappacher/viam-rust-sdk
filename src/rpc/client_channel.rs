use super::client_stream::*;
use crate::gen::proto::rpc::webrtc::v1::{
    request::Type, PacketMessage, Request, RequestHeaders, RequestMessage, Response, Stream,
};
use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use prost::Message;
use std::{
    collections::HashMap,
    hint,
    sync::{
        atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use webrtc::{
    data_channel::{data_channel_message::DataChannelMessage, RTCDataChannel},
    peer_connection::RTCPeerConnection,
};

// see golang/client_stream.go
const MAX_REQUEST_MESSAGE_PACKET_DATA_SIZE: usize = 16373;

pub struct WebrtcClientChannel {
    pub base_channel: Arc<WebrtcBaseChannel>,
    stream_id_counter: AtomicU64,
    message_ready: AtomicBool,
    pub streams: Mutex<HashMap<u64, ActiveWebrtcClientStream>>,
}

pub struct WebrtcBaseChannel {
    pub peer_connection: Arc<RTCPeerConnection>,
    pub data_channel: Arc<RTCDataChannel>,
    // CR erodkin: make sure this is right
    //ready: Channel,
    closed_reason: AtomicPtr<Option<anyhow::Error>>,
    closed: AtomicBool,
}

impl WebrtcClientChannel {
    pub async fn new(
        peer_connection: Arc<RTCPeerConnection>,
        data_channel: Arc<RTCDataChannel>,
    ) -> Arc<Self> {
        let base_channel = WebrtcBaseChannel::new(peer_connection, data_channel.clone()).await;
        let channel = Self {
            base_channel,
            message_ready: AtomicBool::new(false),
            streams: Mutex::new(HashMap::new()),
            stream_id_counter: AtomicU64::new(0),
        };

        let channel = Arc::new(channel);
        let ret_channel = channel.clone();

        // CR erodkin: we need on_error, probably some others
        data_channel
            .on_message(Box::new(move |msg: DataChannelMessage| {
                let channel = channel.clone();
                Box::pin(async move {
                    if let Err(e) = channel.on_channel_message(msg) {
                        log::error!("error deserializing message: {e}");
                    }
                })
            }))
            .await;
        ret_channel
    }

    pub fn new_stream(&self) -> Arc<Mutex<WebrtcClientStream>> {
        let id = self.stream_id_counter.fetch_add(1, Ordering::SeqCst);
        let stream = Stream { id };

        let (message_sender, message_receiver) = std::sync::mpsc::channel();

        let client_stream = Arc::new(Mutex::new(WebrtcClientStream {
            stream,
            message_sender,
            message_receiver,
            closed: AtomicBool::new(false),
            packet_buffer: Vec::new(),
            closed_reason: AtomicPtr::new(&mut None),
            headers_received: AtomicBool::new(false),
            trailers_received: AtomicBool::new(false),
        }));

        let stream = ActiveWebrtcClientStream {
            client_stream: client_stream.clone(),
        };
        let _ = self.streams.lock().unwrap().insert(id, stream);
        client_stream
    }

    fn on_channel_message(&self, msg: DataChannelMessage) -> Result<()> {
        let streams = self.streams.lock().unwrap();
        let response = Response::decode(&*msg.data.to_vec())?;
        let active_stream = match response.stream.as_ref() {
            None => return Ok(()),
            Some(stream) => streams.get(&stream.id),
        };

        let message_sent = match active_stream {
            Some(active_stream) => active_stream
                .client_stream
                .lock()
                .unwrap()
                .on_response(response),
            None => Ok(false),
        }?;
        drop(streams);
        self.message_ready.store(message_sent, Ordering::SeqCst);
        Ok(())
    }

    pub fn recv_from_stream(&self, stream_id: u64) -> Result<Vec<u8>> {
        while !self.message_ready.load(Ordering::SeqCst) {
            hint::spin_loop();
        }
        let streams_lock = self.streams.lock().unwrap();

        match (*streams_lock).get(&stream_id) {
            Some(stream) => {
                let mut msg = stream
                    .client_stream
                    .lock()
                    .unwrap()
                    .message_receiver
                    .recv()?;

                let mut message_buf = vec![0u8];
                let len: u32 = msg.len().try_into()?;
                message_buf.write_u32::<BigEndian>(len)?;
                message_buf.append(&mut msg);
                self.message_ready.store(false, Ordering::SeqCst);
                Ok(message_buf)
            }
            None => Ok(Vec::new()),
        }
    }

    pub async fn write_headers(&self, stream: &Stream, headers: RequestHeaders) -> Result<()> {
        let headers = Request {
            stream: Some(stream.clone()),
            r#type: Some(Type::Headers(headers)),
        };
        let header_vec = prost::Message::encode_to_vec(&headers);
        self.send(&header_vec).await
    }

    pub async fn write_message(
        &self,
        eos: bool,
        stream: Option<Stream>,
        mut data: Vec<u8>,
    ) -> Result<()> {
        // even if no meaningful data, any actual message will include at least frame header bytes
        let has_message = !data.is_empty();

        data = if data.len() >= 5 {
            data.split_off(5)
        } else {
            data
        };

        // always run the loop at least once, check at completion if we've sent all data and
        // break the loop accordingly
        loop {
            let split_at = MAX_REQUEST_MESSAGE_PACKET_DATA_SIZE.min(data.len());
            let (to_send, remaining) = data.split_at(split_at);
            let stream = stream.clone();
            let request = Request {
                stream,
                r#type: Some(Type::Message(RequestMessage {
                    has_message,
                    eos,
                    packet_message: Some(PacketMessage {
                        eom: remaining.is_empty(),
                        data: to_send.to_vec(),
                    }),
                })),
            };

            let request = prost::Message::encode_to_vec(&request);
            if let Err(e) = self.send(&request).await {
                log::error!("error sending message: {e}");
                return Err(e);
            }

            data = remaining.to_vec();
            if data.is_empty() {
                break;
            }
        }
        Ok(())
    }

    async fn send(&self, data: &[u8]) -> Result<()> {
        let data = &bytes::Bytes::copy_from_slice(data);
        self.base_channel
            .data_channel
            .send(data)
            .await
            .map_err(anyhow::Error::from)
            .map(|_: usize| ())
    }

    pub async fn close_with_reason(&self, err: anyhow::Error) -> Result<()> {
        self.base_channel.close_with_reason(err).await
    }

    pub fn close_stream_with_recv_error(&self, stream_id: u64, error: anyhow::Error) {
        let mut stream_lock = self.streams.lock().unwrap();
        match stream_lock.remove(&stream_id) {
            Some(stream) => stream
                .client_stream
                .lock()
                .unwrap()
                .close_with_recv_error(&mut Some(&error)),
            None => {
                log::error!("attempted to close stream with id {stream_id}, but it wasn't found!")
            }
        }
    }
}

impl WebrtcBaseChannel {
    pub async fn new(
        peer_connection: Arc<RTCPeerConnection>,
        data_channel: Arc<RTCDataChannel>,
    ) -> Arc<Self> {
        let dc = data_channel.clone();
        let channel = Arc::new(Self {
            peer_connection,
            data_channel,
            closed_reason: AtomicPtr::new(&mut None),
            closed: AtomicBool::new(false),
        });

        let c = channel.clone();
        dc.on_error(Box::new(move |err: webrtc::Error| {
            let c = c.clone();
            Box::pin(async move {
                if let Err(e) = c.close_with_reason(anyhow::Error::from(err)).await {
                    log::error!("error closing channel: {e}")
                }
            })
        }))
        .await;
        channel
    }

    async fn close_with_reason(&self, err: anyhow::Error) -> Result<()> {
        if self.closed.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.closed.store(true, Ordering::SeqCst);
        self.closed_reason.store(&mut Some(err), Ordering::SeqCst);

        self.peer_connection
            .close()
            .await
            .map_err(anyhow::Error::from)
    }
}
