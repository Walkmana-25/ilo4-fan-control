/// HPE iLO4 Fan Control Library
/// 
/// This library provides functionality to control fan speeds on HPE servers
/// through their iLO4 management interface. It supports both temperature-based
/// automatic control and manual fan speed settings.
/// 
/// # Modules
/// 
/// * `config` - Configuration structures and parsing
/// * `cputemp` - CPU temperature monitoring
/// * `ssh` - SSH connection management
/// * `gen_ssh` - SSH key generation and command generation
pub mod config;
pub mod cputemp;
pub mod ssh;
pub mod gen_ssh;