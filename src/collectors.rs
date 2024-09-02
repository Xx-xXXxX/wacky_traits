use crate::{collector::*, mapper::Mapper};

/// convert Fn(Input,Next)->Output into collector, with mapper T->T
/// 
/// FnCollector is Collector
pub struct FnCollector<TFn>(pub TFn);

impl<'a,T,TFn> Mapper<T> for FnCollector<TFn> {
    type Output=T;
    fn map(self,value:T)->(Self::Output,Self) {
        (value,self)
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for FnCollector<TFn>
    where TFn:Fn(Input,Next)->Output
{
    type Output=Output;

    fn collect(self,value:Input,next:Next)->(<Self as Collector<Input,Next>>::Output,Self) {
        ((self.0)(value,next),self)
    }
}

/// convert FnMut(Input,Next)->Output into collector, with mapper T->T
/// 
/// &'a mut FnMutCollector is Collector
pub struct FnMutCollector<TFn>(pub TFn);

impl<'a,T,TFn> Mapper<T> for FnMutCollector<TFn> {
    type Output=T;
    fn map(self,value:T)->(Self::Output,Self) {
        (value,self)
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for FnMutCollector<TFn>
    where TFn:FnMut(Input,Next)->Output
{
    type Output=Output;

    fn collect(mut self,value:Input,next:Next)->(<Self as Collector<Input,Next>>::Output,Self) {
        ((self.0)(value,next),self)
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
/// 
/// &'a MapCollector is Collector
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

    fn map(self,value:Input)->(Self::Output,Self) {
        let (ma,mb)=(self.0,self.1);
        let (v1,ma2)=ma.map(value);
        let (v2,mb2)=mb.map(v1);
        return (v2,MapperCollector(ma2,mb2));
        //self.1.clone().map(self.0.clone().map(value))
    }
}

impl <'a,TMapper,TCollector,Input,TNext> Collector<Input,TNext> for MapperCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>,
    TCollector:Collector<<TMapper as Mapper<Input>>::Output,TNext>
{
    type Output= <TCollector as Collector<<TMapper as Mapper<Input>>::Output,TNext>>::Output ;

    fn collect(self,value:Input,next:TNext)->(<Self as Collector<Input,TNext>>::Output,Self) {
        //self.1.clone().collect(self.0.clone().map(value), next)
        let (m,c)=(self.0,self.1);
        let (v1,m2)=m.map(value);
        let (r,c2)=c.collect(v1, next);
        return (r,MapperCollector(m2,c2));
    }
}

/// () are implemented as Mapper, () -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Mapper<()> for () {
    type Output=();
    fn map(self,_value:())->(Self::Output,()) {((),())}
}

/// () are implemented as Collector, ((),()) -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Collector<(),()> for () {
    type Output=();
    fn collect(self,_value:(),_next:())->((),()) {((),())}
}