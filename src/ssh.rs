use anyhow::{Result, Context};
use std::net::TcpStream;

pub struct SshClient {
    host: String,
    user: String,
    password: String,
    session: Option<ssh2::Session>,
    channel: Option<ssh2::Channel>,
}

impl SshClient {

    pub fn new(host: String, user: String, password: String) -> Self {
        SshClient {
            host,
            user,
            password,
            session: None,
            channel: None
        }
    }


    pub fn connect(&mut self) -> Result<()> {
        let tcp = TcpStream::connect(format!("{}:22", self.host))
            .with_context(|| format!("SSH接続に失敗しました: {}", self.host))?;
        
        self.session = Some(ssh2::Session::new().with_context(|| "SSHセッションの作成に失敗しました")?);
        
        let session = self.session.as_mut().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        session.userauth_password(&self.user, &self.password)?;
        
        self.channel = Some(session.channel_session()?);
        
        Ok(())
        
    }
    
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn test_ssh_connect() {
        let ssh_user = "ssh".to_string();
        let ssh_password = "password".to_string();

        let mut client = super::SshClient::new("localhost".to_string(), ssh_user, ssh_password);
        assert!(client.connect().is_ok());
    }
    
}