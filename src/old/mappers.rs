use crate::old::mapper::Mapper;

/// |x|{x}
#[derive(Default,Clone, Copy)]
pub struct AsSelf;

impl<T> Mapper<T> for AsSelf {
    type Output = T;
    fn map(&self,value:T)->Self::Output {value}
} 
/// combine 2 map
pub struct MapperMapper<'a,TMapperA,TMapperB>(pub &'a TMapperA,pub &'a TMapperB);


impl<'a,TMapperA,TMapperB,Input> Mapper<Input> for MapperMapper<'a,TMapperA,TMapperB>
    where TMapperA:Mapper<Input>,
    TMapperB:Mapper< <TMapperA as Mapper<Input>>::Output >
{
    type Output=<TMapperB as Mapper< <TMapperA as Mapper<Input>>::Output >>::Output;

    fn map(&self,value:Input)->Self::Output {
        /*
        let (ma,mb)=(self.0,self.1);
        let (v1,ma2)=ma.map(value);
        let (v2,mb2)=mb.map(v1); */
        self.1.map(self.0.map(value))
    }
} 