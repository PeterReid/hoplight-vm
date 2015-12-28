use std::cmp::{Eq, PartialEq};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Noun {
    ByteAtom(u8),
    Atom(Rc<Vec<u8>>),
    Cell(Rc<Noun>, Rc<Noun>),
}

impl PartialEq for Noun {
    fn eq(&self, other: &Noun) -> bool {
        match (self, other) {
            (&Noun::Cell(ref a, ref b), &Noun::Cell(ref x, ref y)) => a==x && b==y,
            (&Noun::ByteAtom(a), &Noun::ByteAtom(x)) => a==x,
            (&Noun::Atom(ref a), &Noun::Atom(ref x)) => a==x,
            (&Noun::ByteAtom(a), &Noun::Atom(ref x)) => x.len()==1 && a == x[0],
            (&Noun::Atom(ref a), &Noun::ByteAtom(x)) => a.len()==1 && a[0] == x,
            _ => false
        }
    }
}
impl Eq for Noun {}



impl Noun {
    pub fn new_cell(left: Noun, right: Noun) -> Noun {
        Noun::Cell(Rc::new(left), Rc::new(right))
    }
    
    pub fn from_bool(source: bool) -> Noun {
        Noun::ByteAtom(if source { 0 } else { 1 })
    }
    
    pub fn equal(&self, other: &Noun) -> Noun {
        Noun::from_bool(self == other)
    }
    
    pub fn as_byte(&self) -> Option<u8> {
        match self {
            &Noun::ByteAtom(x) => { Some(x) }
            &Noun::Atom(ref xs) => {
                if (&xs[1..]).iter().position(|x| *x!=0).is_some() {
                    return None;
                }
                
                Some(xs.get(0).map(|x| *x).unwrap_or(0))
            }
            _ => {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use as_noun::AsNoun;
    //use noun::Noun;

    #[test]
    fn eq() {
        assert_eq!((1, 2).as_noun(), (1, 2).as_noun());
    }
}