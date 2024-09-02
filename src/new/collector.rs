
/*
/// to help Collector and get result
/// 
/// use collector to iterate values in it
pub trait Collectable<TCollector> {
    type Output;
    fn collected(self,collector:TCollector)->(Self::Output,TCollector);
}
 */


/*
impl<Input,Next,Output,TFn:Fn(Input,Next)->Output> Collector<Input,Next> for TFn {
    type Output=Output;

    fn collect(self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output {
        
    }
} */
/*
pub trait Collector<Input,Next> {
    type Output;
    type NextCollector;
    fn unwrap(self)->(Self::NextCollector,impl FnOnce(Input,Next)->Self::Output);
    //fn get_next_collector(&self)->Self::NextCollector;
    //fn collect(&self,value:Input,next:Next)->Self::Output;
}*/

pub trait CollectableNode {
    type Output;
    type NextCollectable:CollectableNode;
    fn unwrap(self)->(Self::Output,Self::NextCollectable);
}

pub trait CollectableEnd {
    type Output;
    fn unwrap(self)->Self::Output;
}

pub trait Collector<Input,NextCollectable> {
    type Output;
    fn collect(self,value:Input,next:NextCollectable)->Self::Output;
}
pub trait CollectorEnd<Input> {
    type Output;
    fn collect(self,value:Input)->Self::Output;
}