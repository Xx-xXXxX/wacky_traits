
use crate::tuple::Tuple;
use crate::tuple::TupleEnd;
use crate::tuple::TupleNode;
use wacky_traits::collector::Collectable;
use wacky_traits::collector::Collector;
use wacky_traits::mapper::Mapper;
/*
use common_traits::visitor::*;

impl<T,TVisitor> Visitable<TVisitor> for TupleEnd<T>
    where TVisitor:Visitor<T>
{
    type T=T;
    fn visited(self,visitor:TVisitor) {
        visitor.visit(self.get())
    }
}

impl<T,TNext,TVisitor> Visitable<TVisitor> for TupleNode<T,TNext>
    where 
    TVisitor:Visitor<T>,
    TNext:Visitable<TVisitor>,
    TVisitor:Visitor< <TNext as Visitable<TVisitor>>::T >,
    TVisitor:Clone
{
    type T=T;
    fn visited(self,visitor:TVisitor) {
        let (value,next)=self.unwrap();
        //let visitor2=visitor;
        visitor.clone().visit(value);
        next.visited(visitor);
    }
} */
#[derive(Clone)]
pub struct CollectAsTuple;

impl<TIn> Mapper<TIn> for CollectAsTuple {
    type Output=TupleEnd<TIn>;

    fn map(self,value:TIn)->Self::Output {
       TupleEnd::new(value)
    }
}

impl<TIn,TNext> Collector<TIn,TNext> for CollectAsTuple
    //where TNext:;
{
    type Output=TupleNode<TIn,TNext>;

    fn collect(self,value:TIn,next:TNext)-><Self as Collector<TIn,TNext>>::Output {
        TupleNode::new(value, next)
    }
}

impl<T,TCollector> Collectable<TCollector> for TupleEnd<T>
    where TCollector:Mapper<T>
{
    type Output=TCollector::Output;

    fn collected(self,collector:TCollector)->Self::Output {
        collector.map(self.get())
    }
}

impl<T,TNext,TCollector> Collectable<TCollector> for TupleNode<T,TNext>
    where TNext:Collectable<TCollector>,
    TCollector:Collector<T,<TNext as Collectable<TCollector>>::Output>+Clone
{
    type Output= <TCollector as Collector<T,<TNext as Collectable<TCollector>>::Output>>::Output ;

    fn collected(self,collector:TCollector)->Self::Output {
        let (value,next)=self.unwrap();
        collector.clone().collect(value,next.collected(collector))
    }
}
