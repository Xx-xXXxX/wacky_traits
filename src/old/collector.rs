use crate::old::mapper::Mapper;
/// visit Collectable and get result
/// 
/// Mapper may used when reach the end of iterating when there no next should be given
pub trait Collector<Input,Next>:Mapper<Input>{
    type Output;
    fn collect(&self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output;
}

/// to help Collector and get result
/// 
/// use collector to iterate values in it
pub trait Collectable<TCollector> {
    type Output;
    fn collected(self,collector:&TCollector)->Self::Output;
}



/*
impl<Input,Next,Output,TFn:Fn(Input,Next)->Output> Collector<Input,Next> for TFn {
    type Output=Output;

    fn collect(self,value:Input,next:Next)-><Self as Collector<Input,Next>>::Output {
        
    }
} */
