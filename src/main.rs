
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailBox: Mailbox,
}

// 복잡한 로직 추상화
fn check_status(sat_id: CubeSat) -> CubeSat {

    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);

    sat_id
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message){
        to.mailBox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailBox.messages.pop()
    }
}

fn main() {

    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailBox: Mailbox { messages:vec![] }
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("Hello, there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
