use std::{io::Error, mem::size_of, time::Duration};

enum State {
    Tracking,
    Idle,
}

#[repr(packed)]
#[derive(Debug, Clone)]
pub struct FaceTrackerData {
    pub timestamp: f64,
    pub face_id: i32,
    pub width: f32,
    pub height: f32,
    pub blink_l: f32,
    pub blink_r: f32,
    pub success: u8,
    pub pnp_error: f32,
    pub q0: f32,
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub y: f32,
    pub x: f32,
    pub z: f32,
}

impl FaceTrackerData {
    fn from_bytes(bytes: &[u8; size_of::<Self>()]) -> Self {
        unsafe { std::mem::transmute(*bytes) }
    }
}

pub struct FaceTracker<DataCallback, FaceLostCallback, TimeoutCallback>
where
    DataCallback: FnMut(&FaceTrackerData),
    FaceLostCallback: FnMut(),
    TimeoutCallback: FnMut(),
{
    socket: std::net::UdpSocket,
    state: State,
    pub on_data: DataCallback,
    pub on_face_lost: FaceLostCallback,
    pub on_timeout: TimeoutCallback,
}

impl<DataCallback, FaceLostCallback, TimeoutCallback>
    FaceTracker<DataCallback, FaceLostCallback, TimeoutCallback>
where
    DataCallback: FnMut(&FaceTrackerData),
    FaceLostCallback: FnMut(),
    TimeoutCallback: FnMut(),
{
    pub fn new(
        address: &str,
        on_data: DataCallback,
        on_face_lost: FaceLostCallback,
        on_timeout: TimeoutCallback,
    ) -> Self {
        let socket = std::net::UdpSocket::bind(address).expect("Unable to connect");
        socket.set_nonblocking(false).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(15)))
            .unwrap();
        Self {
            socket,
            state: State::Idle,
            on_data,
            on_face_lost,
            on_timeout,
        }
    }

    pub fn start(&mut self) {
        let mut buf = [0; std::mem::size_of::<FaceTrackerData>()];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok(_) => {
                    self.handle_data_recv(&buf);
                }
                Err(err) => {
                    self.handle_err(&err);
                }
            }
        }
    }

    fn handle_data_recv(&mut self, buf: &[u8; size_of::<FaceTrackerData>()]) {
        self.state = State::Tracking;
        self.socket
            .set_read_timeout(Some(Duration::from_millis(200)))
            .unwrap();
        let data = FaceTrackerData::from_bytes(buf);
        (self.on_data)(&data);
    }

    fn handle_err(&mut self, error: &Error) {
        if error.kind() != std::io::ErrorKind::TimedOut
            && error.kind() != std::io::ErrorKind::WouldBlock
        {
            panic!("{error:?}");
        }
        match self.state {
            State::Tracking => {
                self.state = State::Idle;
                self.socket
                    .set_read_timeout(Some(Duration::from_secs(15)))
                    .unwrap();
                (self.on_face_lost)()
            }
            State::Idle => {
                self.socket.set_read_timeout(None).unwrap();
                (self.on_timeout)()
            }
        }
    }
}
