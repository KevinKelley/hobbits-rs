
// // Package encoding implements message encoding and decoding for Hobbits, a Lightweight,
// // Multiclient Wire Protocol For ETH2.0 Communications.
// //
// // By Rene Nayman
// package encoding
//
// // Message represents a parsed Hobbits message.
// // See examples of unparsed and parsed messages here: https://github.com/deltap2p/hobbits/blob/master/specs/protocol.md
// type Message struct {
// 	Version  string
// 	Protocol string
// 	Header   []byte
// 	Body     []byte
// }

#[derive(Clone, Hash, Default, PartialEq, Debug)]
pub struct Message {
    pub version: String,
    pub protocol: String,
    pub header: Vec<u8>,
    pub body: Vec<u8>,
}

impl Message {

    pub fn new(proto: &str, hdr: &[u8], bdy: &[u8]) -> Message {
        return Message {
            version: "0.2".to_string(),
            protocol: proto.to_string(),
            header: hdr.to_vec(),
            body: bdy.to_vec()
        }
    }

}

// func reqParse(req string) Request {
// 	res := strings.Split(req, "\n")
//
// 	reqLine := res[0]
// 	payload := strings.Join(res[1:], "\n")
// 	r := strings.Split(reqLine, " ")
// 	if len(r) < 8 {
// 		r = append(r, " ")
// 	}
// 	headersLen, _ := strconv.Atoi(r[3])
// 	bodyLen, _ := strconv.Atoi(r[4])
// 	headers := payload[0:headersLen]
// 	body := payload[headersLen : headersLen+bodyLen]
//
// 	request := Request{
// 		proto:   r[0],
// 		version: r[1],
// 		command: r[2],
// 		headers: []byte(headers),
// 		body:    []byte(body),
// 	}
// 	return request
// }
//
// func reqMarshal(req Request) string {
// 	requestLine := fmt.Sprintf("%s %s %s %d %d",
// 		req.proto,
// 		req.version,
// 		req.command,
// 		len(req.headers),
// 		len(req.body))
//
// 	r := fmt.Sprintf("%s\n%s%s", requestLine, string(req.headers), string(req.body))
// 	return r
// }
