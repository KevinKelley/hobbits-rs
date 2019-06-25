
use super::message::Message;

pub fn unmarshal(_msg: &[u8]) -> Option<Message> {
    None
}


// import (
// 	"errors"
// 	"regexp"
// 	"strconv"
// 	"strings"
// )
//
// var versionNumRegex = regexp.MustCompile(`^(\d+\.)(\d+)*$`)
//
// // Unmarshal takes a wire protocol message and parses it
// func Unmarshal(message string) (*Message, error) {
// 	var decoded Message
//
// 	lines := strings.Split(message, "\n")
// 	if len(lines) != 2 {
// 		return nil, errors.New("message request must contain 2 lines")
// 	}
//
// 	metadata := strings.Split(lines[0], " ")
// 	if len(metadata) != 5 {
// 		return nil, errors.New("not all metadata provided")
// 	}
//
// 	if !versionNumRegex.MatchString(metadata[1]) {
// 		return nil, errors.New("EWP version cannot be parsed")
// 	}
// 	decoded.Version = metadata[1]
//
// 	if metadata[2] != "RPC" && metadata[2] != "GOSSIP" {
// 		return nil, errors.New("communication protocol unsupported")
// 	}
// 	decoded.Protocol = metadata[2]
//
// 	headLength, err := strconv.Atoi(metadata[3])
// 	if err != nil {
// 		return nil, errors.New("incorrect metadata format, cannot parse header-length")
// 	}
// 	decoded.Header = []byte(lines[1][:headLength])
//
// 	bodyLength, err := strconv.Atoi(metadata[4])
// 	if err != nil {
// 		return nil, errors.New("incorrect metadata format, cannot parse body-length")
// 	}
// 	decoded.Body = []byte(lines[1][headLength:headLength+bodyLength])
//
// 	return &decoded, nil
// }










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
    // func TestUnmarshal_Successful(t *testing.T) {
    // 	var test = []struct {
    // 		message string
    // 		output  encoding.Message
    // 	}{
    // 		{
    // 			message: "EWP 13.05 RPC 16 14\nthis is a headerthis is a body",
    // 			output: encoding.Message{
    // 				Version:     "13.05",
    // 				Protocol:    "RPC",
    // 				Header:     []byte("this is a header"),
    // 				Body:        []byte("this is a body"),
    // 			},
    // 		},
    // 		{
    // 			message: "EWP 13.05 GOSSIP 7 12\ntestingtesting body",
    // 			output: encoding.Message{
    // 				Version:     "13.05",
    // 				Protocol:    "GOSSIP",
    // 				Header:     []byte("testing"),
    // 				Body:        []byte("testing body"),
    // 			},
    // 		},
    // 		{
    // 			message: "EWP 1230329483.05392489 RPC 4 4\ntesttest",
    // 			output: encoding.Message{
    // 				Version:     "1230329483.05392489",
    // 				Protocol:    "RPC",
    // 				Header:     []byte("test"),
    // 				Body:        []byte("test"),
    // 			},
    // 		},
    // 	}
    //
    // 	for i, tt := range test {
    // 		t.Run(strconv.Itoa(i), func(t *testing.T) {
    // 			output, _ := encoding.Unmarshal(tt.message)
    // 			if !reflect.DeepEqual(*output, tt.output) {
    // 				t.Errorf("return value of Unmarshal does not match expected value")
    // 			}
    // 		})
    // 	}
    // }
    //
    // func TestUnmarshal_Unsuccessful(t *testing.T) {
    // 	var test = []struct {
    // 		message string
    // 		err     error
    // 	}{
    // 		{
    // 			message: "EWP 13.05 RPC blahblahblah json 16 14this is a headerthis is a body",
    // 			err:     errors.New("message request must contain 2 lines"),
    // 		},
    // 		{
    // 			message: "EWP 13.05 7 12\ntestingtesting body",
    // 			err:     errors.New("not all metadata provided"),
    // 		},
    // 		{
    // 			message: "EWP 123032948392489 RPC 4 4\ntesttest",
    // 			err:     errors.New("EWP version cannot be parsed"),
    // 		},
    // 		{
    // 			message: "EWP 123032948.392489 notrpc 4 4\ntesttest",
    // 			err:     errors.New("communication protocol unsupported"),
    // 		},
    // 		{
    // 			message: "EWP 123032948.392489 GOSSIP f 4\ntesttest",
    // 			err:     errors.New("incorrect metadata format, cannot parse header-length"),
    // 		},
    // 		{
    // 			message: "EWP 123032948.392489 GOSSIP 4 f\ntesttest",
    // 			err:     errors.New("incorrect metadata format, cannot parse body-length"),
    // 		},
    // 	}
    //
    // 	for i, tt := range test {
    // 		t.Run(strconv.Itoa(i), func(t *testing.T) {
    // 			_, err := encoding.Unmarshal(tt.message)
    // 			if !reflect.DeepEqual(err, tt.err) {
    // 				t.Errorf("return value of Unmarshal did not match expected value")
    // 			}
    // 		})
    // 	}
    // }
    //
    // func BenchmarkUnmarshal(b *testing.B) {
    // 	for i := 0; i <= b.N; i++ {
    // 		encoding.Unmarshal("EWP 13.05 RPC 16 14\nthis is a headerthis is a body")
    // 	}
    // }

}
