
pub trait Mapper<Input> {
    type Output;
    fn map(&self,value:Input)->Self::Output;
}

impl<Input,Output,TFn:Fn(Input)->Output> Mapper<Input> for TFn {
    type Output = Output;
    //type TOutput = TOut;
    fn map(&self,value:Input)->Self::Output {
        self(value)
    }
}