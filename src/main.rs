//
// Changed to read bk first and hd child
//
use std::path::{Path};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process::Command as stdCommand;
use std::time::Instant as timeInstant;
use chrono::Local;

fn main()  {
    let mut bolok: bool = true;
    let bkrows_num: u64;
    let hdrows_num: u64;
    let exrows_num: u64;
    let mut parm1dir = String::new();
    let mut parm2dir = String::new();
    let mut parm3dir = String::new();
    let mut vecexcludef: Vec<String> = Vec::new();
    let mut vecexcluded: Vec<String> = Vec::new();
    let mut linestrtnum: u64 = 1;

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!(" no input parameters");
        bolok = false;
    } else {
        println!("The first argument is {}", args[1]);
        if args.len() < 3 {
            println!("The Only first argument and no other arguments");
            bolok = false;
        } else {
            println!("The second argument is {}", args[2]);
            if args.len() < 4 {
                println!("The Only first and second arguments and no other arguments");
                bolok = false;
            } else {
                println!("The third argument is {}", args[3]);
                if Path::new(&args[1]).exists() {
                    println!("The first argument {} exists", args[1]);
                    parm1dir = args[1].to_string();                    
                    let outputy = stdCommand::new("wc")
                         .arg("-l")
                         .arg(&parm1dir)
                         .output()
                         .expect("failed to execute process");
                    let strouty = String::from_utf8_lossy(&outputy.stdout);
                    let vecout: Vec<&str> = strouty.split(" ").collect();
                    let numlinesy: i64 = vecout[0].parse().unwrap_or(-9999);
                    if numlinesy == -9999 {
                        println!("size of {} is invalid for wc -l command call", vecout[0]);
                        bolok = false;
                    } else {
                        bkrows_num = numlinesy as u64;
                        if bkrows_num < 10 {
                            println!("size of {} is less than 10 for {}", bkrows_num, parm1dir);
                            bolok = false;
                        } else {
                            let filey = File::open(parm1dir.clone()).unwrap();
                            let mut readery = BufReader::new(filey);
                            let mut linebk = String::new();
                            let mut linenumy: u64 = 0;                            
                            loop {
                               match readery.read_line(&mut linebk) {
                                  Ok(bytes_read) => {
                                     if bytes_read == 0 {
                                         println!("bytes_read == 0 for {}", parm1dir);
                                         bolok = false;
                                         break;
                                     }
                                     linenumy = linenumy + 1;
                                     if linenumy == 1 {
                                         if linebk.trim().to_string() == "refname|filename|dirname|filesize|filedate|md5sum|locations|notes".to_string() {
                                             println!("bk file is ok with size of {} rows", bkrows_num);
                                             break;
                                         } else {
                                             println!("first line of hd file is not valid: {}", linebk);
                                             bolok = false;
                                             break;
                                         }
                                     }         
                                  }
                                  Err(err) => {  
                                     println!("error of {} reading {}", err, parm1dir);
                                     bolok = false;
                                     break;
                                  }
                               };
                            }
                        }
                    }
                } else {
                    println!("The first argument {} does not exist", args[1]);
                    bolok = false;
                }
                if !Path::new(&args[2]).exists() {
                    println!("The second argument {} does not exist", args[2]);
                    bolok = false;
                } else {
                    println!("The second argument {} exists", args[2]);
                    parm2dir = args[2].to_string();
                    let outputx = stdCommand::new("wc")
                         .arg("-l")
                         .arg(&parm2dir)
                         .output()
                         .expect("failed to execute process");
                    let stroutx = String::from_utf8_lossy(&outputx.stdout);
                    let vecout: Vec<&str> = stroutx.split(" ").collect();
                    let numlinesx: i64 = vecout[0].parse().unwrap_or(-9999);
                    if numlinesx == -9999 {
                        println!("size of {} is invalid for wc -l command call", vecout[0]);
                        bolok = false;
                    } else {
                        hdrows_num = numlinesx as u64;
                        if hdrows_num < 2 {
                            println!("size of {} is less than 2 for {}", hdrows_num, parm2dir);
                            bolok = false;
                        } else {
                            let file = File::open(parm2dir.clone()).unwrap();
                            let mut reader = BufReader::new(file);
                            let mut linehd = String::new();
                            let mut linenum: u64 = 0;
                            loop {
                               match reader.read_line(&mut linehd) {
                                  Ok(bytes_read) => {
                                     if bytes_read == 0 {
                                         println!("bytes_read == 0 for {}", parm2dir);
                                         bolok = false;
                                         break;
                                     }
                                     linenum = linenum + 1;
                                     if linenum == 1 {
                                         if linehd.trim().to_string() == "filename|filesize|filedate|dirname|refname|md5sum|locations|notes".to_string() {
                                             println!("hd file is ok with size of {} rows", hdrows_num);
                                             break;
                                         } else {
                                             println!("first line of hd file is not valid: {}", linehd);
                                             bolok = false;
                                             break;
                                         }
                                     } else {
                                         println!("linenum after 1 for {}", parm2dir);
                                         break;
                                     }
                                  }
                                  Err(err) => {  
                                     println!("error of {} reading {}", err, parm2dir);
                                     bolok = false;
                                     break;
                                  }
                               };
                            }
                        }
                    }
                }
                if !Path::new(&args[3]).exists() {
                    println!("The third argument {} does not exist", args[3]);
                    bolok = false;
                } else {
                    println!("The third argument {} exists", args[3]);
                    parm3dir = args[3].to_string();
                    let outputy = stdCommand::new("wc")
                         .arg("-l")
                         .arg(&parm3dir)
                         .output()
                         .expect("failed to execute process");
                    let strouty = String::from_utf8_lossy(&outputy.stdout);
                    let vecouty: Vec<&str> = strouty.split(" ").collect();
                    let numlinesy: i64 = vecouty[0].parse().unwrap_or(-9999);
                    if numlinesy == -9999 {
                        println!("size of {} is invalid for wc -l command call", vecouty[0]);
                        bolok = false;
                    } else {
                        exrows_num = numlinesy as u64;
                        if exrows_num < 2 {
                            println!("size of {} is less than 2 for {}", exrows_num, parm3dir);
                            bolok = false;
                        } else {
                            let filey = File::open(parm3dir.clone()).unwrap();
                            let mut readery = BufReader::new(filey);
                            let mut lineex = String::new();
                            let mut linenumy: u64 = 0;
                            loop {
                               match readery.read_line(&mut lineex) {
                                  Ok(bytes_read) => {
                                     if bytes_read == 0 {
                                         println!("exclude file is has no records");
                                         bolok = false;
                                         break;
                                     }
                                     linenumy = linenumy + 1;
                                     if linenumy == 1 {
                                         if lineex.trim().to_string() == "exclude file".to_string() {
                                             println!("exclude file is ok");
                                         } else {
                                             println!("first line of exclude file is not valid: {}", lineex);
                                             bolok = false;
                                         }
                                     } else {
                                         break;
                                     }
                                  }
                                  Err(err) => {  
                                     println!("error of {} reading {}", err, parm3dir);
                                     bolok = false;
                                     break;
                                  }
                               };
                            }
                        }
                    }
                    if args.len() > 4 {
                       let arg4 = args[4].to_string();
                       let numarg4: i64 = arg4.parse().unwrap_or(-9999);
                       if numarg4 < 2 {
                           println!("argument 4 is not valid value: {}", arg4);
                           bolok = false;
                       } else {
                           println!("argument 4 is valid value: {}", arg4);
                           linestrtnum = numarg4 as u64;
                       }
                    }
                }
            }
        }
    }
    if bolok {
        let fileex = File::open(parm3dir.clone()).unwrap();
        let mut readerex = BufReader::new(fileex);
        let mut lineex = String::new();
        let mut lineexnum: u64 = 0;
        loop {
              match readerex.read_line(&mut lineex) {
                 Ok(bytes_read) => {
                     if bytes_read == 0 {
                         break;
                     }
                     lineexnum = lineexnum + 1;
                     if lineexnum > 1 {
                         let excl: String = lineex.trim().to_string();
                         if excl.len() < 3 {
                             println!("exclusion less than 3 characters: {}", excl);
                             bolok = false;
                             break;
                         } else {
                             let exclv: String = excl[2..].to_string();
//                             println!("exclusion value:-{}-", exclv);
                             if excl[..2].to_string() == "d ".to_string() {
                                 vecexcluded.push(exclv);
                             } else if excl[..2].to_string() == "f ".to_string() {
                                 vecexcludef.push(exclv);
                             } else {
                                 println!("exclusion invalid first two characters {}", excl);
                                 bolok = false;
                                 break;
                             }
                         }   
                     }
                     lineex.clear();
                 }
                 Err(err) => {
                     println!("error {} when reading exclusion file", err);
                     bolok = false;   
                     break;
                 }
              };
        }
        if lineexnum < 2 {
            println!("exclusion file {} has no records", parm3dir);
            bolok = false;
        } else {
            lineexnum = lineexnum - 1;
            println!("exclusion file {} has {} records", parm3dir, lineexnum);
        }
    }
    if bolok {
        let mut outseq: u32 = 1;
        let mut more1out: String = format!("./more1{:02}.excout", outseq);
        let mut just1out: String = format!("./just1{:02}.neout", outseq);
        let mut excludout: String = format!("./excluded{:02}.excout", outseq);
        let mut nobkupout: String = format!("./nobkup{:02}.neout", outseq);
        let mut errout: String = format!("./generrors{:02}.errout", outseq);
        loop {
               if !Path::new(&errout).exists() && !Path::new(&more1out).exists() && !Path::new(&just1out).exists()
                  && !Path::new(&excludout).exists() && !Path::new(&nobkupout).exists() {
                   break;
               } else {
                   outseq = outseq + 1;
                   more1out = format!("./more1{:02}.excout", outseq);
                   just1out = format!("./just1{:02}.neout", outseq);
                   excludout = format!("./excluded{:02}.excout", outseq);
                   nobkupout = format!("./nobkup{:02}.neout", outseq);
                   errout = format!("./generrors{:02}.errout", outseq);
               }
        }          
        let mut excludefile = File::create(excludout).unwrap();
        let mut nobkupfile = File::create(nobkupout).unwrap();
        let mut more1file = File::create(more1out).unwrap();
        let mut just1file = File::create(just1out).unwrap();
        let mut errfile = File::create(errout).unwrap();
        let filehd = File::open(parm2dir.clone()).unwrap();
        let mut readerhd = BufReader::new(filehd);
        let filebk = File::open(parm1dir.clone()).unwrap();
        let mut readerbk = BufReader::new(filebk);
        let mut linehd = String::new();
        let mut linehdfmt = String::new();
        let mut linebk = String::new();
        let mut linestatnum: u64 = 0;
        let mut linenumx: u64 = 0;
        let mut vecbksavefiles: Vec<String> = Vec::new();
        let mut bkmd5curr: String;
        let mut bkfilecurr: String;
        let mut bkmd5save: String = "none".to_string();
        let mut hdfilemd5: String = "none".to_string();
        let mut inptfilenm = String::new();
        let mut bolrdhd = true;
        let mut bolhdend = false;
        let start_time = timeInstant::now();
        let mut linenumbk: u64 = 0; 
        loop {
              if !bolok {
                  break;
              }
              if bolhdend {
                  break;
              }
              match readerbk.read_line(&mut linebk) {
                 Ok(bytes_read) => {
                     if bytes_read == 0 {
                         println!("{} files end of backup list", linenumbk);
                         break;
                     }
                     linenumbk = linenumbk + 1;
                     if linenumbk > 1 {
                         let veclinea: Vec<&str> = linebk.split("|").collect();
                         if veclinea.len() < 6 {
                             let stroutput = format!("invalid bk record {} line {}", linebk, linenumbk);
                             writeln!(&mut errfile, "{}", stroutput).unwrap();
                         } else {
                             let mut bkmd5a: String = veclinea[5].to_string();
                             if bkmd5a.len() > 32 {
                                 if bkmd5a[..1].to_string() == '"'.to_string() {
                                     bkmd5a = bkmd5a[1..33].to_string();
                                 } else {
                                     bkmd5a = bkmd5a[..32].to_string();
                                 }
                             }
                             let mut bkfilenma: String = veclinea[1].to_string();
                             if bkfilenma[..1].to_string() == '"'.to_string() {
                                 bkfilenma = bkfilenma[1..(bkfilenma.len()-1)].to_string();
                             }
                             if linenumbk == 2 {
                                 bkmd5save = bkmd5a;
                                 vecbksavefiles.push(bkfilenma);
                             } else {
                                 if bkmd5save == bkmd5a {
                                     bkmd5save = bkmd5a;
                                     vecbksavefiles.push(bkfilenma);
                                 } else {
                                     bkmd5curr = bkmd5a;
                                     bkfilecurr = bkfilenma;
                                     loop {
                                           if !bolok {
                                               break;
                                           }
                                           if bolrdhd {
                                               match readerhd.read_line(&mut linehd) {
                                                  Ok(bytes_read) => {
                                                      bolrdhd = false;
                                                      if bytes_read == 0 {
                                                          let diffy = start_time.elapsed();
                                                          let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                                          let dateyy = Local::now();
                                                          println!("line number {} records elapsed time {:.1} mins at {} completed", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                                          bolhdend = true;
                                                          break;
                                                      }
                                                      linestatnum = linestatnum + 1;
                                                      linenumx = linenumx + 1;
                                                      if linenumx <= linestrtnum {
                                                          bolrdhd = true;
                                                      } else {
                                                          if linestatnum > 50000 {
                                                              let diffy = start_time.elapsed();
                                                              let minsy: f64 = diffy.as_secs() as f64/60 as f64;
                                                              let dateyy = Local::now();
                                                              println!("line number {} records elapsed time {:.1} mins at {}", linenumx, minsy, dateyy.format("%H:%M:%S"));
                                                              linestatnum = 0;
                                                          }
                                                          let vecline: Vec<&str> = linehd.split("|").collect();
                                                          hdfilemd5 = vecline[5].to_string();
                                                          if hdfilemd5.len() > 32 {
                                                              hdfilemd5 = hdfilemd5[..32].to_string();
                                                          }
                                                          linehdfmt = format!("{}|{}|{}|{}|{}|{}|{}", vecline[4], vecline[0], vecline[3], vecline[1], vecline[2], hdfilemd5, linenumx);
                                                          let inptdir = vecline[3].to_string();
                                                          inptfilenm = vecline[0].to_string();
                                                          if inptfilenm[..1].to_string() == '"'.to_string() {
                                                              inptfilenm = inptfilenm[1..(inptfilenm.len()-1)].to_string();
                                                          }
                                                          let mut bolex = false;
                                                          for strexclf in &vecexcludef {
                                                               if inptfilenm.contains(strexclf) {
                                                                   bolex = true;
                                                                   writeln!(&mut excludefile, "f {}", linehdfmt).unwrap();
                                                                   break;
                                                               }
                                                           }
                                                           if !bolex {
                                                               for strexcld in &vecexcluded {
                                                                    if inptdir.contains(strexcld) {
                                                                        bolex = true;
                                                                        writeln!(&mut excludefile, "d {}", linehdfmt).unwrap();
                                                                        break;
                                                                    }
                                                               }
                                                           }
                                                           if bolex {
                                                               bolrdhd = true;
                                                           }
                                                      }
                                                      linehd.clear()
                                                  }
                                                  Err(err) => {  
                                                      println!("error of {} reading {}", err, parm2dir);
                                                      bolok = false;
                                                      break;
                                                  }
                                               }
                                           }
                                           if !bolrdhd {
                                               if hdfilemd5 > bkmd5save {
                                                   writeln!(&mut nobkupfile, "1-{}", linehdfmt).unwrap();
                                                   bolrdhd = true;
                                               } else if hdfilemd5 == bkmd5save {
                                                   bolrdhd = true;
                                                   let mut nummatch = 0;
                                                   for bk in &vecbksavefiles {
                                                        if bk == &inptfilenm {
                                                            nummatch = nummatch + 1;
                                                        }
                                                   }
                                                   if nummatch < 1 {
                                                       writeln!(&mut nobkupfile, "2-{}", linehdfmt).unwrap();
                                                   } else if nummatch < 2 {
                                                       writeln!(&mut just1file, "2-{}", linehdfmt).unwrap();
                                                   } else {
                                                       writeln!(&mut more1file, "2-{}", linehdfmt).unwrap();
                                                   }
                                               } else {
                                                   vecbksavefiles.clear();
                                                   bkmd5save = bkmd5curr.clone();
                                                   vecbksavefiles.push(bkfilecurr.clone());
                                                   break;
                                               }
                                           }
                                     }  // end loop
                                 }
                             }
                         }
                     }
                     linebk.clear();
                 }
                 Err(err) => {
                     let stroutput = format!("error of {} reading {}", err, parm1dir);
                     println!("{}", stroutput);
                     writeln!(&mut errfile, "{}", stroutput).unwrap();
                     break;
                 }
              };
        }
        println!("{} files", (linenumx-1));
    }
}
