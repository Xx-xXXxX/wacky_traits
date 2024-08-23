use crate::mapper::Mapper;

/// |x|{x}
#[derive(Default,Clone, Copy)]
pub struct AsSelf;
impl<T> Mapper<T> for AsSelf {
    type Output = T;
    fn map(self,value:T)->Self::Output {value}
}
/// combine 2 map
pub struct MapperMapper<TMapperA,TMapperB>(pub TMapperA,pub TMapperB);

impl<'a,TMapperA,TMapperB,Input> Mapper<Input> for &'a MapperMapper<TMapperA,TMapperB>
    where TMapperA:Mapper<Input>+Clone,
    TMapperB:Mapper< <TMapperA as Mapper<Input>>::Output >+Clone
{
    type Output=<TMapperB as Mapper< <TMapperA as Mapper<Input>>::Output >>::Output;

    fn map(self,value:Input)->Self::Output {
        self.1.clone().map(self.0.clone().map(value))
    }
}
/*
impl<T> FnOnce(T)->T for AsSelf {
    type Output=T;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        args
    }
}

impl<T> FnMut(T)->T for AsSelf {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        todo!()
    }
}

impl<T> Fn(T)->T for AsSelf {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        todo!()
    }
}
 */
/*
#[derive(Default)]
pub struct AsSelf;
impl<T:Tuple> Fn(T) for AsSelf {
    type Output=T;
    extern "rust-call" fn call(&self, args: T) -> T {
        args
    }
}
impl<T> Mapper<T> for AsSelf {
    type Output = T;
    fn map(&self,value:T)->Self::Output {
        value
    }
}
 */
/*
#[derive(Clone, Copy)]
pub struct VisitPrint;
impl<T:ToString> Mapper<T> for VisitPrint {
   type Output = ();
    fn map(self,value:T) {
        println!("{}",value.to_string())
    }
}
     */