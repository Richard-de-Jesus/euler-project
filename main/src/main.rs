
use std::env;



fn call_fn(arg: &str) {

    match arg {
        "1" => _1(), 
        "2" => _2(), 
        "3" => _3(), 
        "4" => _4(), 
        "5" => _5(), 
        "6" => _6(), 
        "7" => _7(), 
        "8" => _8(), 
        "9" => _9(), 
        "10" => _10(), 
        "11" => _11(), 
        "12" => _12(),
        "13" => _13(), 
        "14" => _14(), 
        "15" => _15(), 
        "16" => _16(), 
        "17" => _17(), 
        "18" => _18(), 
        "19" => _19(), 
        "20" => _20(), 
        "21" => _21(), 
        "22" => _22(), 
        "23" => _23(), 
        "24" => _24(), 
        "25" => _25(), 
        "26" => _26(), 
        //"27" => _27(), 
        //"28" => _28(), 
        //"29" => _29(), 
        //"30" => _30(), 
        //"31" => _31(), 
        //"32" => _32(), 
        //"33" => _33(), 
        //"34" => _34(), 
        //"35" => _35(), 
        //"36" => _36(), 
        //"37" => _37(), 
        //"38" => _38(), 
        //"39" => _39(), 
        //"40" => _40(), 
        //"41" => _41(), 
        //"42" => _42(), 
        //"43" => _43(), 
        //"44" => _44(), 
        //"45" => _45(), 
        //"46" => _46(), 
        //"47" => _47(), 
        _ => println!("unknow function"),
    };
} 


use problem1_9::*;
use problem10_19::*;
use problem20_29::*;
fn main() {

    let args: Vec<String> = env::args()
        .collect();

    call_fn(&args[1]);

    /*_1();
    _2();
    _3();
    _4();
    _5();
    _6();
    _7();
    _8();
    _9();
    _10();
    _11();
    _12();
    _13();
    _14();
    _15();
    _16();
    _17();
    _18();
    _19(); 
    _20(); 
    _21();
    _22();*/
    //_23();
}
