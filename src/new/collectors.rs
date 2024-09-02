use crate::new::{collector::*, mapper::Mapper};

/// convert Fn(Input,Next)->Output into collector, with mapper T->T
pub struct FnCollector<TFn>(pub TFn);
impl<'a,T,TFn> Mapper<T> for &'a FnCollector<TFn> {
    type Output=T;
    fn map(&self,value:T)->Self::Output {
        value
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for &'a FnCollector<TFn>
    where TFn:Fn(Input,Next)->Output
{
    type Output=Output;
    
    type NextCollector=&'a FnCollector<TFn>;
    
    fn unwrap(self)->(Self::NextCollector,impl FnOnce(Input,Next)->Self::Output) {
        (self,& self.0)
    }
    
}

/// convert FnMut(Input,Next)->Output into collector, with mapper T->T
pub struct FnMutCollector<'a,TFn>(pub TFn);

impl<'a,T,TFn> Mapper<T> for FnMutCollector<'a,TFn> {
    type Output=T;
    fn map(& self,value:T)->Self::Output {
        value
    }
}
impl<'a,Input,Next,Output,TFn> Collector<Input,Next> for FnMutCollector<'a,TFn>
    where TFn:Fn(Input,Next)->Output
{
    type Output=Output;
    
    type NextCollector=FnMutCollector<'a,TFn>;
    
    fn unwrap(self)->(Self::NextCollector,impl FnOnce(Input,Next)->Self::Output) {
        let aref=&self.0;
        (self,aref)
    }
    
}

/// combine a mapper and a collector together, use mapper to cover collector's value input
pub struct MapperCollector<'a,TMapper,TCollector>(pub &'a TMapper,pub TCollector);
/*
impl<TMapper:Clone,TCollector:Clone> Clone for MapCollector<TMapper,TCollector> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
} */

impl<TMapper,TCollector,Input> Mapper<Input> for MapperCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>,
    TCollector:Mapper< <TMapper as Mapper<Input>>::Output >
{
    type Output=< TCollector as Mapper< <TMapper as Mapper<Input>>::Output >>::Output;

    fn map(&self,value:Input)->Self::Output {
        let (ma,mb)=(self.0,self.1);
        let v1=ma.map(value);
        let v2=mb.map(v1);
        return v2;
        //self.1.clone().map(self.0.clone().map(value))
    }
}

impl <TMapper,TCollector,Input,TNext> Collector<Input,TNext> for MapperCollector<TMapper,TCollector>
    where TMapper:Mapper<Input>,
    TCollector:Collector<<TMapper as Mapper<Input>>::Output,TNext>
{
    type Output= <TCollector as Collector<<TMapper as Mapper<Input>>::Output,TNext>>::Output ;
    /*
    fn collect(self,value:Input,next:TNext)->(<Self as OldCollector<Input,TNext>>::Output,Self) {
        //self.1.clone().collect(self.0.clone().map(value), next)
        let (m,c)=(self.0,self.1);
        let (v1,m2)=m.map(value);
        let (r,c2)=c.collect(v1, next);
        return (r,MapperCollector(m2,c2));
    } */
    
    type NextCollector=MapperCollector<TMapper,TCollector::NextCollector>;
    
    fn unwrap(self)->(Self::NextCollector,impl FnOnce(Input,TNext)->Self::Output) {
        let (nc,f)=self.1.unwrap();
        let refm=&self.0;
        (
            MapperCollector(self.0,nc),
            |value:Input,next:TNext|{
                let v_m=refm.map(value);
                let v_r=f(v_m,next);
                return v_r;
            }
        )
    }
}

/// () are implemented as Mapper, () -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Mapper<()> for () {
    type Output=();
    fn map(&self,_value:())->(){()}
}

/// () are implemented as Collector, ((),()) -> ()
/// 
/// to support Mapper< Output=() >, visitor
impl Collector<(),()> for () {
    type Output=();
    
    type NextCollector=();
    
    fn unwrap(self)->(Self::NextCollector,impl FnOnce((),())->Self::Output) {
        ((),|a,b|{()})
    }
}