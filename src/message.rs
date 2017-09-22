use std;


#[derive(Debug, PartialEq)]
pub enum Message {
    START {
        rid: Option<i32>,
        name: Option<String>,
    },
    DONE,
    NODE {
        sid: i32,
        pid: i32,
        alt: i32,
        kids: i32,
        status: Status,
        rid: Option<i32>,
        tid: Option<i32>,
        label: Option<String>,
        solution: Option<String>,
        nogood: Option<String>,
        info: Option<String>,
    },
}

#[derive(Debug)]
pub struct Node {
    msg_type: Type,
    sid: Option<i32>,
    pid: Option<i32>,
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
}

#[derive(Debug, PartialEq)]
enum Status {
    SOLVED = 0,
    ///< Node representing a solution
    FAILED = 1,
    ///< Node representing failure
    BRANCH = 2,
    ///< Node representing a branch
    UNDETERMINED = 3,
    ///< Node that has not been explored yet
    STOP = 4,
    ///< Node representing stop point
    UNSTOP = 5,
    ///< Node representing ignored stop point
    SKIPPED = 6,
    ///< Skipped by backjumping
    MERGING = 7,
}

#[derive(Debug)]
enum Field {
    ID = 0,
    PID = 1,
    ALT = 2,
    KIDS = 3,
    STATUS = 4,
    RESTART_ID = 5,
    THREAD_ID = 6,
    LABEL = 7,
    SOLUTION = 8,
    NOGOOD = 9,
    INFO = 10,
}

pub fn parse_from_bytes(buf: &Box<[u8]>) -> Result<Message, &'static str> {
    let mut buf = buf.into_iter();
    let msg_type = deserializeType(&mut buf);

    // println!("msg type: {:?}", msg_type);

    let mut sid: Option<i32> = None;
    let mut pid: Option<i32> = None;
    let mut alt: Option<i32> = None;
    let mut kids: Option<i32> = None;
    let mut status: Option<Status> = None;

    match msg_type {
        Type::NODE => {
            sid = Some(deserializeInt(&mut buf));
            pid = Some(deserializeInt(&mut buf));
            alt = Some(deserializeInt(&mut buf));
            kids = Some(deserializeInt(&mut buf));
            status = Some(deserializeStatus(&mut buf));
        }
        _ => {}
    }

    let mut rid: Option<i32> = None;
    let mut tid: Option<i32> = None;
    let mut label: Option<String> = None;
    let mut solution: Option<String> = None;
    let mut nogood: Option<String> = None;
    let mut info: Option<String> = None;

    loop {

        // TODO(maxim): Figure out the Field first!!!
        if let Some(field) = deserializeField(&mut buf) {

            match field {
                Field::RESTART_ID => {
                    // println!("RESTART_ID");
                    rid = Some(deserializeInt(&mut buf));
                    // println!("rid: {:?}", rid);
                }
                Field::THREAD_ID => {
                    // println!("THREAD_ID");
                    tid = Some(deserializeInt(&mut buf));
                    // println!("tid: {:?}", tid);
                }
                Field::LABEL => {
                    // println!("LABEL");
                    label = Some(deserializeString(&mut buf));
                    // println!("label: {:?}", label);
                }
                Field::SOLUTION => {
                    // println!("SOLUTION");
                    solution = Some(deserializeString(&mut buf));
                    // println!("solution: {:?}", solution);
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
                _ => {
                    // println!("unknown field");
                }
            }

        } else {
            break;
        }
    }

    let msg = match msg_type {
        Type::START => Message::START { rid, name: label },
        Type::DONE => Message::DONE,
        Type::NODE => {

            Message::NODE {
                sid: sid.unwrap(),
                pid: pid.unwrap(),
                alt: alt.unwrap(),
                kids: kids.unwrap(),
                status: status.unwrap(),
                rid,
                tid,
                label,
                solution,
                nogood,
                info,
            }
        }
    };

    Ok(msg)
}

fn deserializeType(buf: &mut std::slice::Iter<u8>) -> Type {
    match *buf.next().unwrap() {
        0 => Type::NODE,
        1 => Type::DONE,
        2 => Type::START,
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
        6 => Status::SKIPPED,
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
        Some(&0) => Some(Field::ID),
        Some(&1) => Some(Field::PID),
        Some(&2) => Some(Field::ALT),
        Some(&3) => Some(Field::KIDS),
        Some(&4) => Some(Field::STATUS),
        Some(&5) => Some(Field::RESTART_ID),
        Some(&6) => Some(Field::THREAD_ID),
        Some(&7) => Some(Field::LABEL),
        Some(&8) => Some(Field::SOLUTION),
        Some(&9) => Some(Field::NOGOOD),
        Some(&10) => Some(Field::INFO),
        _ => None,
    }
}
