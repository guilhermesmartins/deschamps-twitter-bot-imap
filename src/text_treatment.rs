//O(n^2 + 3n)
pub fn text_treatment(mut arr: std::vec::Vec<std::string::String>) -> Vec<std::string::String> {
    let mut iter = 0;
    for text in arr.clone() {
        if text.contains("Notícias que chamaram a nossa atenção nesta") {
            let mut i = 0;
            while i < iter {
                arr.remove(i);
                i = i + 1;
            }
        }

        iter = iter + 1;
    }

    arr.retain(|text| text.chars().count() > 1500 );

    //let text = arr[0].split("*\n\n*");

    //split the str by the *\n\n*, and create a new news array 
    //by the split \n\n* from this one
    let pure_text = arr[0].split("*\n\n*");

    let mut unformated_text = "_";

    for i in pure_text {
        if i.chars().count() > 100 {
            unformated_text = i;
            break;
        }
    }

    let news = unformated_text.split("\n\n*");

    let mut formated_news: Vec<std::string::String> = Vec::new();//Vec::with_capacity(20);

    for info in news {
        println!("{}\n", info);
        formated_news.push(info.replace("\n", " ").replace("*", ""));
    }

    formated_news
}