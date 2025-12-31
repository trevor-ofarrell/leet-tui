use anyhow::Result;
use portable_pty::{CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct PtyManager {
    pub pair: PtyPair,
    pub writer: Box<dyn Write + Send>,
    pub parser: Arc<RwLock<vt100::Parser>>,
}

impl PtyManager {
    pub fn new(rows: u16, cols: u16, file_path: PathBuf) -> Result<Self> {
        let pty_system = NativePtySystem::default();

        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = CommandBuilder::new("nvim");
        // Use --clean to ignore user config and plugins that might persist sessions
        cmd.arg("--clean");
        cmd.arg("-i");
        cmd.arg("NONE"); // Explicitly no shada
        cmd.arg("--");
        cmd.arg(file_path.to_str().unwrap_or("solution.js"));

        // Spawn nvim process
        let _child = pair.slave.spawn_command(cmd)?;

        let writer = pair.master.take_writer()?;
        let reader = pair.master.try_clone_reader()?;

        // Create VT parser for terminal emulation
        let parser = Arc::new(RwLock::new(vt100::Parser::new(rows, cols, 1000)));

        // Spawn thread to read PTY output and update parser
        let parser_clone = parser.clone();
        thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 8192];
            loop {
                match reader.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        if let Ok(mut p) = parser_clone.write() {
                            p.process(&buffer[..n]);
                        }
                    }
                    Ok(_) => break,
                    Err(_) => break,
                }
            }
        });

        Ok(PtyManager {
            pair,
            writer,
            parser,
        })
    }

    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        self.pair.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        if let Ok(mut parser) = self.parser.write() {
            *parser = vt100::Parser::new(rows, cols, 1000);
        }

        Ok(())
    }

    pub fn send_key(&mut self, data: &[u8]) -> Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()?;
        Ok(())
    }
}
