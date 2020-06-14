// Copyright 2020 Eray Erdin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io;

/// A stream to use as output and input for game shell.
struct Stream {
    buffer: Vec<u8>,
}

impl Stream {
    fn new() -> Self {
        Stream { buffer: vec![] }
    }
}

impl io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let length = buf.len();

        let mut byte_count = 0usize;
        for i in 0..length {
            match self.buffer.get(0) {
                Some(byte) => {
                    buf[i] = byte.clone();
                    byte_count += 1;
                    self.buffer.remove(0);
                }
                None => {
                    break;
                }
            }
        }

        Ok(byte_count)
    }
}

impl io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        use std::io::Write;

        self.buffer.write(buf);
        Ok(buf.len())
    }
    /// Does nothing.
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::io::{Read, Write};

    #[fixture]
    fn stream() -> Stream {
        let mut stream = Stream::new();
        stream.buffer.extend(b"lorem");
        stream
    }

    #[rstest]
    fn read(mut stream: Stream) {
        let mut msg = String::new();
        let mut buffer: [u8; 8] = [0; 8];

        let byte_count = stream
            .read(&mut buffer)
            .expect("Could not read from Stream.");
        msg.extend(
            buffer
                .iter()
                .filter(|n| **n != 0u8)
                .map(|n| n.clone() as char),
        );

        assert_eq!(byte_count, 5);
        assert_eq!(msg, "lorem");
        assert!(stream.buffer.is_empty());
    }

    #[rstest]
    fn write(mut stream: Stream) {
        let buffer = b" ipsum";
        let mut expected_msg = String::new();
        expected_msg.extend("lorem".chars());
        buffer
            .iter()
            .for_each(|c| expected_msg.push(c.clone() as char));

        let byte_count = stream.write(buffer).expect("Could not write to Stream.");
        let mut msg = String::new();
        msg.extend(stream.buffer.iter().map(|c| c.clone() as char));

        assert_eq!(byte_count, buffer.len());
        assert_eq!(stream.buffer.len(), byte_count + 5);
        assert_eq!(expected_msg, msg);
    }
}
