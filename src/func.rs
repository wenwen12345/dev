pub trait Base64 {
    fn encode(&self);
    fn decode(&self);
}

pub trait Wttr {
    fn wttr(&self);
}

pub trait Run {
    fn run(&self);
}
