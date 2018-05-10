use std;


#[derive(Debug, PartialEq)]
pub enum Message {
    NODE {
        n_uid: NodeUID,
        p_uid: NodeUID,
        alt: i32,
        kids: i32,
        status: Status,
        label: Option<String>,
        nogood: Option<String>,
        info: Option<String>,
    },
    DONE,
    START {
        info: Option<String>,
        version: i32,
    },
    RESTART {
        info: Option<String>,
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeUID {
    nid: i32,
    rid: i32,
    tid: i32,
}

#[derive(Debug)]
pub struct Node {
    msg_type: Type,
    n_uid: Option<NodeUID>,
    p_uid: Option<NodeUID>,
    alt: Option<i32>,
    kids: Option<i32>,
    status: Option<Status>,
}

impl Node {
    pub fn get_type(self) -> Type {
        self.msg_type
    }
}

#[derive(Debug)]
pub enum Type {
    NODE = 0,
    DONE = 1,
    START = 2,
    RESTART = 3
}

#[derive(Debug, PartialEq)]
pub enum Status {
    ///< Node representing a solution
    SOLVED = 0,
    ///< Node representing failure
    FAILED = 1,
    ///< Node representing a branch
    BRANCH = 2,
    ///< Skipped by backjumping
    SKIPPED = 3,
}

#[derive(Debug)]
enum Field {
    LABEL = 0,
    NOGOOD = 1,
    INFO = 2,
    VERSION = 3
}

pub fn parse_from_bytes(buf: &Box<[u8]>) -> Result<Message, &'static str> {
    let mut buf = buf.into_iter();
    let msg_type = deserializeType(&mut buf);

    let mut n_uid: Option<NodeUID> = None;
    let mut p_uid: Option<NodeUID> = None;

    let mut alt: Option<i32> = None;
    let mut kids: Option<i32> = None;
    let mut status: Option<Status> = None;

    match msg_type {
        Type::NODE => {
            n_uid = Some(deserializeUID(&mut buf));
            p_uid = Some(deserializeUID(&mut buf));
            alt = Some(deserializeInt(&mut buf));
            kids = Some(deserializeInt(&mut buf));
            status = Some(deserializeStatus(&mut buf));
        }
        _ => {}
    }

    let mut rid: Option<i32> = None;
    let mut label: Option<String> = None;
    let mut nogood: Option<String> = None;
    let mut info: Option<String> = None;
    let mut version: Option<i32> = None;

    loop {

        // TODO(maxim): Figure out the Field first!!!
        if let Some(field) = deserializeField(&mut buf) {

            match field {
                Field::LABEL => {
                    label = Some(deserializeString(&mut buf));
                    // println!("label: {:?}", label);
                }
                Field::NOGOOD => {
                    // println!("NOGOOD");
                    nogood = Some(deserializeString(&mut buf));
                    // println!("nogood: {:?}", nogood);
                }
                Field::INFO => {
                    // println!("INFO");
                    info = Some(deserializeString(&mut buf));
                    // println!("info: {:?}", info);
                }
                Field::VERSION => {
                    version = Some(deserializeInt(&mut buf));
                }
                _ => {
                    // println!("unknown field");
                }
            }

        } else {
            break;
        }
    }

    let msg = match msg_type {
        Type::START => Message::START {version: version.unwrap(), info },
        Type::RESTART => Message::RESTART { info },
        Type::DONE => Message::DONE,
        Type::NODE => {

            Message::NODE {
                n_uid: n_uid.unwrap(),
                p_uid: p_uid.unwrap(),
                alt: alt.unwrap(),
                kids: kids.unwrap(),
                status: status.unwrap(),
                label,
                nogood,
                info,
            }
        }
    };

    Ok(msg)
}

fn deserializeUID(mut buf: &mut std::slice::Iter<u8>) -> NodeUID {
    let nid = deserializeInt(&mut buf);
    let rid = deserializeInt(&mut buf);
    let tid = deserializeInt(&mut buf);

    NodeUID{nid, rid, tid}
}

fn deserializeType(buf: &mut std::slice::Iter<u8>) -> Type {
    match *buf.next().unwrap() {
        0 => Type::NODE,
        1 => Type::DONE,
        2 => Type::START,
        3 => Type::RESTART,
        _ => panic!(),
    }
}

fn deserializeInt(buf: &mut std::slice::Iter<u8>) -> i32 {
    let b1 = *buf.next().unwrap() as i32;
    let b2 = *buf.next().unwrap() as i32;
    let b3 = *buf.next().unwrap() as i32;
    let b4 = *buf.next().unwrap() as i32;

    let res: i32 = b1 << 24 | b2 << 16 | b3 << 8 | b4;

    // println!("res: {:?}", res);
    res
}

fn deserializeStatus(buf: &mut std::slice::Iter<u8>) -> Status {
    match *buf.next().unwrap() {
        0 => Status::SOLVED,
        1 => Status::FAILED,
        2 => Status::BRANCH,
        3 => Status::SKIPPED,
        _ => panic!(),
    }
}

fn deserializeString(mut buf: &mut std::slice::Iter<u8>) -> String {
    let len = deserializeInt(&mut buf);

    let mut res = String::new();

    for i in 0..len {
        let c = *buf.next().unwrap() as char;
        res.push(c);
    }
    res
}

fn deserializeField(buf: &mut std::slice::Iter<u8>) -> Option<Field> {

    match buf.next() {
        Some(&0) => Some(Field::LABEL),
        Some(&1) => Some(Field::NOGOOD),
        Some(&2) => Some(Field::INFO),
        Some(&3) => Some(Field::VERSION),
        _ => None,
    }
}
