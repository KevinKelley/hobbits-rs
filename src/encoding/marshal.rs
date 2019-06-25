// package encoding
//
// import (
// 	"errors"
// 	"fmt"
// )
//
// // Marshal takes a parsed message and encodes it to a wire protocol message
// 	func Marshal(message Message) (string, error) {
// 	if message.Version == "" {
// 		return "", errors.New("cannot marshal message, version not found")
// 	}
//
// 	if message.Protocol == "" {
// 		return "", errors.New("cannot marshal message, protocol not found")
// 	}
//
// 	return fmt.Sprintf(
// 		"EWP %s %s %d %d\n%s%s",
// 		message.Version,
// 		message.Protocol,
// 		len(string(message.Header)),
// 		len(string(message.Body)),
// 		string(message.Header),
// 		string(message.Body),
// 	), nil
// }

pub use super::message::{Message};

pub fn marshal(msg: Message) -> Option<Vec<u8>> {
    assert!(msg.version == "0.2");
    assert!(msg.protocol != "");

    let header: String = format!("EWP {} {} {} {}\n",
                                msg.version,
                                msg.protocol,
                                msg.header.len(),
                                msg.body.len());

    let mut outbytes: Vec<u8> = header.into_bytes();
    outbytes.extend(msg.header);
    outbytes.extend(msg.body);

    return Some(outbytes)
}



#[cfg(test)]
mod tests {
    use super::{Message, marshal};

    #[test]
    fn basic_sanity() {

        //   - desc: 'no body'
        //     marshalled: "EWP 0.2 PING 0 0\n"
        let mut msg = Message::new("PING", &vec!(), &vec!());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 0 0\n".as_bytes());
        //   - desc: '10 byte body'
        //     marshalled: "EWP 0.2 PING 0 10\n0123456789"
        msg = Message::new("PING", &vec!(), "0123456789".as_bytes());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 0 10\n0123456789".as_bytes());
        //   - desc: '10 byte header'
        //     marshalled: "EWP 0.2 PING 10 0\n0123456789"
        msg = Message::new("PING", "0123456789".as_bytes(), &vec!());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 10 0\n0123456789".as_bytes());
        //   - desc: '9 byte header, 10 byte body'
        //     marshalled: "EWP 0.2 PING 9 10\n9876543210123456789"
        msg = Message::new("PING", "987654321".as_bytes(), "0123456789".as_bytes());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 9 10\n9876543210123456789".as_bytes());
        //   - desc: '9 byte header, 10 byte body, extra newlines'
        //     marshalled: "EWP 0.2 PING 9 10\n\n876543210123456\n89"
        msg = Message::new("PING", "\n87654321".as_bytes(), "0123456\n89".as_bytes());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 9 10\n\n876543210123456\n89".as_bytes());
        //   - desc: '9 byte header, 10 byte body, extra extra newlines'
        //     marshalled: "EWP 0.2 PING 9 10\n\n87654321\n\n\n\n\n\n\n\n\n\n"
        msg = Message::new("PING", "\n87654321".as_bytes(), "\n\n\n\n\n\n\n\n\n\n".as_bytes());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 9 10\n\n87654321\n\n\n\n\n\n\n\n\n\n".as_bytes());
        //   - desc: '9 byte header, 10 byte body, control character montage'
        //     marshalled: "EWP 0.2 PING 9 10\n\n87654321\n\0\a\b\f\n\r\t\v\\"
        // NOTE: those aren't valid Rust control characters...
        msg = Message::new("PING", "\n87654321".as_bytes(), "\n\0\x0a\x0b\x0f\n\r\t\x01\\".as_bytes());
        assert_eq!(marshal(msg).unwrap(), "EWP 0.2 PING 9 10\n\n87654321\n\0\x0a\x0b\x0f\n\r\t\x01\\".as_bytes());

    }

    // - suite: 'test different commands'
    //   requests:
    //   - desc: 'PING'
    //     marshalled: "EWP 0.2 PING 0 0\n"
    //   - desc: 'FOO'
    //     marshalled: "EWP 0.2 FOO 0 0\n"
    //   - desc: 'BAR'
    //     marshalled: "EWP 0.2 BAR 0 0\n"
    //   - desc: 'PONG'
    //     marshalled: "EWP 0.2 PONG 0 0\n"
    //   responses: []


    //
    // import (
    // 	"errors"
    // 	"reflect"
    // 	"strconv"
    // 	"testing"
    //
    // 	"github.com/renaynay/go-hobbits/encoding"
    // )
    //
    // func TestMarshal_Successful(t *testing.T) {
    // 	var test = []struct {
    // 		encoded encoding.Message
    // 		message string
    // 	}{
    // 		{
    // 			encoded: encoding.Message{
    // 				Version:     "13.05",
    // 				Protocol:    "RPC",
    // 				Header:     []byte("this is a header"),
    // 				Body:        []byte("this is a body"),
    // 			},
    // 			message: "EWP 13.05 RPC 16 14\nthis is a headerthis is a body",
    // 		},
    // 		{
    // 			encoded: encoding.Message{
    // 				Version:     "13.05",
    // 				Protocol:    "GOSSIP",
    // 				Header:     []byte("testing"),
    // 				Body:        []byte("testing body"),
    // 			},
    // 			message: "EWP 13.05 GOSSIP 7 12\ntestingtesting body",
    // 		},
    // 		{
    // 			encoded: encoding.Message{
    // 				Version:     "1230329483.05392489",
    // 				Protocol:    "RPC",
    // 				Header:     []byte("test"),
    // 				Body:        []byte("test"),
    // 			},
    // 			message: "EWP 1230329483.05392489 RPC 4 4\ntesttest",
    // 		},
    // 	}
    //
    // 	for i, tt := range test {
    // 		t.Run(strconv.Itoa(i), func(t *testing.T) {
    // 			string, _ := encoding.Marshal(tt.encoded)
    // 			if !reflect.DeepEqual(string, tt.message) {
    // 				t.Errorf("return value of Marshal did not match expected value. wanted: %v, got: %v", tt.message, string)
    // 			}
    // 		})
    // 	}
    // }
    //
    // func TestMarshal_Unsuccessful(t *testing.T) {
    // 	var test = []struct {
    // 		encoded encoding.Message
    // 		err     error
    // 	}{
    // 		{
    // 			encoded: encoding.Message{
    // 				Version:     "",
    // 				Protocol:    "RPC",
    // 				Header:     []byte("this is a header"),
    // 				Body:        []byte("this is a body"),
    // 			},
    // 			err: errors.New("cannot marshal message, version not found"),
    // 		},
    // 		{
    // 			encoded: encoding.Message{
    // 				Version:     "1230329483.05392489",
    // 				Protocol:    "",
    // 				Header:     []byte("test"),
    // 				Body:        []byte("test"),
    // 			},
    // 			err: errors.New("cannot marshal message, protocol not found"),
    // 		},
    // 	}
    //
    // 	for i, tt := range test {
    // 		t.Run(strconv.Itoa(i), func(t *testing.T) {
    // 			_, err := encoding.Marshal(tt.encoded)
    // 			if !reflect.DeepEqual(err, tt.err) {
    // 				t.Errorf("return value of Marshal did not match expected value")
    // 			}
    // 		})
    // 	}
    // }
    //
    //
    // func BenchmarkMarshal(b *testing.B) {
    // 	message := encoding.Message{
    // 		Version: "13.5",
    // 		Protocol: "RPC",
    // 		Header: []byte("this is a header"),
    // 		Body: []byte("this is a body"),
    // 	}
    //
    // 	for i := 0; i <= b.N; i++ {
    // 		encoding.Marshal(message)
    // 	}
    // }


}
