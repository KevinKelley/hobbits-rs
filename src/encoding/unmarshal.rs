
pub use super::message::Message;
pub use super::EwpError;

pub fn unmarshal(msg: &[u8]) -> Result<Message,EwpError> {
    let index = msg.iter().position(|&r| r == '\n' as u8);
    if index == None { return Err(EwpError::new("message request must contain 2 lines")) }
    let index = index.unwrap();

    let hdr = &msg[0..index];
    let payload = &msg[(index+1)..];
    let hdr_str = String::from_utf8(hdr.to_vec())?; // fail out with Err if unparseable
    let hdr_parts: Vec<&str> = hdr_str.split(' ').collect();
    if hdr_parts.len() != 5 { return Err(EwpError::new("not all metadata provided")) }
    if hdr_parts[0] != "EWP" { return Err(EwpError::new("malformed EWP envelope: must start with 'EWP'")) }

    let version = hdr_parts[1];
    // TODO check for proper version matching regexp(`^(\d+\.)(\d+)*$`)
    if !version.contains('.') { return Err(EwpError::new("EWP version cannot be parsed")) }

    let protocol = hdr_parts[2];
    // check for allowed protocol
    if protocol != "GOSSIP"
    && protocol != "RPC"
    && protocol != "PING" { return Err(EwpError::new("communication protocol unsupported")) }

    let msg_hdr_len = hdr_parts[3].parse();
    if let Err(_) = msg_hdr_len {
        return Err(EwpError::new("incorrect metadata format, cannot parse header-length"))
    }
    let msg_hdr_len = msg_hdr_len.unwrap();

    let msg_bdy_len:Result<usize,std::num::ParseIntError> = hdr_parts[4].parse();
    if let Err(_) = msg_bdy_len {
        return Err(EwpError::new("incorrect metadata format, cannot parse body-length"))
    }
    let msg_bdy_len:usize = msg_bdy_len.unwrap();

    // check for correctly-parsed sizes, instead of failing out
    // validate payload length matches sum of header and body length
    if payload.len() != msg_hdr_len + msg_bdy_len {
        return Err(EwpError::new(&format!("unexpected payload size: {} != {} + {}",
                                    payload.len(),
                                    msg_hdr_len,
                                    msg_bdy_len)))
    }

    let msg_hdr = &payload[0..msg_hdr_len];
    let msg_bdy = &payload[msg_hdr_len..];

    Ok( Message {
        version: version.to_string(),
        protocol: protocol.to_string(),
        header: msg_hdr.to_owned(),
        body: msg_bdy.to_owned()
    })
}


#[cfg(test)]
mod tests {
    use super::{Message, unmarshal};

    #[test]
    fn test_unmarshal_successful() {
        struct Test {
            message: Vec<u8>, // WARN! we're loading this from utf-8 strings, so don't use non-ascii string content
            output: Message
        }
        let tests: Vec<Test> = vec!(
    		Test {
    			message: "EWP 13.05 RPC 16 14\nthis is a headerthis is a body".to_string().into_bytes(),
    			output: Message {
    				version:     "13.05".to_string(),
    				protocol:    "RPC".to_string(),
    				header:      "this is a header".to_string().into_bytes(),
    				body:        "this is a body".to_string().into_bytes(),
    			},
    		},
    		Test {
    			message: "EWP 13.05 GOSSIP 7 12\ntestingtesting body".to_string().into_bytes(),
    			output: Message {
    				version:     "13.05".to_string(),
    				protocol:    "GOSSIP".to_string(),
    				header:      "testing".to_string().into_bytes(),
    				body:        "testing body".to_string().into_bytes(),
    			},
    		},
    		Test {
    			message: "EWP 1230329483.05392489 RPC 4 4\ntesttest".to_string().into_bytes(),
    			output: Message {
    				version:     "1230329483.05392489".to_string(),
    				protocol:    "RPC".to_string(),
    				header:      "test".to_string().into_bytes(),
    				body:        "test".to_string().into_bytes(),
    			},
    		},
    	);

        for t in tests.iter() {
            let unmarshalled = unmarshal(&t.message);
            if let Ok(msg) = unmarshalled {
                println!("{}", t.output);
                assert!(msg == t.output);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_unmarshal_unsuccessful() {

        use super::*;

        struct Test {
            message: Vec<u8>,
            err: EwpError
        }
        let tests: Vec<Test> = vec!(
    		Test {
    			message: "EWP 13.05 RPC blahblahblah json 16 14this is a headerthis is a body".to_string().into_bytes(),
    			err:     EwpError::new("message request must contain 2 lines"),
    		},
    		Test {
    			message: "EWP 13.05 7 12\ntestingtesting body".to_string().into_bytes(),
    			err:     EwpError::new("not all metadata provided"),
    		},
    		Test {
    			message: "EWP 123032948392489 RPC 4 4\ntesttest".to_string().into_bytes(),
    			err:     EwpError::new("EWP version cannot be parsed"),
    		},
    		Test {
    			message: "EWP 123032948.392489 notrpc 4 4\ntesttest".to_string().into_bytes(),
    			err:     EwpError::new("communication protocol unsupported"),
    		},
    		Test {
    			message: "EWP 123032948.392489 GOSSIP f 4\ntesttest".to_string().into_bytes(),
                err:     EwpError::new("incorrect metadata format, cannot parse header-length"),
                //err:     EwpError::new("invalid digit found in string"),
    		},
    		Test {
    			message: "EWP 123032948.392489 GOSSIP 4 f\ntesttest".to_string().into_bytes(),
                err:     EwpError::new("incorrect metadata format, cannot parse body-length"),
                //err:     EwpError::new("invalid digit found in string"),
    		},
    	);
        for t in tests.iter() {
            let unmarshalled = unmarshal(&t.message);
            match unmarshalled {
                Ok(msg) => {
                    // error expected!
                    println!("expected: {}", &t.err);
                    println!("received: {}", msg);
                    assert!(msg != msg) // force fail
                }
                Err(err) => {
                    println!("expected: '{}'", t.err.details);
                    println!("received: '{}'", err.details);
                    assert!(t.err.details == err.details)
                }
            }
        }
    }

}
