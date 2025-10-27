use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use d3d11::fxc::wine::RemoteFxcRequest;
use d3d11_ffi::Foundation::{E_FAIL, E_INVALIDARG, S_OK};

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:18008"; // Port is 0x4658 ("FX")
    let sock = TcpListener::bind(addr)?;
    println!("FXC server listening on {addr}");
    for stream in sock.incoming() {
        match stream {
            Ok(stream) => {
                println!("Received connection from {}", stream.peer_addr()?);
                stream.set_read_timeout(None)?;
                std::thread::spawn(move || {
                    compiler_thread(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn compiler_thread(mut stream: TcpStream) {
    _ = stream.set_nodelay(true);
    while let Ok(data_length) = stream.read_u32::<BigEndian>() {
        let mut data = vec![0u8; data_length as usize];
        if let Err(e) = stream.read_exact(&mut data) {
            eprintln!("Failed to read FXC request data: {}", e);
            break;
        }
        let (
            RemoteFxcRequest {
                data,
                source_name,
                defines,
                entry_point,
                target,
            },
            _,
        ): (RemoteFxcRequest, _) =
            match bincode::decode_from_slice(&data, bincode::config::standard()) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Failed to decode FXC request: {}", e);
                    break;
                }
            };

        println!(
            "Compiling shader: entry_point='{}', target='{}', defines={:?}",
            entry_point, target, defines
        );
        let result = d3d11::fxc::native::compile(
            &data,
            source_name.as_deref(),
            &defines
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>(),
            &entry_point,
            match target.as_str() {
                "vs_5_0" => d3d11::fxc::ShaderTarget::Vertex,
                "ps_5_0" => d3d11::fxc::ShaderTarget::Pixel,
                "gs_5_0" => d3d11::fxc::ShaderTarget::Geometry,
                "hs_5_0" => d3d11::fxc::ShaderTarget::Hull,
                "ds_5_0" => d3d11::fxc::ShaderTarget::Domain,
                "cs_5_0" => d3d11::fxc::ShaderTarget::Compute,
                _ => {
                    let error_message = format!("Unknown shader target: {}", target);
                    let response = d3d11::fxc::wine::RemoteFxcResult {
                        hresult: E_INVALIDARG.0,
                        compiled_code: None,
                        error_message: Some(error_message),
                    };
                    let _ = bincode::encode_into_std_write(
                        &response,
                        &mut stream,
                        bincode::config::standard(),
                    );
                    continue;
                }
            },
        );

        let response = match result {
            Ok(compiled_code) => d3d11::fxc::wine::RemoteFxcResult {
                hresult: S_OK.0,
                compiled_code: Some(compiled_code),
                error_message: None,
            },
            Err(e) => d3d11::fxc::wine::RemoteFxcResult {
                hresult: E_FAIL.0,
                compiled_code: None,
                error_message: Some(format!("Compilation failed: {}", e)),
            },
        };

        let mut write_buffer = Vec::<u8>::with_capacity(1024 * 1024);
        let _ = bincode::encode_into_std_write(
            &response,
            &mut write_buffer,
            bincode::config::standard(),
        );
        let _ = stream.write_u32::<BigEndian>(write_buffer.len() as u32);
        let _ = stream.write_all(&write_buffer);
    }
}
