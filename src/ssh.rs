use anyhow::{Result, Context};
use std::net::TcpStream;
use std::io::Read;

pub struct SshClient {
    host: String,
    user: String,
    password: String,
    session: Option<ssh2::Session>,
}

impl SshClient {

    pub fn new(host: String, user: String, password: String) -> Self {
        SshClient {
            host,
            user,
            password,
            session: None,
        }
    }


    pub fn connect(&mut self) -> Result<()> {
        let tcp = TcpStream::connect(format!("{}:22", self.host))
            .with_context(|| format!("SSH接続に失敗しました: {}", self.host))?;
        
        self.session = Some(ssh2::Session::new().with_context(|| "SSHセッションの作成に失敗しました")?);
        
        let session = self.session.as_mut().unwrap();
        
        // 古いキー交換アルゴリズムを追加
        session.method_pref(ssh2::MethodType::Kex, 
            "diffie-hellman-group-exchange-sha256,diffie-hellman-group-exchange-sha1,diffie-hellman-group14-sha1,diffie-hellman-group1-sha1")?;
        
        // 古いホストキーアルゴリズムを追加
        session.method_pref(ssh2::MethodType::HostKey, 
            "ssh-ed25519,ecdsa-sha2-nistp256,ecdsa-sha2-nistp384,ecdsa-sha2-nistp521,ssh-rsa")?;
        
        // 古い暗号化アルゴリズムを追加
        session.method_pref(ssh2::MethodType::CryptCs, 
            "aes128-ctr,aes192-ctr,aes256-ctr,aes128-cbc,3des-cbc,aes192-cbc,aes256-cbc")?;
        session.method_pref(ssh2::MethodType::CryptSc, 
            "aes128-ctr,aes192-ctr,aes256-ctr,aes128-cbc,3des-cbc,aes192-cbc,aes256-cbc")?;
        
        session.set_tcp_stream(tcp);
        session.handshake()?;
        
        session.userauth_password(&self.user, &self.password)?;
        
        
        Ok(())
        
    }
    
    pub fn exec(&mut self, commands: Vec<String>) -> Result<Vec<String>> {
        
        let mut result = Vec::new();

        for command in commands {
            let mut channel = self.session.as_mut().unwrap().channel_session()?;
            channel.exec(&command)?;
            
            let mut s = String::new();
            channel.read_to_string(&mut s)?;
            result.push(s);
        }
        Ok(result)
    }
    
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ssh_connect() {
        let ssh_user = "test".to_string();
        let ssh_password = "password".to_string();

        let mut client = super::SshClient::new("localhost".to_string(), ssh_user, ssh_password);
        assert!(client.connect().is_ok());
    }

    
    #[test]
    fn test_ssh_exec_cmd() {
        let ssh_user = "test".to_string();
        let ssh_password = "password".to_string();

        let mut client = super::SshClient::new("localhost".to_string(), ssh_user, ssh_password);
        assert!(client.connect().is_ok());
        
        let commands = vec!["echo hello".to_string(), "echo world".to_string()];
        let results = client.exec(commands).expect("Failed to execute commands");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].trim(), "hello");
        assert_eq!(results[1].trim(), "world");
    }
}