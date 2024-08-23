

/// Fn(Input)->Output
/// 
/// note that map "borrows" the value, normally you need to implement Clone also
/// 
/// or, you can `impl Mapper for &'a YourType`
pub trait Mapper<Input>{
    type Output;
    fn map(self,value:Input)->Self::Output;
}

/// Fn(Input)->Output is Mapper<Input>
/// 
/// due to this, rustc will say "no T:Mapper" as "expect Fn, found T"
impl<Input,Output,TFn:Fn(Input)->Output> Mapper<Input> for TFn {
    type Output = Output;
    //type TOutput = TOut;
    fn map(self,value:Input)->Self::Output {
        self(value)
    }
}


/*
use MapCollector as collectable
pub trait Mappable<TMapper>
    where TMapper:Mapper<Self::TThis>
{
    type TThis;
    type TOutput;
    fn map(self,mapper:&TMapper)->Self::TOutput;
}
*/
