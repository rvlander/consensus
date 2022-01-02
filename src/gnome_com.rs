pub struct Message {};

pub trait GnomeCom {
    pub say(msg: Message);
    pub listen();    
}