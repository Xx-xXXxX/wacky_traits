
use wacky_traits::{collector::Collectable, collectors::*};

//use crate::tuple::{TupleEnd, TupleNode};
use crate::{m_tup, m_tup_t, patterns, tuple::IntoTuple};

fn bind_string_with_comma(a:String,b:String)->String{format!("{}, {}",a,b)}
fn doprint<T:ToString>(v:T){println!("{}",v.to_string())}

//pub fn DoToString<T:ToString>(a:&T)->String{a.to_string()}

type testT=m_tup_t!(i32,i32,i32);
#[test]
fn testaaaa(){
    //let tup=TupleNode::new(1,TupleNode::new(2,TupleEnd::new(3)));
    let tup:testT=m_tup!(1,2,3);
    //let tupref:TupleNode<&i32,TupleNode<&i32,TupleEnd<&i32>>>=(&tup).into();
    let tupleref2=(&tup).into_tup();
    /*
    let mut  bindstr=BindString::new(|a:&mut String,b|{a.push_str(&b);});
    tupleref2.visited(&mut bindstr);
    println!("{}",bindstr.get()); */

    //let collectResult=tupleref2.collected(&CollectString(bind_string_with_comma));
    //println!("{}",collectResult);

    let tostrTuple=(&tup).into_tup().collected(&MapCollector( ToString::to_string ,patterns::CollectAsTuple));

    //let collectResult2=(&tostrTuple).into_tup().collected(&CollectString(bind_string_with_comma));
    //println!("2: {}",collectResult2);

    (&tup).into_tup().collected(&MapCollector(doprint,()));

}

