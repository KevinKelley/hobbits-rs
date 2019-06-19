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

use super::message::{Message};

pub fn marshal(_msg: Message) -> Option<String> {
    return Some("ok".to_string())
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


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
