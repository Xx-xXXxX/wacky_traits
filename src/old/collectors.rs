use crate::old::{collector::*, mapper::Mapper};

/// convert Fn(Input,Next)->Output into collector, with mapper T->T
pub struct FnCollector<'a,TFn>(pub &'a TFn);
impl<'a,T,TFn> Mapper<T> for FnCollector<'a,TFn> {
    type Output=T;
    fn map(&self,value:T)->Self::Output {
        value
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for FnCollector<'a,TFn>
    where TFn:Fn(Input,Next)->Output
{
    type Output=Output;

    fn collect(&self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output {
        (self.0)(value,next)
    }
}

/// convert FnMut(Input,Next)->Output into collector, with mapper T->T
pub struct FnMutCollector<TFn>(pub TFn);

impl<'a,T,TFn> Mapper<T> for FnMutCollector<TFn> {
    type Output=T;
    fn map(&self,value:T)->Self::Output {
        value
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for FnMutCollector<TFn>
    where TFn:FnMut(Input,Next)->Output
{
    type Output=Output;

    fn collect(&self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output {
        (self.0)(value,next)
    }
}
/// combine a mapper and a collector together, use mapper to cover collector's value input
pub struct MapperCollector<TMapper,TCollector>(pub TMapper,pub TCollector);
/*
impl<TMapper:Clone,TCollector:Clone> Clone for MapCollector<TMapper,TCollector> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
} */

impl<'a,TMapper,TCollector,Input> Mapper<Input> for MapperCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>,
    TCollector:Mapper< <TMapper as Mapper<Input>>::Output >
{
    type Output=< TCollector as Mapper< <TMapper as Mapper<Input>>::Output >>::Output;

    fn map(&self,value:Input)->Self::Output {
        self.1.map(self.0.map(value))
    }
}

impl <'a,TMapper,TCollector,Input,TNext> Collector<Input,TNext> for MapperCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>,
    TCollector:Collector<<TMapper as Mapper<Input>>::Output,TNext>
{
    type Output= <TCollector as Collector<<TMapper as Mapper<Input>>::Output,TNext>>::Output ;

    fn collect(&self,value:Input,next:TNext)-><Self as Collector<Input,TNext>>::Output {
        //self.1.clone().collect(self.0.clone().map(value), next)
        let (m,c)=(self.0,self.1);
        let v1=m.map(value);
        let r=c.collect(v1, next);
        return r;
    }
}

/// () are implemented as Mapper, () -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Mapper<()> for () {
    type Output=();
    fn map(&self,_value:())->() {()}
}

/// () are implemented as Collector, ((),()) -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Collector<(),()> for () {
    type Output=();
    fn collect(&self,_value:(),_next:())->(){()}
}