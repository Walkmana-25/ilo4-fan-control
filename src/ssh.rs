use anyhow::{Result, Context};

pub struct SshClient {
    host: String,
    user: String,
    password: Option<String>,
    ssh_secret_key_path: Option<String>,
}

impl SshClient {

    pub fn new(host: String, user: String, password: Option<String>, ssh_secret_key_path: Option<String>) -> Self {
        SshClient {
            host,
            user,
            password,
            ssh_secret_key_path,
        }
    }


    pub fn connect(&self) -> Result<()> {
        
        //Inspect SSH-Agent



        Ok(())
    }
    
    fn inspect_ssh_agent(&self) -> Result<()> {
        let sess = ssh2::Session::new().context("Failed to create a new SSH session")?;
        let mut agent = sess.agent().context("Failed to create a new SSH agent")?;
        
        agent.connect().with_context(|| "Failed to connect to SSH agent".to_string())?;
        agent.list_identities().with_context(|| "Failed to list SSH agent identities".to_string())?;
        
        let mut flag = false;

        for identity in agent.identities()? {
            if !identity.comment().is_empty() {
                flag = true;
                break;
            }
        }
        
        if flag {
            Ok(())
        } else {
            Err(anyhow::anyhow!("SSH agent does not have any identities"))
        }

    }
}
