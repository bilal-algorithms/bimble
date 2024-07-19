use clearscreen::clear;
use colored::Colorize;
use regex::Regex;
use std::{
    env::args,
    fs::{self, remove_dir, remove_file, DirBuilder, File},
    io::{Read, Write},
    path::Path,
    process::exit,
    thread::sleep,
    time::Duration,
};

#[derive(Clone, Debug)]
struct Varr {
    name: String,
    vtype: Vartypes,
    vval: String,
}

#[derive(Debug, Clone)]
struct CFG {
    name: String,
    date: String,
    auth: String,
}

trait Gen {
    fn gen(c: CFG) -> String {
        let dat = format!("{}|{}|{}", c.name, c.date, c.auth);
        return dat;
    }
}
impl Gen for CFG {}

#[derive(Debug, Clone, PartialEq)]
enum Vartypes {
    String,
    Fsf,
    I,
}

trait Dis {
    fn dis(v: Varr) {
        println!(
            "{}",
            format!(
                "var_name : {} : var_type : {:?} : var_val : {} :",
                v.name, v.vtype, v.vval
            )
            .green()
        );
    }
}

impl Dis for Varr {}

#[allow(path_statements)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() {
    let mut vrs: Vec<Varr> = Vec::new();
    let mut undefined_fn_calls: Vec<String> = Vec::new();
    clear().unwrap();
    let mut isinfn = false;
    let pf: Vec<String> = args().collect();
    let mut pfci = 0;

    if pf.len() <= 1 {
        println!(
            "{}",
            "ERROR - NEED AT LEAST 1 PROJECT FOLDER TO COMPILE!!".red()
        );
        return;
    }

    let mut fns: Vec<String> = Vec::new();

    for project_folder in pf.iter().skip(1) {
        match File::open(format!("{}/main.bb", project_folder)) {
            Ok(mut mf) => {
                println!("{}{:?}", "found main file!! -- ".green(), mf);
                sleep(Duration::from_millis(500));
                let mut wc = String::new();
                match mf.read_to_string(&mut wc) {
                    Ok(_) => {
                        let nlsepcode = wc.split('\n');
                        for line in nlsepcode.clone() {
                            if line.starts_with("ON") && !isinfn {
                                println!("{}", "Handling function declaration".blue());

                                let funcdeclarerg = Regex::new(r"ON\s+(\w+)\(\)\{").unwrap();
                                if let Some(cap) = funcdeclarerg.captures(line) {
                                    if let Some(funcnm) = cap.get(1) {
                                        fns.push(funcnm.as_str().to_string());
                                        println!(
                                            "{}{}",
                                            "Function declared: ".cyan(),
                                            funcnm.as_str().cyan()
                                        );
                                    } else {
                                        println!(
                                            "{}{}",
                                            "ERROR - Could not capture function name in line: "
                                                .red(),
                                            line.red()
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Function Declare using wrong syntax: ".red(),
                                        line.red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                                isinfn = !line.ends_with("}");
                            } else if line.starts_with("ON") && isinfn {
                                println!(
                                    "{}{}",
                                    "Cannot declare functions inside other functions! - ".red(),
                                    line.red()
                                );
                            } else if line.trim() == "}" {
                                isinfn = false;
                            } else if line.trim().starts_with("takein") {
                                println!("{}", "Handeling takein()..".green());
                                match Regex::new(r#"takein\((.*?)\);"#) {
                                    Ok(tirg) => {
                                        if let Some(cap) = tirg.captures(line.trim()) {
                                            let tkvr = cap.get(1).unwrap().as_str();
                                            let mut vb = false;
                                            for vr in vrs.clone() {
                                                if vr.name == tkvr {
                                                    if vr.vtype == Vartypes::String {
                                                        vb = true;
                                                    } else {
                                                        println!(
                                                            "{} {}",
                                                            "Variable isnt of string type : ".red(),
                                                            tkvr.red()
                                                        );
                                                        println!(
                                                            "{}{}",
                                                            "in 'takein()' - ".red(),
                                                            line.trim().red()
                                                        );
                                                        exit(0);
                                                    }
                                                }
                                            }
                                            if vb {
                                                continue;
                                            } else {
                                                println!(
                                                    "{} {}",
                                                    "Variable doesnt exists : ".red(),
                                                    tkvr.red()
                                                );
                                                println!(
                                                    "{}{}",
                                                    "in 'takein()' - ".red(),
                                                    line.trim().red()
                                                );
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        println!(
                                            "{} {}",
                                            "Unable to create 'takein()' regex err - ".red(),
                                            err.to_string().red()
                                        );
                                    }
                                }
                            } else if line.trim().starts_with("if") {
                                let ifrg =
                                    Regex::new(r#"if\s*\[(.*?)\]\s*=>\s*(.*?);\s*"#).unwrap();
                                if let Some(cap) = ifrg.captures(line.trim()) {
                                    if cap.len() != 3 {
                                        if cap.get(1).is_none()
                                            || cap.get(1).unwrap().as_str().trim().is_empty()
                                        {
                                            println!("{} {}", "ERR - The Conditional Statement is missing a condition:".red().bold(), line.trim().red().bold());
                                            std::process::exit(1);
                                        } else if cap.get(2).is_none()
                                            || cap.get(2).unwrap().as_str().trim().is_empty()
                                        {
                                            println!("{} {}", "ERR - The Conditional Statement is missing a method call:".red().bold(), line.trim().red().bold());
                                            std::process::exit(1);
                                        } else {
                                            continue;
                                        }
                                    }

                                    // Extract condition and method call
                                    //println!("cap -> {:?}",cap);
                                    let cnd = cap.get(1).unwrap().as_str().trim();
                                    let mthd = cap.get(2).unwrap().as_str().trim();

                                    println!("Condition -> {}, Method Call -> {}", cnd, mthd);
                                    let mthdd = mthd.trim_end_matches("()");
                                   // let mut ex = false;
                                       println!("fns : {:?} || mthdd : {} || mthd : {}",fns.clone(),mthdd,mthd);
                                    // for i in fns.clone() {
                                    //    if i == mthdd {
                                    //        ex = true;
                                    //    }
                                   // }
                                    //if !ex{
                                        undefined_fn_calls.push(format!("{}();",mthdd));
                                    //}
                                    // Process condition characters
                                    let mut last_char: Option<char> = None;
                                    let cnds: Vec<char> = cnd.chars().collect();
                                    for (index, cnd_char) in cnds.iter().enumerate() {
                                        println!("cnd - {}", cnd_char);
                                        let cnd_str = cnd_char.to_string();

                                        // Check if the character represents a number, string, or empty
                                        if cnd_str.is_empty() {
                                            continue;
                                        } else if cnd_str.parse::<f64>().is_ok() {
                                            continue;
                                        } else if cnd_str.parse::<i128>().is_ok() {
                                            continue;
                                        } else if cnd_str.starts_with('"') && cnd_str.ends_with('"')
                                        {
                                            continue;
                                        }

                                        // Check for operators and valid sequences
                                        if let Some(last) = last_char {
                                            match (last, cnd_char) {
                                                ('=', '=') => {
                                                    println!(
                                                        "{} {}",
                                                        "ERR - Consecutive '=' operators:"
                                                            .red()
                                                            .bold(),
                                                        line.trim().red().bold()
                                                    );
                                                    std::process::exit(1);
                                                }
                                                ('&', '&') => {
                                                    println!(
                                                        "{} {}",
                                                        "ERR - Consecutive '&&' operators:"
                                                            .red()
                                                            .bold(),
                                                        line.trim().red().bold()
                                                    );
                                                    std::process::exit(1);
                                                }
                                                ('>', next) => {
                                                    // let next = *next
                                                    if *next == '='
                                                        || next.is_numeric()
                                                        || next.to_string().is_empty()
                                                        || next.to_string().trim().is_empty()
                                                    {
                                                        // Valid continuation
                                                    } else {
                                                        println!("{} {} {} {}", "ERR - Invalid continuation after '>' operator : err (".red().bold(), next.to_string().red().bold() , ") :- line -> ",line.trim().red().bold());
                                                        std::process::exit(1);
                                                    }
                                                }
                                                // Add more cases as needed
                                                _ => {}
                                            }
                                        }

                                        last_char = Some(*cnd_char);
                                    }

                                    println!("mthd - {}", mthd);
                                } else {
                                    println!(
                                        "{} {}",
                                        "ERR - Conditional Statement syntax is wrong:".red().bold(),
                                        line.trim().red().bold()
                                    );
                                    std::process::exit(1);
                                }
                            } else if line.trim().starts_with("add") {
                                println!("{}", "Handling addition method".green());
                                //let mut fval = String::new();
                                match Regex::new(r"add\((.*?)\);") {
                                    Ok(addrg) => {
                                        //println!("add regex - {:?}", addrg);
                                        if let Some(cap) = addrg.captures(line.trim()) {
                                            match cap.get(1) {
                                                Some(dat) => {
                                                    println!(
                                                        "'add()' method got data: {}",
                                                        dat.as_str()
                                                    );
                                                    let dat = dat.as_str();
                                                    let sepdat = dat.split(":");
                                                    let dati = 0;
                                                    // let mut ex = false;
                                                    for args in sepdat {
                                                        if dati == 0 {
                                                            let exprs = args.split(",");
                                                            for expr in exprs {
                                                                let mut ex = false; // Reset ex for each expr
                                                                                    // dbg!(expr);
                                                                if expr.parse::<i128>().is_ok() {
                                                                    ex = true;
                                                                    // println!(
                                                                    //     "matched i128 in expt : {}",
                                                                    //     expr
                                                                    // );
                                                                } else if expr
                                                                    .parse::<f64>()
                                                                    .is_ok()
                                                                {
                                                                    ex = true;
                                                                    // println!(
                                                                    //     "matched f64 in expt : {}",
                                                                    //     expr
                                                                    // );
                                                                } else if expr.starts_with("\"")
                                                                    && expr.ends_with("\"")
                                                                {
                                                                    ex = true;
                                                                    // println!(
                                                                    //     "matched txt in expt : {}",
                                                                    //     expr
                                                                    // );
                                                                } else {
                                                                    for i in vrs.clone() {
                                                                        if i.name == expr {
                                                                            ex = true;
                                                                            // println!("matched var in expt : {}", expr);
                                                                            break;
                                                                            // Exit the loop once a match is found
                                                                        }
                                                                    }
                                                                }
                                                                if !ex {
                                                                    println!(
                                                                        "{}{}{}{}{}",
                                                                        "ERR - THE ARGUMENT : "
                                                                            .red(),
                                                                        expr.red(),
                                                                        " : is invalid at line : "
                                                                            .red(),
                                                                        line.trim().red(),
                                                                        " :".red()
                                                                    );
                                                                    exit(0);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                None => {
                                                    println!("ERR - The 'add()' method has invalid amount of arguments.");
                                                    exit(0);
                                                }
                                            }
                                        } else {
                                            println!("{}{}","Invalid amount of arguments found in 'add()' method - ".red(),line.trim().red());
                                            exit(0);
                                        }
                                    }
                                    Err(err) => {
                                        println!(
                                            "{} {}",
                                            "Unable to make regex pattern for add method - ".red(),
                                            err.to_string().red()
                                        );
                                    }
                                }
                            } else if line.trim().starts_with("sub") {
                                println!("{}", "Handling substraction method".green());
                                //let mut fval = String::new();
                                match Regex::new(r"sub\((.*?)\);") {
                                    Ok(addrg) => {
                                        println!("sub regex - {:?}", addrg);
                                        if let Some(cap) = addrg.captures(line.trim()) {
                                            match cap.get(1) {
                                                Some(dat) => {
                                                    println!(
                                                        "'sub()' method got data: {}",
                                                        dat.as_str()
                                                    );
                                                    let dat = dat.as_str();
                                                    let sepdat = dat.split(":");
                                                    let dati = 0;
                                                    // let mut ex = false;
                                                    for args in sepdat {
                                                        if dati == 0 {
                                                            let exprs = args.split(",");
                                                            for expr in exprs {
                                                                let mut ex = false; // Reset ex for each expr
                                                                                    //dbg!(expr);
                                                                if expr.parse::<i128>().is_ok() {
                                                                    ex = true;
                                                                    //println!(
                                                                    //    "matched i128 in expt : {}",
                                                                    //    expr
                                                                    //);
                                                                } else if expr
                                                                    .parse::<f64>()
                                                                    .is_ok()
                                                                {
                                                                    ex = true;
                                                                    //println!(
                                                                    //     "matched f64 in expt : {}",
                                                                    //     expr
                                                                    // );
                                                                } else if expr.starts_with("\"")
                                                                    && expr.ends_with("\"")
                                                                {
                                                                    ex = true;
                                                                    // println!(
                                                                    //     "matched txt in expt : {}",
                                                                    //     expr
                                                                    //);
                                                                } else {
                                                                    for i in vrs.clone() {
                                                                        if i.name == expr {
                                                                            ex = true;
                                                                            //    println!("matched var in expt : {}", expr);
                                                                            break;
                                                                            // Exit the loop once a match is found
                                                                        }
                                                                    }
                                                                }
                                                                if !ex {
                                                                    println!(
                                                                        "{}{}{}{}{}",
                                                                        "ERR - THE ARGUMENT : "
                                                                            .red(),
                                                                        expr.red(),
                                                                        " : is invalid at line : "
                                                                            .red(),
                                                                        line.trim().red(),
                                                                        " :".red()
                                                                    );
                                                                    exit(0);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                None => {
                                                    println!("ERR - The 'sub()' method has invalid amount of arguments.");
                                                    exit(0);
                                                }
                                            }
                                        } else {
                                            println!("{}{}","Invalid amount of arguments found in 'sub()' method - ".red(),line.trim().red());
                                            exit(0);
                                        }
                                    }
                                    Err(err) => {
                                        println!(
                                            "{} {}",
                                            "Unable to make regex pattern for sub method - ".red(),
                                            err.to_string().red()
                                        );
                                    }
                                }
                            } else if line.trim().starts_with("may") {
                                println!(
                                    "{}{}",
                                    "Handling 'variables' - ".green(),
                                    line.trim().green()
                                );

                                let vardecltrg =
                                    Regex::new(r#"may\s+(\w+)\s*=\s*(.+)\s*;"#).unwrap();
                                if let Some(cap) = vardecltrg.captures(line.trim()) {
                                    let varnm = cap.get(1).unwrap().as_str().to_string();
                                    let varval = cap.get(2).unwrap().as_str().to_string();
                                    // Replace this part within your code
                                    //let mut val = String::new();
                                    let vartype =
                                        if varval.starts_with('"') && varval.ends_with('"') {
                                            Vartypes::String
                                        } else if varval.parse::<i32>().is_ok() {
                                            Vartypes::I
                                        } else if varval.parse::<f32>().is_ok() {
                                            Vartypes::Fsf
                                        } else {
                                            if varval.contains("+")
                                                || varval.contains("+")
                                                || varval.contains("-")
                                                || varval.contains("*")
                                                || varval.contains("/")
                                            {
                                                println!(
                                                    "{}{}",
                                                    "Please use arithematic functions for math : "
                                                        .red(),
                                                    line.trim().red()
                                                );
                                                exit(0);
                                            } else {
                                                println!(
                                                    "{}{}{}{}{}",
                                                    "invalid variable type : ".red(),
                                                    varval.red(),
                                                    " : in variable declaration : ".red(),
                                                    line.trim(),
                                                    " :".red()
                                                );
                                                exit(0);
                                            }
                                        };

                                    let var = Varr {
                                        name: varnm,
                                        vtype: vartype,
                                        vval: varval.clone(),
                                    };

                                    vrs.push(var.clone());
                                    Varr::dis(var);
                                } else {
                                    println!("{}", "Unable to parse variable declaration".red());
                                    exit(0);
                                }
                            } else if line.trim().starts_with("echoln") {
                                println!(
                                    "{}{}",
                                    "Handling 'echoln' - ".green(),
                                    line.trim().green()
                                );
                                let enlrg = Regex::new(r#"echoln\((.*?)\)\;"#).unwrap();
                                if let Some(cap) = enlrg.captures(line) {
                                    if let Some(text) = cap.get(1) {
                                        let text = text.as_str();
                                        let txt = text.split(',');
                                        for text in txt {
                                            let text = text.trim();
                                            if text.starts_with('"') && text.ends_with('"') {
                                                println!(
                                                    "{}{}",
                                                    "Echoing literal: ".cyan(),
                                                    text.cyan()
                                                );
                                            } else {
                                                let mut found = false;
                                                for var in vrs.iter() {
                                                    if var.name == text {
                                                        found = true;
                                                        println!(
                                                            "{}{}",
                                                            "Echoing variable: ".cyan(),
                                                            text.cyan()
                                                        );
                                                        break;
                                                    }
                                                }
                                                if !found {
                                                    println!(
                                                        "{}{}{}{}",
                                                        "Variable not found in scope: ".red(),
                                                        text.red(),
                                                        " in echonl statement: ".red(),
                                                        line.trim().red()
                                                    );
                                                    exit(0);
                                                }
                                            }
                                        }
                                    } else {
                                        println!(
                                            "ERROR - Could not capture text inside echonl in line: {}",
                                            line
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Invalid 'echoln()' syntax :: ".red(),
                                        line.trim().red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                            } else if line.trim() == "out.flush();" {
                                println!("{} {}", "buffer flusher called here : ", line.trim());
                            } else if line.trim().starts_with("echo") {
                                println!("{}{}", "Handling 'echo' - ".green(), line.trim().green());

                                let enlrg = Regex::new(r#"echo\((.*?)\)\;"#).unwrap();
                                if let Some(cap) = enlrg.captures(line) {
                                    if let Some(text) = cap.get(1) {
                                        let text = text.as_str();
                                        let txt = text.split(',');
                                        for text in txt {
                                            let text = text.trim();
                                            if text.starts_with('"') && text.ends_with('"') {
                                                println!(
                                                    "{}{}",
                                                    "Echoing literal: ".cyan(),
                                                    text.cyan()
                                                );
                                            } else {
                                                let mut found = false;
                                                for var in vrs.iter() {
                                                    if var.name == text {
                                                        found = true;
                                                        println!(
                                                            "{}{}",
                                                            "Echoing variable: ".cyan(),
                                                            text.cyan()
                                                        );
                                                        break;
                                                    }
                                                }
                                                if !found {
                                                    println!(
                                                        "{}{}{}{}",
                                                        "Variable not found in scope: ".red(),
                                                        text.red(),
                                                        " in echo statement: ".red(),
                                                        line.trim().red()
                                                    );
                                                    exit(0);
                                                }
                                            }
                                        }
                                    } else {
                                        println!(
                                            "ERROR - Could not capture text inside echol in line: {}",
                                            line
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Invalid 'echo()' syntax :: ".red(),
                                        line.trim().red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                            } else if line.trim().starts_with("#") {
                                if !isinfn {
                                    println!(
                                        "{}{}",
                                        "Comment Not going in final build : ",
                                        line.trim()
                                    );
                                } else {
                                    println!(
                                        "{}{}",
                                        "Comment Not going in final build : ",
                                        line.trim()
                                    );
                                }
                            } else if line.trim().is_empty() {
                                continue;
                            } else {
                                let mut found_function_call = false;
                                for i in fns.iter() {
                                    if line.trim().starts_with(&(i.clone() + "();")) {
                                        println!(
                                            "{}{}",
                                            "Handling function call: ".green(),
                                            line.trim().green()
                                        );
                                        found_function_call = true;
                                        break;
                                    }
                                }

                                if !found_function_call {
                                    undefined_fn_calls.push(line.trim().to_string());
                                    println!(
                                        "{}{} :",
                                        "Undefined function call found, will recheck later: "
                                            .yellow(),
                                        line.trim().yellow()
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}{}{}{}",
                            "Error Opening main file in the project: ".red(),
                            project_folder,
                            " : ERR - ".red(),
                            err.to_string().red()
                        );
                    }
                }
                for undefined_fn_call in &undefined_fn_calls {
                    let mut found = false;
                    for func in &fns {
                        if undefined_fn_call.starts_with(&(func.clone() + "();")) {
                            found = true;
                            println!(
                                "{} {} {} :",
                                "function call declared fixing stuff...: ",
                                func.clone(),
                                undefined_fn_call
                            );
                            break;
                        }
                    }
                    if !found {
                        println!(
                            "{}{}",
                            "ERROR - Undefined function call found: ".red(),
                            undefined_fn_call.red()
                        );
                        exit(1);
                    }
                }

                for i in vrs.clone() {
                    Varr::dis(i);
                }

                let cd = wc;
                let bcd = cd.as_bytes();
                let tmpfol = DirBuilder::new();
                if Path::exists(Path::new("./tmp/vstartups.txt")) {
                    remove_file("./tmp/vstartups.txt").unwrap();
                }
                if Path::exists(Path::new("./tmp")) {
                    remove_dir("./tmp").unwrap();
                }

                match tmpfol.create("./tmp") {
                    Ok(_tmpfol) => {
                        let tempfol = "./tmp";
                        match File::create(tempfol.to_owned() + "/vstartups.txt") {
                            Ok(mut tf) => {
                                match tf.write_all(bcd) {
                                    Ok(_m) => {
                                        let mut lcd = String::new();
                                        let mut f =
                                            File::open(tempfol.to_owned() + "/vstartups.txt")
                                                .unwrap();
                                        f.read_to_string(&mut lcd).unwrap();
                                        //dbg!(lcd.clone());

                                        if Path::exists(Path::new(
                                            &(project_folder.to_owned() + "/cfg.bcf"),
                                        )) {
                                            match File::open(project_folder.to_owned() + "/cfg.bcf")
                                            {
                                                Ok(mut cfgf) => {
                                                    let mut cfgs = String::new();
                                                    cfgf.read_to_string(&mut cfgs).unwrap();
                                                    //println!("\n\ncfg : {}",cfgs.trim());
                                                    let cfg = cfgs.split("\n");
                                                    let mut c = CFG {
                                                        name: String::new(),
                                                        date: String::new(),
                                                        auth: String::new(),
                                                    };
                                                    for cfg in cfg {
                                                        if cfg.starts_with("NAME") {
                                                            let i = cfg.split(":");
                                                            for m in i {
                                                                if m != "NAME" {
                                                                    c.name = m.trim().to_string();
                                                                }
                                                            }
                                                        } else if cfg.starts_with("DATE") {
                                                            let i = cfg.split(":");
                                                            for m in i {
                                                                if m != "DATE" {
                                                                    c.date = m.trim().to_string();
                                                                }
                                                            }
                                                        } else if cfg.starts_with("AUTHORS") {
                                                            let i = cfg.split(":");
                                                            for m in i {
                                                                if m != "DATE" {
                                                                    c.auth = m.trim().to_string();
                                                                }
                                                            }
                                                        } else {
                                                            continue;
                                                        }
                                                    }
                                                    let mut newcd = String::new();
                                                    for i in lcd.clone().split("\n") {
                                                        let i = i.trim();
                                                        //dbg!(i);
                                                        if !i.starts_with("#") {
                                                            newcd.push_str(
                                                                format!("\n{}", i).as_str(),
                                                            );
                                                        }
                                                    }
                                                    let lcd = newcd;
                                                    //dbg!("lcd : {}", lcd.clone());
                                                    // After creating `topacc`
                                                    let topacc =
                                                        format!("{}@{}", CFG::gen(c.clone()), lcd);
                                                    //let fd = topacc.clone().into_bytes(); // Convert to bytes using into_bytes()

                                                    // Clear temporary files and directory
                                                    fs::remove_file(
                                                        tempfol.to_owned() + "/vstartups.txt",
                                                    )
                                                    .unwrap();
                                                    fs::remove_dir(tempfol.to_owned()).unwrap();

                                                    // Create BXE file and write bytes to it
                                                    let bxef = format!(
                                                        "{}/{}.bxe",
                                                        project_folder, c.name
                                                    );
                                                    let mut bindat = String::new();
                                                    for i in topacc.into_bytes() {
                                                        if i == u8::MAX {
                                                            bindat +=
                                                                &format!("0{:b}`", i).to_string();
                                                        } else {
                                                            bindat += &format!(
                                                                "0{:b}`",
                                                                i.wrapping_add(1)
                                                            )
                                                            .to_string();
                                                        }
                                                    }
                                                    match File::create(&bxef) {
                                                        Ok(mut bxe) => match bxe
                                                            .write_all(&bindat.as_bytes())
                                                        {
                                                            Ok(_) => println!(
                                                                "Successfully wrote to BXE file."
                                                            ),
                                                            Err(err) => println!(
                                                                "{} - {}",
                                                                "Error writing to BXE file:".red(),
                                                                err.to_string().red()
                                                            ),
                                                        },
                                                        Err(err) => {
                                                            println!("{} - {}", "Unable to create BXE (Bimble executable) file, err - ".red(), err.to_string().red());
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    println!("{} {}","unable to open/find config file named 'cfg.cfg' in project folder - ",project_folder);
                                                    println!(
                                                        "{}{}",
                                                        "err - ".red(),
                                                        err.to_string().red()
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        println!(
                                            "{} {}",
                                            "err writting temp data : err - ",
                                            err.to_string()
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                println!("{} {}", "err making temp file - : ", err.to_string());
                            }
                        }
                    }
                    Err(err) => {
                        println!("{} {}", "err making temp folder - : ", err.to_string())
                    }
                }
            }
            Err(err) => {
                if pfci != 0 {
                    println!(
                        "{}{}",
                        "Error opening file 'main.bb' in project folder provided! \nerr - ".red(),
                        err.to_string().red()
                    );
                    exit(-1);
                } else {
                    pfci += 1;
                }
            }
        }
    }

    println!("{}", "Build successful!".green());
}
