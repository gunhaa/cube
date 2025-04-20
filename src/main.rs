

// 위성
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mail_box: Mailbox,
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

// type Message = String;
#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mail_box: &mut Mailbox, msg: Message){
        // to.mailBox.messages.push(msg);
        mail_box.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id, mail_box: Mailbox { messages: vec![] } }
    }
}

impl CubeSat {
    fn recv(&mut self, mail_box: &mut Mailbox) -> Option<Message> {
        // self.mailBox.messages.pop()
        mail_box.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message){
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message>{
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

fn main() {

    lifetime_refactoring();
}

fn lifetime_refactoring(){
    let mut mail_box = Mailbox {messages: vec![]};

    let base = GroundStation{};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let msg = Message { to: sat_id, content: String::from("hello")};
        base.send(&mut mail_box, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail_box);
        println!("{:?}: {:?}", sat, msg);
    }
}