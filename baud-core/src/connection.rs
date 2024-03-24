use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_serial::{DataBits, FlowControl, Parity, SerialPortBuilderExt, SerialStream, StopBits};

pub struct SerialConnection {
    port: SerialStream,
}

impl SerialConnection {
    /// Connects to a serial port with the given name and baud rate.
    pub async fn connect(port_name: &str, baud_rate: u32) -> io::Result<Self> {
        let stream = tokio_serial::new(port_name, baud_rate)
            .data_bits(DataBits::Eight)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(std::time::Duration::from_secs(5))
            .open_native_async()?;

        Ok(Self { port: stream })
    }

    /// Reads data from the serial port into a buffer and returns the buffer.
    pub async fn read_data(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut temp_buffer = [0u8; 1024];
        let n = self.port.read(&mut temp_buffer).await?;
        if n > 0 {
            buffer.extend_from_slice(&temp_buffer[..n]);
        }
        Ok(buffer)
    }

    /// Writes data to the serial port from the provided buffer.
    pub async fn write_data(&mut self, data: &[u8]) -> io::Result<()> {
        self.port.write_all(data).await
    }
}
