

/// Fn(Input)->Output
/// 
/// You need to decide how the value to be transferred
/// 
/// You can try `impl Mapper for & YourType`
pub trait Mapper<Input>{
    type Output;
    fn map(&self,value:Input)->Self::Output;
}

/// Fn(Input)->Output is Mapper<Input>
/// 
/// due to this, rustc will say "no T:Mapper" as "expect Fn, found T"
impl<'a,Input,Output,TFn:FnMut(Input)->Output> Mapper<Input> for &'a mut TFn {
    type Output = Output;
    //type TOutput = TOut;
    fn map(&self,value:Input)->Self::Output {
        (*self)(value)
    }
}