use anyhow::Result;
use async_hid::{AccessMode, Device, DeviceInfo, HidResult};

pub const WRITE_FILE: u16 = 0x0404;
pub const ERROR_STATUS: u16 = 0x0505;
pub const JUMP_ADDRESS: u16 = 0x0b0b;
pub const SET_BAUDRATE: u16 = 0x0d0d;

pub enum Command<'a> {
    WriteFile { addr: u32, data: &'a [u8] },
    ErrorStatus,
    JumpAddress { addr: u32 },
    SetBaudrate {},
}

pub enum Error {
    //
}

impl<'a> Command<'a> {
    pub async fn run(&self, dev: &mut Device) -> Result<()> {
        match self {
            Command::WriteFile { addr, data } => {
                let mut cmd = [0u8; 17];
                cmd[0] = 1;
                cmd[1..3].clone_from_slice(&WRITE_FILE.to_be_bytes());
                cmd[3..7].clone_from_slice(&addr.to_be_bytes());
                cmd[8..12].clone_from_slice(&(data.len() as u32).to_be_bytes());
                dev.write_output_report(&cmd).await?;

                for chunk in data.chunks(1024) {
                    dev.write_output_report(&[&[2], chunk].concat()).await?;
                }

                let mut buf = [0u8; 5];
                dev.read_input_report(&mut buf).await?;
                let _security_status = buf;
                dev.read_input_report(&mut buf).await?;
                let _error_status = buf;
            }
            Command::JumpAddress { addr } => {
                let mut cmd = [0u8; 17];
                cmd[0] = 1;
                cmd[1..3].clone_from_slice(&JUMP_ADDRESS.to_be_bytes());
                cmd[3..7].clone_from_slice(&addr.to_be_bytes());
                dev.write_output_report(&cmd).await?;

                let mut buf = [0u8; 5];
                dev.read_input_report(&mut buf).await?;
                let _security_status = buf;
                //dev.read_input_report(&mut buf).await?;
                //let _error_status = buf;
            }
            Command::ErrorStatus => {
                let mut cmd = [0u8; 17];
                cmd[0] = 1;
                cmd[1..3].clone_from_slice(&ERROR_STATUS.to_le_bytes());

                dev.write_output_report(&cmd).await?;
                let mut buf = [0u8; 5];
                dev.read_input_report(&mut buf).await?;
                let _security_status = buf;
                dev.read_input_report(&mut buf).await?;
                let _error_status = buf;
            }
            Command::SetBaudrate {} => {
                todo!();
            }
        }

        Ok(())
    }
}
