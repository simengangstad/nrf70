use embedded_hal::spi::Operation;
use embedded_hal_async::spi::SpiDevice;

use crate::{slice8, util::slice8_mut};

pub trait Bus {
    async fn read(&mut self, addr: u32, buf: &mut [u32]);
    async fn write(&mut self, addr: u32, buf: &[u32]);
    async fn read_sr0(&mut self) -> u8;
    async fn read_sr1(&mut self) -> u8;
    async fn read_sr2(&mut self) -> u8;
    async fn write_sr2(&mut self, val: u8);
}

pub struct SpiBus<T> {
    spi: T,
}

impl<T> SpiBus<T> {
    pub fn new(spi: T) -> Self {
        Self { spi }
    }
}

impl<T: SpiDevice> Bus for SpiBus<T> {
    #[allow(clippy::cast_possible_truncation)]
    async fn read(&mut self, addr: u32, buf: &mut [u32]) {
        self.spi
            .transaction(&mut [
                Operation::Write(&[0x0B, (addr >> 16) as u8, (addr >> 8) as u8, addr as u8, 0x00]),
                Operation::Read(slice8_mut(buf)),
            ])
            .await
            .unwrap();
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn write(&mut self, addr: u32, buf: &[u32]) {
        self.spi
            .transaction(&mut [
                Operation::Write(&[0x02, (addr >> 16) as u8 | 0x80, (addr >> 8) as u8, addr as u8]),
                Operation::Write(slice8(buf)),
            ])
            .await
            .unwrap();
    }

    async fn read_sr0(&mut self) -> u8 {
        let mut buf = [0; 2];
        self.spi.transfer(&mut buf, &[0x05]).await.unwrap();
        let val = buf[1];
        trace!("read sr0 = {:02x}", val);
        val
    }

    async fn read_sr1(&mut self) -> u8 {
        let mut buf = [0; 2];
        self.spi.transfer(&mut buf, &[0x1f]).await.unwrap();
        let val = buf[1];
        trace!("read sr1 = {:02x}", val);
        val
    }

    async fn read_sr2(&mut self) -> u8 {
        let mut buf = [0; 2];
        self.spi.transfer(&mut buf, &[0x2f]).await.unwrap();
        let val = buf[1];
        trace!("read sr2 = {:02x}", val);
        val
    }

    async fn write_sr2(&mut self, val: u8) {
        trace!("write sr2 = {:02x}", val);
        self.spi.write(&[0x3f, val]).await.unwrap();
    }
}

/*
pub struct QspiBus<'a> {
    qspi: Qspi<'a, QSPI>,
}

impl<'a> QspiBus<'a> {}

impl<'a> Bus for QspiBus<'a> {
    async fn read(&mut self, addr: u32, buf: &mut [u32]) {
        self.qspi.read(addr, slice8_mut(buf)).await.unwrap();
    }

    async fn write(&mut self, addr: u32, buf: &[u32]) {
        self.qspi.write(addr, slice8(buf)).await.unwrap();
    }

    async fn read_sr0(&mut self) -> u8 {
        let mut status = [4; 1];
        unwrap!(self.qspi.custom_instruction(0x05, &[0x00], &mut status).await);
        trace!("read sr0 = {:02x}", status[0]);
        status[0]
    }

    async fn read_sr1(&mut self) -> u8 {
        let mut status = [4; 1];
        unwrap!(self.qspi.custom_instruction(0x1f, &[0x00], &mut status).await);
        trace!("read sr1 = {:02x}", status[0]);
        status[0]
    }

    async fn read_sr2(&mut self) -> u8 {
        let mut status = [4; 1];
        unwrap!(self.qspi.custom_instruction(0x2f, &[0x00], &mut status).await);
        trace!("read sr2 = {:02x}", status[0]);
        status[0]
    }

    async fn write_sr2(&mut self, val: u8) {
        trace!("write sr2 = {:02x}", val);
        unwrap!(self.qspi.custom_instruction(0x3f, &[val], &mut []).await);
    }
}
 */
