pub trait Tuple{
    type T;
    fn get(self)->Self::T;
}

pub struct TupleEnd<T>{
    value:T
}

impl<T> Tuple for TupleEnd<T> {
    type T=T;
    fn get(self)->T {
        self.value
    }
}

impl<T> TupleEnd<T> {
    pub fn new(value:T)->Self{Self{value}}
    pub fn unwrap(self)->T {self.value}
}

pub struct TupleNode<T,TNext>
    //where TNext:Tuple
{
    value:T,
    next:TNext
}

impl<T,TNext/*: Tuple */> Tuple for TupleNode<T,TNext> {
    type T=T;

    fn get(self)->Self::T {
        self.value
    }
}

impl<T,TNext/*: Tuple */> TupleNode<T,TNext> {
    pub fn new(value:T,next:TNext)->Self{Self{value,next}}

    pub fn next(self)->TNext {
        self.next
    }

    pub fn unwrap(self)->(T,TNext) {
        (self.value,self.next)
    }
}

pub trait IntoTuple {
    type TOutput;
    fn into_tup(self)->Self::TOutput;
}

impl<'a,T> IntoTuple for &'a TupleEnd<T> {
    type TOutput = TupleEnd<&'a T>;
    fn into_tup(self)->Self::TOutput{TupleEnd::new(&self.value)}
}
impl<'a,T> IntoTuple for &'a mut TupleEnd<T> {
    type TOutput = TupleEnd<&'a mut T>;
    fn into_tup(self)->Self::TOutput{TupleEnd::new(&mut self.value)}
}

impl<'a,T,TNext/*:Tuple */> IntoTuple for &'a TupleNode<T,TNext>
    where &'a TNext:IntoTuple
{
    type TOutput=TupleNode<&'a T,<&'a TNext as IntoTuple>::TOutput>;
    fn into_tup(self)->Self::TOutput {
        TupleNode::new( &self.value,self.next.into_tup())
    }
}

impl<'a,T,TNext/*:Tuple */> IntoTuple for &'a mut TupleNode<T,TNext>
    where &'a mut TNext:IntoTuple
{
    type TOutput=TupleNode<&'a mut T,<&'a mut TNext as IntoTuple>::TOutput>;
    fn into_tup(self)->Self::TOutput {
        TupleNode::new( &mut self.value,self.next.into_tup())
    }
}
#[macro_export]
macro_rules! m_tup {
    ($v:expr) => {
        $crate::tuple::TupleEnd::new($v)
    };
    ($v:expr,$($then:expr),*)=>{
        $crate::tuple::TupleNode::new($v,m_tup!($($then),*))
    }
}

#[macro_export]
macro_rules! m_tup_t {
    ($v:ty) => {
        $crate::tuple::TupleEnd::<$v>
    };
    ($v:ty,$($then:ty),*)=>{
        $crate::tuple::TupleNode::<$v,m_tup_t!($($then),*)>
    }
}