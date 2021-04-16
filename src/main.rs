fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("iftar v0.1.0 by omerakgoz34");
        println!("Kullanım: iftar <şehir>");
        return
    }

    let url_diyanet = "https://namazvakitleri.diyanet.gov.tr/tr-TR/";
    let mut cities = std::collections::HashMap::new();

    cities.insert("istanbul", "9541");


    let mut city_code = "0";
    match cities.get(&args[1][..]) {
        Some(&city) => city_code = city,
        _ => println!("HATA: Girdiğiniz şehir bulunamadı!"),
    }
    if city_code == "0" { return }

    let mut url: String = url_diyanet.to_owned();
    url.push_str(city_code);
    let response = minreq::get(url).send().unwrap();

    let fragment = scraper::Html::parse_fragment(response.as_str().unwrap());
    let selector = scraper::Selector::parse("div.tpt-cell[data-vakit-name='aksam'] .tpt-time").unwrap();

    println!("{}: {}", args[1], fragment.select(&selector).next().unwrap().inner_html());
}
