
pub use super::envelope::{Envelope};
use crate::encoding::EwpError;


/// Marshal takes a parsed message and encodes it to a wire protocol message
pub fn marshal(msg: &Envelope) -> Result<Vec<u8>, EwpError> {

    if msg.version == "" { return Err(EwpError::new("missing version!")) }
    if msg.protocol == "" { return Err(EwpError::new("missing protocol!")) }

    let header: String = format!("EWP {} {} {} {}\n",
                                msg.version,
                                msg.protocol,
                                msg.header.len(),
                                msg.body.len());

    let mut outbytes: Vec<u8> = header.into_bytes();
    outbytes.extend(&msg.header);
    outbytes.extend(&msg.body);

    return Ok(outbytes)
}



#[cfg(test)]
mod tests {
    use super::{Envelope, marshal, EwpError};

    #[test]
    fn basic_sanity() {
        //   - desc: 'no body'
        //     marshalled: "EWP 0.2 PING 0 0\n"
        let mut msg = Envelope::new("PING", &vec!(), &vec!());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 0 0\n".as_bytes());
        //   - desc: '10 byte body'
        //     marshalled: "EWP 0.2 PING 0 10\n0123456789"
        msg = Envelope::new("PING", &vec!(), "0123456789".as_bytes());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 0 10\n0123456789".as_bytes());
        //   - desc: '10 byte header'
        //     marshalled: "EWP 0.2 PING 10 0\n0123456789"
        msg = Envelope::new("PING", "0123456789".as_bytes(), &vec!());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 10 0\n0123456789".as_bytes());
        //   - desc: '9 byte header, 10 byte body'
        //     marshalled: "EWP 0.2 PING 9 10\n9876543210123456789"
        msg = Envelope::new("PING", "987654321".as_bytes(), "0123456789".as_bytes());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 9 10\n9876543210123456789".as_bytes());
        //   - desc: '9 byte header, 10 byte body, extra newlines'
        //     marshalled: "EWP 0.2 PING 9 10\n\n876543210123456\n89"
        msg = Envelope::new("PING", "\n87654321".as_bytes(), "0123456\n89".as_bytes());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 9 10\n\n876543210123456\n89".as_bytes());
        //   - desc: '9 byte header, 10 byte body, extra extra newlines'
        //     marshalled: "EWP 0.2 PING 9 10\n\n87654321\n\n\n\n\n\n\n\n\n\n"
        msg = Envelope::new("PING", "\n87654321".as_bytes(), "\n\n\n\n\n\n\n\n\n\n".as_bytes());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 9 10\n\n87654321\n\n\n\n\n\n\n\n\n\n".as_bytes());
        //   - desc: '9 byte header, 10 byte body, control character montage'
        //     marshalled: "EWP 0.2 PING 9 10\n\n87654321\n\0\a\b\f\n\r\t\v\\"
        // NOTE: those aren't valid Rust control characters...
        msg = Envelope::new("PING", "\n87654321".as_bytes(), "\n\0\x0a\x0b\x0f\n\r\t\x01\\".as_bytes());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 9 10\n\n87654321\n\0\x0a\x0b\x0f\n\r\t\x01\\".as_bytes());
    }

    #[test]
    fn different_commands() {
        //   - desc: 'PING'
        //     marshalled: "EWP 0.2 PING 0 0\n"
        let mut msg = Envelope::new("PING", &vec!(), &vec!());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PING 0 0\n".as_bytes());
        //   - desc: 'FOO'
        //     marshalled: "EWP 0.2 FOO 0 0\n"
        msg = Envelope::new("FOO", &vec!(), &vec!());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 FOO 0 0\n".as_bytes());
        //   - desc: 'BAR'
        //     marshalled: "EWP 0.2 BAR 0 0\n"
        msg = Envelope::new("BAR", &vec!(), &vec!());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 BAR 0 0\n".as_bytes());
        //   - desc: 'PONG'
        //     marshalled: "EWP 0.2 PONG 0 0\n"
        msg = Envelope::new("PONG", &vec!(), &vec!());
        assert_eq!(marshal(&msg).unwrap(), "EWP 0.2 PONG 0 0\n".as_bytes());
    }

    #[test]
    fn test_marshal_successful() {
        struct Test {
            encoded: Envelope,
            message: String
        }
        let tests: Vec<Test> = vec!(
    		Test{
    			encoded: Envelope{
    				version:     "13.05".to_string(),
    				protocol:    "RPC".to_string(),
    				header:      "this is a header".to_string().into_bytes(),
    				body:        "this is a body".to_string().into_bytes(),
    			},
    			message: "EWP 13.05 RPC 16 14\nthis is a headerthis is a body".to_string(),
    		},
    		Test{
    			encoded: Envelope{
    				version:     "13.05".to_string(),
    				protocol:    "GOSSIP".to_string(),
    				header:      "testing".to_string().into_bytes(),
    				body:        "testing body".to_string().into_bytes(),
    			},
    			message: "EWP 13.05 GOSSIP 7 12\ntestingtesting body".to_string(),
    		},
    		Test{
    			encoded: Envelope{
    				version:     "1230329483.05392489".to_string(),
    				protocol:    "RPC".to_string(),
    				header:      "test".to_string().into_bytes(),
    				body:        "test".to_string().into_bytes(),
    			},
    			message: "EWP 1230329483.05392489 RPC 4 4\ntesttest".to_string(),
    		},
    	);

        for t in tests.iter() {
            let marshalled = marshal(&t.encoded).unwrap();
            println!("{}", t.message);
            assert!(marshalled == t.message.as_bytes());
        }
    }

    #[test]
    fn test_marshal_unsuccessful() {
        struct Test {
            encoded: Envelope,
            err: EwpError
        }
        let tests: Vec<Test> = vec!(
    		Test{
    			encoded: Envelope{
    				version:     "".to_string(),
    				protocol:    "RPC".to_string(),
    				header:     "this is a header".to_string().into_bytes(),
    				body:        "this is a body".to_string().into_bytes(),
    			},
    			err: EwpError::new("cannot marshal message, version not found"),
    		},
    		Test{
    			encoded: Envelope{
    				version:     "1230329483.05392489".to_string(),
    				protocol:    "".to_string(),
    				header:     "test".to_string().into_bytes(),
    				body:        "test".to_string().into_bytes(),
    			},
    			err: EwpError::new("cannot marshal message, protocol not found"),
    		},
    	);

        for t in tests.iter() {
            let marshalled = marshal(&t.encoded);
            println!("{}", t.err);
            //assert!(marshalled == t.message.as_bytes());
        }
    }
}
