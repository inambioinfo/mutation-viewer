#[macro_use]
extern crate rouille;
use rouille::{Response, start_server};

#[macro_use]
extern crate horrorshow;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use std::fs::{read_dir, ReadDir};
use std::path::Path;

const PORT: &'static str = "8080";
const URL: &'static str = "localhost";

fn do_read_dir(p: &Path) -> ReadDir {
    read_dir(p).unwrap()
}

struct dataset_struct {}

fn main () {

    let port: &str = &PORT[..];
    let url: &str = &URL[..];

    let addr : String = format!("{}:{}", url, port);

    println!("Server listening at {}", &addr);

    start_server(addr, move |request| {

        router!(request,
            (GET)(/) => {

                let paths = do_read_dir(Path::new("/home/ftabaro/IdeaProjects/RustyVCF/data"));

                for path in paths {
                    let unwrapped_path = path.unwrap();
                    let files = do_read_dir(&unwrapped_path.path());
                    for file in files {
                        println!("{}\t{}", &unwrapped_path.file_name().into_string().unwrap(), file.unwrap().path().display())
                    }
                }

                Response::html(html!{
                    : doctype::HTML;
                    html {
                        head {
                            title : "Betastasis mutation viewer";
                        }
                        body {
                            // attributes
                            h1(id="heading") {
                                // Insert escaped text
                                : "Mutation viewer"
                            }
                            h2 {
                                : "Select dataset"
                            }
//                            p {
//                                // Insert raw text (unescaped)
//                                : Raw("Let's <i>count</i> to 10!")
//                            }
                            ul(id="dataset-list") {
                                // You can embed for loops, while loops, and if statements.
//                                @ for (dataset_path, dataset_files) in available_datasets {
//                                    li {
//                                        : format_args!("{}", dataset_path);
//
//                                        a(href="#") {
//                                            // Format some text.
//                                            : format_args!("{}", dataset_files)
//
//                                        }
//                                    }
//                                }
                            }
                            // You need semi-colons for tags without children.
//                            br; br;
//                            p {
//                                // You can also embed closures.
//                                |tmpl| {
//                                    tmpl << "Easy!";
//                                }
//                            }
                        }
                    }
                }.into_string().unwrap())
            },

            (GET)(/{dataset: String}/{vcf: String}/mutations) => {
                println!("{}/{}",&dataset, &vcf);
                Response::text("this is the view page")
            },

            (POST)(/update_mutation_blacklist) => {
                Response::text("post to update_blacklist")
            },

            _ => Response::empty_404()
        )
    });
}

