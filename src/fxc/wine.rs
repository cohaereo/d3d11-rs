use std::{
    io::{Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    sync::{LazyLock, Mutex},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::fxc::ShaderTarget;

#[derive(bincode::Decode, bincode::Encode)]
pub struct RemoteFxcRequest {
    pub data: Vec<u8>,
    pub source_name: Option<String>,
    pub defines: Vec<(String, String)>,
    pub entry_point: String,
    pub target: String,
}

#[derive(bincode::Decode, bincode::Encode)]
pub struct RemoteFxcResult {
    pub hresult: i32,
    pub compiled_code: Option<Vec<u8>>,
    pub error_message: Option<String>,
}

struct RemoteFxc {
    _process: std::process::Child,
    stream: TcpStream,
}

impl RemoteFxc {
    pub fn new() -> std::io::Result<Self> {
        let process = std::process::Command::new("wine")
            .arg("fxc-server.exe")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        // Wait a moment for the server to start
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 18008);
        for _ in 0..50 {
            let Ok(stream) = TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(5))
            else {
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            };
            _ = stream.set_nodelay(true);
            return Ok(Self {
                _process: process,
                stream,
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "Failed to connect to FXC server",
        ))
    }
}

static REMOTE_COMPILER: LazyLock<Mutex<RemoteFxc>> = LazyLock::new(|| {
    RemoteFxc::new()
        .map(Mutex::new)
        .expect("Failed to start remote FXC compiler")
});

/// Provides a safe wrapper around the D3DCompile function.
pub fn compile(
    data: &[u8],
    source_name: Option<&str>,
    defines: &[(&str, &str)],
    // include: ???, // TODO
    entry_point: &str,
    target: ShaderTarget,
) -> crate::Result<Vec<u8>> {
    let mut fxc = REMOTE_COMPILER
        .lock()
        .expect("Remote FXC compiler mutex poisoned");

    let request = RemoteFxcRequest {
        data: data.to_vec(),
        source_name: source_name.map(|s| s.to_string()),
        defines: defines
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        entry_point: entry_point.to_string(),
        target: target.profile().to_string(),
    };

    let mut write_buffer = Vec::<u8>::with_capacity(1024 * 1024);

    bincode::encode_into_std_write(&request, &mut write_buffer, bincode::config::standard())
        .map_err(|e| crate::error::Error::Other(format!("Failed to encode FXC request: {e}")))?;

    fxc.stream
        .write_u32::<BigEndian>(write_buffer.len() as u32)?;
    fxc.stream.write_all(&write_buffer)?;

    let response_len = fxc.stream.read_u32::<BigEndian>()? as usize;
    let mut response_buffer = vec![0u8; response_len];
    fxc.stream.read_exact(&mut response_buffer)?;
    let response: RemoteFxcResult =
        bincode::decode_from_slice(&response_buffer, bincode::config::standard())
            .map_err(|e| {
                crate::error::Error::Other(format!("Failed to receive FXC response: {e}"))
            })?
            .0;

    if response.hresult == 0 {
        Ok(response.compiled_code.unwrap_or_default())
    } else {
        let error_message = response
            .error_message
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(crate::error::Error::Other(format!(
            "FXC compilation failed (HRESULT: 0x{:X}): {}",
            response.hresult, error_message
        )))
    }
}
