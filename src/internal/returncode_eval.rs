use crate::internal::*;

pub fn exec_eval(
    return_code: std::result::Result<std::process::ExitStatus, std::io::Error>,
    logmsg: &str,
) {
    match &return_code {
        Ok(_) => {
            log::info!("{}", logmsg);
        }
        Err(e) => {
            crash(
                format!("{}  ERROR: {}", logmsg, e),
                return_code.unwrap_err().raw_os_error().unwrap(),
            );
        }
    }
}

pub fn exec_eval_noerr(
    return_code: std::result::Result<std::process::ExitStatus, std::io::Error>,
    logmsg: &str,
    warn: bool,
) {
    match &return_code {
        Ok(_) => {
            log::info!("{}", logmsg);
        }
        Err(e) => {
            if warn {
                log::warn!("{} ERROR: {}", logmsg, e);
            }
        }
    }
}

pub fn files_eval(return_code: std::result::Result<(), std::io::Error>, logmsg: &str) {
    match &return_code {
        Ok(_) => {
            log::info!("{}", logmsg);
        }
        Err(e) => {
            crash(
                format!("{} ERROR: {}", logmsg, e),
                return_code.unwrap_err().raw_os_error().unwrap(),
            );
        }
    }
}
