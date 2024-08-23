use crate::{collector::*, mapper::Mapper};

/// convert Fn(Input,Next)->Output into collector, with mapper T->T
pub struct FnCollector<TFn>(pub TFn);

impl<'a,T,TFn> Mapper<T> for &'a FnCollector<TFn> {
    type Output=T;
    fn map(self,value:T)->Self::Output {
        value
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for &'a FnCollector<TFn>
    where TFn:Fn(Input,Next)->Output
{
    type Output=Output;

    fn collect(self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output {
        (self.0)(value,next)
    }
}

/// convert FnMut(Input,Next)->Output into collector, with mapper T->T
pub struct FnMutCollector<TFn>(pub TFn);

impl<'a,T,TFn> Mapper<T> for &'a mut FnMutCollector<TFn> {
    type Output=T;
    fn map(self,value:T)->Self::Output {
        value
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for &'a mut FnMutCollector<TFn>
    where TFn:FnMut(Input,Next)->Output
{
    type Output=Output;

    fn collect(self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output {
        (self.0)(value,next)
    }
}
/*
use &MapCollector(ToString::to_string,&FnCollector( |a,b|{format!("{}, {}",a,b)} ))
pub struct CollectString<TFn:Fn(String,String)->String>(pub TFn);

impl<'a,Input:ToString,TFn:Fn(String,String)->String> Mapper<Input> for &'a CollectString<TFn> {
    type Output=String;
    
    fn map(self,value:Input)->Self::Output {
        value.to_string()
    }
    
}
impl<'a,Input:ToString,TFn:Fn(String,String)->String> Collector<Input,String> for &'a CollectString<TFn> {
    
    fn collect(self,value:Input,next:String)->String {
        (self.0)(value.to_string(),next)
    }
    type Output=String;
} */

/// combine a mapper and a collector together, use mapper to cover collector's value input
pub struct MapCollector<TMapper:Clone,TCollector:Clone>(pub TMapper,pub TCollector);
/*
impl<TMapper:Clone,TCollector:Clone> Clone for MapCollector<TMapper,TCollector> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
} */

impl<'a,TMapper,TCollector,Input> Mapper<Input> for &'a MapCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>+Clone,
    TCollector:Mapper< <TMapper as Mapper<Input>>::Output >+Clone
{
    type Output=< TCollector as Mapper< <TMapper as Mapper<Input>>::Output >>::Output;

    fn map(self,value:Input)->Self::Output {
        self.1.clone().map(self.0.clone().map(value))
    }
}

impl <'a,TMapper,TCollector,Input,TNext> Collector<Input,TNext> for &'a MapCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>+Clone,
    TCollector:Collector<<TMapper as Mapper<Input>>::Output,TNext>+Clone
{
    type Output= <TCollector as Collector<<TMapper as Mapper<Input>>::Output,TNext>>::Output ;

    fn collect(self,value:Input,next:TNext)-><Self as Collector<Input,TNext>>::Output {
        self.1.clone().collect(self.0.clone().map(value), next)
    }
}

/// () are implemented as Mapper, () -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Mapper<()> for () {
    type Output=();
    fn map(self,_value:())->Self::Output {()}
}

/// () are implemented as Collector, ((),()) -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Collector<(),()> for () {
    type Output=();
    fn collect(self,_value:(),_next:())->() {()}
}