use crate::mapper::Mapper;

/// |x|{x}
#[derive(Default,Clone, Copy)]
pub struct AsSelf;
impl<T> Mapper<T> for AsSelf {
    type Output = T;
    fn map(self,value:T)->(Self::Output,AsSelf) {(value,AsSelf)}
}
/// combine 2 map
pub struct MapperMapper<TMapperA,TMapperB>(pub TMapperA,pub TMapperB);

impl<'a,TMapperA,TMapperB,Input> Mapper<Input> for MapperMapper<TMapperA,TMapperB>
    where TMapperA:Mapper<Input>,
    TMapperB:Mapper< <TMapperA as Mapper<Input>>::Output >
{
    type Output=<TMapperB as Mapper< <TMapperA as Mapper<Input>>::Output >>::Output;

    fn map(self,value:Input)->(Self::Output,Self) {

        let (ma,mb)=(self.0,self.1);
        let (v1,ma2)=ma.map(value);
        let (v2,mb2)=mb.map(v1);
        return (v2,MapperMapper(ma2,mb2));
        //self.1.clone().map(self.0.clone().map(value))
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