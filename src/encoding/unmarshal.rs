
pub use super::message::Message;

pub fn unmarshal(msg: &[u8]) -> Option<Message> {
    let index = msg.iter().position(|&r| r == '\n' as u8);
    if index == None { assert!(false) } // switch to proper error handling
    let index = index.unwrap();
    let hdr = &msg[0..index];
    let payload = &msg[(index+1)..];
    let hdr_str = String::from_utf8_lossy(&hdr); // fail out with None if unparseable
    let hdr_parts: Vec<&str> = hdr_str.split(' ').collect();
    if hdr_parts.len() != 5 { assert!(false) }
    if hdr_parts[0] != "EWP" { assert!(false) }
    // check for proper version matching regexp(`^(\d+\.)(\d+)*$`)
    // check for allowed protocol;
    let version = hdr_parts[1];
    let protocol = hdr_parts[2];
    let msg_hdr_len: usize = hdr_parts[3].parse().unwrap();
    let msg_bdy_len: usize = hdr_parts[4].parse().unwrap();
    // check for correctly-parsed sizes, instead of failing out
    // validate payload length matches sum of header and body length
    if payload.len() != msg_hdr_len + msg_bdy_len { assert!(false) }

    let msg_hdr = &payload[0..msg_hdr_len];
    let msg_bdy = &payload[msg_hdr_len..];

    Some( Message {
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
            if let Some(msg) = unmarshalled {
                println!("{}", t.output);
                assert!(msg == t.output);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_unmarshal_unsuccessful() {
        struct Test {
            message: Vec<u8>, // WARN! we're loading this from utf-8 strings, so don't use non-ascii string content
            err: String
        }
        let tests: Vec<Test> = vec!(
    		// Test {
    		// 	message: "EWP 13.05 RPC blahblahblah json 16 14this is a headerthis is a body",
    		// 	err:     errors.New("message request must contain 2 lines"),
    		// },
    		// Test {
    		// 	message: "EWP 13.05 7 12\ntestingtesting body",
    		// 	err:     errors.New("not all metadata provided"),
    		// },
    		// Test {
    		// 	message: "EWP 123032948392489 RPC 4 4\ntesttest",
    		// 	err:     errors.New("EWP version cannot be parsed"),
    		// },
    		// Test {
    		// 	message: "EWP 123032948.392489 notrpc 4 4\ntesttest",
    		// 	err:     errors.New("communication protocol unsupported"),
    		// },
    		// Test {
    		// 	message: "EWP 123032948.392489 GOSSIP f 4\ntesttest",
    		// 	err:     errors.New("incorrect metadata format, cannot parse header-length"),
    		// },
    		// Test {
    		// 	message: "EWP 123032948.392489 GOSSIP 4 f\ntesttest",
    		// 	err:     errors.New("incorrect metadata format, cannot parse body-length"),
    		// },
    	);
    }

}
